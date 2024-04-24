#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaStreamTrackProcessorInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamTrackProcessorInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackProcessorInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type MediaStreamTrackProcessorInit;
    #[wasm_bindgen(method, getter = "maxBufferSize")]
    fn max_buffer_size_shim(this: &MediaStreamTrackProcessorInit) -> u16;
    #[wasm_bindgen(method, setter = "maxBufferSize")]
    fn set_max_buffer_size_shim(this: &MediaStreamTrackProcessorInit, val: u16);
    #[cfg(feature = "MediaStreamTrack")]
    #[wasm_bindgen(method, getter = "track")]
    fn track_shim(this: &MediaStreamTrackProcessorInit) -> &MediaStreamTrack;
    #[cfg(feature = "MediaStreamTrack")]
    #[wasm_bindgen(method, setter = "track")]
    fn set_track_shim(this: &MediaStreamTrackProcessorInit, val: &MediaStreamTrack);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `MediaStreamTrackProcessorInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackProcessorInit`*"]
pub trait MediaStreamTrackProcessorInitGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `maxBufferSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackProcessorInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn max_buffer_size(&self) -> u16;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Get the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaStreamTrackProcessorInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn track(&self) -> &MediaStreamTrack;
}
#[cfg(web_sys_unstable_apis)]
impl MediaStreamTrackProcessorInitGetters for MediaStreamTrackProcessorInit {
    #[cfg(web_sys_unstable_apis)]
    fn max_buffer_size(&self) -> u16 {
        self.max_buffer_size_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaStreamTrack")]
    fn track(&self) -> &MediaStreamTrack {
        self.track_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl MediaStreamTrackProcessorInit {
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Construct a new `MediaStreamTrackProcessorInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaStreamTrackProcessorInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(track: &MediaStreamTrack) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.track(track);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `maxBufferSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackProcessorInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_buffer_size(&mut self, val: u16) -> &mut Self {
        self.set_max_buffer_size_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Change the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaStreamTrackProcessorInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn track(&mut self, val: &MediaStreamTrack) -> &mut Self {
        self.set_track_shim(val);
        self
    }
}
