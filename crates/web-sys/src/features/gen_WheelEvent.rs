use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = MouseEvent , extends = UiEvent , extends = Event , extends = :: js_sys :: Object , js_name = WheelEvent , typescript_name = WheelEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WheelEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent)\n\n*This API requires the following crate features to be activated: `WheelEvent`*"]
    pub type WheelEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = deltaX ) ]
    #[doc = "Getter for the `deltaX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaX)\n\n*This API requires the following crate features to be activated: `WheelEvent`*"]
    pub fn delta_x(this: &WheelEvent) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = deltaY ) ]
    #[doc = "Getter for the `deltaY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaY)\n\n*This API requires the following crate features to be activated: `WheelEvent`*"]
    pub fn delta_y(this: &WheelEvent) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = deltaZ ) ]
    #[doc = "Getter for the `deltaZ` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaZ)\n\n*This API requires the following crate features to be activated: `WheelEvent`*"]
    pub fn delta_z(this: &WheelEvent) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = deltaMode ) ]
    #[doc = "Getter for the `deltaMode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaMode)\n\n*This API requires the following crate features to be activated: `WheelEvent`*"]
    pub fn delta_mode(this: &WheelEvent) -> u32;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new WheelEvent(..)` constructor, creating a new instance of `WheelEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/WheelEvent)\n\n*This API requires the following crate features to be activated: `WheelEvent`*"]
    pub fn new(this: &WheelEvent, type_: &str) -> Result<WheelEvent, JsValue>;
    #[cfg(feature = "WheelEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new WheelEvent(..)` constructor, creating a new instance of `WheelEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/WheelEvent)\n\n*This API requires the following crate features to be activated: `WheelEvent`, `WheelEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &WheelEvent,
        type_: &str,
        event_init_dict: &WheelEventInit,
    ) -> Result<WheelEvent, JsValue>;
}
impl WheelEvent {
    pub const DOM_DELTA_PIXEL: u32 = 0u64 as u32;
    pub const DOM_DELTA_LINE: u32 = 1u64 as u32;
    pub const DOM_DELTA_PAGE: u32 = 2u64 as u32;
}
