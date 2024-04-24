#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUShaderModuleDescriptor)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuShaderModuleDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuShaderModuleDescriptor;
    #[wasm_bindgen(method, getter = "label")]
    fn label_shim(this: &GpuShaderModuleDescriptor) -> &str;
    #[wasm_bindgen(method, setter = "label")]
    fn set_label_shim(this: &GpuShaderModuleDescriptor, val: &str);
    #[wasm_bindgen(method, getter = "code")]
    fn code_shim(this: &GpuShaderModuleDescriptor) -> &str;
    #[wasm_bindgen(method, setter = "code")]
    fn set_code_shim(this: &GpuShaderModuleDescriptor, val: &str);
    #[wasm_bindgen(method, getter = "compilationHints")]
    fn compilation_hints_shim(this: &GpuShaderModuleDescriptor) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "compilationHints")]
    fn set_compilation_hints_shim(this: &GpuShaderModuleDescriptor, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "sourceMap")]
    fn source_map_shim(this: &GpuShaderModuleDescriptor) -> &::js_sys::Object;
    #[wasm_bindgen(method, setter = "sourceMap")]
    fn set_source_map_shim(this: &GpuShaderModuleDescriptor, val: &::js_sys::Object);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuShaderModuleDescriptor` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleDescriptor`*"]
pub trait GpuShaderModuleDescriptorGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn label(&self) -> &str;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `code` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn code(&self) -> &str;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `compilationHints` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn compilation_hints(&self) -> &::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `sourceMap` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn source_map(&self) -> &::js_sys::Object;
}
#[cfg(web_sys_unstable_apis)]
impl GpuShaderModuleDescriptorGetters for GpuShaderModuleDescriptor {
    #[cfg(web_sys_unstable_apis)]
    fn label(&self) -> &str {
        self.label_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn code(&self) -> &str {
        self.code_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn compilation_hints(&self) -> &::wasm_bindgen::JsValue {
        self.compilation_hints_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn source_map(&self) -> &::js_sys::Object {
        self.source_map_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuShaderModuleDescriptor {
    #[doc = "Construct a new `GpuShaderModuleDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(code: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.code(code);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `code` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn code(&mut self, val: &str) -> &mut Self {
        self.set_code_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `compilationHints` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn compilation_hints(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_compilation_hints_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `sourceMap` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn source_map(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_source_map_shim(val);
        self
    }
}
