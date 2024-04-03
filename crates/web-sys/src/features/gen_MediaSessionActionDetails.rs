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
    #[wasm_bindgen(method, setter = "action")]
    fn action_shim(this: &MediaSessionActionDetails, val: MediaSessionAction);
    #[wasm_bindgen(method, setter = "fastSeek")]
    fn fast_seek_shim(this: &MediaSessionActionDetails, val: Option<bool>);
    #[wasm_bindgen(method, setter = "seekOffset")]
    fn seek_offset_shim(this: &MediaSessionActionDetails, val: Option<f64>);
    #[wasm_bindgen(method, setter = "seekTime")]
    fn seek_time_shim(this: &MediaSessionActionDetails, val: Option<f64>);
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
        ret.action(action);
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
        self.action_shim(val);
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
        self.fast_seek_shim(val);
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
        self.seek_offset_shim(val);
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
        self.seek_time_shim(val);
        self
    }
}
