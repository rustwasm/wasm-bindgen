use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WorkletGlobalScope , typescript_type = "WorkletGlobalScope" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WorkletGlobalScope` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkletGlobalScope)
    ///
    ///*This API requires the following crate features to be activated: `WorkletGlobalScope`*
    pub type WorkletGlobalScope;

}
