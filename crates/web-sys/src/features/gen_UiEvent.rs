use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = UIEvent , typescript_name = UIEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UiEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    pub type UiEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "UIEvent" , js_name = view ) ]
    #[cfg(feature = "Window")]
    #[doc = "Getter for the `view` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/view)\n\n*This API requires the following crate features to be activated: `UiEvent`, `Window`*"]
    pub fn view(this: &UiEvent) -> Option<Window>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "UIEvent" , js_name = detail ) ]
    #[doc = "Getter for the `detail` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/detail)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn detail(this: &UiEvent) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "UIEvent" , js_name = layerX ) ]
    #[doc = "Getter for the `layerX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/layerX)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn layer_x(this: &UiEvent) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "UIEvent" , js_name = layerY ) ]
    #[doc = "Getter for the `layerY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/layerY)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn layer_y(this: &UiEvent) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "UIEvent" , js_name = pageX ) ]
    #[doc = "Getter for the `pageX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/pageX)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn page_x(this: &UiEvent) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "UIEvent" , js_name = pageY ) ]
    #[doc = "Getter for the `pageY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/pageY)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn page_y(this: &UiEvent) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "UIEvent" , js_name = which ) ]
    #[doc = "Getter for the `which` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/which)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn which(this: &UiEvent) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "UIEvent" , js_name = rangeParent ) ]
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `rangeParent` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/rangeParent)\n\n*This API requires the following crate features to be activated: `Node`, `UiEvent`*"]
    pub fn range_parent(this: &UiEvent) -> Option<Node>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "UIEvent" , js_name = rangeOffset ) ]
    #[doc = "Getter for the `rangeOffset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/rangeOffset)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn range_offset(this: &UiEvent) -> i32;
    #[wasm_bindgen(catch, js_class = "UIEvent", constructor)]
    #[doc = "The `new UiEvent(..)` constructor, creating a new instance of `UiEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/UIEvent)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn new(this: &UiEvent, type_: &str) -> Result<UiEvent, JsValue>;
    #[cfg(feature = "UiEventInit")]
    #[wasm_bindgen(catch, js_class = "UIEvent", constructor)]
    #[doc = "The `new UiEvent(..)` constructor, creating a new instance of `UiEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/UIEvent)\n\n*This API requires the following crate features to be activated: `UiEvent`, `UiEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &UiEvent,
        type_: &str,
        event_init_dict: &UiEventInit,
    ) -> Result<UiEvent, JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "UIEvent" , js_name = initUIEvent ) ]
    #[doc = "The `initUIEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn init_ui_event(this: &UiEvent, a_type: &str);
    # [ wasm_bindgen ( method , structural , js_class = "UIEvent" , js_name = initUIEvent ) ]
    #[doc = "The `initUIEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn init_ui_event_with_a_can_bubble(this: &UiEvent, a_type: &str, a_can_bubble: bool);
    # [ wasm_bindgen ( method , structural , js_class = "UIEvent" , js_name = initUIEvent ) ]
    #[doc = "The `initUIEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn init_ui_event_with_a_can_bubble_and_a_cancelable(
        this: &UiEvent,
        a_type: &str,
        a_can_bubble: bool,
        a_cancelable: bool,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "UIEvent" , js_name = initUIEvent ) ]
    #[doc = "The `initUIEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)\n\n*This API requires the following crate features to be activated: `UiEvent`, `Window`*"]
    pub fn init_ui_event_with_a_can_bubble_and_a_cancelable_and_a_view(
        this: &UiEvent,
        a_type: &str,
        a_can_bubble: bool,
        a_cancelable: bool,
        a_view: Option<&Window>,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "UIEvent" , js_name = initUIEvent ) ]
    #[doc = "The `initUIEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)\n\n*This API requires the following crate features to be activated: `UiEvent`, `Window`*"]
    pub fn init_ui_event_with_a_can_bubble_and_a_cancelable_and_a_view_and_a_detail(
        this: &UiEvent,
        a_type: &str,
        a_can_bubble: bool,
        a_cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
    );
}
impl UiEvent {
    pub const SCROLL_PAGE_UP: i32 = -32768i64 as i32;
    pub const SCROLL_PAGE_DOWN: i32 = 32768u64 as i32;
}
