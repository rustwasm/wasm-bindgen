use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = DeviceLightEvent , typescript_name = DeviceLightEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceLightEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceLightEvent)\n\n*This API requires the following crate features to be activated: `DeviceLightEvent`*"]
    pub type DeviceLightEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceLightEvent" , js_name = value ) ]
    #[doc = "Getter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceLightEvent/value)\n\n*This API requires the following crate features to be activated: `DeviceLightEvent`*"]
    pub fn value(this: &DeviceLightEvent) -> f64;
    #[wasm_bindgen(catch, js_class = "DeviceLightEvent", constructor)]
    #[doc = "The `new DeviceLightEvent(..)` constructor, creating a new instance of `DeviceLightEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceLightEvent/DeviceLightEvent)\n\n*This API requires the following crate features to be activated: `DeviceLightEvent`*"]
    pub fn new(this: &DeviceLightEvent, type_: &str) -> Result<DeviceLightEvent, JsValue>;
    #[cfg(feature = "DeviceLightEventInit")]
    #[wasm_bindgen(catch, js_class = "DeviceLightEvent", constructor)]
    #[doc = "The `new DeviceLightEvent(..)` constructor, creating a new instance of `DeviceLightEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceLightEvent/DeviceLightEvent)\n\n*This API requires the following crate features to be activated: `DeviceLightEvent`, `DeviceLightEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &DeviceLightEvent,
        type_: &str,
        event_init_dict: &DeviceLightEventInit,
    ) -> Result<DeviceLightEvent, JsValue>;
}
