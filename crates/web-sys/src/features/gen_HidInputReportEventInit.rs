#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HIDInputReportEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HidInputReportEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidInputReportEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type HidInputReportEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &HidInputReportEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &HidInputReportEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &HidInputReportEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &HidInputReportEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &HidInputReportEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &HidInputReportEventInit, val: bool);
    #[wasm_bindgen(method, getter = "data")]
    fn data_shim(this: &HidInputReportEventInit) -> &::js_sys::DataView;
    #[wasm_bindgen(method, setter = "data")]
    fn set_data_shim(this: &HidInputReportEventInit, val: &::js_sys::DataView);
    #[cfg(feature = "HidDevice")]
    #[wasm_bindgen(method, getter = "device")]
    fn device_shim(this: &HidInputReportEventInit) -> &HidDevice;
    #[cfg(feature = "HidDevice")]
    #[wasm_bindgen(method, setter = "device")]
    fn set_device_shim(this: &HidInputReportEventInit, val: &HidDevice);
    #[wasm_bindgen(method, getter = "reportId")]
    fn report_id_shim(this: &HidInputReportEventInit) -> u8;
    #[wasm_bindgen(method, setter = "reportId")]
    fn set_report_id_shim(this: &HidInputReportEventInit, val: u8);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `HidInputReportEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HidInputReportEventInit`*"]
pub trait HidInputReportEventInitGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidInputReportEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn bubbles(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidInputReportEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn cancelable(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidInputReportEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn composed(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidInputReportEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn data(&self) -> &::js_sys::DataView;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidDevice")]
    #[doc = "Get the `device` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDevice`, `HidInputReportEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn device(&self) -> &HidDevice;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `reportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidInputReportEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn report_id(&self) -> u8;
}
#[cfg(web_sys_unstable_apis)]
impl HidInputReportEventInitGetters for HidInputReportEventInit {
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
    fn data(&self) -> &::js_sys::DataView {
        self.data_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidDevice")]
    fn device(&self) -> &HidDevice {
        self.device_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn report_id(&self) -> u8 {
        self.report_id_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl HidInputReportEventInit {
    #[cfg(feature = "HidDevice")]
    #[doc = "Construct a new `HidInputReportEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDevice`, `HidInputReportEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(data: &::js_sys::DataView, device: &HidDevice, report_id: u8) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.data(data);
        ret.device(device);
        ret.report_id(report_id);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidInputReportEventInit`*"]
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
    #[doc = "*This API requires the following crate features to be activated: `HidInputReportEventInit`*"]
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
    #[doc = "*This API requires the following crate features to be activated: `HidInputReportEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidInputReportEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn data(&mut self, val: &::js_sys::DataView) -> &mut Self {
        self.set_data_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidDevice")]
    #[doc = "Change the `device` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDevice`, `HidInputReportEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device(&mut self, val: &HidDevice) -> &mut Self {
        self.set_device_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `reportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidInputReportEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn report_id(&mut self, val: u8) -> &mut Self {
        self.set_report_id_shim(val);
        self
    }
}
