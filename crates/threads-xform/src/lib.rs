use anyhow::{anyhow, bail, Error};
use std::cmp;
use std::env;
use walrus::ir::Value;
use walrus::{ExportItem, GlobalId, GlobalKind, MemoryId, Module};
use walrus::{FunctionId, InitExpr, ValType};
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
    pub fn is_enabled(&self, module: &Module) -> bool {
        if self.enabled {
            return true;
        }

        // Compatibility with older LLVM outputs. Newer LLVM outputs, when
        // atomics are enabled, emit a shared memory. That's a good indicator
        // that we have work to do. If shared memory isn't enabled, though then
        // this isn't an atomic module so there's nothing to do. We still allow,
        // though, an environment variable to force us to go down this path to
        // remain compatibile with older LLVM outputs.
        match wasm_conventions::get_memory(module) {
            Ok(memory) => module.memories.get(memory).shared,
            Err(_) => false,
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
        if !self.is_enabled(module) {
            return Ok(());
        }

        let memory = wasm_conventions::get_memory(module)?;
        let stack_pointer = wasm_conventions::get_shadow_stack_pointer(module)
            .ok_or_else(|| anyhow!("failed to find shadow stack pointer"))?;
        let addr = allocate_static_data(module, memory, 4, 4)?;

        let mem = module.memories.get_mut(memory);
        assert!(mem.shared);
        let prev_max = mem.maximum.unwrap();
        assert!(mem.import.is_some());
        mem.maximum = Some(cmp::max(self.maximum_memory / PAGE_SIZE, prev_max));
        assert!(mem.data_segments.is_empty());

        delete_synthetic_func(module, "__wasm_init_memory")?;
        let tls = Tls {
            init: delete_synthetic_func(module, "__wasm_init_tls")?,
            size: delete_synthetic_global(module, "__tls_size")?,
            align: delete_synthetic_global(module, "__tls_align")?,
        };
        inject_start(
            module,
            tls,
            addr,
            stack_pointer,
            self.thread_stack_size,
            memory,
        )?;

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
        .ok_or_else(|| anyhow!("failed to find `{}`", name))?;
    let ret = item.item;
    let id = item.id();
    module.exports.delete(id);
    Ok(ret)
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

struct Tls {
    init: walrus::FunctionId,
    size: u32,
    align: u32,
}

fn inject_start(
    module: &mut Module,
    tls: Tls,
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

    // Call previous start function if one is available. Currently this is
    // always true because LLVM injects a call to `__wasm_init_memory` as the
    // start function which, well, initializes memory.
    if let Some(prev) = module.start.take() {
        body.call(prev);
    }

    // Perform an if/else based on whether we're the first thread or not. Our
    // thread ID will be zero if we're the first thread, otherwise it'll be
    // nonzero (assuming we don't overflow...)
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
        .if_else(
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
            // If the thread id is zero then the default stack pointer works for
            // us.
            |_| {},
        );

    // Afterwards we need to initialize our thread-local state.
    let malloc = find_wbindgen_malloc(module)?;
    body.i32_const(tls.size as i32)
        .i32_const(tls.align as i32)
        .drop() // TODO: need to actually respect alignment
        .call(malloc)
        .call(tls.init);

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
        .ok_or_else(|| anyhow!("failed to find `__wbindgen_malloc`"))?;
    match e.item {
        walrus::ExportItem::Function(f) => Ok(f),
        _ => bail!("`__wbindgen_malloc` wasn't a funtion"),
    }
}
