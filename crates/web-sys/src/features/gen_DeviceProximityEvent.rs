use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = DeviceProximityEvent , typescript_type = "DeviceProximityEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DeviceProximityEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent)
    ///
    ///*This API requires the following crate features to be activated: `DeviceProximityEvent`*
    pub type DeviceProximityEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceProximityEvent" , js_name = value ) ]
    ///Getter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/value)
    ///
    ///*This API requires the following crate features to be activated: `DeviceProximityEvent`*
    pub fn value(this: &DeviceProximityEvent) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceProximityEvent" , js_name = min ) ]
    ///Getter for the `min` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/min)
    ///
    ///*This API requires the following crate features to be activated: `DeviceProximityEvent`*
    pub fn min(this: &DeviceProximityEvent) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceProximityEvent" , js_name = max ) ]
    ///Getter for the `max` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/max)
    ///
    ///*This API requires the following crate features to be activated: `DeviceProximityEvent`*
    pub fn max(this: &DeviceProximityEvent) -> f64;

    #[wasm_bindgen(catch, constructor, js_class = "DeviceProximityEvent")]
    ///The `new DeviceProximityEvent(..)` constructor, creating a new instance of `DeviceProximityEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/DeviceProximityEvent)
    ///
    ///*This API requires the following crate features to be activated: `DeviceProximityEvent`*
    pub fn new(type_: &str) -> Result<DeviceProximityEvent, JsValue>;

    #[cfg(feature = "DeviceProximityEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "DeviceProximityEvent")]
    ///The `new DeviceProximityEvent(..)` constructor, creating a new instance of `DeviceProximityEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/DeviceProximityEvent)
    ///
    ///*This API requires the following crate features to be activated: `DeviceProximityEvent`, `DeviceProximityEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &DeviceProximityEventInit,
    ) -> Result<DeviceProximityEvent, JsValue>;

}
