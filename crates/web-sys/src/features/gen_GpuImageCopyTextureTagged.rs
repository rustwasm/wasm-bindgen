#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUImageCopyTextureTagged)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuImageCopyTextureTagged` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTextureTagged`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuImageCopyTextureTagged;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureAspect")]
    #[doc = "Get the `aspect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTextureTagged`, `GpuTextureAspect`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "aspect")]
    pub fn get_aspect(this: &GpuImageCopyTextureTagged) -> Option<GpuTextureAspect>;
    #[cfg(feature = "GpuTextureAspect")]
    #[wasm_bindgen(method, setter = "aspect")]
    fn set_aspect(this: &GpuImageCopyTextureTagged, val: GpuTextureAspect);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `mipLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTextureTagged`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "mipLevel")]
    pub fn get_mip_level(this: &GpuImageCopyTextureTagged) -> Option<u32>;
    #[wasm_bindgen(method, setter = "mipLevel")]
    fn set_mip_level(this: &GpuImageCopyTextureTagged, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTextureTagged`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "origin")]
    pub fn get_origin(this: &GpuImageCopyTextureTagged) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "origin")]
    fn set_origin(this: &GpuImageCopyTextureTagged, val: &::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTexture")]
    #[doc = "Get the `texture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTextureTagged`, `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "texture")]
    pub fn get_texture(this: &GpuImageCopyTextureTagged) -> GpuTexture;
    #[cfg(feature = "GpuTexture")]
    #[wasm_bindgen(method, setter = "texture")]
    fn set_texture(this: &GpuImageCopyTextureTagged, val: &GpuTexture);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `premultipliedAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTextureTagged`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "premultipliedAlpha")]
    pub fn get_premultiplied_alpha(this: &GpuImageCopyTextureTagged) -> Option<bool>;
    #[wasm_bindgen(method, setter = "premultipliedAlpha")]
    fn set_premultiplied_alpha(this: &GpuImageCopyTextureTagged, val: bool);
}
#[cfg(web_sys_unstable_apis)]
impl GpuImageCopyTextureTagged {
    #[cfg(feature = "GpuTexture")]
    #[doc = "Construct a new `GpuImageCopyTextureTagged`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTextureTagged`, `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(texture: &GpuTexture) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.texture(texture);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureAspect")]
    #[doc = "Change the `aspect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTextureTagged`, `GpuTextureAspect`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn aspect(&mut self, val: GpuTextureAspect) -> &mut Self {
        self.set_aspect(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `mipLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTextureTagged`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn mip_level(&mut self, val: u32) -> &mut Self {
        self.set_mip_level(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTextureTagged`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn origin(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_origin(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTexture")]
    #[doc = "Change the `texture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTextureTagged`, `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn texture(&mut self, val: &GpuTexture) -> &mut Self {
        self.set_texture(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `premultipliedAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTextureTagged`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn premultiplied_alpha(&mut self, val: bool) -> &mut Self {
        self.set_premultiplied_alpha(val);
        self
    }
}
