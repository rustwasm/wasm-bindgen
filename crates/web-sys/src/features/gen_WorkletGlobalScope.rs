use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WorkletGlobalScope , typescript_name = WorkletGlobalScope ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WorkletGlobalScope` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkletGlobalScope)\n\n*This API requires the following crate features to be activated: `WorkletGlobalScope`*"]
    pub type WorkletGlobalScope;
}
