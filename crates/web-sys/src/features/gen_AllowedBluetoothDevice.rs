#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AllowedBluetoothDevice)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AllowedBluetoothDevice` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type AllowedBluetoothDevice;
    #[wasm_bindgen(method, getter = "allowedServices")]
    fn allowed_services_shim(this: &AllowedBluetoothDevice) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "allowedServices")]
    fn set_allowed_services_shim(this: &AllowedBluetoothDevice, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "deviceId")]
    fn device_id_shim(this: &AllowedBluetoothDevice) -> String;
    #[wasm_bindgen(method, setter = "deviceId")]
    fn set_device_id_shim(this: &AllowedBluetoothDevice, val: &str);
    #[wasm_bindgen(method, getter = "mayUseGATT")]
    fn may_use_gatt_shim(this: &AllowedBluetoothDevice) -> bool;
    #[wasm_bindgen(method, setter = "mayUseGATT")]
    fn set_may_use_gatt_shim(this: &AllowedBluetoothDevice, val: bool);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `AllowedBluetoothDevice` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`*"]
pub trait AllowedBluetoothDeviceGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `allowedServices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn allowed_services(&self) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn device_id(&self) -> String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `mayUseGATT` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn may_use_gatt(&self) -> bool;
}
#[cfg(web_sys_unstable_apis)]
impl AllowedBluetoothDeviceGetters for AllowedBluetoothDevice {
    #[cfg(web_sys_unstable_apis)]
    fn allowed_services(&self) -> ::wasm_bindgen::JsValue {
        self.allowed_services_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn device_id(&self) -> String {
        self.device_id_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn may_use_gatt(&self) -> bool {
        self.may_use_gatt_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl AllowedBluetoothDevice {
    #[doc = "Construct a new `AllowedBluetoothDevice`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        allowed_services: &::wasm_bindgen::JsValue,
        device_id: &str,
        may_use_gatt: bool,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::allowed_services(&mut ret, allowed_services);
        Self::device_id(&mut ret, device_id);
        Self::may_use_gatt(&mut ret, may_use_gatt);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `allowedServices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn allowed_services(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_allowed_services_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device_id(&mut self, val: &str) -> &mut Self {
        self.set_device_id_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `mayUseGATT` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn may_use_gatt(&mut self, val: bool) -> &mut Self {
        self.set_may_use_gatt_shim(val);
        self
    }
}
