#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUPrimitiveState)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuPrimitiveState` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPrimitiveState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuPrimitiveState;
    #[cfg(feature = "GpuCullMode")]
    #[wasm_bindgen(method, getter = "cullMode")]
    fn cull_mode_shim(this: &GpuPrimitiveState) -> GpuCullMode;
    #[cfg(feature = "GpuCullMode")]
    #[wasm_bindgen(method, setter = "cullMode")]
    fn set_cull_mode_shim(this: &GpuPrimitiveState, val: GpuCullMode);
    #[cfg(feature = "GpuFrontFace")]
    #[wasm_bindgen(method, getter = "frontFace")]
    fn front_face_shim(this: &GpuPrimitiveState) -> GpuFrontFace;
    #[cfg(feature = "GpuFrontFace")]
    #[wasm_bindgen(method, setter = "frontFace")]
    fn set_front_face_shim(this: &GpuPrimitiveState, val: GpuFrontFace);
    #[cfg(feature = "GpuIndexFormat")]
    #[wasm_bindgen(method, getter = "stripIndexFormat")]
    fn strip_index_format_shim(this: &GpuPrimitiveState) -> GpuIndexFormat;
    #[cfg(feature = "GpuIndexFormat")]
    #[wasm_bindgen(method, setter = "stripIndexFormat")]
    fn set_strip_index_format_shim(this: &GpuPrimitiveState, val: GpuIndexFormat);
    #[cfg(feature = "GpuPrimitiveTopology")]
    #[wasm_bindgen(method, getter = "topology")]
    fn topology_shim(this: &GpuPrimitiveState) -> GpuPrimitiveTopology;
    #[cfg(feature = "GpuPrimitiveTopology")]
    #[wasm_bindgen(method, setter = "topology")]
    fn set_topology_shim(this: &GpuPrimitiveState, val: GpuPrimitiveTopology);
    #[wasm_bindgen(method, getter = "unclippedDepth")]
    fn unclipped_depth_shim(this: &GpuPrimitiveState) -> bool;
    #[wasm_bindgen(method, setter = "unclippedDepth")]
    fn set_unclipped_depth_shim(this: &GpuPrimitiveState, val: bool);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuPrimitiveState` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuPrimitiveState`*"]
pub trait GpuPrimitiveStateGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCullMode")]
    #[doc = "Get the `cullMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCullMode`, `GpuPrimitiveState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn cull_mode(&self) -> GpuCullMode;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFrontFace")]
    #[doc = "Get the `frontFace` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFrontFace`, `GpuPrimitiveState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn front_face(&self) -> GpuFrontFace;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuIndexFormat")]
    #[doc = "Get the `stripIndexFormat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuIndexFormat`, `GpuPrimitiveState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn strip_index_format(&self) -> GpuIndexFormat;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPrimitiveTopology")]
    #[doc = "Get the `topology` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPrimitiveState`, `GpuPrimitiveTopology`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn topology(&self) -> GpuPrimitiveTopology;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `unclippedDepth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPrimitiveState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn unclipped_depth(&self) -> bool;
}
#[cfg(web_sys_unstable_apis)]
impl GpuPrimitiveStateGetters for GpuPrimitiveState {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCullMode")]
    fn cull_mode(&self) -> GpuCullMode {
        self.cull_mode_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFrontFace")]
    fn front_face(&self) -> GpuFrontFace {
        self.front_face_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuIndexFormat")]
    fn strip_index_format(&self) -> GpuIndexFormat {
        self.strip_index_format_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPrimitiveTopology")]
    fn topology(&self) -> GpuPrimitiveTopology {
        self.topology_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn unclipped_depth(&self) -> bool {
        self.unclipped_depth_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuPrimitiveState {
    #[doc = "Construct a new `GpuPrimitiveState`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPrimitiveState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCullMode")]
    #[doc = "Change the `cullMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCullMode`, `GpuPrimitiveState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn cull_mode(&mut self, val: GpuCullMode) -> &mut Self {
        self.set_cull_mode_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFrontFace")]
    #[doc = "Change the `frontFace` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFrontFace`, `GpuPrimitiveState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn front_face(&mut self, val: GpuFrontFace) -> &mut Self {
        self.set_front_face_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuIndexFormat")]
    #[doc = "Change the `stripIndexFormat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuIndexFormat`, `GpuPrimitiveState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn strip_index_format(&mut self, val: GpuIndexFormat) -> &mut Self {
        self.set_strip_index_format_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPrimitiveTopology")]
    #[doc = "Change the `topology` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPrimitiveState`, `GpuPrimitiveTopology`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn topology(&mut self, val: GpuPrimitiveTopology) -> &mut Self {
        self.set_topology_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `unclippedDepth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPrimitiveState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn unclipped_depth(&mut self, val: bool) -> &mut Self {
        self.set_unclipped_depth_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for GpuPrimitiveState {
    fn default() -> Self {
        Self::new()
    }
}
