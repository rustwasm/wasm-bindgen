use std::cmp;
use std::collections::HashMap;
use std::mem;

use failure::{bail, format_err, Error};
use walrus::ir::Value;
use walrus::{DataId, FunctionId, InitExpr, LocalFunction, ValType};
use walrus::{ExportItem, GlobalId, GlobalKind, ImportKind, MemoryId, Module};

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
            maximum_memory: 1 << 30,    // 1GB
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
        let memory = update_memory(module, self.maximum_memory)?;
        let segments = switch_data_segments_to_passive(module, memory)?;
        let stack_pointer = find_stack_pointer(module)?;

        let zero = InitExpr::Value(Value::I32(0));
        let globals = Globals {
            thread_id: module.globals.add_local(ValType::I32, true, zero),
            thread_tcb: module.globals.add_local(ValType::I32, true, zero),
        };
        let addr = inject_thread_id_counter(module, memory)?;
        start_with_init_memory(
            module,
            &segments,
            &globals,
            addr,
            stack_pointer,
            self.thread_stack_size,
            memory,
        );
        implement_thread_intrinsics(module, &globals)?;
        Ok(())
    }
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
    let data = mem::replace(&mut memory.data, Default::default());
    for (offset, value) in data.into_iter() {
        let len = value.len() as u32;
        let id = module.data.add(value);
        ret.push(PassiveSegment { id, offset, len });
    }

    Ok(ret)
}

fn update_memory(module: &mut Module, max: u32) -> Result<MemoryId, Error> {
    assert!(max % PAGE_SIZE == 0);
    let mut memories = module.memories.iter_mut();
    let memory = memories
        .next()
        .ok_or_else(|| format_err!("currently incompatible with no memory modules"))?;
    if memories.next().is_some() {
        bail!("only one memory is currently supported");
    }

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

fn inject_thread_id_counter(module: &mut Module, memory: MemoryId) -> Result<u32, Error> {
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
        let address = (*offset as u32 + 3) & !3; // align up
        let add_a_page = (address + 4) / PAGE_SIZE != address / PAGE_SIZE;
        *offset = (address + 4) as i32;
        (address, add_a_page)
    };

    if add_a_page {
        let memory = module.memories.get_mut(memory);
        memory.initial += 1;
        memory.maximum = memory.maximum.map(|m| cmp::max(m, memory.initial));
    }
    Ok(address)
}

fn find_stack_pointer(module: &mut Module) -> Result<Option<GlobalId>, Error> {
    let candidates = module
        .globals
        .iter()
        .filter(|g| g.ty == ValType::I32)
        .filter(|g| g.mutable)
        .filter(|g| match g.kind {
            GlobalKind::Local(_) => true,
            GlobalKind::Import(_) => false,
        })
        .collect::<Vec<_>>();

    match candidates.len() {
        // If there are no mutable i32 globals, assume this module doesn't even
        // need a stack pointer!
        0 => Ok(None),

        // If there's more than one global give up for now. Eventually we can
        // probably do better by pattern matching on functions, but this should
        // be sufficient for LLVM's output for now.
        1 => Ok(Some(candidates[0].id())),
        _ => bail!("too many mutable globals to infer the stack pointer"),
    }
}

