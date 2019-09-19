use std::cmp;
use std::collections::HashMap;
use std::env;
use std::mem;

use failure::{bail, format_err, Error};
use walrus::ir::Value;
use walrus::{DataId, FunctionId, InitExpr, ValType};
use walrus::{ExportItem, GlobalId, GlobalKind, ImportKind, MemoryId, Module};
use wasm_bindgen_wasm_conventions as wasm_conventions;

const PAGE_SIZE: u32 = 1 << 16;

/// Configuration for the transformation pass in this module.
///
/// Created primarily through `new` and then executed through `run`.
pub struct Config {
    maximum_memory: u32,
    thread_stack_size: u32,
    enabled: bool,
}

impl Config {
    /// Create a new configuration with default settings.
    pub fn new() -> Config {
        Config {
            maximum_memory: 1 << 30,    // 1GB
            thread_stack_size: 1 << 20, // 1MB
            enabled: env::var("WASM_BINDGEN_THREADS").is_ok(),
        }
    }

    /// Is threaded Wasm enabled?
    pub fn is_enabled(&self) -> bool {
        self.enabled
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
        if !self.enabled {
            return Ok(());
        }

        // Compatibility with older LLVM outputs. Newer LLVM outputs, when
        // atomics are enabled, emit a shared memory. That's a good indicator
        // that we have work to do. If shared memory isn't enabled, though then
        // this isn't an atomic module so there's nothing to do. We still allow,
        // though, an environment variable to force us to go down this path to
        // remain compatibile with older LLVM outputs.
        let memory = wasm_conventions::get_memory(module)?;
        if !module.memories.get(memory).shared {
            return Ok(());
        }

        let stack_pointer = wasm_conventions::get_shadow_stack_pointer(module)?;
        let addr = allocate_static_data(module, memory, 4, 4)?;
        let zero = InitExpr::Value(Value::I32(0));
        let globals = Globals {
            thread_id: module.globals.add_local(ValType::I32, true, zero),
            thread_tcb: module.globals.add_local(ValType::I32, true, zero),
        };

        // There was an "inflection point" at the LLVM 9 release where LLD
        // started having better support for producing binaries capable of being
        // used with multi-threading. Prior to LLVM 9 (e.g. nightly releases
        // before July 2019 basically) we had to sort of paper over a lot of
        // support that hadn't been added to LLD. With LLVM 9 and onwards though
        // we expect Rust binaries to be pretty well formed if prepared for
        // threading when they come out of LLD. This `if` statement basically
        // switches on these two cases, figuring out if we're "old style" or
        // "new style".
        let mem = module.memories.get_mut(memory);
        let memory_init = if mem.shared {
            let prev_max = mem.maximum.unwrap();
            assert!(mem.import.is_some());
            mem.maximum = Some(cmp::max(self.maximum_memory / PAGE_SIZE, prev_max));
            assert!(mem.data_segments.is_empty());

            InitMemory::Call {
                wasm_init_memory: delete_synthetic_func(module, "__wasm_init_memory")?,
                wasm_init_tls: delete_synthetic_func(module, "__wasm_init_tls")?,
                tls_size: delete_synthetic_global(module, "__tls_size")?,
            }
        } else {
            update_memory(module, memory, self.maximum_memory)?;
            InitMemory::Segments(switch_data_segments_to_passive(module, memory)?)
        };
        inject_start(
            module,
            memory_init,
            &globals,
            addr,
            stack_pointer,
            self.thread_stack_size,
            memory,
        )?;

        implement_thread_intrinsics(module, &globals)?;
        Ok(())
    }
}

fn delete_synthetic_func(module: &mut Module, name: &str) -> Result<FunctionId, Error> {
    match delete_synthetic_export(module, name)? {
        walrus::ExportItem::Function(f) => Ok(f),
        _ => bail!("`{}` must be a function", name),
    }
}

fn delete_synthetic_global(module: &mut Module, name: &str) -> Result<u32, Error> {
    let id = match delete_synthetic_export(module, name)? {
        walrus::ExportItem::Global(g) => g,
        _ => bail!("`{}` must be a global", name),
    };
    let g = match module.globals.get(id).kind {
        walrus::GlobalKind::Local(g) => g,
        walrus::GlobalKind::Import(_) => bail!("`{}` must not be an imported global", name),
    };
    match g {
        InitExpr::Value(Value::I32(v)) => Ok(v as u32),
        _ => bail!("`{}` was not an `i32` constant", name),
    }
}

fn delete_synthetic_export(module: &mut Module, name: &str) -> Result<ExportItem, Error> {
    let item = module
        .exports
        .iter()
        .find(|e| e.name == name)
        .ok_or_else(|| format_err!("failed to find `{}`", name))?;
    let ret = item.item;
    let id = item.id();
    module.exports.delete(id);
    Ok(ret)
}

struct PassiveSegment {
    id: DataId,
    offset: InitExpr,
    len: u32,
}

