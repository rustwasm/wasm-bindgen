use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = PromiseNativeHandler , typescript_name = PromiseNativeHandler ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PromiseNativeHandler` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PromiseNativeHandler)
    ///
    ///*This API requires the following crate features to be activated: `PromiseNativeHandler`*
    pub type PromiseNativeHandler;

}
