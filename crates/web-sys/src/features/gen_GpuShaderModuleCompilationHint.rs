#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUShaderModuleCompilationHint)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuShaderModuleCompilationHint` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleCompilationHint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuShaderModuleCompilationHint;
    #[wasm_bindgen(method, getter = "entryPoint")]
    fn entry_point_shim(this: &GpuShaderModuleCompilationHint) -> String;
    #[wasm_bindgen(method, setter = "entryPoint")]
    fn set_entry_point_shim(this: &GpuShaderModuleCompilationHint, val: &str);
    #[wasm_bindgen(method, getter = "layout")]
    fn layout_shim(this: &GpuShaderModuleCompilationHint) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "layout")]
    fn set_layout_shim(this: &GpuShaderModuleCompilationHint, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuShaderModuleCompilationHint` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleCompilationHint`*"]
pub trait GpuShaderModuleCompilationHintGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `entryPoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleCompilationHint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn entry_point(&self) -> String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleCompilationHint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn layout(&self) -> ::wasm_bindgen::JsValue;
}
#[cfg(web_sys_unstable_apis)]
impl GpuShaderModuleCompilationHintGetters for GpuShaderModuleCompilationHint {
    #[cfg(web_sys_unstable_apis)]
    fn entry_point(&self) -> String {
        self.entry_point_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn layout(&self) -> ::wasm_bindgen::JsValue {
        self.layout_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuShaderModuleCompilationHint {
    #[doc = "Construct a new `GpuShaderModuleCompilationHint`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleCompilationHint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(entry_point: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.entry_point(entry_point);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `entryPoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleCompilationHint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn entry_point(&mut self, val: &str) -> &mut Self {
        self.set_entry_point_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleCompilationHint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn layout(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_layout_shim(val);
        self
    }
}
