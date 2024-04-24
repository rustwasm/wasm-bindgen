#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaSessionActionDetails)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaSessionActionDetails` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type MediaSessionActionDetails;
    #[cfg(feature = "MediaSessionAction")]
    #[wasm_bindgen(method, getter = "action")]
    fn action_shim(this: &MediaSessionActionDetails) -> MediaSessionAction;
    #[cfg(feature = "MediaSessionAction")]
    #[wasm_bindgen(method, setter = "action")]
    fn set_action_shim(this: &MediaSessionActionDetails, val: MediaSessionAction);
    #[wasm_bindgen(method, getter = "fastSeek")]
    fn fast_seek_shim(this: &MediaSessionActionDetails) -> Option<bool>;
    #[wasm_bindgen(method, setter = "fastSeek")]
    fn set_fast_seek_shim(this: &MediaSessionActionDetails, val: Option<bool>);
    #[wasm_bindgen(method, getter = "seekOffset")]
    fn seek_offset_shim(this: &MediaSessionActionDetails) -> Option<f64>;
    #[wasm_bindgen(method, setter = "seekOffset")]
    fn set_seek_offset_shim(this: &MediaSessionActionDetails, val: Option<f64>);
    #[wasm_bindgen(method, getter = "seekTime")]
    fn seek_time_shim(this: &MediaSessionActionDetails) -> Option<f64>;
    #[wasm_bindgen(method, setter = "seekTime")]
    fn set_seek_time_shim(this: &MediaSessionActionDetails, val: Option<f64>);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `MediaSessionActionDetails` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
pub trait MediaSessionActionDetailsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaSessionAction")]
    #[doc = "Get the `action` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionAction`, `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn action(&self) -> MediaSessionAction;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `fastSeek` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn fast_seek(&self) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `seekOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn seek_offset(&self) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `seekTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn seek_time(&self) -> Option<f64>;
}
#[cfg(web_sys_unstable_apis)]
impl MediaSessionActionDetailsGetters for MediaSessionActionDetails {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaSessionAction")]
    fn action(&self) -> MediaSessionAction {
        self.action_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn fast_seek(&self) -> Option<bool> {
        self.fast_seek_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn seek_offset(&self) -> Option<f64> {
        self.seek_offset_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn seek_time(&self) -> Option<f64> {
        self.seek_time_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl MediaSessionActionDetails {
    #[cfg(feature = "MediaSessionAction")]
    #[doc = "Construct a new `MediaSessionActionDetails`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionAction`, `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(action: MediaSessionAction) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::action(&mut ret, action);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaSessionAction")]
    #[doc = "Change the `action` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionAction`, `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn action(&mut self, val: MediaSessionAction) -> &mut Self {
        self.set_action_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `fastSeek` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn fast_seek(&mut self, val: Option<bool>) -> &mut Self {
        self.set_fast_seek_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `seekOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn seek_offset(&mut self, val: Option<f64>) -> &mut Self {
        self.set_seek_offset_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `seekTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn seek_time(&mut self, val: Option<f64>) -> &mut Self {
        self.set_seek_time_shim(val);
        self
    }
}
