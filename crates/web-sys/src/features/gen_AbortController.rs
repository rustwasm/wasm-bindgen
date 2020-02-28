use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = AbortController , typescript_name = AbortController ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AbortController` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortController)\n\n*This API requires the following crate features to be activated: `AbortController`*"]
    pub type AbortController;
    # [ wasm_bindgen ( structural , method , getter , js_class = "AbortController" , js_name = signal ) ]
    #[cfg(feature = "AbortSignal")]
    #[doc = "Getter for the `signal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortController/signal)\n\n*This API requires the following crate features to be activated: `AbortController`, `AbortSignal`*"]
    pub fn signal(this: &AbortController) -> AbortSignal;
    #[wasm_bindgen(catch, js_class = "AbortController", constructor)]
    #[doc = "The `new AbortController(..)` constructor, creating a new instance of `AbortController`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortController/AbortController)\n\n*This API requires the following crate features to be activated: `AbortController`*"]
    pub fn new(this: &AbortController) -> Result<AbortController, JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "AbortController" , js_name = abort ) ]
    #[doc = "The `abort()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortController/abort)\n\n*This API requires the following crate features to be activated: `AbortController`*"]
    pub fn abort(this: &AbortController);
}
