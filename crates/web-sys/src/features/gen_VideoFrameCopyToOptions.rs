#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = VideoFrameCopyToOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoFrameCopyToOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type VideoFrameCopyToOptions;
    #[wasm_bindgen(method, getter = "layout")]
    fn layout_shim(this: &VideoFrameCopyToOptions) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "layout")]
    fn set_layout_shim(this: &VideoFrameCopyToOptions, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "DomRectInit")]
    #[wasm_bindgen(method, getter = "rect")]
    fn rect_shim(this: &VideoFrameCopyToOptions) -> &DomRectInit;
    #[cfg(feature = "DomRectInit")]
    #[wasm_bindgen(method, setter = "rect")]
    fn set_rect_shim(this: &VideoFrameCopyToOptions, val: &DomRectInit);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `VideoFrameCopyToOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `VideoFrameCopyToOptions`*"]
pub trait VideoFrameCopyToOptionsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn layout(&self) -> &::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DomRectInit")]
    #[doc = "Get the `rect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `VideoFrameCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn rect(&self) -> &DomRectInit;
}
#[cfg(web_sys_unstable_apis)]
impl VideoFrameCopyToOptionsGetters for VideoFrameCopyToOptions {
    #[cfg(web_sys_unstable_apis)]
    fn layout(&self) -> &::wasm_bindgen::JsValue {
        self.layout_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DomRectInit")]
    fn rect(&self) -> &DomRectInit {
        self.rect_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl VideoFrameCopyToOptions {
    #[doc = "Construct a new `VideoFrameCopyToOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn layout(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_layout_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DomRectInit")]
    #[doc = "Change the `rect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `VideoFrameCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn rect(&mut self, val: &DomRectInit) -> &mut Self {
        self.set_rect_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for VideoFrameCopyToOptions {
    fn default() -> Self {
        Self::new()
    }
}
