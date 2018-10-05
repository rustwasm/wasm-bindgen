#[macro_use]
extern crate failure;
extern crate parity_wasm;

use std::collections::HashMap;

use failure::{Error, ResultExt};
use parity_wasm::elements::*;

const PAGE_SIZE: u32 = 1 << 16;

/// Configuration for the transformation pass in this module.
///
/// Created primarily through `new` and then executed through `run`.
pub struct Config {
    maximum_memory: u32,
    thread_stack_size: u32,
}

impl Config {
    /// Create a new configuration with default settings.
    pub fn new() -> Config {
        Config {
            maximum_memory: 1 << 30, // 1GB
            thread_stack_size: 1 << 20, // 1MB
        }
    }

    /// Specify the maximum amount of memory the wasm module can ever have.
    ///
    /// We'll be specifying that the memory for this wasm module is shared, and
    /// all shared memories must have their maximum limit specified (whereas
    /// by default Rust/LLVM/LLD don't specify a maximum).
    ///
    /// The default for this option is 16MB, and this can be used to change
    /// the maximum memory we'll be specifying.
    ///
    /// The `max` argument is in units of bytes.
    ///
    /// If the maximum memory is already specified this setting won't have any
    /// affect.
    pub fn maximum_memory(&mut self, max: u32) -> &mut Config {
        self.maximum_memory = max;
        self
    }

    /// Specify the stack size for all threads spawned.
    ///
    /// The stack size is typically set by rustc as an argument to LLD and
    /// defaults to 1MB for the main thread. All threads spawned by the
    /// main thread, however, need to allocate their own stack!
    ///
    /// This configuration option indicates how large the stack of each child
    /// thread will be. This will be allocated as part of the `start` function
    /// and will be stored in LLVM's global stack pointer.
    pub fn thread_stack_size(&mut self, size: u32) -> &mut Config {
        self.thread_stack_size = size;
        self
    }

    /// Execute the transformation on the parsed wasm module specified.
    ///
    /// This function will prepare `Module` to be run on multiple threads,
    /// performing steps such as:
    ///
    /// * All data segments are switched to "passive" data segments to ensure
    ///   they're only initialized once (coming later)
    /// * If memory is exported from this module, it is instead switched to
    ///   being imported (with the same parameters).
    /// * The imported memory is required to be `shared`, ensuring it's backed
    ///   by a `SharedArrayBuffer` on the web.
    /// * A `global` for a thread ID is injected.
    /// * Four bytes in linear memory are reserved for the counter of thread
    ///   IDs.
    /// * A `start` function is injected (or prepended if one already exists)
    ///   which initializes memory for the first thread and otherwise allocates
    ///   thread ids for all threads.
    ///
    /// More and/or less may happen here over time, stay tuned!
    pub fn run(&self, module: &mut Module) -> Result<(), Error> {
        let segments = switch_data_segments_to_passive(module)?;
        import_memory_zero(module)?;
        share_imported_memory_zero(module, self.maximum_memory)?;
        let stack_pointer_idx = find_stack_pointer(module)?;
        let globals = inject_thread_globals(module);
        let addr = inject_thread_id_counter(module)?;
        start_with_init_memory(
            module,
            &segments,
            &globals,
            addr,
            stack_pointer_idx,
            self.thread_stack_size,
        );
        implement_thread_intrinsics(module, &globals)?;
        Ok(())
    }
}

struct PassiveSegment {
    idx: u32,
    offset: u32,
    len: u32,
}

fn switch_data_segments_to_passive(module: &mut Module)
    -> Result<Vec<PassiveSegment>, Error>
{
    // If there's no data, nothing to make passive!
    let section = match module.data_section_mut() {
        Some(section) => section,
        None => return Ok(Vec::new()),
    };

    let mut ret = Vec::new();
    for (i, segment) in section.entries_mut().iter_mut().enumerate() {
        let mut offset = match segment.offset_mut().take() {
            Some(offset) => offset,
            // already passive ...
            None => continue,
        };
        assert!(!segment.passive());

        let offset = *get_offset(&mut offset)
            .with_context(|_| format!("failed to read data segment {}", i))?;

        // Flag it as passive after validation, and we've removed the offset via
        // `take`, so time to process the next one
        *segment.passive_mut() = true;
        ret.push(PassiveSegment {
            idx: i as u32,
            offset: offset as u32,
            len: segment.value().len() as u32,
        });
    }

    Ok(ret)
}

