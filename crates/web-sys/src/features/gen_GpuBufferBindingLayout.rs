#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUBufferBindingLayout)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuBufferBindingLayout` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuBufferBindingLayout;
    #[wasm_bindgen(method, getter = "hasDynamicOffset")]
    fn has_dynamic_offset_shim(this: &GpuBufferBindingLayout) -> bool;
    #[wasm_bindgen(method, setter = "hasDynamicOffset")]
    fn set_has_dynamic_offset_shim(this: &GpuBufferBindingLayout, val: bool);
    #[wasm_bindgen(method, getter = "minBindingSize")]
    fn min_binding_size_shim(this: &GpuBufferBindingLayout) -> f64;
    #[wasm_bindgen(method, setter = "minBindingSize")]
    fn set_min_binding_size_shim(this: &GpuBufferBindingLayout, val: f64);
    #[cfg(feature = "GpuBufferBindingType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &GpuBufferBindingLayout) -> GpuBufferBindingType;
    #[cfg(feature = "GpuBufferBindingType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &GpuBufferBindingLayout, val: GpuBufferBindingType);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuBufferBindingLayout` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuBufferBindingLayout`*"]
pub trait GpuBufferBindingLayoutGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `hasDynamicOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn has_dynamic_offset(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `minBindingSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn min_binding_size(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBufferBindingType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBindingLayout`, `GpuBufferBindingType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn type_(&self) -> GpuBufferBindingType;
}
#[cfg(web_sys_unstable_apis)]
impl GpuBufferBindingLayoutGetters for GpuBufferBindingLayout {
    #[cfg(web_sys_unstable_apis)]
    fn has_dynamic_offset(&self) -> bool {
        self.has_dynamic_offset_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn min_binding_size(&self) -> f64 {
        self.min_binding_size_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBufferBindingType")]
    fn type_(&self) -> GpuBufferBindingType {
        self.type__shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuBufferBindingLayout {
    #[doc = "Construct a new `GpuBufferBindingLayout`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `hasDynamicOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn has_dynamic_offset(&mut self, val: bool) -> &mut Self {
        self.set_has_dynamic_offset_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `minBindingSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn min_binding_size(&mut self, val: f64) -> &mut Self {
        self.set_min_binding_size_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBufferBindingType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBindingLayout`, `GpuBufferBindingType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn type_(&mut self, val: GpuBufferBindingType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for GpuBufferBindingLayout {
    fn default() -> Self {
        Self::new()
    }
}
