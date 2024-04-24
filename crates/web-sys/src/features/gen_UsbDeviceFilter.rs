#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = USBDeviceFilter)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbDeviceFilter` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UsbDeviceFilter;
    #[wasm_bindgen(method, getter = "classCode")]
    fn class_code_shim(this: &UsbDeviceFilter) -> u8;
    #[wasm_bindgen(method, setter = "classCode")]
    fn set_class_code_shim(this: &UsbDeviceFilter, val: u8);
    #[wasm_bindgen(method, getter = "productId")]
    fn product_id_shim(this: &UsbDeviceFilter) -> u16;
    #[wasm_bindgen(method, setter = "productId")]
    fn set_product_id_shim(this: &UsbDeviceFilter, val: u16);
    #[wasm_bindgen(method, getter = "protocolCode")]
    fn protocol_code_shim(this: &UsbDeviceFilter) -> u8;
    #[wasm_bindgen(method, setter = "protocolCode")]
    fn set_protocol_code_shim(this: &UsbDeviceFilter, val: u8);
    #[wasm_bindgen(method, getter = "serialNumber")]
    fn serial_number_shim(this: &UsbDeviceFilter) -> String;
    #[wasm_bindgen(method, setter = "serialNumber")]
    fn set_serial_number_shim(this: &UsbDeviceFilter, val: &str);
    #[wasm_bindgen(method, getter = "subclassCode")]
    fn subclass_code_shim(this: &UsbDeviceFilter) -> u8;
    #[wasm_bindgen(method, setter = "subclassCode")]
    fn set_subclass_code_shim(this: &UsbDeviceFilter, val: u8);
    #[wasm_bindgen(method, getter = "vendorId")]
    fn vendor_id_shim(this: &UsbDeviceFilter) -> u16;
    #[wasm_bindgen(method, setter = "vendorId")]
    fn set_vendor_id_shim(this: &UsbDeviceFilter, val: u16);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `UsbDeviceFilter` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
pub trait UsbDeviceFilterGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `classCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn class_code(&self) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `productId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn product_id(&self) -> u16;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `protocolCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn protocol_code(&self) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `serialNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn serial_number(&self) -> String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `subclassCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn subclass_code(&self) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `vendorId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn vendor_id(&self) -> u16;
}
#[cfg(web_sys_unstable_apis)]
impl UsbDeviceFilterGetters for UsbDeviceFilter {
    #[cfg(web_sys_unstable_apis)]
    fn class_code(&self) -> u8 {
        self.class_code_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn product_id(&self) -> u16 {
        self.product_id_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn protocol_code(&self) -> u8 {
        self.protocol_code_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn serial_number(&self) -> String {
        self.serial_number_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn subclass_code(&self) -> u8 {
        self.subclass_code_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn vendor_id(&self) -> u16 {
        self.vendor_id_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl UsbDeviceFilter {
    #[doc = "Construct a new `UsbDeviceFilter`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `classCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn class_code(&mut self, val: u8) -> &mut Self {
        self.set_class_code_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `productId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn product_id(&mut self, val: u16) -> &mut Self {
        self.set_product_id_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `protocolCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn protocol_code(&mut self, val: u8) -> &mut Self {
        self.set_protocol_code_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `serialNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn serial_number(&mut self, val: &str) -> &mut Self {
        self.set_serial_number_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `subclassCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn subclass_code(&mut self, val: u8) -> &mut Self {
        self.set_subclass_code_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `vendorId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn vendor_id(&mut self, val: u16) -> &mut Self {
        self.set_vendor_id_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for UsbDeviceFilter {
    fn default() -> Self {
        Self::new()
    }
}