fn get_offset(offset: &mut InitExpr) -> Result<&mut i32, Error> {
    if offset.code().len() != 2 || offset.code()[1] != Instruction::End {
        bail!("unrecognized offset")
    }
    match &mut offset.code_mut()[0] {
        Instruction::I32Const(n) => Ok(n),
        _ => bail!("unrecognized offset"),
    }
}

fn import_memory_zero(module: &mut Module)
    -> Result<(), Error>
{
    // If memory is exported, let's switch it to imported. If memory isn't
    // exported then there's nothing to do as we'll deal with importing it
    // later.
    let limits = {
        let section = match module.memory_section_mut() {
            Some(section) => section,
            None => return Ok(()),
        };
        let limits = match section.entries_mut().pop() {
            Some(limits) => limits,
            None => return Ok(()),
        };
        if section.entries().len() > 0 {
            bail!("too many memories in wasm module for this tool to work");
        }
        limits
    };

    // Remove all memory sections as well as exported memory, we're switching to
    // an import
    module.sections_mut().retain(|s| {
        match s {
            Section::Memory(_) => false,
            _ => true,
        }
    });
    if let Some(s) = module.export_section_mut() {
        s.entries_mut().retain(|s| {
            match s.internal() {
                Internal::Memory(_) => false,
                _ => true,
            }
        });
    }

    // Add our new import to the import section
    let pos = maybe_add_import_section(module);
    let imports = match &mut module.sections_mut()[pos] {
        Section::Import(s) => s,
        _ => unreachable!(),
    };

    // Hardcode the field names for now, these are all internal details anyway
    let entry = ImportEntry::new(
        "env".to_string(),
        "memory".to_string(),
        External::Memory(limits),
    );
    imports.entries_mut().push(entry);
    Ok(())
}

fn maybe_add_import_section(module: &mut Module) -> usize {
    let mut pos = None;
    // See this URL for section orderings, but the import section comes just
    // after the type section.
    //
    // https://github.com/WebAssembly/design/blob/master/BinaryEncoding.md#high-level-structure
    for i in 0..module.sections().len() {
        match &mut module.sections_mut()[i] {
            Section::Type(_) => continue,
            Section::Import(_) => return i,
            _ => {}
        }
        pos = Some(i);
        break
    }
    let empty = ImportSection::with_entries(Vec::new());
    let section = Section::Import(empty);
    let len = module.sections().len();
    let pos = pos.unwrap_or_else(|| len - 1);
    module.sections_mut().insert(pos, section);
    return pos
}

fn share_imported_memory_zero(module: &mut Module, memory_max: u32) -> Result<(), Error> {
    assert!(memory_max % PAGE_SIZE == 0);
    // NB: this function assumes `import_memory_zero` has been called first to
    // function correctly, which means we should find an imported memory here
    // which we can rewrite to be unconditionally shared.
    let imports = match module.import_section_mut() {
        Some(s) => s,
        None => panic!("failed to find an import section"),
    };

    for entry in imports.entries_mut() {
        let mem = match entry.external_mut() {
            External::Memory(m) => m,
            _ => continue,
        };
        *mem = MemoryType::new(
            mem.limits().initial(),
            Some(mem.limits().maximum().unwrap_or(memory_max / PAGE_SIZE)),
            true,
        );
        return Ok(())
    }
    panic!("failed to find an imported memory")
}

struct Globals {
    thread_id: u32,
    thread_tcb: u32,
}

fn inject_thread_globals(module: &mut Module) -> Globals {
    let pos = maybe_add_global_section(module);
    let globals = match &mut module.sections_mut()[pos] {
        Section::Global(s) => s,
        _ => unreachable!(),
    };

    // First up, our thread ID. The initial expression here isn't actually ever
    // used but it's required. All threads will start off by setting this
    // global to the thread's id.
    globals.entries_mut().push(GlobalEntry::new(
        GlobalType::new(ValueType::I32, true),
        InitExpr::new(vec![Instruction::I32Const(0), Instruction::End]),
    ));

    // Next up the thread TCB, this is always set to null to start off with.
    globals.entries_mut().push(GlobalEntry::new(
        GlobalType::new(ValueType::I32, true),
        InitExpr::new(vec![Instruction::I32Const(0), Instruction::End]),
    ));

    // ... and note that if either of the above globals isn't actually necessary
    // we'll gc it away later.

    let len = globals.entries().len() as u32;
    Globals {
        thread_id: len - 2,
        thread_tcb: len - 1,
    }
}

