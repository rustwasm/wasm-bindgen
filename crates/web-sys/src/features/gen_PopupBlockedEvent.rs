use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = PopupBlockedEvent , typescript_name = PopupBlockedEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PopupBlockedEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent)
    ///
    ///*This API requires the following crate features to be activated: `PopupBlockedEvent`*
    pub type PopupBlockedEvent;

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "PopupBlockedEvent" , js_name = requestingWindow ) ]
    ///Getter for the `requestingWindow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/requestingWindow)
    ///
    ///*This API requires the following crate features to be activated: `PopupBlockedEvent`, `Window`*
    pub fn requesting_window(this: &PopupBlockedEvent) -> Option<Window>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PopupBlockedEvent" , js_name = popupWindowName ) ]
    ///Getter for the `popupWindowName` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/popupWindowName)
    ///
    ///*This API requires the following crate features to be activated: `PopupBlockedEvent`*
    pub fn popup_window_name(this: &PopupBlockedEvent) -> Option<String>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PopupBlockedEvent" , js_name = popupWindowFeatures ) ]
    ///Getter for the `popupWindowFeatures` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/popupWindowFeatures)
    ///
    ///*This API requires the following crate features to be activated: `PopupBlockedEvent`*
    pub fn popup_window_features(this: &PopupBlockedEvent) -> Option<String>;

    #[wasm_bindgen(catch, constructor, js_class = "PopupBlockedEvent")]
    ///The `new PopupBlockedEvent(..)` constructor, creating a new instance of `PopupBlockedEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/PopupBlockedEvent)
    ///
    ///*This API requires the following crate features to be activated: `PopupBlockedEvent`*
    pub fn new(type_: &str) -> Result<PopupBlockedEvent, JsValue>;

    #[cfg(feature = "PopupBlockedEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "PopupBlockedEvent")]
    ///The `new PopupBlockedEvent(..)` constructor, creating a new instance of `PopupBlockedEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/PopupBlockedEvent)
    ///
    ///*This API requires the following crate features to be activated: `PopupBlockedEvent`, `PopupBlockedEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &PopupBlockedEventInit,
    ) -> Result<PopupBlockedEvent, JsValue>;

}
