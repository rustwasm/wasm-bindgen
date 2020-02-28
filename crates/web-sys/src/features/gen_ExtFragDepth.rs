use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = EXT_frag_depth , typescript_name = EXT_frag_depth ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtFragDepth` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_frag_depth)\n\n*This API requires the following crate features to be activated: `ExtFragDepth`*"]
    pub type ExtFragDepth;
}
