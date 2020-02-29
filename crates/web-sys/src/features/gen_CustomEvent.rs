use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = CustomEvent , typescript_type = "CustomEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CustomEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent)
    ///
    ///*This API requires the following crate features to be activated: `CustomEvent`*
    pub type CustomEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CustomEvent" , js_name = detail ) ]
    ///Getter for the `detail` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/detail)
    ///
    ///*This API requires the following crate features to be activated: `CustomEvent`*
    pub fn detail(this: &CustomEvent) -> ::wasm_bindgen::JsValue;

    #[wasm_bindgen(catch, constructor, js_class = "CustomEvent")]
    ///The `new CustomEvent(..)` constructor, creating a new instance of `CustomEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/CustomEvent)
    ///
    ///*This API requires the following crate features to be activated: `CustomEvent`*
    pub fn new(type_: &str) -> Result<CustomEvent, JsValue>;

    #[cfg(feature = "CustomEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "CustomEvent")]
    ///The `new CustomEvent(..)` constructor, creating a new instance of `CustomEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/CustomEvent)
    ///
    ///*This API requires the following crate features to be activated: `CustomEvent`, `CustomEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &CustomEventInit,
    ) -> Result<CustomEvent, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "CustomEvent" , js_name = initCustomEvent ) ]
    ///The `initCustomEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/initCustomEvent)
    ///
    ///*This API requires the following crate features to be activated: `CustomEvent`*
    pub fn init_custom_event(this: &CustomEvent, type_: &str);

    # [ wasm_bindgen ( method , structural , js_class = "CustomEvent" , js_name = initCustomEvent ) ]
    ///The `initCustomEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/initCustomEvent)
    ///
    ///*This API requires the following crate features to be activated: `CustomEvent`*
    pub fn init_custom_event_with_can_bubble(this: &CustomEvent, type_: &str, can_bubble: bool);

    # [ wasm_bindgen ( method , structural , js_class = "CustomEvent" , js_name = initCustomEvent ) ]
    ///The `initCustomEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/initCustomEvent)
    ///
    ///*This API requires the following crate features to be activated: `CustomEvent`*
    pub fn init_custom_event_with_can_bubble_and_cancelable(
        this: &CustomEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
    );

    # [ wasm_bindgen ( method , structural , js_class = "CustomEvent" , js_name = initCustomEvent ) ]
    ///The `initCustomEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/initCustomEvent)
    ///
    ///*This API requires the following crate features to be activated: `CustomEvent`*
    pub fn init_custom_event_with_can_bubble_and_cancelable_and_detail(
        this: &CustomEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        detail: &::wasm_bindgen::JsValue,
    );

}
