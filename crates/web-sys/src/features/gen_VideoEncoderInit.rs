#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = VideoEncoderInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoEncoderInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type VideoEncoderInit;
    #[wasm_bindgen(method, getter = "error")]
    fn error_shim(this: &VideoEncoderInit) -> ::js_sys::Function;
    #[wasm_bindgen(method, setter = "error")]
    fn set_error_shim(this: &VideoEncoderInit, val: &::js_sys::Function);
    #[wasm_bindgen(method, getter = "output")]
    fn output_shim(this: &VideoEncoderInit) -> ::js_sys::Function;
    #[wasm_bindgen(method, setter = "output")]
    fn set_output_shim(this: &VideoEncoderInit, val: &::js_sys::Function);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `VideoEncoderInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `VideoEncoderInit`*"]
pub trait VideoEncoderInitGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn error(&self) -> ::js_sys::Function;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `output` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn output(&self) -> ::js_sys::Function;
}
#[cfg(web_sys_unstable_apis)]
impl VideoEncoderInitGetters for VideoEncoderInit {
    #[cfg(web_sys_unstable_apis)]
    fn error(&self) -> ::js_sys::Function {
        self.error_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn output(&self) -> ::js_sys::Function {
        self.output_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl VideoEncoderInit {
    #[doc = "Construct a new `VideoEncoderInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(error: &::js_sys::Function, output: &::js_sys::Function) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::error(&mut ret, error);
        Self::output(&mut ret, output);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn error(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_error_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `output` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn output(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_output_shim(val);
        self
    }
}
