extern crate js_sys;
extern crate wasm_bindgen;

use js_sys::{Function, Object, Reflect, Uint8Array, WebAssembly};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

const WASM: &[u8] = include_bytes!("add.wasm");

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_log!("instantiating a new wasm module directly");
    let my_memory = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap();

    // Note that this is somewhat dangerous, once we look at our
    // `WebAssembly.Memory` buffer then if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change.
    // That means we can't actually do any memory allocations after we do this
    // until we pass it back to JS.
    let my_memory = Uint8Array::new(&my_memory.buffer()).subarray(
        WASM.as_ptr() as u32,
        WASM.as_ptr() as u32 + WASM.len() as u32,
    );
    let a = WebAssembly::Module::new(my_memory.as_ref())?;
    let b = WebAssembly::Instance::new(&a, &Object::new())?;
    let c = b.exports();

    let add = Reflect::get(c.as_ref(), &"add".into())?
        .dyn_into::<Function>()
        .expect("add export wasn't a function");

    let three = add.call2(&JsValue::undefined(), &1.into(), &2.into())?;
    console_log!("1 + 2 = {:?}", three);
    let mem = Reflect::get(c.as_ref(), &"memory".into())?
        .dyn_into::<WebAssembly::Memory>()
        .expect("memory export wasn't a `WebAssembly.Memory`");
    console_log!("created module has {} pages of memory", mem.grow(0));
    console_log!("giving the module 4 more pages of memory");
    mem.grow(4);
    console_log!("now the module has {} pages of memory", mem.grow(0));

    Ok(())
}
