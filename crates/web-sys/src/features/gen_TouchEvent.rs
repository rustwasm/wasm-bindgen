use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = UiEvent , extends = Event , extends = :: js_sys :: Object , js_name = TouchEvent , typescript_name = TouchEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TouchEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`*"]
    pub type TouchEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = touches ) ]
    #[cfg(feature = "TouchList")]
    #[doc = "Getter for the `touches` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/touches)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`*"]
    pub fn touches(this: &TouchEvent) -> TouchList;
    # [ wasm_bindgen ( structural , method , getter , js_name = targetTouches ) ]
    #[cfg(feature = "TouchList")]
    #[doc = "Getter for the `targetTouches` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/targetTouches)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`*"]
    pub fn target_touches(this: &TouchEvent) -> TouchList;
    # [ wasm_bindgen ( structural , method , getter , js_name = changedTouches ) ]
    #[cfg(feature = "TouchList")]
    #[doc = "Getter for the `changedTouches` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/changedTouches)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`*"]
    pub fn changed_touches(this: &TouchEvent) -> TouchList;
    # [ wasm_bindgen ( structural , method , getter , js_name = altKey ) ]
    #[doc = "Getter for the `altKey` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/altKey)\n\n*This API requires the following crate features to be activated: `TouchEvent`*"]
    pub fn alt_key(this: &TouchEvent) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = metaKey ) ]
    #[doc = "Getter for the `metaKey` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/metaKey)\n\n*This API requires the following crate features to be activated: `TouchEvent`*"]
    pub fn meta_key(this: &TouchEvent) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = ctrlKey ) ]
    #[doc = "Getter for the `ctrlKey` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/ctrlKey)\n\n*This API requires the following crate features to be activated: `TouchEvent`*"]
    pub fn ctrl_key(this: &TouchEvent) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = shiftKey ) ]
    #[doc = "Getter for the `shiftKey` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/shiftKey)\n\n*This API requires the following crate features to be activated: `TouchEvent`*"]
    pub fn shift_key(this: &TouchEvent) -> bool;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new TouchEvent(..)` constructor, creating a new instance of `TouchEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/TouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`*"]
    pub fn new(this: &TouchEvent, type_: &str) -> Result<TouchEvent, JsValue>;
    #[cfg(feature = "TouchEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new TouchEvent(..)` constructor, creating a new instance of `TouchEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/TouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `TouchEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &TouchEvent,
        type_: &str,
        event_init_dict: &TouchEventInit,
    ) -> Result<TouchEvent, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = initTouchEvent ) ]
    #[doc = "The `initTouchEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`*"]
    pub fn init_touch_event(this: &TouchEvent, type_: &str);
    # [ wasm_bindgen ( method , structural , js_name = initTouchEvent ) ]
    #[doc = "The `initTouchEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`*"]
    pub fn init_touch_event_with_can_bubble(this: &TouchEvent, type_: &str, can_bubble: bool);
    # [ wasm_bindgen ( method , structural , js_name = initTouchEvent ) ]
    #[doc = "The `initTouchEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`*"]
    pub fn init_touch_event_with_can_bubble_and_cancelable(
        this: &TouchEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initTouchEvent ) ]
    #[doc = "The `initTouchEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view(
        this: &TouchEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initTouchEvent ) ]
    #[doc = "The `initTouchEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail(
        this: &TouchEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initTouchEvent ) ]
    #[doc = "The `initTouchEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key(
        this: &TouchEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        ctrl_key: bool,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initTouchEvent ) ]
    #[doc = "The `initTouchEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key(
        this: &TouchEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        ctrl_key: bool,
        alt_key: bool,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initTouchEvent ) ]
    #[doc = "The `initTouchEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key(
        this: &TouchEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initTouchEvent ) ]
    #[doc = "The `initTouchEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key(
        this: &TouchEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
    );
    #[cfg(all(feature = "TouchList", feature = "Window",))]
    # [ wasm_bindgen ( method , structural , js_name = initTouchEvent ) ]
    #[doc = "The `initTouchEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`, `Window`*"]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches(
        this: &TouchEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
        touches: Option<&TouchList>,
    );
    #[cfg(all(feature = "TouchList", feature = "Window",))]
    # [ wasm_bindgen ( method , structural , js_name = initTouchEvent ) ]
    #[doc = "The `initTouchEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`, `Window`*"]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_and_target_touches(
        this: &TouchEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
        touches: Option<&TouchList>,
        target_touches: Option<&TouchList>,
    );
    #[cfg(all(feature = "TouchList", feature = "Window",))]
    # [ wasm_bindgen ( method , structural , js_name = initTouchEvent ) ]
    #[doc = "The `initTouchEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`, `Window`*"]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_and_target_touches_and_changed_touches(
        this: &TouchEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
        touches: Option<&TouchList>,
        target_touches: Option<&TouchList>,
        changed_touches: Option<&TouchList>,
    );
}
