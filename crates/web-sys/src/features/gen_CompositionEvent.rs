use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = UiEvent , extends = Event , extends = :: js_sys :: Object , js_name = CompositionEvent , typescript_name = CompositionEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CompositionEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent)\n\n*This API requires the following crate features to be activated: `CompositionEvent`*"]
    pub type CompositionEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = data ) ]
    #[doc = "Getter for the `data` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/data)\n\n*This API requires the following crate features to be activated: `CompositionEvent`*"]
    pub fn data(this: &CompositionEvent) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = locale ) ]
    #[doc = "Getter for the `locale` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/locale)\n\n*This API requires the following crate features to be activated: `CompositionEvent`*"]
    pub fn locale(this: &CompositionEvent) -> String;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new CompositionEvent(..)` constructor, creating a new instance of `CompositionEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/CompositionEvent)\n\n*This API requires the following crate features to be activated: `CompositionEvent`*"]
    pub fn new(this: &CompositionEvent, type_: &str) -> Result<CompositionEvent, JsValue>;
    #[cfg(feature = "CompositionEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new CompositionEvent(..)` constructor, creating a new instance of `CompositionEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/CompositionEvent)\n\n*This API requires the following crate features to be activated: `CompositionEvent`, `CompositionEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &CompositionEvent,
        type_: &str,
        event_init_dict: &CompositionEventInit,
    ) -> Result<CompositionEvent, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = initCompositionEvent ) ]
    #[doc = "The `initCompositionEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)\n\n*This API requires the following crate features to be activated: `CompositionEvent`*"]
    pub fn init_composition_event(this: &CompositionEvent, type_arg: &str);
    # [ wasm_bindgen ( method , structural , js_name = initCompositionEvent ) ]
    #[doc = "The `initCompositionEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)\n\n*This API requires the following crate features to be activated: `CompositionEvent`*"]
    pub fn init_composition_event_with_can_bubble_arg(
        this: &CompositionEvent,
        type_arg: &str,
        can_bubble_arg: bool,
    );
    # [ wasm_bindgen ( method , structural , js_name = initCompositionEvent ) ]
    #[doc = "The `initCompositionEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)\n\n*This API requires the following crate features to be activated: `CompositionEvent`*"]
    pub fn init_composition_event_with_can_bubble_arg_and_cancelable_arg(
        this: &CompositionEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initCompositionEvent ) ]
    #[doc = "The `initCompositionEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)\n\n*This API requires the following crate features to be activated: `CompositionEvent`, `Window`*"]
    pub fn init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg(
        this: &CompositionEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initCompositionEvent ) ]
    #[doc = "The `initCompositionEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)\n\n*This API requires the following crate features to be activated: `CompositionEvent`, `Window`*"]
    pub fn init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_data_arg(
        this: &CompositionEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        data_arg: Option<&str>,
    );
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initCompositionEvent ) ]
    #[doc = "The `initCompositionEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)\n\n*This API requires the following crate features to be activated: `CompositionEvent`, `Window`*"]
    pub fn init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_data_arg_and_locale_arg(
        this: &CompositionEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        data_arg: Option<&str>,
        locale_arg: &str,
    );
}
