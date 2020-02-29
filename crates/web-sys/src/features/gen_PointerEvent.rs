use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = MouseEvent , extends = UiEvent , extends = Event , extends = :: js_sys :: Object , js_name = PointerEvent , typescript_name = PointerEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PointerEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent)
    ///
    ///*This API requires the following crate features to be activated: `PointerEvent`*
    pub type PointerEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PointerEvent" , js_name = pointerId ) ]
    ///Getter for the `pointerId` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pointerId)
    ///
    ///*This API requires the following crate features to be activated: `PointerEvent`*
    pub fn pointer_id(this: &PointerEvent) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PointerEvent" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/width)
    ///
    ///*This API requires the following crate features to be activated: `PointerEvent`*
    pub fn width(this: &PointerEvent) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PointerEvent" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/height)
    ///
    ///*This API requires the following crate features to be activated: `PointerEvent`*
    pub fn height(this: &PointerEvent) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PointerEvent" , js_name = pressure ) ]
    ///Getter for the `pressure` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pressure)
    ///
    ///*This API requires the following crate features to be activated: `PointerEvent`*
    pub fn pressure(this: &PointerEvent) -> f32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PointerEvent" , js_name = tangentialPressure ) ]
    ///Getter for the `tangentialPressure` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tangentialPressure)
    ///
    ///*This API requires the following crate features to be activated: `PointerEvent`*
    pub fn tangential_pressure(this: &PointerEvent) -> f32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PointerEvent" , js_name = tiltX ) ]
    ///Getter for the `tiltX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tiltX)
    ///
    ///*This API requires the following crate features to be activated: `PointerEvent`*
    pub fn tilt_x(this: &PointerEvent) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PointerEvent" , js_name = tiltY ) ]
    ///Getter for the `tiltY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tiltY)
    ///
    ///*This API requires the following crate features to be activated: `PointerEvent`*
    pub fn tilt_y(this: &PointerEvent) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PointerEvent" , js_name = twist ) ]
    ///Getter for the `twist` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/twist)
    ///
    ///*This API requires the following crate features to be activated: `PointerEvent`*
    pub fn twist(this: &PointerEvent) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PointerEvent" , js_name = pointerType ) ]
    ///Getter for the `pointerType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pointerType)
    ///
    ///*This API requires the following crate features to be activated: `PointerEvent`*
    pub fn pointer_type(this: &PointerEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PointerEvent" , js_name = isPrimary ) ]
    ///Getter for the `isPrimary` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/isPrimary)
    ///
    ///*This API requires the following crate features to be activated: `PointerEvent`*
    pub fn is_primary(this: &PointerEvent) -> bool;

    #[wasm_bindgen(catch, constructor, js_class = "PointerEvent")]
    ///The `new PointerEvent(..)` constructor, creating a new instance of `PointerEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/PointerEvent)
    ///
    ///*This API requires the following crate features to be activated: `PointerEvent`*
    pub fn new(type_: &str) -> Result<PointerEvent, JsValue>;

    #[cfg(feature = "PointerEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "PointerEvent")]
    ///The `new PointerEvent(..)` constructor, creating a new instance of `PointerEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/PointerEvent)
    ///
    ///*This API requires the following crate features to be activated: `PointerEvent`, `PointerEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &PointerEventInit,
    ) -> Result<PointerEvent, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "PointerEvent" , js_name = getCoalescedEvents ) ]
    ///The `getCoalescedEvents()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/getCoalescedEvents)
    ///
    ///*This API requires the following crate features to be activated: `PointerEvent`*
    pub fn get_coalesced_events(this: &PointerEvent) -> ::js_sys::Array;

}
