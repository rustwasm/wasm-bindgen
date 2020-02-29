use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = ErrorEvent , typescript_type = "ErrorEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ErrorEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent)
    ///
    ///*This API requires the following crate features to be activated: `ErrorEvent`*
    pub type ErrorEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ErrorEvent" , js_name = message ) ]
    ///Getter for the `message` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/message)
    ///
    ///*This API requires the following crate features to be activated: `ErrorEvent`*
    pub fn message(this: &ErrorEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ErrorEvent" , js_name = filename ) ]
    ///Getter for the `filename` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/filename)
    ///
    ///*This API requires the following crate features to be activated: `ErrorEvent`*
    pub fn filename(this: &ErrorEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ErrorEvent" , js_name = lineno ) ]
    ///Getter for the `lineno` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/lineno)
    ///
    ///*This API requires the following crate features to be activated: `ErrorEvent`*
    pub fn lineno(this: &ErrorEvent) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ErrorEvent" , js_name = colno ) ]
    ///Getter for the `colno` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/colno)
    ///
    ///*This API requires the following crate features to be activated: `ErrorEvent`*
    pub fn colno(this: &ErrorEvent) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ErrorEvent" , js_name = error ) ]
    ///Getter for the `error` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/error)
    ///
    ///*This API requires the following crate features to be activated: `ErrorEvent`*
    pub fn error(this: &ErrorEvent) -> ::wasm_bindgen::JsValue;

    #[wasm_bindgen(catch, constructor, js_class = "ErrorEvent")]
    ///The `new ErrorEvent(..)` constructor, creating a new instance of `ErrorEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/ErrorEvent)
    ///
    ///*This API requires the following crate features to be activated: `ErrorEvent`*
    pub fn new(type_: &str) -> Result<ErrorEvent, JsValue>;

    #[cfg(feature = "ErrorEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "ErrorEvent")]
    ///The `new ErrorEvent(..)` constructor, creating a new instance of `ErrorEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/ErrorEvent)
    ///
    ///*This API requires the following crate features to be activated: `ErrorEvent`, `ErrorEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &ErrorEventInit,
    ) -> Result<ErrorEvent, JsValue>;

}
