use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = PopStateEvent , typescript_name = PopStateEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PopStateEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopStateEvent)
    ///
    ///*This API requires the following crate features to be activated: `PopStateEvent`*
    pub type PopStateEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PopStateEvent" , js_name = state ) ]
    ///Getter for the `state` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopStateEvent/state)
    ///
    ///*This API requires the following crate features to be activated: `PopStateEvent`*
    pub fn state(this: &PopStateEvent) -> ::wasm_bindgen::JsValue;

    #[wasm_bindgen(catch, constructor, js_class = "PopStateEvent")]
    ///The `new PopStateEvent(..)` constructor, creating a new instance of `PopStateEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopStateEvent/PopStateEvent)
    ///
    ///*This API requires the following crate features to be activated: `PopStateEvent`*
    pub fn new(type_: &str) -> Result<PopStateEvent, JsValue>;

    #[cfg(feature = "PopStateEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "PopStateEvent")]
    ///The `new PopStateEvent(..)` constructor, creating a new instance of `PopStateEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopStateEvent/PopStateEvent)
    ///
    ///*This API requires the following crate features to be activated: `PopStateEvent`, `PopStateEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &PopStateEventInit,
    ) -> Result<PopStateEvent, JsValue>;

}
