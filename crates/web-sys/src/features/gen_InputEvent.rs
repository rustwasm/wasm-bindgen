#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = UiEvent , extends = Event , extends = :: js_sys :: Object , js_name = InputEvent , typescript_type = "InputEvent")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `InputEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEvent`*"]
    pub type InputEvent;
    # [wasm_bindgen (structural , method , getter , js_class = "InputEvent" , js_name = isComposing)]
    #[doc = "Getter for the `isComposing` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/isComposing)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEvent`*"]
    pub fn is_composing(this: &InputEvent) -> bool;
    #[wasm_bindgen(catch, constructor, js_class = "InputEvent")]
    #[doc = "The `new InputEvent(..)` constructor, creating a new instance of `InputEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/InputEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEvent`*"]
    pub fn new(type_: &str) -> Result<InputEvent, JsValue>;
    #[cfg(feature = "InputEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "InputEvent")]
    #[doc = "The `new InputEvent(..)` constructor, creating a new instance of `InputEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/InputEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEvent`, `InputEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &InputEventInit,
    ) -> Result<InputEvent, JsValue>;
}
