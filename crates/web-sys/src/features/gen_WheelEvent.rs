use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = MouseEvent , extends = UiEvent , extends = Event , extends = :: js_sys :: Object , js_name = WheelEvent , typescript_name = WheelEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WheelEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent)
    ///
    ///*This API requires the following crate features to be activated: `WheelEvent`*
    pub type WheelEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WheelEvent" , js_name = deltaX ) ]
    ///Getter for the `deltaX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaX)
    ///
    ///*This API requires the following crate features to be activated: `WheelEvent`*
    pub fn delta_x(this: &WheelEvent) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WheelEvent" , js_name = deltaY ) ]
    ///Getter for the `deltaY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaY)
    ///
    ///*This API requires the following crate features to be activated: `WheelEvent`*
    pub fn delta_y(this: &WheelEvent) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WheelEvent" , js_name = deltaZ ) ]
    ///Getter for the `deltaZ` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaZ)
    ///
    ///*This API requires the following crate features to be activated: `WheelEvent`*
    pub fn delta_z(this: &WheelEvent) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WheelEvent" , js_name = deltaMode ) ]
    ///Getter for the `deltaMode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaMode)
    ///
    ///*This API requires the following crate features to be activated: `WheelEvent`*
    pub fn delta_mode(this: &WheelEvent) -> u32;

    #[wasm_bindgen(catch, constructor, js_class = "WheelEvent")]
    ///The `new WheelEvent(..)` constructor, creating a new instance of `WheelEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/WheelEvent)
    ///
    ///*This API requires the following crate features to be activated: `WheelEvent`*
    pub fn new(type_: &str) -> Result<WheelEvent, JsValue>;

    #[cfg(feature = "WheelEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "WheelEvent")]
    ///The `new WheelEvent(..)` constructor, creating a new instance of `WheelEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/WheelEvent)
    ///
    ///*This API requires the following crate features to be activated: `WheelEvent`, `WheelEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &WheelEventInit,
    ) -> Result<WheelEvent, JsValue>;

}

impl WheelEvent {
    ///The `WheelEvent.DOM_DELTA_PIXEL` const.
    ///
    ///*This API requires the following crate features to be activated: `WheelEvent`*

    pub const DOM_DELTA_PIXEL: u32 = 0u64 as u32;

    ///The `WheelEvent.DOM_DELTA_LINE` const.
    ///
    ///*This API requires the following crate features to be activated: `WheelEvent`*

    pub const DOM_DELTA_LINE: u32 = 1u64 as u32;

    ///The `WheelEvent.DOM_DELTA_PAGE` const.
    ///
    ///*This API requires the following crate features to be activated: `WheelEvent`*

    pub const DOM_DELTA_PAGE: u32 = 2u64 as u32;
}