fn maybe_add_global_section(module: &mut Module) -> usize {
    let mut pos = None;
    // See this URL for section orderings:
    //
    // https://github.com/WebAssembly/design/blob/master/BinaryEncoding.md#high-level-structure
    for i in 0..module.sections().len() {
        match &mut module.sections_mut()[i] {
            Section::Type(_) |
            Section::Import(_) |
            Section::Function(_) |
            Section::Table(_) |
            Section::Memory(_) => continue,
            Section::Global(_) => return i,
            _ => {}
        }
        pos = Some(i);
        break
    }
    let empty = GlobalSection::with_entries(Vec::new());
    let section = Section::Global(empty);
    let len = module.sections().len();
    let pos = pos.unwrap_or_else(|| len - 1);
    module.sections_mut().insert(pos, section);
    return pos
}

fn inject_thread_id_counter(module: &mut Module) -> Result<u32, Error> {
    // First up, look for a `__heap_base` export which is injected by LLD as
    // part of the linking process. Note that `__heap_base` should in theory be
    // *after* the stack and data, which means it's at the very end of the
    // address space and should be safe for us to inject 4 bytes of data at.
    let heap_base = {
        let exports = match module.export_section() {
            Some(s) => s,
            None => bail!("failed to find `__heap_base` for injecting thread id"),
        };

        exports.entries()
            .iter()
            .filter(|e| e.field() == "__heap_base")
            .filter_map(|e| {
                match e.internal() {
                    Internal::Global(idx) => Some(*idx),
                    _ => None,
                }
            })
            .next()
    };
    let heap_base = match heap_base {
        Some(idx) => idx,
        None => bail!("failed to find `__heap_base` for injecting thread id"),
    };

    // Now we need to bump up `__heap_base` by 4 bytes as we'd like to reserve
    // those 4 bytes for our thread id counter. Do lots of validation here to
    // make sure that `__heap_base` is an non-mutable integer, and then do
    // some logic:
    //
    // * We require that `__heap_base` is aligned to 4 as that's what the atomic
    //   will require anyway.
    // * We *may* have to add another page to the minimum for this module. If by
    //   reserving 4 bytes the heap base now lies on a different page then we
    //   probably went past our minimum page requirement, so we'll need to
    //   update our memory limits to add one.
    //
    // Otherwise here we'll rewrite the `__heap_base` global's initializer to be
    // 4 larger, reserving us those 4 bytes for a thread id counter.
    let (address, add_a_page) = {
        let globals = match module.global_section_mut() {
            Some(s) => s,
            None => bail!("failed to find globals section"),
        };
        let entry = match globals.entries_mut().get_mut(heap_base as usize) {
            Some(i) => i,
            None => bail!("the `__heap_base` export index is out of bounds"),
        };
        if entry.global_type().content_type() != ValueType::I32 {
            bail!("the `__heap_base` global doesn't have the type `i32`");
        }
        if entry.global_type().is_mutable() {
            bail!("the `__heap_base` global is unexpectedly mutable");
        }
        let offset = get_offset(entry.init_expr_mut())?;
        let address = (*offset as u32 + 3) & !3; // align up
        let add_a_page = (address + 4) / PAGE_SIZE != address / PAGE_SIZE;
        *offset = (address + 4) as i32;
        (address, add_a_page)
    };

    if add_a_page {
        add_one_to_imported_memory_limits_minimum(module);
    }
    Ok(address)
}

// see `inject_thread_id_counter` for why this is used and where it's called
fn add_one_to_imported_memory_limits_minimum(module: &mut Module) {
    let imports = match module.import_section_mut() {
        Some(s) => s,
        None => panic!("failed to find import section"),
    };

    for entry in imports.entries_mut() {
        let mem = match entry.external_mut() {
            External::Memory(m) => m,
            _ => continue,
        };
        *mem = MemoryType::new(
            mem.limits().initial() + 1,
            mem.limits().maximum().map(|m| {
                if m == mem.limits().initial() {
                    m + 1
                } else {
                    m
                }
            }),
            mem.limits().shared(),
        );
        return;
    }
    panic!("failed to find an imported memory")
}

