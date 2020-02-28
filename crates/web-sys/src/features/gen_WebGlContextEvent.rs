use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = WebGLContextEvent , typescript_name = WebGLContextEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGlContextEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent)\n\n*This API requires the following crate features to be activated: `WebGlContextEvent`*"]
    pub type WebGlContextEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "WebGLContextEvent" , js_name = statusMessage ) ]
    #[doc = "Getter for the `statusMessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent/statusMessage)\n\n*This API requires the following crate features to be activated: `WebGlContextEvent`*"]
    pub fn status_message(this: &WebGlContextEvent) -> String;
    #[wasm_bindgen(catch, js_class = "WebGLContextEvent", constructor)]
    #[doc = "The `new WebGlContextEvent(..)` constructor, creating a new instance of `WebGlContextEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent/WebGLContextEvent)\n\n*This API requires the following crate features to be activated: `WebGlContextEvent`*"]
    pub fn new(this: &WebGlContextEvent, type_: &str) -> Result<WebGlContextEvent, JsValue>;
    #[cfg(feature = "WebGlContextEventInit")]
    #[wasm_bindgen(catch, js_class = "WebGLContextEvent", constructor)]
    #[doc = "The `new WebGlContextEvent(..)` constructor, creating a new instance of `WebGlContextEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent/WebGLContextEvent)\n\n*This API requires the following crate features to be activated: `WebGlContextEvent`, `WebGlContextEventInit`*"]
    pub fn new_with_event_init(
        this: &WebGlContextEvent,
        type_: &str,
        event_init: &WebGlContextEventInit,
    ) -> Result<WebGlContextEvent, JsValue>;
}
