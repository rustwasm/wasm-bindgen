use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = UiEvent , extends = Event , extends = :: js_sys :: Object , js_name = MouseEvent , typescript_name = MouseEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MouseEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub type MouseEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = screenX ) ]
    #[doc = "Getter for the `screenX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/screenX)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn screen_x(this: &MouseEvent) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_name = screenY ) ]
    #[doc = "Getter for the `screenY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/screenY)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn screen_y(this: &MouseEvent) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_name = clientX ) ]
    #[doc = "Getter for the `clientX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/clientX)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn client_x(this: &MouseEvent) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_name = clientY ) ]
    #[doc = "Getter for the `clientY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/clientY)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn client_y(this: &MouseEvent) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/x)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn x(this: &MouseEvent) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/y)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn y(this: &MouseEvent) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_name = offsetX ) ]
    #[doc = "Getter for the `offsetX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/offsetX)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn offset_x(this: &MouseEvent) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_name = offsetY ) ]
    #[doc = "Getter for the `offsetY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/offsetY)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn offset_y(this: &MouseEvent) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_name = ctrlKey ) ]
    #[doc = "Getter for the `ctrlKey` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/ctrlKey)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn ctrl_key(this: &MouseEvent) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = shiftKey ) ]
    #[doc = "Getter for the `shiftKey` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/shiftKey)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn shift_key(this: &MouseEvent) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = altKey ) ]
    #[doc = "Getter for the `altKey` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/altKey)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn alt_key(this: &MouseEvent) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = metaKey ) ]
    #[doc = "Getter for the `metaKey` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/metaKey)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn meta_key(this: &MouseEvent) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = button ) ]
    #[doc = "Getter for the `button` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/button)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn button(this: &MouseEvent) -> i16;
    # [ wasm_bindgen ( structural , method , getter , js_name = buttons ) ]
    #[doc = "Getter for the `buttons` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/buttons)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn buttons(this: &MouseEvent) -> u16;
    # [ wasm_bindgen ( structural , method , getter , js_name = relatedTarget ) ]
    #[cfg(feature = "EventTarget")]
    #[doc = "Getter for the `relatedTarget` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/relatedTarget)\n\n*This API requires the following crate features to be activated: `EventTarget`, `MouseEvent`*"]
    pub fn related_target(this: &MouseEvent) -> Option<EventTarget>;
    # [ wasm_bindgen ( structural , method , getter , js_name = region ) ]
    #[doc = "Getter for the `region` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/region)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn region(this: &MouseEvent) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = movementX ) ]
    #[doc = "Getter for the `movementX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/movementX)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn movement_x(this: &MouseEvent) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_name = movementY ) ]
    #[doc = "Getter for the `movementY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/movementY)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn movement_y(this: &MouseEvent) -> i32;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new MouseEvent(..)` constructor, creating a new instance of `MouseEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/MouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn new(this: &MouseEvent, type_arg: &str) -> Result<MouseEvent, JsValue>;
    #[cfg(feature = "MouseEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new MouseEvent(..)` constructor, creating a new instance of `MouseEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/MouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `MouseEventInit`*"]
    pub fn new_with_mouse_event_init_dict(
        this: &MouseEvent,
        type_arg: &str,
        mouse_event_init_dict: &MouseEventInit,
    ) -> Result<MouseEvent, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = getModifierState ) ]
    #[doc = "The `getModifierState()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/getModifierState)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn get_modifier_state(this: &MouseEvent, key_arg: &str) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = initMouseEvent ) ]
    #[doc = "The `initMouseEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn init_mouse_event(this: &MouseEvent, type_arg: &str);
    # [ wasm_bindgen ( method , structural , js_name = initMouseEvent ) ]
    #[doc = "The `initMouseEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn init_mouse_event_with_can_bubble_arg(
        this: &MouseEvent,
        type_arg: &str,
        can_bubble_arg: bool,
    );
    # [ wasm_bindgen ( method , structural , js_name = initMouseEvent ) ]
    #[doc = "The `initMouseEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg(
        this: &MouseEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initMouseEvent ) ]
    #[doc = "The `initMouseEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg(
        this: &MouseEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initMouseEvent ) ]
    #[doc = "The `initMouseEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg(
        this: &MouseEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initMouseEvent ) ]
    #[doc = "The `initMouseEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg(
        this: &MouseEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
        screen_x_arg: i32,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initMouseEvent ) ]
    #[doc = "The `initMouseEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg(
        this: &MouseEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initMouseEvent ) ]
    #[doc = "The `initMouseEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg(
        this: &MouseEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initMouseEvent ) ]
    #[doc = "The `initMouseEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg(
        this: &MouseEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initMouseEvent ) ]
    #[doc = "The `initMouseEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg(
        this: &MouseEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
        ctrl_key_arg: bool,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initMouseEvent ) ]
    #[doc = "The `initMouseEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg(
        this: &MouseEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
        ctrl_key_arg: bool,
        alt_key_arg: bool,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initMouseEvent ) ]
    #[doc = "The `initMouseEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg(
        this: &MouseEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
        ctrl_key_arg: bool,
        alt_key_arg: bool,
        shift_key_arg: bool,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initMouseEvent ) ]
    #[doc = "The `initMouseEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg(
        this: &MouseEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
        ctrl_key_arg: bool,
        alt_key_arg: bool,
        shift_key_arg: bool,
        meta_key_arg: bool,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initMouseEvent ) ]
    #[doc = "The `initMouseEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_and_button_arg(
        this: &MouseEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
        ctrl_key_arg: bool,
        alt_key_arg: bool,
        shift_key_arg: bool,
        meta_key_arg: bool,
        button_arg: i16,
    );
    #[cfg(all(feature = "EventTarget", feature = "Window",))]
    # [ wasm_bindgen ( method , structural , js_name = initMouseEvent ) ]
    #[doc = "The `initMouseEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `EventTarget`, `MouseEvent`, `Window`*"]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_and_button_arg_and_related_target_arg(
        this: &MouseEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
        ctrl_key_arg: bool,
        alt_key_arg: bool,
        shift_key_arg: bool,
        meta_key_arg: bool,
        button_arg: i16,
        related_target_arg: Option<&EventTarget>,
    );
}
