use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = WEBGL_lose_context , typescript_name = WEBGL_lose_context ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebglLoseContext` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_lose_context)\n\n*This API requires the following crate features to be activated: `WebglLoseContext`*"]
    pub type WebglLoseContext;
    # [ wasm_bindgen ( method , structural , js_class = "WEBGL_lose_context" , js_name = loseContext ) ]
    #[doc = "The `loseContext()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_lose_context/loseContext)\n\n*This API requires the following crate features to be activated: `WebglLoseContext`*"]
    pub fn lose_context(this: &WebglLoseContext);
    # [ wasm_bindgen ( method , structural , js_class = "WEBGL_lose_context" , js_name = restoreContext ) ]
    #[doc = "The `restoreContext()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_lose_context/restoreContext)\n\n*This API requires the following crate features to be activated: `WebglLoseContext`*"]
    pub fn restore_context(this: &WebglLoseContext);
}
