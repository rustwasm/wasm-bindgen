#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AudioDecoderSupport)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioDecoderSupport` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type AudioDecoderSupport;
    #[cfg(feature = "AudioDecoderConfig")]
    #[wasm_bindgen(method, getter = "config")]
    fn config_shim(this: &AudioDecoderSupport) -> &AudioDecoderConfig;
    #[cfg(feature = "AudioDecoderConfig")]
    #[wasm_bindgen(method, setter = "config")]
    fn set_config_shim(this: &AudioDecoderSupport, val: &AudioDecoderConfig);
    #[wasm_bindgen(method, getter = "supported")]
    fn supported_shim(this: &AudioDecoderSupport) -> bool;
    #[wasm_bindgen(method, setter = "supported")]
    fn set_supported_shim(this: &AudioDecoderSupport, val: bool);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `AudioDecoderSupport` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AudioDecoderSupport`*"]
pub trait AudioDecoderSupportGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDecoderConfig")]
    #[doc = "Get the `config` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderConfig`, `AudioDecoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn config(&self) -> &AudioDecoderConfig;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `supported` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn supported(&self) -> bool;
}
#[cfg(web_sys_unstable_apis)]
impl AudioDecoderSupportGetters for AudioDecoderSupport {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDecoderConfig")]
    fn config(&self) -> &AudioDecoderConfig {
        self.config_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn supported(&self) -> bool {
        self.supported_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl AudioDecoderSupport {
    #[doc = "Construct a new `AudioDecoderSupport`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDecoderConfig")]
    #[doc = "Change the `config` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderConfig`, `AudioDecoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn config(&mut self, val: &AudioDecoderConfig) -> &mut Self {
        self.set_config_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `supported` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn supported(&mut self, val: bool) -> &mut Self {
        self.set_supported_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for AudioDecoderSupport {
    fn default() -> Self {
        Self::new()
    }
}