fn switch_data_segments_to_passive(
    module: &mut Module,
    memory: MemoryId,
) -> Result<Vec<PassiveSegment>, Error> {
    let mut ret = Vec::new();
    let memory = module.memories.get_mut(memory);
    for id in mem::replace(&mut memory.data_segments, Default::default()) {
        let data = module.data.get_mut(id);
        let kind = match &data.kind {
            walrus::DataKind::Active(kind) => kind,
            walrus::DataKind::Passive => continue,
        };
        let offset = match kind.location {
            walrus::ActiveDataLocation::Absolute(n) => {
                walrus::InitExpr::Value(walrus::ir::Value::I32(n as i32))
            }
            walrus::ActiveDataLocation::Relative(global) => walrus::InitExpr::Global(global),
        };
        data.kind = walrus::DataKind::Passive;
        ret.push(PassiveSegment {
            id,
            offset,
            len: data.value.len() as u32,
        });
    }

    Ok(ret)
}

fn update_memory(module: &mut Module, memory: MemoryId, max: u32) -> Result<MemoryId, Error> {
    assert!(max % PAGE_SIZE == 0);
    let memory = module.memories.get_mut(memory);

    // For multithreading if we want to use the exact same module on all
    // threads we'll need to be sure to import memory, so switch it to an
    // import if it's already here.
    if memory.import.is_none() {
        let id = module
            .imports
            .add("env", "memory", ImportKind::Memory(memory.id()));
        memory.import = Some(id);
    }

    // If the memory isn't already shared, make it so as that's the whole point
    // here!
    if !memory.shared {
        memory.shared = true;
        if memory.maximum.is_none() {
            memory.maximum = Some(max / PAGE_SIZE);
        }
    }

    Ok(memory.id())
}

struct Globals {
    thread_id: GlobalId,
    thread_tcb: GlobalId,
}

fn allocate_static_data(
    module: &mut Module,
    memory: MemoryId,
    size: u32,
    align: u32,
) -> Result<u32, Error> {
    // First up, look for a `__heap_base` export which is injected by LLD as
    // part of the linking process. Note that `__heap_base` should in theory be
    // *after* the stack and data, which means it's at the very end of the
    // address space and should be safe for us to inject 4 bytes of data at.
    let heap_base = module
        .exports
        .iter()
        .filter(|e| e.name == "__heap_base")
        .filter_map(|e| match e.item {
            ExportItem::Global(id) => Some(id),
            _ => None,
        })
        .next();
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
        let global = module.globals.get_mut(heap_base);
        if global.ty != ValType::I32 {
            bail!("the `__heap_base` global doesn't have the type `i32`");
        }
        if global.mutable {
            bail!("the `__heap_base` global is unexpectedly mutable");
        }
        let offset = match &mut global.kind {
            GlobalKind::Local(InitExpr::Value(Value::I32(n))) => n,
            _ => bail!("`__heap_base` not a locally defined `i32`"),
        };
        let address = (*offset as u32 + (align - 1)) & !(align - 1); // align up
        let add_a_page = (address + size) / PAGE_SIZE != address / PAGE_SIZE;
        *offset = (address + size) as i32;
        (address, add_a_page)
    };

    if add_a_page {
        let memory = module.memories.get_mut(memory);
        memory.initial += 1;
        memory.maximum = memory.maximum.map(|m| cmp::max(m, memory.initial));
    }
    Ok(address)
}

enum InitMemory {
    Segments(Vec<PassiveSegment>),
    Call {
        wasm_init_memory: walrus::FunctionId,
        wasm_init_tls: walrus::FunctionId,
        tls_size: u32,
    },
}

