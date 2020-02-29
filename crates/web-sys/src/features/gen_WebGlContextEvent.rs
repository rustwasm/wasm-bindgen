use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = WebGLContextEvent , typescript_type = "WebGLContextEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebGlContextEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent)
    ///
    ///*This API requires the following crate features to be activated: `WebGlContextEvent`*
    pub type WebGlContextEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebGLContextEvent" , js_name = statusMessage ) ]
    ///Getter for the `statusMessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent/statusMessage)
    ///
    ///*This API requires the following crate features to be activated: `WebGlContextEvent`*
    pub fn status_message(this: &WebGlContextEvent) -> String;

    #[wasm_bindgen(catch, constructor, js_class = "WebGLContextEvent")]
    ///The `new WebGlContextEvent(..)` constructor, creating a new instance of `WebGlContextEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent/WebGLContextEvent)
    ///
    ///*This API requires the following crate features to be activated: `WebGlContextEvent`*
    pub fn new(type_: &str) -> Result<WebGlContextEvent, JsValue>;

    #[cfg(feature = "WebGlContextEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "WebGLContextEvent")]
    ///The `new WebGlContextEvent(..)` constructor, creating a new instance of `WebGlContextEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent/WebGLContextEvent)
    ///
    ///*This API requires the following crate features to be activated: `WebGlContextEvent`, `WebGlContextEventInit`*
    pub fn new_with_event_init(
        type_: &str,
        event_init: &WebGlContextEventInit,
    ) -> Result<WebGlContextEvent, JsValue>;

}
