use anyhow::{anyhow, bail, Error};
use std::cmp;
use std::env;
use walrus::ir::Value;
use walrus::{
    ir::MemArg, ExportItem, FunctionId, GlobalId, GlobalKind, InitExpr, InstrSeqBuilder, MemoryId,
    Module, ValType,
};
use wasm_bindgen_wasm_conventions as wasm_conventions;

const PAGE_SIZE: u32 = 1 << 16;
const ATOMIC_MEM_ARG: MemArg = MemArg {
    align: 4,
    offset: 0,
};

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
    /// * Some stack space is prepared for each thread after the first one.
    ///
    /// More and/or less may happen here over time, stay tuned!
    pub fn run(&self, module: &mut Module) -> Result<(), Error> {
        if !self.is_enabled(module) {
            return Ok(());
        }

        let memory = wasm_conventions::get_memory(module)?;

        // Now we need to allocate extra static memory for:
        // - A thread id counter.
        // - A temporary stack for calls to `malloc()` and `free()`.
        // - A lock to synchronize usage of the above stack.
        // For this, we allocate 1 extra page of memory (should be enough as temporary
        // stack) and grab the first 2 _aligned_ i32 words to use as counter and lock.
        let static_data_align = 4;
        let static_data_pages = 1;
        let (base, addr) =
            allocate_static_data(module, memory, static_data_pages, static_data_align)?;

        let mem = module.memories.get_mut(memory);
        assert!(mem.shared);
        let prev_max = mem.maximum.unwrap();
        assert!(mem.import.is_some());
        mem.maximum = Some(cmp::max(self.maximum_memory / PAGE_SIZE, prev_max));
        assert!(mem.data_segments.is_empty());

        let tls = Tls {
            init: delete_synthetic_func(module, "__wasm_init_tls")?,
            size: delete_synthetic_global(module, "__tls_size")?,
            align: delete_synthetic_global(module, "__tls_align")?,
            base: wasm_conventions::get_tls_base(module)
                .ok_or_else(|| anyhow!("failed to find tls base"))?,
        };

        let thread_counter_addr = addr as i32;

        let stack_alloc =
            module
                .globals
                .add_local(ValType::I32, true, InitExpr::Value(Value::I32(0)));

        // Make sure the temporary stack is aligned down
        let temp_stack = (base + static_data_pages * PAGE_SIZE) & !(static_data_align - 1);

        let stack = Stack {
            pointer: wasm_conventions::get_shadow_stack_pointer(module)
                .ok_or_else(|| anyhow!("failed to find shadow stack pointer"))?,
            temp: temp_stack as i32,
            temp_lock: thread_counter_addr + 4,
            alloc: stack_alloc,
            size: self.thread_stack_size,
        };

        inject_start(module, &tls, &stack, thread_counter_addr, memory)?;

        // we expose a `__wbindgen_thread_destroy()` helper function that deallocates stack space.
        //
        // ## Safety
        // After calling this function in a given agent, the instance should be considered
        // "destroyed" and any further invocations into it will trigger UB. This function
        // should not be called from an agent that cannot block (e.g. the main document thread).
        inject_destroy(module, &tls, &stack, memory)?;

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

/// Allocates extra space for static data. Returns `(addr, base)`, where:
/// - `base` is the starting address of the extra `pages`.
/// - `addr` is the _first_ address in that chunk that is aligned to `align`.
fn allocate_static_data(
    module: &mut Module,
    memory: MemoryId,
    pages: u32,
    align: u32,
) -> Result<(u32, u32), Error> {
    // First up, look for a `__heap_base` export which is injected by LLD as
    // part of the linking process. Note that `__heap_base` should in theory be
    // *after* the stack and data, which means it's at the very end of the
    // address space and should be safe for us to inject extra pages of data at.
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

    // Now we need to bump up `__heap_base` by a few pages. Do lots of validation
    // here to make sure that `__heap_base` is an non-mutable integer, and then do
    // some logic to ensure that the return the correct, aligned `address` as specified
    // by `align`.
    let (base, address) = {
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
        let base = *offset;

        *offset = *offset + (pages * PAGE_SIZE) as i32;

        (base, address)
    };

    let memory = module.memories.get_mut(memory);
    memory.initial += pages;
    memory.maximum = memory.maximum.map(|m| cmp::max(m, memory.initial));

    Ok((base as u32, address))
}

struct Tls {
    init: walrus::FunctionId,
    size: u32,
    align: u32,
    base: GlobalId,
}

struct Stack {
    /// The stack pointer global
    pointer: GlobalId,
    /// The address of a small, "scratch-space" stack
    temp: i32,
    /// The address of a lock for the temporary stack
    temp_lock: i32,
    /// A global to store allocated stack
    alloc: GlobalId,
    /// The size of the stack
    size: u32,
}

fn inject_start(
    module: &mut Module,
    tls: &Tls,
    stack: &Stack,
    thread_counter_addr: i32,
    memory: MemoryId,
) -> Result<(), Error> {
    use walrus::ir::*;

    assert!(stack.size % PAGE_SIZE == 0);
    let mut builder = walrus::FunctionBuilder::new(&mut module.types, &[], &[]);
    let local = module.locals.add(ValType::I32);

    let mut body = builder.func_body();

    // Call previous start function if one is available. Currently this is
    // always true because LLVM injects a call to `__wasm_init_memory` as the
    // start function which, well, initializes memory.
    if let Some(prev) = module.start.take() {
        body.call(prev);
    }

    let malloc = find_function(module, "__wbindgen_malloc")?;

    // Perform an if/else based on whether we're the first thread or not. Our
    // thread ID will be zero if we're the first thread, otherwise it'll be
    // nonzero (assuming we don't overflow...)
    body.i32_const(thread_counter_addr)
        .i32_const(1)
        .atomic_rmw(memory, AtomicOp::Add, AtomicWidth::I32, ATOMIC_MEM_ARG)
        .if_else(
            None,
            // If our thread id is nonzero then we're the second or greater thread, so
            // we give ourselves a stack and we update our stack
            // pointer as the default stack pointer is surely wrong for us.
            |body| {
                // local = malloc(stack.size) [aka base]
                with_temp_stack(body, memory, &stack, |body| {
                    body.i32_const(stack.size as i32)
                        .call(malloc)
                        .local_tee(local);
                });

                // stack.alloc = base
                body.global_set(stack.alloc);

                // stack_pointer = base + stack.size
                body.global_get(stack.alloc)
                    .i32_const(stack.size as i32)
                    .binop(BinaryOp::I32Add)
                    .global_set(stack.pointer);
            },
            // If the thread id is zero then the default stack pointer works for
            // us.
            |_| {},
        );

    // Afterwards we need to initialize our thread-local state.
    body.i32_const(tls.size as i32)
        .i32_const(tls.align as i32)
        .drop() // TODO: need to actually respect alignment
        .call(malloc)
        .global_set(tls.base)
        .global_get(tls.base)
        .call(tls.init);

    // Finish off our newly generated function.
    let start_id = builder.finish(Vec::new(), &mut module.funcs);

    // ... and finally flag it as the new start function
    module.start = Some(start_id);

    Ok(())
}

fn inject_destroy(
    module: &mut Module,
    tls: &Tls,
    stack: &Stack,
    memory: MemoryId,
) -> Result<(), Error> {
    let free = find_function(module, "__wbindgen_free")?;

    let mut builder = walrus::FunctionBuilder::new(&mut module.types, &[], &[]);

    builder.name("__wbindgen_thread_destroy".into());

    let mut body = builder.func_body();

    // Ideally, at this point, we would destroy the values stored in TLS.
    // We can't really do that without help from the standard library.
    // See https://github.com/rustwasm/wasm-bindgen/pull/2769#issuecomment-1015775467.

    // free the TLS space
    body.global_get(tls.base)
        .i32_const(tls.size as i32)
        .call(free);

    // set tls.base = i32::MIN to trigger invalid memory
    body.i32_const(i32::MIN).global_set(tls.base);

    // free the stack callin `__wbindgen_free(stack.alloc, stack.size)`
    with_temp_stack(&mut body, memory, stack, |body| {
        body.global_get(stack.alloc)
            .i32_const(stack.size as i32)
            .call(free);
    });

    // set stack.alloc = 0 to trigger invalid memory
    body.i32_const(0).global_set(stack.alloc);

    let free_id = builder.finish(Vec::new(), &mut module.funcs);

    module.exports.add("__wbindgen_thread_destroy", free_id);

    Ok(())
}

fn find_function(module: &Module, name: &str) -> Result<FunctionId, Error> {
    let e = module
        .exports
        .iter()
        .find(|e| e.name == name)
        .ok_or_else(|| anyhow!("failed to find `{}`", name))?;
    match e.item {
        walrus::ExportItem::Function(f) => Ok(f),
        _ => bail!("`{}` wasn't a funtion", name),
    }
}

/// Wraps the instructions fed by `block()` so that they can assume that the temporary, scratch
/// stack is usable. Clobbers `stack.pointer`.
fn with_temp_stack(
    body: &mut InstrSeqBuilder<'_>,
    memory: MemoryId,
    stack: &Stack,
    block: impl Fn(&mut InstrSeqBuilder<'_>),
) {
    use walrus::ir::*;

    body.i32_const(stack.temp).global_set(stack.pointer);

    body.loop_(None, |loop_| {
        let loop_id = loop_.id();

        loop_
            .i32_const(stack.temp_lock)
            .i32_const(0)
            .i32_const(1)
            .cmpxchg(memory, AtomicWidth::I32, ATOMIC_MEM_ARG)
            .if_else(
                None,
                |body| {
                    body.i32_const(stack.temp_lock)
                        .i32_const(1)
                        .i64_const(-1)
                        .atomic_wait(memory, ATOMIC_MEM_ARG, false)
                        .drop()
                        .br(loop_id);
                },
                |_| {},
            );
    });

    block(body);

    body.i32_const(stack.temp_lock)
        .i32_const(0)
        .store(memory, StoreKind::I32 { atomic: true }, ATOMIC_MEM_ARG)
        .i32_const(stack.temp_lock)
        .i32_const(1)
        .atomic_notify(memory, ATOMIC_MEM_ARG)
        .drop();
}
