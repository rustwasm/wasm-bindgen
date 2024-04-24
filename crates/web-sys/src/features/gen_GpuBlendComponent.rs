#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUBlendComponent)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuBlendComponent` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuBlendComponent;
    #[cfg(feature = "GpuBlendFactor")]
    #[wasm_bindgen(method, getter = "dstFactor")]
    fn dst_factor_shim(this: &GpuBlendComponent) -> GpuBlendFactor;
    #[cfg(feature = "GpuBlendFactor")]
    #[wasm_bindgen(method, setter = "dstFactor")]
    fn set_dst_factor_shim(this: &GpuBlendComponent, val: GpuBlendFactor);
    #[cfg(feature = "GpuBlendOperation")]
    #[wasm_bindgen(method, getter = "operation")]
    fn operation_shim(this: &GpuBlendComponent) -> GpuBlendOperation;
    #[cfg(feature = "GpuBlendOperation")]
    #[wasm_bindgen(method, setter = "operation")]
    fn set_operation_shim(this: &GpuBlendComponent, val: GpuBlendOperation);
    #[cfg(feature = "GpuBlendFactor")]
    #[wasm_bindgen(method, getter = "srcFactor")]
    fn src_factor_shim(this: &GpuBlendComponent) -> GpuBlendFactor;
    #[cfg(feature = "GpuBlendFactor")]
    #[wasm_bindgen(method, setter = "srcFactor")]
    fn set_src_factor_shim(this: &GpuBlendComponent, val: GpuBlendFactor);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuBlendComponent` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`*"]
pub trait GpuBlendComponentGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendFactor")]
    #[doc = "Get the `dstFactor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendFactor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn dst_factor(&self) -> GpuBlendFactor;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendOperation")]
    #[doc = "Get the `operation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendOperation`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn operation(&self) -> GpuBlendOperation;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendFactor")]
    #[doc = "Get the `srcFactor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendFactor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn src_factor(&self) -> GpuBlendFactor;
}
#[cfg(web_sys_unstable_apis)]
impl GpuBlendComponentGetters for GpuBlendComponent {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendFactor")]
    fn dst_factor(&self) -> GpuBlendFactor {
        self.dst_factor_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendOperation")]
    fn operation(&self) -> GpuBlendOperation {
        self.operation_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendFactor")]
    fn src_factor(&self) -> GpuBlendFactor {
        self.src_factor_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuBlendComponent {
    #[doc = "Construct a new `GpuBlendComponent`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendFactor")]
    #[doc = "Change the `dstFactor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendFactor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn dst_factor(&mut self, val: GpuBlendFactor) -> &mut Self {
        self.set_dst_factor_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendOperation")]
    #[doc = "Change the `operation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendOperation`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn operation(&mut self, val: GpuBlendOperation) -> &mut Self {
        self.set_operation_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendFactor")]
    #[doc = "Change the `srcFactor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendFactor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn src_factor(&mut self, val: GpuBlendFactor) -> &mut Self {
        self.set_src_factor_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for GpuBlendComponent {
    fn default() -> Self {
        Self::new()
    }
}
