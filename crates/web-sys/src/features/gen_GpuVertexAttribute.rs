#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUVertexAttribute)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuVertexAttribute` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuVertexAttribute;
    #[cfg(feature = "GpuVertexFormat")]
    #[wasm_bindgen(method, getter = "format")]
    fn format_shim(this: &GpuVertexAttribute) -> GpuVertexFormat;
    #[cfg(feature = "GpuVertexFormat")]
    #[wasm_bindgen(method, setter = "format")]
    fn set_format_shim(this: &GpuVertexAttribute, val: GpuVertexFormat);
    #[wasm_bindgen(method, getter = "offset")]
    fn offset_shim(this: &GpuVertexAttribute) -> f64;
    #[wasm_bindgen(method, setter = "offset")]
    fn set_offset_shim(this: &GpuVertexAttribute, val: f64);
    #[wasm_bindgen(method, getter = "shaderLocation")]
    fn shader_location_shim(this: &GpuVertexAttribute) -> u32;
    #[wasm_bindgen(method, setter = "shaderLocation")]
    fn set_shader_location_shim(this: &GpuVertexAttribute, val: u32);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuVertexAttribute` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`*"]
pub trait GpuVertexAttributeGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexFormat")]
    #[doc = "Get the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`, `GpuVertexFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn format(&self) -> GpuVertexFormat;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn offset(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `shaderLocation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn shader_location(&self) -> u32;
}
#[cfg(web_sys_unstable_apis)]
impl GpuVertexAttributeGetters for GpuVertexAttribute {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexFormat")]
    fn format(&self) -> GpuVertexFormat {
        self.format_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn offset(&self) -> f64 {
        self.offset_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn shader_location(&self) -> u32 {
        self.shader_location_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuVertexAttribute {
    #[cfg(feature = "GpuVertexFormat")]
    #[doc = "Construct a new `GpuVertexAttribute`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`, `GpuVertexFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(format: GpuVertexFormat, offset: f64, shader_location: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::format(&mut ret, format);
        Self::offset(&mut ret, offset);
        Self::shader_location(&mut ret, shader_location);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexFormat")]
    #[doc = "Change the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`, `GpuVertexFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn format(&mut self, val: GpuVertexFormat) -> &mut Self {
        self.set_format_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn offset(&mut self, val: f64) -> &mut Self {
        self.set_offset_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `shaderLocation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn shader_location(&mut self, val: u32) -> &mut Self {
        self.set_shader_location_shim(val);
        self
    }
}
