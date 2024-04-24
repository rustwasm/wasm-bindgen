#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = VideoColorSpaceInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoColorSpaceInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type VideoColorSpaceInit;
    #[wasm_bindgen(method, getter = "fullRange")]
    fn full_range_shim(this: &VideoColorSpaceInit) -> bool;
    #[wasm_bindgen(method, setter = "fullRange")]
    fn set_full_range_shim(this: &VideoColorSpaceInit, val: bool);
    #[cfg(feature = "VideoMatrixCoefficients")]
    #[wasm_bindgen(method, getter = "matrix")]
    fn matrix_shim(this: &VideoColorSpaceInit) -> VideoMatrixCoefficients;
    #[cfg(feature = "VideoMatrixCoefficients")]
    #[wasm_bindgen(method, setter = "matrix")]
    fn set_matrix_shim(this: &VideoColorSpaceInit, val: VideoMatrixCoefficients);
    #[cfg(feature = "VideoColorPrimaries")]
    #[wasm_bindgen(method, getter = "primaries")]
    fn primaries_shim(this: &VideoColorSpaceInit) -> VideoColorPrimaries;
    #[cfg(feature = "VideoColorPrimaries")]
    #[wasm_bindgen(method, setter = "primaries")]
    fn set_primaries_shim(this: &VideoColorSpaceInit, val: VideoColorPrimaries);
    #[cfg(feature = "VideoTransferCharacteristics")]
    #[wasm_bindgen(method, getter = "transfer")]
    fn transfer_shim(this: &VideoColorSpaceInit) -> VideoTransferCharacteristics;
    #[cfg(feature = "VideoTransferCharacteristics")]
    #[wasm_bindgen(method, setter = "transfer")]
    fn set_transfer_shim(this: &VideoColorSpaceInit, val: VideoTransferCharacteristics);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `VideoColorSpaceInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`*"]
pub trait VideoColorSpaceInitGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `fullRange` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn full_range(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoMatrixCoefficients")]
    #[doc = "Get the `matrix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`, `VideoMatrixCoefficients`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn matrix(&self) -> VideoMatrixCoefficients;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoColorPrimaries")]
    #[doc = "Get the `primaries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorPrimaries`, `VideoColorSpaceInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn primaries(&self) -> VideoColorPrimaries;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoTransferCharacteristics")]
    #[doc = "Get the `transfer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`, `VideoTransferCharacteristics`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn transfer(&self) -> VideoTransferCharacteristics;
}
#[cfg(web_sys_unstable_apis)]
impl VideoColorSpaceInitGetters for VideoColorSpaceInit {
    #[cfg(web_sys_unstable_apis)]
    fn full_range(&self) -> bool {
        self.full_range_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoMatrixCoefficients")]
    fn matrix(&self) -> VideoMatrixCoefficients {
        self.matrix_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoColorPrimaries")]
    fn primaries(&self) -> VideoColorPrimaries {
        self.primaries_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoTransferCharacteristics")]
    fn transfer(&self) -> VideoTransferCharacteristics {
        self.transfer_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl VideoColorSpaceInit {
    #[doc = "Construct a new `VideoColorSpaceInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `fullRange` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn full_range(&mut self, val: bool) -> &mut Self {
        self.set_full_range_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoMatrixCoefficients")]
    #[doc = "Change the `matrix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`, `VideoMatrixCoefficients`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn matrix(&mut self, val: VideoMatrixCoefficients) -> &mut Self {
        self.set_matrix_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoColorPrimaries")]
    #[doc = "Change the `primaries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorPrimaries`, `VideoColorSpaceInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn primaries(&mut self, val: VideoColorPrimaries) -> &mut Self {
        self.set_primaries_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoTransferCharacteristics")]
    #[doc = "Change the `transfer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`, `VideoTransferCharacteristics`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn transfer(&mut self, val: VideoTransferCharacteristics) -> &mut Self {
        self.set_transfer_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for VideoColorSpaceInit {
    fn default() -> Self {
        Self::new()
    }
}
