#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUBlendState)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuBlendState` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuBlendState;
    #[cfg(feature = "GpuBlendComponent")]
    #[wasm_bindgen(method, getter = "alpha")]
    fn alpha_shim(this: &GpuBlendState) -> GpuBlendComponent;
    #[cfg(feature = "GpuBlendComponent")]
    #[wasm_bindgen(method, setter = "alpha")]
    fn set_alpha_shim(this: &GpuBlendState, val: &GpuBlendComponent);
    #[cfg(feature = "GpuBlendComponent")]
    #[wasm_bindgen(method, getter = "color")]
    fn color_shim(this: &GpuBlendState) -> GpuBlendComponent;
    #[cfg(feature = "GpuBlendComponent")]
    #[wasm_bindgen(method, setter = "color")]
    fn set_color_shim(this: &GpuBlendState, val: &GpuBlendComponent);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuBlendState` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuBlendState`*"]
pub trait GpuBlendStateGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendComponent")]
    #[doc = "Get the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn alpha(&self) -> GpuBlendComponent;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendComponent")]
    #[doc = "Get the `color` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn color(&self) -> GpuBlendComponent;
}
#[cfg(web_sys_unstable_apis)]
impl GpuBlendStateGetters for GpuBlendState {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendComponent")]
    fn alpha(&self) -> GpuBlendComponent {
        self.alpha_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendComponent")]
    fn color(&self) -> GpuBlendComponent {
        self.color_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuBlendState {
    #[cfg(feature = "GpuBlendComponent")]
    #[doc = "Construct a new `GpuBlendState`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(alpha: &GpuBlendComponent, color: &GpuBlendComponent) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::alpha(&mut ret, alpha);
        Self::color(&mut ret, color);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendComponent")]
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn alpha(&mut self, val: &GpuBlendComponent) -> &mut Self {
        self.set_alpha_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendComponent")]
    #[doc = "Change the `color` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn color(&mut self, val: &GpuBlendComponent) -> &mut Self {
        self.set_color_shim(val);
        self
    }
}
