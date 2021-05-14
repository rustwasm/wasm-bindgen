#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = EventTarget , extends = :: js_sys :: Object , js_name = DevicePosture , typescript_type = "DevicePosture")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DevicePosture` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DevicePosture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DevicePosture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type DevicePosture;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DevicePostureType")]
    # [wasm_bindgen (structural , method , getter , js_class = "DevicePosture" , js_name = type)]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DevicePosture/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DevicePosture`, `DevicePostureType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn type_(this: &DevicePosture) -> DevicePostureType;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "DevicePosture" , js_name = onchange)]
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DevicePosture/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DevicePosture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn onchange(this: &DevicePosture) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , setter , js_class = "DevicePosture" , js_name = onchange)]
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DevicePosture/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DevicePosture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_onchange(this: &DevicePosture, value: Option<&::js_sys::Function>);
}
