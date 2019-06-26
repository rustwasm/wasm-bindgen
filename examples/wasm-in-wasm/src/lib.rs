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

    // Note that `Uint8Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Uint8Array` to be invalid.
    //
    // As a result, after `Uint8Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    let a = unsafe {
        let array = Uint8Array::view(WASM);
        WebAssembly::Module::new(array.as_ref())?
    };

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
