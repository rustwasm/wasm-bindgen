use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = ConsoleInstance , typescript_name = ConsoleInstance ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ConsoleInstance` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConsoleInstance)
    ///
    ///*This API requires the following crate features to be activated: `ConsoleInstance`*
    pub type ConsoleInstance;

}
