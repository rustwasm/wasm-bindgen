use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = UiEvent , extends = Event , extends = :: js_sys :: Object , js_name = CompositionEvent , typescript_type = "CompositionEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CompositionEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent)
    ///
    ///*This API requires the following crate features to be activated: `CompositionEvent`*
    pub type CompositionEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CompositionEvent" , js_name = data ) ]
    ///Getter for the `data` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/data)
    ///
    ///*This API requires the following crate features to be activated: `CompositionEvent`*
    pub fn data(this: &CompositionEvent) -> Option<String>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CompositionEvent" , js_name = locale ) ]
    ///Getter for the `locale` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/locale)
    ///
    ///*This API requires the following crate features to be activated: `CompositionEvent`*
    pub fn locale(this: &CompositionEvent) -> String;

    #[wasm_bindgen(catch, constructor, js_class = "CompositionEvent")]
    ///The `new CompositionEvent(..)` constructor, creating a new instance of `CompositionEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/CompositionEvent)
    ///
    ///*This API requires the following crate features to be activated: `CompositionEvent`*
    pub fn new(type_: &str) -> Result<CompositionEvent, JsValue>;

    #[cfg(feature = "CompositionEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "CompositionEvent")]
    ///The `new CompositionEvent(..)` constructor, creating a new instance of `CompositionEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/CompositionEvent)
    ///
    ///*This API requires the following crate features to be activated: `CompositionEvent`, `CompositionEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &CompositionEventInit,
    ) -> Result<CompositionEvent, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "CompositionEvent" , js_name = initCompositionEvent ) ]
    ///The `initCompositionEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)
    ///
    ///*This API requires the following crate features to be activated: `CompositionEvent`*
    pub fn init_composition_event(this: &CompositionEvent, type_arg: &str);

    # [ wasm_bindgen ( method , structural , js_class = "CompositionEvent" , js_name = initCompositionEvent ) ]
    ///The `initCompositionEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)
    ///
    ///*This API requires the following crate features to be activated: `CompositionEvent`*
    pub fn init_composition_event_with_can_bubble_arg(
        this: &CompositionEvent,
        type_arg: &str,
        can_bubble_arg: bool,
    );

    # [ wasm_bindgen ( method , structural , js_class = "CompositionEvent" , js_name = initCompositionEvent ) ]
    ///The `initCompositionEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)
    ///
    ///*This API requires the following crate features to be activated: `CompositionEvent`*
    pub fn init_composition_event_with_can_bubble_arg_and_cancelable_arg(
        this: &CompositionEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
    );

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "CompositionEvent" , js_name = initCompositionEvent ) ]
    ///The `initCompositionEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)
    ///
    ///*This API requires the following crate features to be activated: `CompositionEvent`, `Window`*
    pub fn init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg(
        this: &CompositionEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
    );

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "CompositionEvent" , js_name = initCompositionEvent ) ]
    ///The `initCompositionEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)
    ///
    ///*This API requires the following crate features to be activated: `CompositionEvent`, `Window`*
    pub fn init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_data_arg(
        this: &CompositionEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        data_arg: Option<&str>,
    );

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "CompositionEvent" , js_name = initCompositionEvent ) ]
    ///The `initCompositionEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)
    ///
    ///*This API requires the following crate features to be activated: `CompositionEvent`, `Window`*
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