fn find_stack_pointer(module: &mut Module) -> Result<Option<u32>, Error> {
    let globals = match module.global_section() {
        Some(s) => s,
        None => bail!("failed to find the stack pointer"),
    };
    let candidates = globals.entries()
        .iter()
        .enumerate()
        .filter(|(_, g)| g.global_type().content_type() == ValueType::I32)
        .filter(|(_, g)| g.global_type().is_mutable())
        .collect::<Vec<_>>();

    // If there are no mutable i32 globals, assume this module doesn't even need
    // a stack pointer!
    if candidates.len() == 0 {
        return Ok(None)
    }

    // Currently LLVM/LLD always use global 0 as the stack pointer, let's just
    // blindly assume that.
    if candidates[0].0 == 0 {
        return Ok(Some(0))
    }

    bail!("the first global wasn't a mutable i32, has LLD changed or was \
           this wasm file not produced by LLD?")
}

fn start_with_init_memory(
    module: &mut Module,
    segments: &[PassiveSegment],
    globals: &Globals,
    addr: u32,
    stack_pointer_idx: Option<u32>,
    stack_size: u32,
) {
    assert!(stack_size % PAGE_SIZE == 0);
    let mut instrs = Vec::new();

    // Execute an atomic add to learn what our thread ID is
    instrs.push(Instruction::I32Const(addr as i32));
    instrs.push(Instruction::I32Const(1));
    let mem = parity_wasm::elements::MemArg { align: 2, offset: 0 };
    instrs.push(Instruction::I32AtomicRmwAdd(mem));

    // Store this thread ID into our thread ID global
    instrs.push(Instruction::TeeLocal(0));
    instrs.push(Instruction::SetGlobal(globals.thread_id));

    // Perform an if/else based on whether we're the first thread or not. Our
    // thread ID will be zero if we're the first thread, otherwise it'll be
    // nonzero (assuming we don't overflow...)
    //
    // In the nonzero case (the first block) we give ourselves a stack via
    // memory.grow and we update our stack pointer.
    //
    // In the zero case (the second block) we can skip both of those operations,
    // but we need to initialize all our memory data segments.
    instrs.push(Instruction::GetLocal(0));
    instrs.push(Instruction::If(BlockType::NoResult));

    if let Some(stack_pointer_idx) = stack_pointer_idx {
        // local0 = grow_memory(stack_size);
        instrs.push(Instruction::I32Const((stack_size / PAGE_SIZE) as i32));
        instrs.push(Instruction::GrowMemory(0));
        instrs.push(Instruction::SetLocal(0));

        // if local0 == -1 then trap
        instrs.push(Instruction::Block(BlockType::NoResult));
        instrs.push(Instruction::GetLocal(0));
        instrs.push(Instruction::I32Const(-1));
        instrs.push(Instruction::I32Ne);
        instrs.push(Instruction::BrIf(0));
        instrs.push(Instruction::Unreachable);
        instrs.push(Instruction::End); // end block

        // stack_pointer = local0 + stack_size
        instrs.push(Instruction::GetLocal(0));
        instrs.push(Instruction::I32Const(PAGE_SIZE as i32));
        instrs.push(Instruction::I32Mul);
        instrs.push(Instruction::I32Const(stack_size as i32));
        instrs.push(Instruction::I32Add);
        instrs.push(Instruction::SetGlobal(stack_pointer_idx));
    }

    instrs.push(Instruction::Else);
    for segment in segments {
        // offset into memory
        instrs.push(Instruction::I32Const(segment.offset as i32));
        // offset into segment
        instrs.push(Instruction::I32Const(0)); // offset into segment
        // amount to copy
        instrs.push(Instruction::I32Const(segment.len as i32));
        instrs.push(Instruction::MemoryInit(segment.idx));
    }
    instrs.push(Instruction::End); // endif

    // On all threads now memory segments are no longer needed
    for segment in segments {
        instrs.push(Instruction::MemoryDrop(segment.idx));
    }

    // If a start function previously existed we're done with our own
    // initialization so delegate to them now.
    if let Some(idx) = module.start_section() {
        instrs.push(Instruction::Call(idx));
    }

    // End the function
    instrs.push(Instruction::End);

    // Add this newly generated function to the code section ...
    let instrs = Instructions::new(instrs);
    let local = Local::new(1, ValueType::I32);
    let body = FuncBody::new(vec![local], instrs);
    let code_idx = {
        let s = module.code_section_mut().expect("module had no code");
        s.bodies_mut().push(body);
        (s.bodies().len() - 1) as u32
    };
    // ... and also be sure to add its signature to the function section ...
    let type_idx = {
        let section = module.type_section_mut().expect("module has no type section");
        let pos = section.types()
            .iter()
            .map(|t| {
                match t {
                    Type::Function(t) => t,
                }
            })
            .position(|t| t.params().is_empty() && t.return_type().is_none());
        match pos {
            Some(i) => i as u32,
            None => {
                let f = FunctionType::new(Vec::new(), None);
                section.types_mut().push(Type::Function(f));
                (section.types().len() - 1) as u32
            }
        }
    };
    module.function_section_mut()
        .expect("module has no function section")
        .entries_mut()
        .push(Func::new(type_idx));

    // ... and finally flag it as the new start function
    let idx = code_idx + (module.import_count(ImportCountType::Function) as u32);
    update_start_section(module, idx);
}

