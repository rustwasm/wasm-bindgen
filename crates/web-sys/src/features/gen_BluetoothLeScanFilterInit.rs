#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BluetoothLEScanFilterInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothLeScanFilterInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothLeScanFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type BluetoothLeScanFilterInit;
    #[wasm_bindgen(method, getter = "manufacturerData")]
    fn manufacturer_data_shim(this: &BluetoothLeScanFilterInit) -> &::js_sys::Object;
    #[wasm_bindgen(method, setter = "manufacturerData")]
    fn set_manufacturer_data_shim(this: &BluetoothLeScanFilterInit, val: &::js_sys::Object);
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &BluetoothLeScanFilterInit) -> &str;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &BluetoothLeScanFilterInit, val: &str);
    #[wasm_bindgen(method, getter = "namePrefix")]
    fn name_prefix_shim(this: &BluetoothLeScanFilterInit) -> &str;
    #[wasm_bindgen(method, setter = "namePrefix")]
    fn set_name_prefix_shim(this: &BluetoothLeScanFilterInit, val: &str);
    #[wasm_bindgen(method, getter = "serviceData")]
    fn service_data_shim(this: &BluetoothLeScanFilterInit) -> &::js_sys::Object;
    #[wasm_bindgen(method, setter = "serviceData")]
    fn set_service_data_shim(this: &BluetoothLeScanFilterInit, val: &::js_sys::Object);
    #[wasm_bindgen(method, getter = "services")]
    fn services_shim(this: &BluetoothLeScanFilterInit) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "services")]
    fn set_services_shim(this: &BluetoothLeScanFilterInit, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `BluetoothLeScanFilterInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BluetoothLeScanFilterInit`*"]
pub trait BluetoothLeScanFilterInitGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `manufacturerData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothLeScanFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn manufacturer_data(&self) -> &::js_sys::Object;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothLeScanFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn name(&self) -> &str;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `namePrefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothLeScanFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn name_prefix(&self) -> &str;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `serviceData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothLeScanFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn service_data(&self) -> &::js_sys::Object;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `services` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothLeScanFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn services(&self) -> &::wasm_bindgen::JsValue;
}
#[cfg(web_sys_unstable_apis)]
impl BluetoothLeScanFilterInitGetters for BluetoothLeScanFilterInit {
    #[cfg(web_sys_unstable_apis)]
    fn manufacturer_data(&self) -> &::js_sys::Object {
        self.manufacturer_data_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn name(&self) -> &str {
        self.name_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn name_prefix(&self) -> &str {
        self.name_prefix_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn service_data(&self) -> &::js_sys::Object {
        self.service_data_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn services(&self) -> &::wasm_bindgen::JsValue {
        self.services_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl BluetoothLeScanFilterInit {
    #[doc = "Construct a new `BluetoothLeScanFilterInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothLeScanFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `manufacturerData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothLeScanFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn manufacturer_data(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_manufacturer_data_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothLeScanFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `namePrefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothLeScanFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn name_prefix(&mut self, val: &str) -> &mut Self {
        self.set_name_prefix_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `serviceData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothLeScanFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn service_data(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_service_data_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `services` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothLeScanFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn services(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_services_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for BluetoothLeScanFilterInit {
    fn default() -> Self {
        Self::new()
    }
}