fn start_with_init_memory(
    module: &mut Module,
    segments: &[PassiveSegment],
    globals: &Globals,
    addr: u32,
    stack_pointer: Option<GlobalId>,
    stack_size: u32,
    memory: MemoryId,
) {
    use walrus::ir::*;

    assert!(stack_size % PAGE_SIZE == 0);
    let mut builder = walrus::FunctionBuilder::new();
    let mut exprs = Vec::new();
    let local = module.locals.add(ValType::I32);

    let addr = builder.i32_const(addr as i32);
    let one = builder.i32_const(1);
    let thread_id = builder.atomic_rmw(
        memory,
        AtomicOp::Add,
        AtomicWidth::I32,
        MemArg {
            align: 4,
            offset: 0,
        },
        addr,
        one,
    );
    let thread_id = builder.local_tee(local, thread_id);
    let global_set = builder.global_set(globals.thread_id, thread_id);
    exprs.push(global_set);

    // Perform an if/else based on whether we're the first thread or not. Our
    // thread ID will be zero if we're the first thread, otherwise it'll be
    // nonzero (assuming we don't overflow...)
    //
    let thread_id_is_nonzero = builder.local_get(local);

    // If our thread id is nonzero then we're the second or greater thread, so
    // we give ourselves a stack via memory.grow and we update our stack
    // pointer as the default stack pointer is surely wrong for us.
    let mut block = builder.if_else_block(Box::new([]), Box::new([]));
    if let Some(stack_pointer) = stack_pointer {
        // local0 = grow_memory(stack_size);
        let grow_amount = block.i32_const((stack_size / PAGE_SIZE) as i32);
        let memory_growth = block.memory_grow(memory, grow_amount);
        let set_local = block.local_set(local, memory_growth);
        block.expr(set_local);

        // if local0 == -1 then trap
        let if_negative_trap = {
            let mut block = block.block(Box::new([]), Box::new([]));

            let lhs = block.local_get(local);
            let rhs = block.i32_const(-1);
            let condition = block.binop(BinaryOp::I32Ne, lhs, rhs);
            let id = block.id();
            let br_if = block.br_if(condition, id, Box::new([]));
            block.expr(br_if);

            let unreachable = block.unreachable();
            block.expr(unreachable);

            id
        };
        block.expr(if_negative_trap.into());

        // stack_pointer = local0 + stack_size
        let get_local = block.local_get(local);
        let page_size = block.i32_const(PAGE_SIZE as i32);
        let sp_base = block.binop(BinaryOp::I32Mul, get_local, page_size);
        let stack_size = block.i32_const(stack_size as i32);
        let sp = block.binop(BinaryOp::I32Add, sp_base, stack_size);
        let set_stack_pointer = block.global_set(stack_pointer, sp);
        block.expr(set_stack_pointer);
    }
    let if_nonzero_block = block.id();
    drop(block);

    // If the thread ID is zero then we can skip the update of the stack
    // pointer as we know our stack pointer is valid. We need to initialize
    // memory, however, so do that here.
    let if_zero_block = {
        let mut block = builder.if_else_block(Box::new([]), Box::new([]));
        for segment in segments {
            let zero = block.i32_const(0);
            let offset = match segment.offset {
                InitExpr::Global(id) => block.global_get(id),
                InitExpr::Value(v) => block.const_(v),
            };
            let len = block.i32_const(segment.len as i32);
            let init = block.memory_init(memory, segment.id, offset, zero, len);
            block.expr(init);
        }
        block.id()
    };

    let block = builder.if_else(thread_id_is_nonzero, if_nonzero_block, if_zero_block);
    exprs.push(block);

    // On all threads now memory segments are no longer needed
    for segment in segments {
        exprs.push(builder.data_drop(segment.id));
    }

    // If a start function previously existed we're done with our own
    // initialization so delegate to them now.
    if let Some(id) = module.start.take() {
        exprs.push(builder.call(id, Box::new([])));
    }

    // Finish off our newly generated function.
    let ty = module.types.add(&[], &[]);
    let id = builder.finish(ty, Vec::new(), exprs, module);

    // ... and finally flag it as the new start function
    module.start = Some(id);
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
            "__wbindgen_thread_id" => {
                if !ty.params().is_empty() || ty.results() != &[ValType::I32] {
                    bail!("`__wbindgen_thread_id` intrinsic has the wrong signature");
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
        func: &'a mut LocalFunction,
    }

    module.funcs.iter_local_mut().for_each(|(_id, func)| {
        let mut entry = func.entry_block();
        Visitor {
            map: &map,
            globals,
            func,
        }
        .visit_block_id_mut(&mut entry);
    });

    impl VisitorMut for Visitor<'_> {
        fn local_function_mut(&mut self) -> &mut LocalFunction {
            self.func
        }

        fn visit_expr_mut(&mut self, expr: &mut Expr) {
            let call = match expr {
                Expr::Call(e) => e,
                other => return other.visit_mut(self),
            };
            match self.map.get(&call.func) {
                Some(Intrinsic::GetThreadId) => {
                    assert!(call.args.is_empty());
                    *expr = GlobalGet {
                        global: self.globals.thread_id,
                    }
                    .into();
                }
                Some(Intrinsic::GetTcb) => {
                    assert!(call.args.is_empty());
                    *expr = GlobalGet {
                        global: self.globals.thread_tcb,
                    }
                    .into();
                }
                Some(Intrinsic::SetTcb) => {
                    assert_eq!(call.args.len(), 1);
                    call.args[0].visit_mut(self);
                    *expr = GlobalSet {
                        global: self.globals.thread_tcb,
                        value: call.args[0],
                    }
                    .into();
                }
                None => call.visit_mut(self),
            }
        }
    }

    Ok(())
}
