#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = XRWebGLLayer , typescript_type = "XRWebGLLayer" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrWebGlLayer` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrWebGlLayer;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebGlFramebuffer")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "XRWebGLLayer" , js_name = framebuffer ) ]
    #[doc = "Getter for the `framebuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/framebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlFramebuffer`, `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn framebuffer(this: &XrWebGlLayer) -> WebGlFramebuffer;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "WebGlRenderingContext", feature = "XrSession",))]
    #[wasm_bindgen(catch, constructor, js_class = "XRWebGLLayer")]
    #[doc = "The `new XrWebGlLayer(..)` constructor, creating a new instance of `XrWebGlLayer`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/XRWebGLLayer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `XrSession`, `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_web_gl_rendering_context(
        session: &XrSession,
        context: &WebGlRenderingContext,
    ) -> Result<XrWebGlLayer, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "WebGl2RenderingContext", feature = "XrSession",))]
    #[wasm_bindgen(catch, constructor, js_class = "XRWebGLLayer")]
    #[doc = "The `new XrWebGlLayer(..)` constructor, creating a new instance of `XrWebGlLayer`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/XRWebGLLayer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `XrSession`, `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_web_gl2_rendering_context(
        session: &XrSession,
        context: &WebGl2RenderingContext,
    ) -> Result<XrWebGlLayer, JsValue>;
}
