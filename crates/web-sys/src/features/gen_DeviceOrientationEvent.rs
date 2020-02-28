use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = DeviceOrientationEvent , typescript_name = DeviceOrientationEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceOrientationEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    pub type DeviceOrientationEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceOrientationEvent" , js_name = alpha ) ]
    #[doc = "Getter for the `alpha` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/alpha)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    pub fn alpha(this: &DeviceOrientationEvent) -> Option<f64>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceOrientationEvent" , js_name = beta ) ]
    #[doc = "Getter for the `beta` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/beta)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    pub fn beta(this: &DeviceOrientationEvent) -> Option<f64>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceOrientationEvent" , js_name = gamma ) ]
    #[doc = "Getter for the `gamma` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/gamma)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    pub fn gamma(this: &DeviceOrientationEvent) -> Option<f64>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceOrientationEvent" , js_name = absolute ) ]
    #[doc = "Getter for the `absolute` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/absolute)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    pub fn absolute(this: &DeviceOrientationEvent) -> bool;
    #[wasm_bindgen(catch, js_class = "DeviceOrientationEvent", constructor)]
    #[doc = "The `new DeviceOrientationEvent(..)` constructor, creating a new instance of `DeviceOrientationEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/DeviceOrientationEvent)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    pub fn new(
        this: &DeviceOrientationEvent,
        type_: &str,
    ) -> Result<DeviceOrientationEvent, JsValue>;
    #[cfg(feature = "DeviceOrientationEventInit")]
    #[wasm_bindgen(catch, js_class = "DeviceOrientationEvent", constructor)]
    #[doc = "The `new DeviceOrientationEvent(..)` constructor, creating a new instance of `DeviceOrientationEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/DeviceOrientationEvent)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`, `DeviceOrientationEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &DeviceOrientationEvent,
        type_: &str,
        event_init_dict: &DeviceOrientationEventInit,
    ) -> Result<DeviceOrientationEvent, JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "DeviceOrientationEvent" , js_name = initDeviceOrientationEvent ) ]
    #[doc = "The `initDeviceOrientationEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/initDeviceOrientationEvent)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    pub fn init_device_orientation_event(this: &DeviceOrientationEvent, type_: &str);
    # [ wasm_bindgen ( method , structural , js_class = "DeviceOrientationEvent" , js_name = initDeviceOrientationEvent ) ]
    #[doc = "The `initDeviceOrientationEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/initDeviceOrientationEvent)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    pub fn init_device_orientation_event_with_can_bubble(
        this: &DeviceOrientationEvent,
        type_: &str,
        can_bubble: bool,
    );
    # [ wasm_bindgen ( method , structural , js_class = "DeviceOrientationEvent" , js_name = initDeviceOrientationEvent ) ]
    #[doc = "The `initDeviceOrientationEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/initDeviceOrientationEvent)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    pub fn init_device_orientation_event_with_can_bubble_and_cancelable(
        this: &DeviceOrientationEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
    );
    # [ wasm_bindgen ( method , structural , js_class = "DeviceOrientationEvent" , js_name = initDeviceOrientationEvent ) ]
    #[doc = "The `initDeviceOrientationEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/initDeviceOrientationEvent)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    pub fn init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha(
        this: &DeviceOrientationEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        alpha: Option<f64>,
    );
    # [ wasm_bindgen ( method , structural , js_class = "DeviceOrientationEvent" , js_name = initDeviceOrientationEvent ) ]
    #[doc = "The `initDeviceOrientationEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/initDeviceOrientationEvent)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    pub fn init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta(
        this: &DeviceOrientationEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        alpha: Option<f64>,
        beta: Option<f64>,
    );
    # [ wasm_bindgen ( method , structural , js_class = "DeviceOrientationEvent" , js_name = initDeviceOrientationEvent ) ]
    #[doc = "The `initDeviceOrientationEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/initDeviceOrientationEvent)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    pub fn init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta_and_gamma(
        this: &DeviceOrientationEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        alpha: Option<f64>,
        beta: Option<f64>,
        gamma: Option<f64>,
    );
    # [ wasm_bindgen ( method , structural , js_class = "DeviceOrientationEvent" , js_name = initDeviceOrientationEvent ) ]
    #[doc = "The `initDeviceOrientationEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/initDeviceOrientationEvent)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    pub fn init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta_and_gamma_and_absolute(
        this: &DeviceOrientationEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        alpha: Option<f64>,
        beta: Option<f64>,
        gamma: Option<f64>,
        absolute: bool,
    );
}
