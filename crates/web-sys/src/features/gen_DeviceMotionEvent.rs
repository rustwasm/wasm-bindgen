use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = DeviceMotionEvent , typescript_type = "DeviceMotionEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DeviceMotionEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent)
    ///
    ///*This API requires the following crate features to be activated: `DeviceMotionEvent`*
    pub type DeviceMotionEvent;

    #[cfg(feature = "DeviceAcceleration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceMotionEvent" , js_name = acceleration ) ]
    ///Getter for the `acceleration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/acceleration)
    ///
    ///*This API requires the following crate features to be activated: `DeviceAcceleration`, `DeviceMotionEvent`*
    pub fn acceleration(this: &DeviceMotionEvent) -> Option<DeviceAcceleration>;

    #[cfg(feature = "DeviceAcceleration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceMotionEvent" , js_name = accelerationIncludingGravity ) ]
    ///Getter for the `accelerationIncludingGravity` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/accelerationIncludingGravity)
    ///
    ///*This API requires the following crate features to be activated: `DeviceAcceleration`, `DeviceMotionEvent`*
    pub fn acceleration_including_gravity(this: &DeviceMotionEvent) -> Option<DeviceAcceleration>;

    #[cfg(feature = "DeviceRotationRate")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceMotionEvent" , js_name = rotationRate ) ]
    ///Getter for the `rotationRate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/rotationRate)
    ///
    ///*This API requires the following crate features to be activated: `DeviceMotionEvent`, `DeviceRotationRate`*
    pub fn rotation_rate(this: &DeviceMotionEvent) -> Option<DeviceRotationRate>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceMotionEvent" , js_name = interval ) ]
    ///Getter for the `interval` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/interval)
    ///
    ///*This API requires the following crate features to be activated: `DeviceMotionEvent`*
    pub fn interval(this: &DeviceMotionEvent) -> Option<f64>;

    #[wasm_bindgen(catch, constructor, js_class = "DeviceMotionEvent")]
    ///The `new DeviceMotionEvent(..)` constructor, creating a new instance of `DeviceMotionEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/DeviceMotionEvent)
    ///
    ///*This API requires the following crate features to be activated: `DeviceMotionEvent`*
    pub fn new(type_: &str) -> Result<DeviceMotionEvent, JsValue>;

    #[cfg(feature = "DeviceMotionEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "DeviceMotionEvent")]
    ///The `new DeviceMotionEvent(..)` constructor, creating a new instance of `DeviceMotionEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/DeviceMotionEvent)
    ///
    ///*This API requires the following crate features to be activated: `DeviceMotionEvent`, `DeviceMotionEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &DeviceMotionEventInit,
    ) -> Result<DeviceMotionEvent, JsValue>;

}
