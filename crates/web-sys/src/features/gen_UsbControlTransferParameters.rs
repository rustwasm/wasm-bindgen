#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = USBControlTransferParameters)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbControlTransferParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UsbControlTransferParameters;
    #[wasm_bindgen(method, setter = "index")]
    fn index_shim(this: &UsbControlTransferParameters, val: u16);
    #[cfg(feature = "UsbRecipient")]
    #[wasm_bindgen(method, setter = "recipient")]
    fn recipient_shim(this: &UsbControlTransferParameters, val: UsbRecipient);
    #[wasm_bindgen(method, setter = "request")]
    fn request_shim(this: &UsbControlTransferParameters, val: u8);
    #[cfg(feature = "UsbRequestType")]
    #[wasm_bindgen(method, setter = "requestType")]
    fn request_type_shim(this: &UsbControlTransferParameters, val: UsbRequestType);
    #[wasm_bindgen(method, setter = "value")]
    fn value_shim(this: &UsbControlTransferParameters, val: u16);
}
#[cfg(web_sys_unstable_apis)]
impl UsbControlTransferParameters {
    #[cfg(all(feature = "UsbRecipient", feature = "UsbRequestType",))]
    #[doc = "Construct a new `UsbControlTransferParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`, `UsbRecipient`, `UsbRequestType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        index: u16,
        recipient: UsbRecipient,
        request: u8,
        request_type: UsbRequestType,
        value: u16,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.index(index);
        ret.recipient(recipient);
        ret.request(request);
        ret.request_type(request_type);
        ret.value(value);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `index` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn index(&mut self, val: u16) -> &mut Self {
        self.index_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbRecipient")]
    #[doc = "Change the `recipient` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`, `UsbRecipient`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn recipient(&mut self, val: UsbRecipient) -> &mut Self {
        self.recipient_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `request` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request(&mut self, val: u8) -> &mut Self {
        self.request_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbRequestType")]
    #[doc = "Change the `requestType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`, `UsbRequestType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_type(&mut self, val: UsbRequestType) -> &mut Self {
        self.request_type_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn value(&mut self, val: u16) -> &mut Self {
        self.value_shim(val);
        self
    }
}
