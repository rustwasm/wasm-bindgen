#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ImageDecodeOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageDecodeOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecodeOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type ImageDecodeOptions;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `completeFramesOnly` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecodeOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "completeFramesOnly")]
    pub fn get_complete_frames_only(this: &ImageDecodeOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter = "completeFramesOnly")]
    fn set_complete_frames_only(this: &ImageDecodeOptions, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `frameIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecodeOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "frameIndex")]
    pub fn get_frame_index(this: &ImageDecodeOptions) -> Option<u32>;
    #[wasm_bindgen(method, setter = "frameIndex")]
    fn set_frame_index(this: &ImageDecodeOptions, val: u32);
}
#[cfg(web_sys_unstable_apis)]
impl ImageDecodeOptions {
    #[doc = "Construct a new `ImageDecodeOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecodeOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `completeFramesOnly` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecodeOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn complete_frames_only(&mut self, val: bool) -> &mut Self {
        self.set_complete_frames_only(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `frameIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecodeOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn frame_index(&mut self, val: u32) -> &mut Self {
        self.set_frame_index(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for ImageDecodeOptions {
    fn default() -> Self {
        Self::new()
    }
}
