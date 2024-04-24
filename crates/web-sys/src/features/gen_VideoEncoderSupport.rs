#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = VideoEncoderSupport)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoEncoderSupport` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type VideoEncoderSupport;
    #[cfg(feature = "VideoEncoderConfig")]
    #[wasm_bindgen(method, getter = "config")]
    fn config_shim(this: &VideoEncoderSupport) -> VideoEncoderConfig;
    #[cfg(feature = "VideoEncoderConfig")]
    #[wasm_bindgen(method, setter = "config")]
    fn set_config_shim(this: &VideoEncoderSupport, val: &VideoEncoderConfig);
    #[wasm_bindgen(method, getter = "supported")]
    fn supported_shim(this: &VideoEncoderSupport) -> bool;
    #[wasm_bindgen(method, setter = "supported")]
    fn set_supported_shim(this: &VideoEncoderSupport, val: bool);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `VideoEncoderSupport` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `VideoEncoderSupport`*"]
pub trait VideoEncoderSupportGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoEncoderConfig")]
    #[doc = "Get the `config` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`, `VideoEncoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn config(&self) -> VideoEncoderConfig;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `supported` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn supported(&self) -> bool;
}
#[cfg(web_sys_unstable_apis)]
impl VideoEncoderSupportGetters for VideoEncoderSupport {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoEncoderConfig")]
    fn config(&self) -> VideoEncoderConfig {
        self.config_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn supported(&self) -> bool {
        self.supported_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl VideoEncoderSupport {
    #[doc = "Construct a new `VideoEncoderSupport`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoEncoderConfig")]
    #[doc = "Change the `config` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`, `VideoEncoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn config(&mut self, val: &VideoEncoderConfig) -> &mut Self {
        self.set_config_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `supported` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn supported(&mut self, val: bool) -> &mut Self {
        self.set_supported_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for VideoEncoderSupport {
    fn default() -> Self {
        Self::new()
    }
}
