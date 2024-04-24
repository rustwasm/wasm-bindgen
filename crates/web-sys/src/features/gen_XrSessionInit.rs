#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = XRSessionInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrSessionInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSessionInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrSessionInit;
    #[wasm_bindgen(method, getter = "optionalFeatures")]
    fn optional_features_shim(this: &XrSessionInit) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "optionalFeatures")]
    fn set_optional_features_shim(this: &XrSessionInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "requiredFeatures")]
    fn required_features_shim(this: &XrSessionInit) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "requiredFeatures")]
    fn set_required_features_shim(this: &XrSessionInit, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `XrSessionInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `XrSessionInit`*"]
pub trait XrSessionInitGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `optionalFeatures` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSessionInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn optional_features(&self) -> &::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `requiredFeatures` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSessionInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn required_features(&self) -> &::wasm_bindgen::JsValue;
}
#[cfg(web_sys_unstable_apis)]
impl XrSessionInitGetters for XrSessionInit {
    #[cfg(web_sys_unstable_apis)]
    fn optional_features(&self) -> &::wasm_bindgen::JsValue {
        self.optional_features_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn required_features(&self) -> &::wasm_bindgen::JsValue {
        self.required_features_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl XrSessionInit {
    #[doc = "Construct a new `XrSessionInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSessionInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `optionalFeatures` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSessionInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn optional_features(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_optional_features_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `requiredFeatures` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSessionInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn required_features(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_required_features_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for XrSessionInit {
    fn default() -> Self {
        Self::new()
    }
}
