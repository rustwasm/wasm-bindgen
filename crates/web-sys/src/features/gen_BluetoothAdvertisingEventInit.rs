#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BluetoothAdvertisingEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothAdvertisingEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type BluetoothAdvertisingEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &BluetoothAdvertisingEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &BluetoothAdvertisingEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &BluetoothAdvertisingEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &BluetoothAdvertisingEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &BluetoothAdvertisingEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &BluetoothAdvertisingEventInit, val: bool);
    #[wasm_bindgen(method, getter = "appearance")]
    fn appearance_shim(this: &BluetoothAdvertisingEventInit) -> u16;
    #[wasm_bindgen(method, setter = "appearance")]
    fn set_appearance_shim(this: &BluetoothAdvertisingEventInit, val: u16);
    #[cfg(feature = "BluetoothDevice")]
    #[wasm_bindgen(method, getter = "device")]
    fn device_shim(this: &BluetoothAdvertisingEventInit) -> BluetoothDevice;
    #[cfg(feature = "BluetoothDevice")]
    #[wasm_bindgen(method, setter = "device")]
    fn set_device_shim(this: &BluetoothAdvertisingEventInit, val: &BluetoothDevice);
    #[cfg(feature = "BluetoothManufacturerDataMap")]
    #[wasm_bindgen(method, getter = "manufacturerData")]
    fn manufacturer_data_shim(this: &BluetoothAdvertisingEventInit)
        -> BluetoothManufacturerDataMap;
    #[cfg(feature = "BluetoothManufacturerDataMap")]
    #[wasm_bindgen(method, setter = "manufacturerData")]
    fn set_manufacturer_data_shim(
        this: &BluetoothAdvertisingEventInit,
        val: &BluetoothManufacturerDataMap,
    );
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &BluetoothAdvertisingEventInit) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &BluetoothAdvertisingEventInit, val: &str);
    #[wasm_bindgen(method, getter = "rssi")]
    fn rssi_shim(this: &BluetoothAdvertisingEventInit) -> i8;
    #[wasm_bindgen(method, setter = "rssi")]
    fn set_rssi_shim(this: &BluetoothAdvertisingEventInit, val: i8);
    #[cfg(feature = "BluetoothServiceDataMap")]
    #[wasm_bindgen(method, getter = "serviceData")]
    fn service_data_shim(this: &BluetoothAdvertisingEventInit) -> BluetoothServiceDataMap;
    #[cfg(feature = "BluetoothServiceDataMap")]
    #[wasm_bindgen(method, setter = "serviceData")]
    fn set_service_data_shim(this: &BluetoothAdvertisingEventInit, val: &BluetoothServiceDataMap);
    #[wasm_bindgen(method, getter = "txPower")]
    fn tx_power_shim(this: &BluetoothAdvertisingEventInit) -> i8;
    #[wasm_bindgen(method, setter = "txPower")]
    fn set_tx_power_shim(this: &BluetoothAdvertisingEventInit, val: i8);
    #[wasm_bindgen(method, getter = "uuids")]
    fn uuids_shim(this: &BluetoothAdvertisingEventInit) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "uuids")]
    fn set_uuids_shim(this: &BluetoothAdvertisingEventInit, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `BluetoothAdvertisingEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`*"]
pub trait BluetoothAdvertisingEventInitGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn bubbles(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn cancelable(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn composed(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `appearance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn appearance(&self) -> u16;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothDevice")]
    #[doc = "Get the `device` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`, `BluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn device(&self) -> BluetoothDevice;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothManufacturerDataMap")]
    #[doc = "Get the `manufacturerData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`, `BluetoothManufacturerDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn manufacturer_data(&self) -> BluetoothManufacturerDataMap;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn name(&self) -> String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `rssi` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn rssi(&self) -> i8;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothServiceDataMap")]
    #[doc = "Get the `serviceData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`, `BluetoothServiceDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn service_data(&self) -> BluetoothServiceDataMap;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `txPower` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn tx_power(&self) -> i8;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `uuids` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn uuids(&self) -> ::js_sys::Array;
}
#[cfg(web_sys_unstable_apis)]
impl BluetoothAdvertisingEventInitGetters for BluetoothAdvertisingEventInit {
    #[cfg(web_sys_unstable_apis)]
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn appearance(&self) -> u16 {
        self.appearance_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothDevice")]
    fn device(&self) -> BluetoothDevice {
        self.device_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothManufacturerDataMap")]
    fn manufacturer_data(&self) -> BluetoothManufacturerDataMap {
        self.manufacturer_data_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn name(&self) -> String {
        self.name_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn rssi(&self) -> i8 {
        self.rssi_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothServiceDataMap")]
    fn service_data(&self) -> BluetoothServiceDataMap {
        self.service_data_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn tx_power(&self) -> i8 {
        self.tx_power_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn uuids(&self) -> ::js_sys::Array {
        self.uuids_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl BluetoothAdvertisingEventInit {
    #[cfg(feature = "BluetoothDevice")]
    #[doc = "Construct a new `BluetoothAdvertisingEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`, `BluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(device: &BluetoothDevice) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::device(&mut ret, device);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `appearance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn appearance(&mut self, val: u16) -> &mut Self {
        self.set_appearance_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothDevice")]
    #[doc = "Change the `device` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`, `BluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device(&mut self, val: &BluetoothDevice) -> &mut Self {
        self.set_device_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothManufacturerDataMap")]
    #[doc = "Change the `manufacturerData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`, `BluetoothManufacturerDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn manufacturer_data(&mut self, val: &BluetoothManufacturerDataMap) -> &mut Self {
        self.set_manufacturer_data_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `rssi` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn rssi(&mut self, val: i8) -> &mut Self {
        self.set_rssi_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothServiceDataMap")]
    #[doc = "Change the `serviceData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`, `BluetoothServiceDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn service_data(&mut self, val: &BluetoothServiceDataMap) -> &mut Self {
        self.set_service_data_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `txPower` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn tx_power(&mut self, val: i8) -> &mut Self {
        self.set_tx_power_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `uuids` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn uuids(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_uuids_shim(val);
        self
    }
}
