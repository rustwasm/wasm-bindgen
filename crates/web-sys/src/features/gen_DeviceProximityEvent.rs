use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = DeviceProximityEvent , typescript_name = DeviceProximityEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceProximityEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent)\n\n*This API requires the following crate features to be activated: `DeviceProximityEvent`*"]
    pub type DeviceProximityEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceProximityEvent" , js_name = value ) ]
    #[doc = "Getter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/value)\n\n*This API requires the following crate features to be activated: `DeviceProximityEvent`*"]
    pub fn value(this: &DeviceProximityEvent) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceProximityEvent" , js_name = min ) ]
    #[doc = "Getter for the `min` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/min)\n\n*This API requires the following crate features to be activated: `DeviceProximityEvent`*"]
    pub fn min(this: &DeviceProximityEvent) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceProximityEvent" , js_name = max ) ]
    #[doc = "Getter for the `max` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/max)\n\n*This API requires the following crate features to be activated: `DeviceProximityEvent`*"]
    pub fn max(this: &DeviceProximityEvent) -> f64;
    #[wasm_bindgen(catch, js_class = "DeviceProximityEvent", constructor)]
    #[doc = "The `new DeviceProximityEvent(..)` constructor, creating a new instance of `DeviceProximityEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/DeviceProximityEvent)\n\n*This API requires the following crate features to be activated: `DeviceProximityEvent`*"]
    pub fn new(this: &DeviceProximityEvent, type_: &str) -> Result<DeviceProximityEvent, JsValue>;
    #[cfg(feature = "DeviceProximityEventInit")]
    #[wasm_bindgen(catch, js_class = "DeviceProximityEvent", constructor)]
    #[doc = "The `new DeviceProximityEvent(..)` constructor, creating a new instance of `DeviceProximityEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/DeviceProximityEvent)\n\n*This API requires the following crate features to be activated: `DeviceProximityEvent`, `DeviceProximityEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &DeviceProximityEvent,
        type_: &str,
        event_init_dict: &DeviceProximityEventInit,
    ) -> Result<DeviceProximityEvent, JsValue>;
}
