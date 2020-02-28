use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = PopupBlockedEvent , typescript_name = PopupBlockedEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PopupBlockedEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent)\n\n*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
    pub type PopupBlockedEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = requestingWindow ) ]
    #[cfg(feature = "Window")]
    #[doc = "Getter for the `requestingWindow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/requestingWindow)\n\n*This API requires the following crate features to be activated: `PopupBlockedEvent`, `Window`*"]
    pub fn requesting_window(this: &PopupBlockedEvent) -> Option<Window>;
    # [ wasm_bindgen ( structural , method , getter , js_name = popupWindowName ) ]
    #[doc = "Getter for the `popupWindowName` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/popupWindowName)\n\n*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
    pub fn popup_window_name(this: &PopupBlockedEvent) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = popupWindowFeatures ) ]
    #[doc = "Getter for the `popupWindowFeatures` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/popupWindowFeatures)\n\n*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
    pub fn popup_window_features(this: &PopupBlockedEvent) -> Option<String>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new PopupBlockedEvent(..)` constructor, creating a new instance of `PopupBlockedEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/PopupBlockedEvent)\n\n*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
    pub fn new(this: &PopupBlockedEvent, type_: &str) -> Result<PopupBlockedEvent, JsValue>;
    #[cfg(feature = "PopupBlockedEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new PopupBlockedEvent(..)` constructor, creating a new instance of `PopupBlockedEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/PopupBlockedEvent)\n\n*This API requires the following crate features to be activated: `PopupBlockedEvent`, `PopupBlockedEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &PopupBlockedEvent,
        type_: &str,
        event_init_dict: &PopupBlockedEventInit,
    ) -> Result<PopupBlockedEvent, JsValue>;
}
