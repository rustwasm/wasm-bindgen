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
    #[wasm_bindgen(method, getter = "children")]
    fn children_shim(this: &HidCollectionInfo) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "children")]
    fn set_children_shim(this: &HidCollectionInfo, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "featureReports")]
    fn feature_reports_shim(this: &HidCollectionInfo) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "featureReports")]
    fn set_feature_reports_shim(this: &HidCollectionInfo, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "inputReports")]
    fn input_reports_shim(this: &HidCollectionInfo) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "inputReports")]
    fn set_input_reports_shim(this: &HidCollectionInfo, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "outputReports")]
    fn output_reports_shim(this: &HidCollectionInfo) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "outputReports")]
    fn set_output_reports_shim(this: &HidCollectionInfo, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &HidCollectionInfo) -> u8;
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &HidCollectionInfo, val: u8);
    #[wasm_bindgen(method, getter = "usage")]
    fn usage_shim(this: &HidCollectionInfo) -> u16;
    #[wasm_bindgen(method, setter = "usage")]
    fn set_usage_shim(this: &HidCollectionInfo, val: u16);
    #[wasm_bindgen(method, getter = "usagePage")]
    fn usage_page_shim(this: &HidCollectionInfo) -> u16;
    #[wasm_bindgen(method, setter = "usagePage")]
    fn set_usage_page_shim(this: &HidCollectionInfo, val: u16);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `HidCollectionInfo` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
pub trait HidCollectionInfoGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `children` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn children(&self) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `featureReports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn feature_reports(&self) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `inputReports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn input_reports(&self) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `outputReports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn output_reports(&self) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn type_(&self) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `usage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn usage(&self) -> u16;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `usagePage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn usage_page(&self) -> u16;
}
#[cfg(web_sys_unstable_apis)]
impl HidCollectionInfoGetters for HidCollectionInfo {
    #[cfg(web_sys_unstable_apis)]
    fn children(&self) -> ::js_sys::Array {
        self.children_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn feature_reports(&self) -> ::js_sys::Array {
        self.feature_reports_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn input_reports(&self) -> ::js_sys::Array {
        self.input_reports_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn output_reports(&self) -> ::js_sys::Array {
        self.output_reports_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn type_(&self) -> u8 {
        self.type__shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn usage(&self) -> u16 {
        self.usage_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn usage_page(&self) -> u16 {
        self.usage_page_shim()
    }
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
        self.set_children_shim(val);
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
        self.set_feature_reports_shim(val);
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
        self.set_input_reports_shim(val);
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
        self.set_output_reports_shim(val);
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
        self.set_type__shim(val);
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
        self.set_usage_shim(val);
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
        self.set_usage_page_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for HidCollectionInfo {
    fn default() -> Self {
        Self::new()
    }
}
