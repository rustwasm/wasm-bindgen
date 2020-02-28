use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUProgrammableStageDescriptor ) ]
    #[doc = "The `GpuProgrammableStageDescriptor` dictionary.\n\n*This API requires the following crate features to be activated: `GpuProgrammableStageDescriptor`, `GpuShaderModule`*"]
    pub type GpuProgrammableStageDescriptor;
}
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuProgrammableStageDescriptor {
    #[cfg(feature = "GpuShaderModule")]
    #[doc = "Construct a new `GpuProgrammableStageDescriptor`.\n\n*This API requires the following crate features to be activated: `GpuProgrammableStageDescriptor`, `GpuShaderModule`*"]
    pub fn new(entry_point: &str, module: &GpuShaderModule) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.entry_point(entry_point);
        ret.module(module);
        ret
    }
    #[doc = "Change the `entryPoint` field of this object.\n\n*This API requires the following crate features to be activated: `GpuProgrammableStageDescriptor`*"]
    pub fn entry_point(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("entryPoint"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "GpuShaderModule")]
    #[doc = "Change the `module` field of this object.\n\n*This API requires the following crate features to be activated: `GpuProgrammableStageDescriptor`, `GpuShaderModule`*"]
    pub fn module(&mut self, val: &GpuShaderModule) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("module"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
