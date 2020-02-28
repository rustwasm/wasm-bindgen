use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUComputePipelineDescriptor ) ]
    #[doc = "The `GpuComputePipelineDescriptor` dictionary.\n\n*This API requires the following crate features to be activated: `GpuComputePipelineDescriptor`, `GpuPipelineLayout`, `GpuProgrammableStageDescriptor`*"]
    pub type GpuComputePipelineDescriptor;
}
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuComputePipelineDescriptor {
    #[cfg(all(
        feature = "GpuPipelineLayout",
        feature = "GpuProgrammableStageDescriptor",
    ))]
    #[doc = "Construct a new `GpuComputePipelineDescriptor`.\n\n*This API requires the following crate features to be activated: `GpuComputePipelineDescriptor`, `GpuPipelineLayout`, `GpuProgrammableStageDescriptor`*"]
    pub fn new(layout: &GpuPipelineLayout, compute_stage: &GpuProgrammableStageDescriptor) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.layout(layout);
        ret.compute_stage(compute_stage);
        ret
    }
    #[doc = "Change the `label` field of this object.\n\n*This API requires the following crate features to be activated: `GpuComputePipelineDescriptor`*"]
    pub fn label(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("label"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "GpuPipelineLayout")]
    #[doc = "Change the `layout` field of this object.\n\n*This API requires the following crate features to be activated: `GpuComputePipelineDescriptor`, `GpuPipelineLayout`*"]
    pub fn layout(&mut self, val: &GpuPipelineLayout) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("layout"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "GpuProgrammableStageDescriptor")]
    #[doc = "Change the `computeStage` field of this object.\n\n*This API requires the following crate features to be activated: `GpuComputePipelineDescriptor`, `GpuProgrammableStageDescriptor`*"]
    pub fn compute_stage(&mut self, val: &GpuProgrammableStageDescriptor) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("computeStage"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
