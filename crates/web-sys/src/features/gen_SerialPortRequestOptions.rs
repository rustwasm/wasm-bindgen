#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SerialPortRequestOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SerialPortRequestOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type SerialPortRequestOptions;
    #[wasm_bindgen(method, getter = "filters")]
    fn filters_shim(this: &SerialPortRequestOptions) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "filters")]
    fn set_filters_shim(this: &SerialPortRequestOptions, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `SerialPortRequestOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SerialPortRequestOptions`*"]
pub trait SerialPortRequestOptionsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `filters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn filters(&self) -> ::js_sys::Array;
}
#[cfg(web_sys_unstable_apis)]
impl SerialPortRequestOptionsGetters for SerialPortRequestOptions {
    #[cfg(web_sys_unstable_apis)]
    fn filters(&self) -> ::js_sys::Array {
        self.filters_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl SerialPortRequestOptions {
    #[doc = "Construct a new `SerialPortRequestOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `filters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn filters(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_filters_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for SerialPortRequestOptions {
    fn default() -> Self {
        Self::new()
    }
}
