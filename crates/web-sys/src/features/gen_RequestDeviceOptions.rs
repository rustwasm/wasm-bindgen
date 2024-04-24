#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RequestDeviceOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RequestDeviceOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestDeviceOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type RequestDeviceOptions;
    #[wasm_bindgen(method, getter = "acceptAllDevices")]
    fn accept_all_devices_shim(this: &RequestDeviceOptions) -> bool;
    #[wasm_bindgen(method, setter = "acceptAllDevices")]
    fn set_accept_all_devices_shim(this: &RequestDeviceOptions, val: bool);
    #[wasm_bindgen(method, getter = "filters")]
    fn filters_shim(this: &RequestDeviceOptions) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "filters")]
    fn set_filters_shim(this: &RequestDeviceOptions, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "optionalServices")]
    fn optional_services_shim(this: &RequestDeviceOptions) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "optionalServices")]
    fn set_optional_services_shim(this: &RequestDeviceOptions, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `RequestDeviceOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RequestDeviceOptions`*"]
pub trait RequestDeviceOptionsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `acceptAllDevices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestDeviceOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn accept_all_devices(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `filters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestDeviceOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn filters(&self) -> &::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `optionalServices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestDeviceOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn optional_services(&self) -> &::wasm_bindgen::JsValue;
}
#[cfg(web_sys_unstable_apis)]
impl RequestDeviceOptionsGetters for RequestDeviceOptions {
    #[cfg(web_sys_unstable_apis)]
    fn accept_all_devices(&self) -> bool {
        self.accept_all_devices_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn filters(&self) -> &::wasm_bindgen::JsValue {
        self.filters_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn optional_services(&self) -> &::wasm_bindgen::JsValue {
        self.optional_services_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl RequestDeviceOptions {
    #[doc = "Construct a new `RequestDeviceOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestDeviceOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `acceptAllDevices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestDeviceOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn accept_all_devices(&mut self, val: bool) -> &mut Self {
        self.set_accept_all_devices_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `filters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestDeviceOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn filters(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_filters_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `optionalServices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestDeviceOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn optional_services(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_optional_services_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for RequestDeviceOptions {
    fn default() -> Self {
        Self::new()
    }
}
