#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUObjectDescriptorBase)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuObjectDescriptorBase` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuObjectDescriptorBase`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuObjectDescriptorBase;
    #[wasm_bindgen(method, getter = "label")]
    fn label_shim(this: &GpuObjectDescriptorBase) -> String;
    #[wasm_bindgen(method, setter = "label")]
    fn set_label_shim(this: &GpuObjectDescriptorBase, val: &str);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuObjectDescriptorBase` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuObjectDescriptorBase`*"]
pub trait GpuObjectDescriptorBaseGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuObjectDescriptorBase`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn label(&self) -> String;
}
#[cfg(web_sys_unstable_apis)]
impl GpuObjectDescriptorBaseGetters for GpuObjectDescriptorBase {
    #[cfg(web_sys_unstable_apis)]
    fn label(&self) -> String {
        self.label_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuObjectDescriptorBase {
    #[doc = "Construct a new `GpuObjectDescriptorBase`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuObjectDescriptorBase`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuObjectDescriptorBase`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for GpuObjectDescriptorBase {
    fn default() -> Self {
        Self::new()
    }
}
