#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUProgrammableStage)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuProgrammableStage` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuProgrammableStage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuProgrammableStage;
    #[wasm_bindgen(method, getter = "entryPoint")]
    fn entry_point_shim(this: &GpuProgrammableStage) -> String;
    #[wasm_bindgen(method, setter = "entryPoint")]
    fn set_entry_point_shim(this: &GpuProgrammableStage, val: &str);
    #[cfg(feature = "GpuShaderModule")]
    #[wasm_bindgen(method, getter = "module")]
    fn module_shim(this: &GpuProgrammableStage) -> GpuShaderModule;
    #[cfg(feature = "GpuShaderModule")]
    #[wasm_bindgen(method, setter = "module")]
    fn set_module_shim(this: &GpuProgrammableStage, val: &GpuShaderModule);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuProgrammableStage` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuProgrammableStage`*"]
pub trait GpuProgrammableStageGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `entryPoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuProgrammableStage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn entry_point(&self) -> String;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuShaderModule")]
    #[doc = "Get the `module` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuProgrammableStage`, `GpuShaderModule`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn module(&self) -> GpuShaderModule;
}
#[cfg(web_sys_unstable_apis)]
impl GpuProgrammableStageGetters for GpuProgrammableStage {
    #[cfg(web_sys_unstable_apis)]
    fn entry_point(&self) -> String {
        self.entry_point_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuShaderModule")]
    fn module(&self) -> GpuShaderModule {
        self.module_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuProgrammableStage {
    #[cfg(feature = "GpuShaderModule")]
    #[doc = "Construct a new `GpuProgrammableStage`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuProgrammableStage`, `GpuShaderModule`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(module: &GpuShaderModule) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::module(&mut ret, module);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `entryPoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuProgrammableStage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn entry_point(&mut self, val: &str) -> &mut Self {
        self.set_entry_point_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuShaderModule")]
    #[doc = "Change the `module` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuProgrammableStage`, `GpuShaderModule`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn module(&mut self, val: &GpuShaderModule) -> &mut Self {
        self.set_module_shim(val);
        self
    }
}
