use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = PopStateEvent , typescript_name = PopStateEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PopStateEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopStateEvent)\n\n*This API requires the following crate features to be activated: `PopStateEvent`*"]
    pub type PopStateEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PopStateEvent" , js_name = state ) ]
    #[doc = "Getter for the `state` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopStateEvent/state)\n\n*This API requires the following crate features to be activated: `PopStateEvent`*"]
    pub fn state(this: &PopStateEvent) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(catch, js_class = "PopStateEvent", constructor)]
    #[doc = "The `new PopStateEvent(..)` constructor, creating a new instance of `PopStateEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopStateEvent/PopStateEvent)\n\n*This API requires the following crate features to be activated: `PopStateEvent`*"]
    pub fn new(this: &PopStateEvent, type_: &str) -> Result<PopStateEvent, JsValue>;
    #[cfg(feature = "PopStateEventInit")]
    #[wasm_bindgen(catch, js_class = "PopStateEvent", constructor)]
    #[doc = "The `new PopStateEvent(..)` constructor, creating a new instance of `PopStateEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopStateEvent/PopStateEvent)\n\n*This API requires the following crate features to be activated: `PopStateEvent`, `PopStateEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &PopStateEvent,
        type_: &str,
        event_init_dict: &PopStateEventInit,
    ) -> Result<PopStateEvent, JsValue>;
}
