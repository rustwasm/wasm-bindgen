use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = UiEvent , extends = Event , extends = :: js_sys :: Object , js_name = ScrollAreaEvent , typescript_type = "ScrollAreaEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ScrollAreaEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent)
    ///
    ///*This API requires the following crate features to be activated: `ScrollAreaEvent`*
    pub type ScrollAreaEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ScrollAreaEvent" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/x)
    ///
    ///*This API requires the following crate features to be activated: `ScrollAreaEvent`*
    pub fn x(this: &ScrollAreaEvent) -> f32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ScrollAreaEvent" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/y)
    ///
    ///*This API requires the following crate features to be activated: `ScrollAreaEvent`*
    pub fn y(this: &ScrollAreaEvent) -> f32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ScrollAreaEvent" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/width)
    ///
    ///*This API requires the following crate features to be activated: `ScrollAreaEvent`*
    pub fn width(this: &ScrollAreaEvent) -> f32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ScrollAreaEvent" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/height)
    ///
    ///*This API requires the following crate features to be activated: `ScrollAreaEvent`*
    pub fn height(this: &ScrollAreaEvent) -> f32;

    # [ wasm_bindgen ( method , structural , js_class = "ScrollAreaEvent" , js_name = initScrollAreaEvent ) ]
    ///The `initScrollAreaEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)
    ///
    ///*This API requires the following crate features to be activated: `ScrollAreaEvent`*
    pub fn init_scroll_area_event(this: &ScrollAreaEvent, type_: &str);

    # [ wasm_bindgen ( method , structural , js_class = "ScrollAreaEvent" , js_name = initScrollAreaEvent ) ]
    ///The `initScrollAreaEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)
    ///
    ///*This API requires the following crate features to be activated: `ScrollAreaEvent`*
    pub fn init_scroll_area_event_with_can_bubble(
        this: &ScrollAreaEvent,
        type_: &str,
        can_bubble: bool,
    );

    # [ wasm_bindgen ( method , structural , js_class = "ScrollAreaEvent" , js_name = initScrollAreaEvent ) ]
    ///The `initScrollAreaEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)
    ///
    ///*This API requires the following crate features to be activated: `ScrollAreaEvent`*
    pub fn init_scroll_area_event_with_can_bubble_and_cancelable(
        this: &ScrollAreaEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
    );

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "ScrollAreaEvent" , js_name = initScrollAreaEvent ) ]
    ///The `initScrollAreaEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)
    ///
    ///*This API requires the following crate features to be activated: `ScrollAreaEvent`, `Window`*
    pub fn init_scroll_area_event_with_can_bubble_and_cancelable_and_view(
        this: &ScrollAreaEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
    );

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "ScrollAreaEvent" , js_name = initScrollAreaEvent ) ]
    ///The `initScrollAreaEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)
    ///
    ///*This API requires the following crate features to be activated: `ScrollAreaEvent`, `Window`*
    pub fn init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail(
        this: &ScrollAreaEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
    );

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "ScrollAreaEvent" , js_name = initScrollAreaEvent ) ]
    ///The `initScrollAreaEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)
    ///
    ///*This API requires the following crate features to be activated: `ScrollAreaEvent`, `Window`*
    pub fn init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x(
        this: &ScrollAreaEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        x: f32,
    );

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "ScrollAreaEvent" , js_name = initScrollAreaEvent ) ]
    ///The `initScrollAreaEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)
    ///
    ///*This API requires the following crate features to be activated: `ScrollAreaEvent`, `Window`*
    pub fn init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y(
        this: &ScrollAreaEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        x: f32,
        y: f32,
    );

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "ScrollAreaEvent" , js_name = initScrollAreaEvent ) ]
    ///The `initScrollAreaEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)
    ///
    ///*This API requires the following crate features to be activated: `ScrollAreaEvent`, `Window`*
    pub fn init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y_and_width(
        this: &ScrollAreaEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        x: f32,
        y: f32,
        width: f32,
    );

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "ScrollAreaEvent" , js_name = initScrollAreaEvent ) ]
    ///The `initScrollAreaEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)
    ///
    ///*This API requires the following crate features to be activated: `ScrollAreaEvent`, `Window`*
    pub fn init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y_and_width_and_height(
        this: &ScrollAreaEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    );

}
