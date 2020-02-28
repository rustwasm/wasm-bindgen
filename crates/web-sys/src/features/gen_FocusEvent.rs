use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = UiEvent , extends = Event , extends = :: js_sys :: Object , js_name = FocusEvent , typescript_name = FocusEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FocusEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusEvent`*"]
    pub type FocusEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "FocusEvent" , js_name = relatedTarget ) ]
    #[cfg(feature = "EventTarget")]
    #[doc = "Getter for the `relatedTarget` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent/relatedTarget)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `FocusEvent`*"]
    pub fn related_target(this: &FocusEvent) -> Option<EventTarget>;
    #[wasm_bindgen(catch, js_class = "FocusEvent", constructor)]
    #[doc = "The `new FocusEvent(..)` constructor, creating a new instance of `FocusEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent/FocusEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusEvent`*"]
    pub fn new(this: &FocusEvent, type_arg: &str) -> Result<FocusEvent, JsValue>;
    #[cfg(feature = "FocusEventInit")]
    #[wasm_bindgen(catch, js_class = "FocusEvent", constructor)]
    #[doc = "The `new FocusEvent(..)` constructor, creating a new instance of `FocusEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent/FocusEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusEvent`, `FocusEventInit`*"]
    pub fn new_with_focus_event_init_dict(
        this: &FocusEvent,
        type_arg: &str,
        focus_event_init_dict: &FocusEventInit,
    ) -> Result<FocusEvent, JsValue>;
}
