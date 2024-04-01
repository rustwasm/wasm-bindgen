#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SerialOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SerialOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type SerialOptions;
    #[wasm_bindgen(method, setter = "baudRate")]
    fn baud_rate_shim(this: &SerialOptions, val: u32);
    #[wasm_bindgen(method, setter = "bufferSize")]
    fn buffer_size_shim(this: &SerialOptions, val: u32);
    #[wasm_bindgen(method, setter = "dataBits")]
    fn data_bits_shim(this: &SerialOptions, val: u8);
    #[cfg(feature = "FlowControlType")]
    #[wasm_bindgen(method, setter = "flowControl")]
    fn flow_control_shim(this: &SerialOptions, val: FlowControlType);
    #[cfg(feature = "ParityType")]
    #[wasm_bindgen(method, setter = "parity")]
    fn parity_shim(this: &SerialOptions, val: ParityType);
    #[wasm_bindgen(method, setter = "stopBits")]
    fn stop_bits_shim(this: &SerialOptions, val: u8);
}
#[cfg(web_sys_unstable_apis)]
impl SerialOptions {
    #[doc = "Construct a new `SerialOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(baud_rate: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.baud_rate(baud_rate);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `baudRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn baud_rate(&mut self, val: u32) -> &mut Self {
        self.baud_rate_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bufferSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn buffer_size(&mut self, val: u32) -> &mut Self {
        self.buffer_size_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `dataBits` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn data_bits(&mut self, val: u8) -> &mut Self {
        self.data_bits_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FlowControlType")]
    #[doc = "Change the `flowControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FlowControlType`, `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn flow_control(&mut self, val: FlowControlType) -> &mut Self {
        self.flow_control_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ParityType")]
    #[doc = "Change the `parity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ParityType`, `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn parity(&mut self, val: ParityType) -> &mut Self {
        self.parity_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `stopBits` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn stop_bits(&mut self, val: u8) -> &mut Self {
        self.stop_bits_shim(val);
        self
    }
}
