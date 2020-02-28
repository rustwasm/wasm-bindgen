use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = UiEvent , extends = Event , extends = :: js_sys :: Object , js_name = InputEvent , typescript_name = InputEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `InputEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent)\n\n*This API requires the following crate features to be activated: `InputEvent`*"]
    pub type InputEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = isComposing ) ]
    #[doc = "Getter for the `isComposing` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/isComposing)\n\n*This API requires the following crate features to be activated: `InputEvent`*"]
    pub fn is_composing(this: &InputEvent) -> bool;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new InputEvent(..)` constructor, creating a new instance of `InputEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/InputEvent)\n\n*This API requires the following crate features to be activated: `InputEvent`*"]
    pub fn new(this: &InputEvent, type_: &str) -> Result<InputEvent, JsValue>;
    #[cfg(feature = "InputEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new InputEvent(..)` constructor, creating a new instance of `InputEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/InputEvent)\n\n*This API requires the following crate features to be activated: `InputEvent`, `InputEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &InputEvent,
        type_: &str,
        event_init_dict: &InputEventInit,
    ) -> Result<InputEvent, JsValue>;
}
