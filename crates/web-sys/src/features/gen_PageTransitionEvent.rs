use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = PageTransitionEvent , typescript_name = PageTransitionEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PageTransitionEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent)
    ///
    ///*This API requires the following crate features to be activated: `PageTransitionEvent`*
    pub type PageTransitionEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PageTransitionEvent" , js_name = persisted ) ]
    ///Getter for the `persisted` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent/persisted)
    ///
    ///*This API requires the following crate features to be activated: `PageTransitionEvent`*
    pub fn persisted(this: &PageTransitionEvent) -> bool;

    #[wasm_bindgen(catch, constructor, js_class = "PageTransitionEvent")]
    ///The `new PageTransitionEvent(..)` constructor, creating a new instance of `PageTransitionEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent/PageTransitionEvent)
    ///
    ///*This API requires the following crate features to be activated: `PageTransitionEvent`*
    pub fn new(type_: &str) -> Result<PageTransitionEvent, JsValue>;

    #[cfg(feature = "PageTransitionEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "PageTransitionEvent")]
    ///The `new PageTransitionEvent(..)` constructor, creating a new instance of `PageTransitionEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent/PageTransitionEvent)
    ///
    ///*This API requires the following crate features to be activated: `PageTransitionEvent`, `PageTransitionEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &PageTransitionEventInit,
    ) -> Result<PageTransitionEvent, JsValue>;

}