fn inject_start(
    module: &mut Module,
    memory_init: InitMemory,
    globals: &Globals,
    addr: u32,
    stack_pointer: GlobalId,
    stack_size: u32,
    memory: MemoryId,
) -> Result<(), Error> {
    use walrus::ir::*;

    assert!(stack_size % PAGE_SIZE == 0);
    let mut builder = walrus::FunctionBuilder::new(&mut module.types, &[], &[]);
    let local = module.locals.add(ValType::I32);
    let mut body = builder.func_body();

    body.i32_const(addr as i32)
        .i32_const(1)
        .atomic_rmw(
            memory,
            AtomicOp::Add,
            AtomicWidth::I32,
            MemArg {
                align: 4,
                offset: 0,
            },
        )
        .local_tee(local)
        .global_set(globals.thread_id);

    // Perform an if/else based on whether we're the first thread or not. Our
    // thread ID will be zero if we're the first thread, otherwise it'll be
    // nonzero (assuming we don't overflow...)
    body.local_get(local);
    body.if_else(
        None,
        // If our thread id is nonzero then we're the second or greater thread, so
        // we give ourselves a stack via memory.grow and we update our stack
        // pointer as the default stack pointer is surely wrong for us.
        |body| {
            // local0 = grow_memory(stack_size);
            body.i32_const((stack_size / PAGE_SIZE) as i32)
                .memory_grow(memory)
                .local_set(local);

            // if local0 == -1 then trap
            body.block(None, |body| {
                let target = body.id();
                body.local_get(local)
                    .i32_const(-1)
                    .binop(BinaryOp::I32Ne)
                    .br_if(target)
                    .unreachable();
            });

            // stack_pointer = local0 + stack_size
            body.local_get(local)
                .i32_const(PAGE_SIZE as i32)
                .binop(BinaryOp::I32Mul)
                .i32_const(stack_size as i32)
                .binop(BinaryOp::I32Add)
                .global_set(stack_pointer);
        },
        // If the thread ID is zero then we can skip the update of the stack
        // pointer as we know our stack pointer is valid. We need to initialize
        // memory, however, so do that here.
        |body| {
            match &memory_init {
                InitMemory::Segments(segments) => {
                    for segment in segments {
                        // let zero = block.i32_const(0);
                        match segment.offset {
                            InitExpr::Global(id) => body.global_get(id),
                            InitExpr::Value(v) => body.const_(v),
                        };
                        body.i32_const(0)
                            .i32_const(segment.len as i32)
                            .memory_init(memory, segment.id)
                            .data_drop(segment.id);
                    }
                }
                InitMemory::Call {
                    wasm_init_memory, ..
                } => {
                    body.call(*wasm_init_memory);
                }
            }
        },
    );

    // If we have these globals then we're using the new thread local system
    // implemented in LLVM, which means that `__wasm_init_tls` needs to be
    // called with a chunk of memory `tls_size` bytes big to set as the threads
    // thread-local data block.
    if let InitMemory::Call {
        wasm_init_tls,
        tls_size,
        ..
    } = memory_init
    {
        let malloc = find_wbindgen_malloc(module)?;
        body.i32_const(tls_size as i32)
            .call(malloc)
            .call(wasm_init_tls);
    }

    // If a start function previously existed we're done with our own
    // initialization so delegate to them now.
    if let Some(id) = module.start.take() {
        body.call(id);
    }

    // Finish off our newly generated function.
    let id = builder.finish(Vec::new(), &mut module.funcs);

    // ... and finally flag it as the new start function
    module.start = Some(id);

    Ok(())
}

fn find_wbindgen_malloc(module: &Module) -> Result<FunctionId, Error> {
    let e = module
        .exports
        .iter()
        .find(|e| e.name == "__wbindgen_malloc")
        .ok_or_else(|| format_err!("failed to find `__wbindgen_malloc`"))?;
    match e.item {
        walrus::ExportItem::Function(f) => Ok(f),
        _ => bail!("`__wbindgen_malloc` wasn't a funtion"),
    }
}

fn implement_thread_intrinsics(module: &mut Module, globals: &Globals) -> Result<(), Error> {
    use walrus::ir::*;

    let mut map = HashMap::new();

    enum Intrinsic {
        GetThreadId,
        GetTcb,
        SetTcb,
    }

    let imports = module
        .imports
        .iter()
        .filter(|i| i.module == "__wbindgen_thread_xform__");
    for import in imports {
        let function = match import.kind {
            ImportKind::Function(id) => module.funcs.get(id),
            _ => bail!("non-function import from special module"),
        };
        let ty = module.types.get(function.ty());

        match &import.name[..] {
            "__wbindgen_current_id" => {
                if !ty.params().is_empty() || ty.results() != &[ValType::I32] {
                    bail!("`__wbindgen_current_id` intrinsic has the wrong signature");
                }
                map.insert(function.id(), Intrinsic::GetThreadId);
            }
            "__wbindgen_tcb_get" => {
                if !ty.params().is_empty() || ty.results() != &[ValType::I32] {
                    bail!("`__wbindgen_tcb_get` intrinsic has the wrong signature");
                }
                map.insert(function.id(), Intrinsic::GetTcb);
            }
            "__wbindgen_tcb_set" => {
                if !ty.results().is_empty() || ty.params() != &[ValType::I32] {
                    bail!("`__wbindgen_tcb_set` intrinsic has the wrong signature");
                }
                map.insert(function.id(), Intrinsic::SetTcb);
            }
            other => bail!("unknown thread intrinsic: {}", other),
        }
    }

    struct Visitor<'a> {
        map: &'a HashMap<FunctionId, Intrinsic>,
        globals: &'a Globals,
    }

    module.funcs.iter_local_mut().for_each(|(_id, func)| {
        let entry = func.entry_block();
        dfs_pre_order_mut(&mut Visitor { map: &map, globals }, func, entry);
    });

    impl VisitorMut for Visitor<'_> {
        fn visit_instr_mut(&mut self, instr: &mut Instr) {
            let call = match instr {
                Instr::Call(e) => e,
                _ => return,
            };
            match self.map.get(&call.func) {
                Some(Intrinsic::GetThreadId) => {
                    *instr = GlobalGet {
                        global: self.globals.thread_id,
                    }
                    .into();
                }
                Some(Intrinsic::GetTcb) => {
                    *instr = GlobalGet {
                        global: self.globals.thread_tcb,
                    }
                    .into();
                }
                Some(Intrinsic::SetTcb) => {
                    *instr = GlobalSet {
                        global: self.globals.thread_tcb,
                    }
                    .into();
                }
                None => {}
            }
        }
    }

    Ok(())
}
