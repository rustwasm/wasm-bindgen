#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUVertexState)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuVertexState` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuVertexState;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `entryPoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "entryPoint")]
    pub fn get_entry_point(this: &GpuVertexState) -> Option<String>;
    #[wasm_bindgen(method, setter = "entryPoint")]
    fn set_entry_point(this: &GpuVertexState, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuShaderModule")]
    #[doc = "Get the `module` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModule`, `GpuVertexState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "module")]
    pub fn get_module(this: &GpuVertexState) -> GpuShaderModule;
    #[cfg(feature = "GpuShaderModule")]
    #[wasm_bindgen(method, setter = "module")]
    fn set_module(this: &GpuVertexState, val: &GpuShaderModule);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `buffers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "buffers")]
    pub fn get_buffers(this: &GpuVertexState) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "buffers")]
    fn set_buffers(this: &GpuVertexState, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
impl GpuVertexState {
    #[cfg(feature = "GpuShaderModule")]
    #[doc = "Construct a new `GpuVertexState`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModule`, `GpuVertexState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(module: &GpuShaderModule) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.module(module);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `entryPoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn entry_point(&mut self, val: &str) -> &mut Self {
        self.set_entry_point(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuShaderModule")]
    #[doc = "Change the `module` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModule`, `GpuVertexState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn module(&mut self, val: &GpuShaderModule) -> &mut Self {
        self.set_module(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `buffers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn buffers(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_buffers(val);
        self
    }
}
