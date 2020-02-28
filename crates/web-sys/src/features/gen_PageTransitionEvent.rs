use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = PageTransitionEvent , typescript_name = PageTransitionEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PageTransitionEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent)\n\n*This API requires the following crate features to be activated: `PageTransitionEvent`*"]
    pub type PageTransitionEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PageTransitionEvent" , js_name = persisted ) ]
    #[doc = "Getter for the `persisted` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent/persisted)\n\n*This API requires the following crate features to be activated: `PageTransitionEvent`*"]
    pub fn persisted(this: &PageTransitionEvent) -> bool;
    #[wasm_bindgen(catch, js_class = "PageTransitionEvent", constructor)]
    #[doc = "The `new PageTransitionEvent(..)` constructor, creating a new instance of `PageTransitionEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent/PageTransitionEvent)\n\n*This API requires the following crate features to be activated: `PageTransitionEvent`*"]
    pub fn new(this: &PageTransitionEvent, type_: &str) -> Result<PageTransitionEvent, JsValue>;
    #[cfg(feature = "PageTransitionEventInit")]
    #[wasm_bindgen(catch, js_class = "PageTransitionEvent", constructor)]
    #[doc = "The `new PageTransitionEvent(..)` constructor, creating a new instance of `PageTransitionEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent/PageTransitionEvent)\n\n*This API requires the following crate features to be activated: `PageTransitionEvent`, `PageTransitionEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &PageTransitionEvent,
        type_: &str,
        event_init_dict: &PageTransitionEventInit,
    ) -> Result<PageTransitionEvent, JsValue>;
}
