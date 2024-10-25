use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Both `catch_me` and `no_catch` should be defined in the JS and invoke
    // their respective JS function inside a JS shim function. This is
    // important, because these 2 function may not be defined when the WASM
    // module is instantiated.
    #[wasm_bindgen(catch)]
    fn catch_me() -> Result<(), JsValue>;
    fn no_catch();

    // Reload needs to be passed the right `this` parameter in JS.
    #[wasm_bindgen(js_namespace = ["window", "location"])]
    fn reload();

    #[wasm_bindgen(js_namespace = ["window", "document"])]
    fn write(s: &str);
}

#[wasm_bindgen]
pub fn exported() -> Result<(), JsValue> {
    reload();
    write("");
    no_catch();
    catch_me()
}
