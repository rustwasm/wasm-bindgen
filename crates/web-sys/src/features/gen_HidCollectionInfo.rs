#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HIDCollectionInfo)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HidCollectionInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type HidCollectionInfo;
    #[wasm_bindgen(method, setter = "children")]
    fn children_shim(this: &HidCollectionInfo, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "featureReports")]
    fn feature_reports_shim(this: &HidCollectionInfo, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "inputReports")]
    fn input_reports_shim(this: &HidCollectionInfo, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "outputReports")]
    fn output_reports_shim(this: &HidCollectionInfo, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &HidCollectionInfo, val: u8);
    #[wasm_bindgen(method, setter = "usage")]
    fn usage_shim(this: &HidCollectionInfo, val: u16);
    #[wasm_bindgen(method, setter = "usagePage")]
    fn usage_page_shim(this: &HidCollectionInfo, val: u16);
}
#[cfg(web_sys_unstable_apis)]
impl HidCollectionInfo {
    #[doc = "Construct a new `HidCollectionInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `children` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn children(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.children_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `featureReports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn feature_reports(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.feature_reports_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `inputReports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn input_reports(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.input_reports_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `outputReports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn output_reports(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.output_reports_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn type_(&mut self, val: u8) -> &mut Self {
        self.type__shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `usage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn usage(&mut self, val: u16) -> &mut Self {
        self.usage_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `usagePage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn usage_page(&mut self, val: u16) -> &mut Self {
        self.usage_page_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for HidCollectionInfo {
    fn default() -> Self {
        Self::new()
    }
}
