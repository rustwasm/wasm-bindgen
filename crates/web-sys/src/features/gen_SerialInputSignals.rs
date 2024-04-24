#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SerialInputSignals)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SerialInputSignals` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type SerialInputSignals;
    #[wasm_bindgen(method, getter = "clearToSend")]
    fn clear_to_send_shim(this: &SerialInputSignals) -> bool;
    #[wasm_bindgen(method, setter = "clearToSend")]
    fn set_clear_to_send_shim(this: &SerialInputSignals, val: bool);
    #[wasm_bindgen(method, getter = "dataCarrierDetect")]
    fn data_carrier_detect_shim(this: &SerialInputSignals) -> bool;
    #[wasm_bindgen(method, setter = "dataCarrierDetect")]
    fn set_data_carrier_detect_shim(this: &SerialInputSignals, val: bool);
    #[wasm_bindgen(method, getter = "dataSetReady")]
    fn data_set_ready_shim(this: &SerialInputSignals) -> bool;
    #[wasm_bindgen(method, setter = "dataSetReady")]
    fn set_data_set_ready_shim(this: &SerialInputSignals, val: bool);
    #[wasm_bindgen(method, getter = "ringIndicator")]
    fn ring_indicator_shim(this: &SerialInputSignals) -> bool;
    #[wasm_bindgen(method, setter = "ringIndicator")]
    fn set_ring_indicator_shim(this: &SerialInputSignals, val: bool);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `SerialInputSignals` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
pub trait SerialInputSignalsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `clearToSend` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn clear_to_send(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `dataCarrierDetect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn data_carrier_detect(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `dataSetReady` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn data_set_ready(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `ringIndicator` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn ring_indicator(&self) -> bool;
}
#[cfg(web_sys_unstable_apis)]
impl SerialInputSignalsGetters for SerialInputSignals {
    #[cfg(web_sys_unstable_apis)]
    fn clear_to_send(&self) -> bool {
        self.clear_to_send_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn data_carrier_detect(&self) -> bool {
        self.data_carrier_detect_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn data_set_ready(&self) -> bool {
        self.data_set_ready_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn ring_indicator(&self) -> bool {
        self.ring_indicator_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl SerialInputSignals {
    #[doc = "Construct a new `SerialInputSignals`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        clear_to_send: bool,
        data_carrier_detect: bool,
        data_set_ready: bool,
        ring_indicator: bool,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::clear_to_send(&mut ret, clear_to_send);
        Self::data_carrier_detect(&mut ret, data_carrier_detect);
        Self::data_set_ready(&mut ret, data_set_ready);
        Self::ring_indicator(&mut ret, ring_indicator);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `clearToSend` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn clear_to_send(&mut self, val: bool) -> &mut Self {
        self.set_clear_to_send_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `dataCarrierDetect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn data_carrier_detect(&mut self, val: bool) -> &mut Self {
        self.set_data_carrier_detect_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `dataSetReady` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn data_set_ready(&mut self, val: bool) -> &mut Self {
        self.set_data_set_ready_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `ringIndicator` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn ring_indicator(&mut self, val: bool) -> &mut Self {
        self.set_ring_indicator_shim(val);
        self
    }
}
