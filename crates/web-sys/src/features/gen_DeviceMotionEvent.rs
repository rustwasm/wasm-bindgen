use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = DeviceMotionEvent , typescript_name = DeviceMotionEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceMotionEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent)\n\n*This API requires the following crate features to be activated: `DeviceMotionEvent`*"]
    pub type DeviceMotionEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceMotionEvent" , js_name = acceleration ) ]
    #[cfg(feature = "DeviceAcceleration")]
    #[doc = "Getter for the `acceleration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/acceleration)\n\n*This API requires the following crate features to be activated: `DeviceAcceleration`, `DeviceMotionEvent`*"]
    pub fn acceleration(this: &DeviceMotionEvent) -> Option<DeviceAcceleration>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceMotionEvent" , js_name = accelerationIncludingGravity ) ]
    #[cfg(feature = "DeviceAcceleration")]
    #[doc = "Getter for the `accelerationIncludingGravity` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/accelerationIncludingGravity)\n\n*This API requires the following crate features to be activated: `DeviceAcceleration`, `DeviceMotionEvent`*"]
    pub fn acceleration_including_gravity(this: &DeviceMotionEvent) -> Option<DeviceAcceleration>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceMotionEvent" , js_name = rotationRate ) ]
    #[cfg(feature = "DeviceRotationRate")]
    #[doc = "Getter for the `rotationRate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/rotationRate)\n\n*This API requires the following crate features to be activated: `DeviceMotionEvent`, `DeviceRotationRate`*"]
    pub fn rotation_rate(this: &DeviceMotionEvent) -> Option<DeviceRotationRate>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceMotionEvent" , js_name = interval ) ]
    #[doc = "Getter for the `interval` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/interval)\n\n*This API requires the following crate features to be activated: `DeviceMotionEvent`*"]
    pub fn interval(this: &DeviceMotionEvent) -> Option<f64>;
    #[wasm_bindgen(catch, js_class = "DeviceMotionEvent", constructor)]
    #[doc = "The `new DeviceMotionEvent(..)` constructor, creating a new instance of `DeviceMotionEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/DeviceMotionEvent)\n\n*This API requires the following crate features to be activated: `DeviceMotionEvent`*"]
    pub fn new(this: &DeviceMotionEvent, type_: &str) -> Result<DeviceMotionEvent, JsValue>;
    #[cfg(feature = "DeviceMotionEventInit")]
    #[wasm_bindgen(catch, js_class = "DeviceMotionEvent", constructor)]
    #[doc = "The `new DeviceMotionEvent(..)` constructor, creating a new instance of `DeviceMotionEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/DeviceMotionEvent)\n\n*This API requires the following crate features to be activated: `DeviceMotionEvent`, `DeviceMotionEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &DeviceMotionEvent,
        type_: &str,
        event_init_dict: &DeviceMotionEventInit,
    ) -> Result<DeviceMotionEvent, JsValue>;
}
