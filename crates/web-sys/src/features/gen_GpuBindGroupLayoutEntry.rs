#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUBindGroupLayoutEntry)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuBindGroupLayoutEntry` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuBindGroupLayoutEntry;
    #[wasm_bindgen(method, setter = "binding")]
    fn binding_shim(this: &GpuBindGroupLayoutEntry, val: u32);
    #[cfg(feature = "GpuBufferBindingLayout")]
    #[wasm_bindgen(method, setter = "buffer")]
    fn buffer_shim(this: &GpuBindGroupLayoutEntry, val: &GpuBufferBindingLayout);
    #[cfg(feature = "GpuExternalTextureBindingLayout")]
    #[wasm_bindgen(method, setter = "externalTexture")]
    fn external_texture_shim(this: &GpuBindGroupLayoutEntry, val: &GpuExternalTextureBindingLayout);
    #[cfg(feature = "GpuSamplerBindingLayout")]
    #[wasm_bindgen(method, setter = "sampler")]
    fn sampler_shim(this: &GpuBindGroupLayoutEntry, val: &GpuSamplerBindingLayout);
    #[cfg(feature = "GpuStorageTextureBindingLayout")]
    #[wasm_bindgen(method, setter = "storageTexture")]
    fn storage_texture_shim(this: &GpuBindGroupLayoutEntry, val: &GpuStorageTextureBindingLayout);
    #[cfg(feature = "GpuTextureBindingLayout")]
    #[wasm_bindgen(method, setter = "texture")]
    fn texture_shim(this: &GpuBindGroupLayoutEntry, val: &GpuTextureBindingLayout);
    #[wasm_bindgen(method, setter = "visibility")]
    fn visibility_shim(this: &GpuBindGroupLayoutEntry, val: u32);
}
#[cfg(web_sys_unstable_apis)]
impl GpuBindGroupLayoutEntry {
    #[doc = "Construct a new `GpuBindGroupLayoutEntry`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(binding: u32, visibility: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.binding(binding);
        ret.visibility(visibility);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `binding` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn binding(&mut self, val: u32) -> &mut Self {
        self.binding_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBufferBindingLayout")]
    #[doc = "Change the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`, `GpuBufferBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn buffer(&mut self, val: &GpuBufferBindingLayout) -> &mut Self {
        self.buffer_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuExternalTextureBindingLayout")]
    #[doc = "Change the `externalTexture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`, `GpuExternalTextureBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn external_texture(&mut self, val: &GpuExternalTextureBindingLayout) -> &mut Self {
        self.external_texture_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuSamplerBindingLayout")]
    #[doc = "Change the `sampler` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`, `GpuSamplerBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn sampler(&mut self, val: &GpuSamplerBindingLayout) -> &mut Self {
        self.sampler_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStorageTextureBindingLayout")]
    #[doc = "Change the `storageTexture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`, `GpuStorageTextureBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn storage_texture(&mut self, val: &GpuStorageTextureBindingLayout) -> &mut Self {
        self.storage_texture_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureBindingLayout")]
    #[doc = "Change the `texture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`, `GpuTextureBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn texture(&mut self, val: &GpuTextureBindingLayout) -> &mut Self {
        self.texture_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `visibility` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn visibility(&mut self, val: u32) -> &mut Self {
        self.visibility_shim(val);
        self
    }
}