fn update_start_section(module: &mut Module, start: u32) {
    // See this URL for section orderings:
    //
    // https://github.com/WebAssembly/design/blob/master/BinaryEncoding.md#high-level-structure
    let mut pos = None;
    for i in 0..module.sections().len() {
        match &mut module.sections_mut()[i] {
            Section::Type(_) |
            Section::Import(_) |
            Section::Function(_) |
            Section::Table(_) |
            Section::Memory(_) |
            Section::Global(_) |
            Section::Export(_) => continue,
            Section::Start(start_idx) => {
                *start_idx = start;
                return
            }
            _ => {}
        }
        pos = Some(i);
        break
    }
    let section = Section::Start(start);
    let len = module.sections().len();
    let pos = pos.unwrap_or_else(|| len - 1);
    module.sections_mut().insert(pos, section);
}

fn implement_thread_intrinsics(
    module: &mut Module,
    globals: &Globals,
) -> Result<(), Error> {
    let mut map = HashMap::new();
    {
        let imports = match module.import_section() {
            Some(i) => i,
            None => return Ok(()),
        };
        let entries = imports.entries()
            .iter()
            .filter(|i| {
                match i.external() {
                    External::Function(_) => true,
                    _ => false,
                }
            })
            .enumerate()
            .filter(|(_, entry)| {
                entry.module() == "__wbindgen_thread_xform__"
            });
        for (idx, entry) in entries {
            let type_idx = match entry.external() {
                External::Function(i) => *i,
                _ => unreachable!(),
            };
            let types = module.type_section().unwrap();
            let fty = match &types.types()[type_idx as usize] {
                Type::Function(f) => f,
            };
            // Validate the type for this intrinsic
            match entry.field() {
                "__wbindgen_thread_id" => {
                    if !fty.params().is_empty() ||
                        fty.return_type() != Some(ValueType::I32)
                    {
                        bail!("__wbindgen_thread_id intrinsic has the wrong signature");
                    }
                    map.insert(idx as u32, Instruction::GetGlobal(globals.thread_id));
                }
                "__wbindgen_tcb_get" => {
                    if !fty.params().is_empty() ||
                        fty.return_type() != Some(ValueType::I32)
                    {
                        bail!("__wbindgen_tcb_get intrinsic has the wrong signature");
                    }
                    map.insert(idx as u32, Instruction::GetGlobal(globals.thread_tcb));
                }
                "__wbindgen_tcb_set" => {
                    if fty.params().len() != 1 || fty.return_type().is_some() {
                        bail!("__wbindgen_tcb_set intrinsic has the wrong signature");
                    }
                    map.insert(idx as u32, Instruction::SetGlobal(globals.thread_tcb));
                }
                other => bail!("unknown thread intrinsic: {}", other),
            }
        }
    };

    // Rewrite everything that calls `import_idx` to instead load the global
    // `thread_id`
    for body in module.code_section_mut().unwrap().bodies_mut() {
        for instr in body.code_mut().elements_mut() {
            let other = match instr {
                Instruction::Call(idx) => {
                    match map.get(idx) {
                        Some(other) => other,
                        None => continue,
                    }
                }
                _ => continue,
            };
            *instr = other.clone();
        }
    }

    // ... and in theory we'd remove `import_idx` here but we let `wasm-gc`
    // take care of that later.

    Ok(())
}
