#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportCloseInfo)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportCloseInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportCloseInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type WebTransportCloseInfo;
    #[wasm_bindgen(method, getter = "closeCode")]
    fn close_code_shim(this: &WebTransportCloseInfo) -> u32;
    #[wasm_bindgen(method, setter = "closeCode")]
    fn set_close_code_shim(this: &WebTransportCloseInfo, val: u32);
    #[wasm_bindgen(method, getter = "reason")]
    fn reason_shim(this: &WebTransportCloseInfo) -> String;
    #[wasm_bindgen(method, setter = "reason")]
    fn set_reason_shim(this: &WebTransportCloseInfo, val: &str);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `WebTransportCloseInfo` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WebTransportCloseInfo`*"]
pub trait WebTransportCloseInfoGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `closeCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportCloseInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn close_code(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportCloseInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn reason(&self) -> String;
}
#[cfg(web_sys_unstable_apis)]
impl WebTransportCloseInfoGetters for WebTransportCloseInfo {
    #[cfg(web_sys_unstable_apis)]
    fn close_code(&self) -> u32 {
        self.close_code_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn reason(&self) -> String {
        self.reason_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl WebTransportCloseInfo {
    #[doc = "Construct a new `WebTransportCloseInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportCloseInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `closeCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportCloseInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn close_code(&mut self, val: u32) -> &mut Self {
        self.set_close_code_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportCloseInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn reason(&mut self, val: &str) -> &mut Self {
        self.set_reason_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for WebTransportCloseInfo {
    fn default() -> Self {
        Self::new()
    }
}
