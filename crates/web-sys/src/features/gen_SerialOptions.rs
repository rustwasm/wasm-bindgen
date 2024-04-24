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
    #[wasm_bindgen(method, getter = "baudRate")]
    fn baud_rate_shim(this: &SerialOptions) -> u32;
    #[wasm_bindgen(method, setter = "baudRate")]
    fn set_baud_rate_shim(this: &SerialOptions, val: u32);
    #[wasm_bindgen(method, getter = "bufferSize")]
    fn buffer_size_shim(this: &SerialOptions) -> u32;
    #[wasm_bindgen(method, setter = "bufferSize")]
    fn set_buffer_size_shim(this: &SerialOptions, val: u32);
    #[wasm_bindgen(method, getter = "dataBits")]
    fn data_bits_shim(this: &SerialOptions) -> u8;
    #[wasm_bindgen(method, setter = "dataBits")]
    fn set_data_bits_shim(this: &SerialOptions, val: u8);
    #[cfg(feature = "FlowControlType")]
    #[wasm_bindgen(method, getter = "flowControl")]
    fn flow_control_shim(this: &SerialOptions) -> FlowControlType;
    #[cfg(feature = "FlowControlType")]
    #[wasm_bindgen(method, setter = "flowControl")]
    fn set_flow_control_shim(this: &SerialOptions, val: FlowControlType);
    #[cfg(feature = "ParityType")]
    #[wasm_bindgen(method, getter = "parity")]
    fn parity_shim(this: &SerialOptions) -> ParityType;
    #[cfg(feature = "ParityType")]
    #[wasm_bindgen(method, setter = "parity")]
    fn set_parity_shim(this: &SerialOptions, val: ParityType);
    #[wasm_bindgen(method, getter = "stopBits")]
    fn stop_bits_shim(this: &SerialOptions) -> u8;
    #[wasm_bindgen(method, setter = "stopBits")]
    fn set_stop_bits_shim(this: &SerialOptions, val: u8);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `SerialOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SerialOptions`*"]
pub trait SerialOptionsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `baudRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn baud_rate(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `bufferSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn buffer_size(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `dataBits` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn data_bits(&self) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FlowControlType")]
    #[doc = "Get the `flowControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FlowControlType`, `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn flow_control(&self) -> FlowControlType;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ParityType")]
    #[doc = "Get the `parity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ParityType`, `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn parity(&self) -> ParityType;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `stopBits` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn stop_bits(&self) -> u8;
}
#[cfg(web_sys_unstable_apis)]
impl SerialOptionsGetters for SerialOptions {
    #[cfg(web_sys_unstable_apis)]
    fn baud_rate(&self) -> u32 {
        self.baud_rate_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn buffer_size(&self) -> u32 {
        self.buffer_size_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn data_bits(&self) -> u8 {
        self.data_bits_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FlowControlType")]
    fn flow_control(&self) -> FlowControlType {
        self.flow_control_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ParityType")]
    fn parity(&self) -> ParityType {
        self.parity_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn stop_bits(&self) -> u8 {
        self.stop_bits_shim()
    }
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
        Self::baud_rate(&mut ret, baud_rate);
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
        self.set_baud_rate_shim(val);
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
        self.set_buffer_size_shim(val);
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
        self.set_data_bits_shim(val);
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
        self.set_flow_control_shim(val);
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
        self.set_parity_shim(val);
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
        self.set_stop_bits_shim(val);
        self
    }
}
