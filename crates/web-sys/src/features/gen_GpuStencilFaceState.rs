#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUStencilFaceState)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuStencilFaceState` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuStencilFaceState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuStencilFaceState;
    #[cfg(feature = "GpuCompareFunction")]
    #[wasm_bindgen(method, setter = "compare")]
    fn compare_shim(this: &GpuStencilFaceState, val: GpuCompareFunction);
    #[cfg(feature = "GpuStencilOperation")]
    #[wasm_bindgen(method, setter = "depthFailOp")]
    fn depth_fail_op_shim(this: &GpuStencilFaceState, val: GpuStencilOperation);
    #[cfg(feature = "GpuStencilOperation")]
    #[wasm_bindgen(method, setter = "failOp")]
    fn fail_op_shim(this: &GpuStencilFaceState, val: GpuStencilOperation);
    #[cfg(feature = "GpuStencilOperation")]
    #[wasm_bindgen(method, setter = "passOp")]
    fn pass_op_shim(this: &GpuStencilFaceState, val: GpuStencilOperation);
}
#[cfg(web_sys_unstable_apis)]
impl GpuStencilFaceState {
    #[doc = "Construct a new `GpuStencilFaceState`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuStencilFaceState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCompareFunction")]
    #[doc = "Change the `compare` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompareFunction`, `GpuStencilFaceState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn compare(&mut self, val: GpuCompareFunction) -> &mut Self {
        self.compare_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilOperation")]
    #[doc = "Change the `depthFailOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuStencilFaceState`, `GpuStencilOperation`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_fail_op(&mut self, val: GpuStencilOperation) -> &mut Self {
        self.depth_fail_op_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilOperation")]
    #[doc = "Change the `failOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuStencilFaceState`, `GpuStencilOperation`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn fail_op(&mut self, val: GpuStencilOperation) -> &mut Self {
        self.fail_op_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilOperation")]
    #[doc = "Change the `passOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuStencilFaceState`, `GpuStencilOperation`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn pass_op(&mut self, val: GpuStencilOperation) -> &mut Self {
        self.pass_op_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for GpuStencilFaceState {
    fn default() -> Self {
        Self::new()
    }
}
