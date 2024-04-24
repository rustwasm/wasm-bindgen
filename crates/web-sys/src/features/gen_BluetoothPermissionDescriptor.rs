#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BluetoothPermissionDescriptor)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothPermissionDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type BluetoothPermissionDescriptor;
    #[cfg(feature = "PermissionName")]
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &BluetoothPermissionDescriptor) -> PermissionName;
    #[cfg(feature = "PermissionName")]
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &BluetoothPermissionDescriptor, val: PermissionName);
    #[wasm_bindgen(method, getter = "acceptAllDevices")]
    fn accept_all_devices_shim(this: &BluetoothPermissionDescriptor) -> bool;
    #[wasm_bindgen(method, setter = "acceptAllDevices")]
    fn set_accept_all_devices_shim(this: &BluetoothPermissionDescriptor, val: bool);
    #[wasm_bindgen(method, getter = "deviceId")]
    fn device_id_shim(this: &BluetoothPermissionDescriptor) -> String;
    #[wasm_bindgen(method, setter = "deviceId")]
    fn set_device_id_shim(this: &BluetoothPermissionDescriptor, val: &str);
    #[wasm_bindgen(method, getter = "filters")]
    fn filters_shim(this: &BluetoothPermissionDescriptor) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "filters")]
    fn set_filters_shim(this: &BluetoothPermissionDescriptor, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "optionalServices")]
    fn optional_services_shim(this: &BluetoothPermissionDescriptor) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "optionalServices")]
    fn set_optional_services_shim(
        this: &BluetoothPermissionDescriptor,
        val: &::wasm_bindgen::JsValue,
    );
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `BluetoothPermissionDescriptor` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionDescriptor`*"]
pub trait BluetoothPermissionDescriptorGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PermissionName")]
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionDescriptor`, `PermissionName`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn name(&self) -> PermissionName;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `acceptAllDevices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn accept_all_devices(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn device_id(&self) -> String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `filters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn filters(&self) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `optionalServices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn optional_services(&self) -> ::js_sys::Array;
}
#[cfg(web_sys_unstable_apis)]
impl BluetoothPermissionDescriptorGetters for BluetoothPermissionDescriptor {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PermissionName")]
    fn name(&self) -> PermissionName {
        self.name_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn accept_all_devices(&self) -> bool {
        self.accept_all_devices_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn device_id(&self) -> String {
        self.device_id_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn filters(&self) -> ::js_sys::Array {
        self.filters_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn optional_services(&self) -> ::js_sys::Array {
        self.optional_services_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl BluetoothPermissionDescriptor {
    #[cfg(feature = "PermissionName")]
    #[doc = "Construct a new `BluetoothPermissionDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionDescriptor`, `PermissionName`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(name: PermissionName) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PermissionName")]
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionDescriptor`, `PermissionName`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn name(&mut self, val: PermissionName) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `acceptAllDevices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn accept_all_devices(&mut self, val: bool) -> &mut Self {
        self.set_accept_all_devices_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device_id(&mut self, val: &str) -> &mut Self {
        self.set_device_id_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `filters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionDescriptor`*"]
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
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn optional_services(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_optional_services_shim(val);
        self
    }
}
