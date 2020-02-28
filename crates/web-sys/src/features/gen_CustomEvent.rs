use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = CustomEvent , typescript_name = CustomEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CustomEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent)\n\n*This API requires the following crate features to be activated: `CustomEvent`*"]
    pub type CustomEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "CustomEvent" , js_name = detail ) ]
    #[doc = "Getter for the `detail` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/detail)\n\n*This API requires the following crate features to be activated: `CustomEvent`*"]
    pub fn detail(this: &CustomEvent) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(catch, js_class = "CustomEvent", constructor)]
    #[doc = "The `new CustomEvent(..)` constructor, creating a new instance of `CustomEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/CustomEvent)\n\n*This API requires the following crate features to be activated: `CustomEvent`*"]
    pub fn new(this: &CustomEvent, type_: &str) -> Result<CustomEvent, JsValue>;
    #[cfg(feature = "CustomEventInit")]
    #[wasm_bindgen(catch, js_class = "CustomEvent", constructor)]
    #[doc = "The `new CustomEvent(..)` constructor, creating a new instance of `CustomEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/CustomEvent)\n\n*This API requires the following crate features to be activated: `CustomEvent`, `CustomEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &CustomEvent,
        type_: &str,
        event_init_dict: &CustomEventInit,
    ) -> Result<CustomEvent, JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "CustomEvent" , js_name = initCustomEvent ) ]
    #[doc = "The `initCustomEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/initCustomEvent)\n\n*This API requires the following crate features to be activated: `CustomEvent`*"]
    pub fn init_custom_event(this: &CustomEvent, type_: &str);
    # [ wasm_bindgen ( method , structural , js_class = "CustomEvent" , js_name = initCustomEvent ) ]
    #[doc = "The `initCustomEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/initCustomEvent)\n\n*This API requires the following crate features to be activated: `CustomEvent`*"]
    pub fn init_custom_event_with_can_bubble(this: &CustomEvent, type_: &str, can_bubble: bool);
    # [ wasm_bindgen ( method , structural , js_class = "CustomEvent" , js_name = initCustomEvent ) ]
    #[doc = "The `initCustomEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/initCustomEvent)\n\n*This API requires the following crate features to be activated: `CustomEvent`*"]
    pub fn init_custom_event_with_can_bubble_and_cancelable(
        this: &CustomEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
    );
    # [ wasm_bindgen ( method , structural , js_class = "CustomEvent" , js_name = initCustomEvent ) ]
    #[doc = "The `initCustomEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/initCustomEvent)\n\n*This API requires the following crate features to be activated: `CustomEvent`*"]
    pub fn init_custom_event_with_can_bubble_and_cancelable_and_detail(
        this: &CustomEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        detail: &::wasm_bindgen::JsValue,
    );
}
