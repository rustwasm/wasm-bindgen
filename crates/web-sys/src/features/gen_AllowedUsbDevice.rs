#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AllowedUSBDevice)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AllowedUsbDevice` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type AllowedUsbDevice;
    #[wasm_bindgen(method, getter = "productId")]
    fn product_id_shim(this: &AllowedUsbDevice) -> u8;
    #[wasm_bindgen(method, setter = "productId")]
    fn set_product_id_shim(this: &AllowedUsbDevice, val: u8);
    #[wasm_bindgen(method, getter = "serialNumber")]
    fn serial_number_shim(this: &AllowedUsbDevice) -> String;
    #[wasm_bindgen(method, setter = "serialNumber")]
    fn set_serial_number_shim(this: &AllowedUsbDevice, val: &str);
    #[wasm_bindgen(method, getter = "vendorId")]
    fn vendor_id_shim(this: &AllowedUsbDevice) -> u8;
    #[wasm_bindgen(method, setter = "vendorId")]
    fn set_vendor_id_shim(this: &AllowedUsbDevice, val: u8);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `AllowedUsbDevice` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
pub trait AllowedUsbDeviceGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `productId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn product_id(&self) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `serialNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn serial_number(&self) -> String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `vendorId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn vendor_id(&self) -> u8;
}
#[cfg(web_sys_unstable_apis)]
impl AllowedUsbDeviceGetters for AllowedUsbDevice {
    #[cfg(web_sys_unstable_apis)]
    fn product_id(&self) -> u8 {
        self.product_id_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn serial_number(&self) -> String {
        self.serial_number_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn vendor_id(&self) -> u8 {
        self.vendor_id_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl AllowedUsbDevice {
    #[doc = "Construct a new `AllowedUsbDevice`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(product_id: u8, vendor_id: u8) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.product_id(product_id);
        ret.vendor_id(vendor_id);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `productId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn product_id(&mut self, val: u8) -> &mut Self {
        self.set_product_id_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `serialNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn serial_number(&mut self, val: &str) -> &mut Self {
        self.set_serial_number_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `vendorId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn vendor_id(&mut self, val: u8) -> &mut Self {
        self.set_vendor_id_shim(val);
        self
    }
}
