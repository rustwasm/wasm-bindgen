#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = UALowEntropyJSON)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UaLowEntropyJson` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UaLowEntropyJson;
    #[wasm_bindgen(method, setter = "brands")]
    fn brands_shim(this: &UaLowEntropyJson, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "mobile")]
    fn mobile_shim(this: &UaLowEntropyJson, val: bool);
    #[wasm_bindgen(method, setter = "platform")]
    fn platform_shim(this: &UaLowEntropyJson, val: &str);
}
#[cfg(web_sys_unstable_apis)]
impl UaLowEntropyJson {
    #[doc = "Construct a new `UaLowEntropyJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `brands` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn brands(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.brands_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `mobile` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn mobile(&mut self, val: bool) -> &mut Self {
        self.mobile_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `platform` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn platform(&mut self, val: &str) -> &mut Self {
        self.platform_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for UaLowEntropyJson {
    fn default() -> Self {
        Self::new()
    }
}
