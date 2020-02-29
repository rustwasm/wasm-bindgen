use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = CloseEvent , typescript_name = CloseEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CloseEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent)
    ///
    ///*This API requires the following crate features to be activated: `CloseEvent`*
    pub type CloseEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CloseEvent" , js_name = wasClean ) ]
    ///Getter for the `wasClean` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/wasClean)
    ///
    ///*This API requires the following crate features to be activated: `CloseEvent`*
    pub fn was_clean(this: &CloseEvent) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CloseEvent" , js_name = code ) ]
    ///Getter for the `code` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/code)
    ///
    ///*This API requires the following crate features to be activated: `CloseEvent`*
    pub fn code(this: &CloseEvent) -> u16;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CloseEvent" , js_name = reason ) ]
    ///Getter for the `reason` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/reason)
    ///
    ///*This API requires the following crate features to be activated: `CloseEvent`*
    pub fn reason(this: &CloseEvent) -> String;

    #[wasm_bindgen(catch, constructor, js_class = "CloseEvent")]
    ///The `new CloseEvent(..)` constructor, creating a new instance of `CloseEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/CloseEvent)
    ///
    ///*This API requires the following crate features to be activated: `CloseEvent`*
    pub fn new(type_: &str) -> Result<CloseEvent, JsValue>;

    #[cfg(feature = "CloseEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "CloseEvent")]
    ///The `new CloseEvent(..)` constructor, creating a new instance of `CloseEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/CloseEvent)
    ///
    ///*This API requires the following crate features to be activated: `CloseEvent`, `CloseEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &CloseEventInit,
    ) -> Result<CloseEvent, JsValue>;

}
