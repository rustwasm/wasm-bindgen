#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = UADataValues)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UaDataValues` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UaDataValues;
    #[wasm_bindgen(method, setter = "architecture")]
    fn architecture_shim(this: &UaDataValues, val: &str);
    #[wasm_bindgen(method, setter = "bitness")]
    fn bitness_shim(this: &UaDataValues, val: &str);
    #[wasm_bindgen(method, setter = "brands")]
    fn brands_shim(this: &UaDataValues, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "formFactors")]
    fn form_factors_shim(this: &UaDataValues, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "fullVersionList")]
    fn full_version_list_shim(this: &UaDataValues, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "mobile")]
    fn mobile_shim(this: &UaDataValues, val: bool);
    #[wasm_bindgen(method, setter = "model")]
    fn model_shim(this: &UaDataValues, val: &str);
    #[wasm_bindgen(method, setter = "platform")]
    fn platform_shim(this: &UaDataValues, val: &str);
    #[wasm_bindgen(method, setter = "platformVersion")]
    fn platform_version_shim(this: &UaDataValues, val: &str);
    #[wasm_bindgen(method, setter = "wow64")]
    fn wow64_shim(this: &UaDataValues, val: bool);
}
#[cfg(web_sys_unstable_apis)]
impl UaDataValues {
    #[doc = "Construct a new `UaDataValues`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `architecture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn architecture(&mut self, val: &str) -> &mut Self {
        self.architecture_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bitness` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn bitness(&mut self, val: &str) -> &mut Self {
        self.bitness_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `brands` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn brands(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.brands_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `formFactors` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn form_factors(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.form_factors_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `fullVersionList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn full_version_list(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.full_version_list_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `mobile` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn mobile(&mut self, val: bool) -> &mut Self {
        self.mobile_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `model` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn model(&mut self, val: &str) -> &mut Self {
        self.model_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `platform` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn platform(&mut self, val: &str) -> &mut Self {
        self.platform_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `platformVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn platform_version(&mut self, val: &str) -> &mut Self {
        self.platform_version_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `wow64` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn wow64(&mut self, val: bool) -> &mut Self {
        self.wow64_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for UaDataValues {
    fn default() -> Self {
        Self::new()
    }
}
