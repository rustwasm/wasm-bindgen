use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPURenderPipelineDescriptor ) ]
    #[doc = "The `GpuRenderPipelineDescriptor` dictionary.\n\n*This API requires the following crate features to be activated: `GpuPipelineLayout`, `GpuPrimitiveTopology`, `GpuProgrammableStageDescriptor`, `GpuRenderPipelineDescriptor`*"]
    pub type GpuRenderPipelineDescriptor;
}
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPipelineDescriptor {
    #[cfg(all(
        feature = "GpuPipelineLayout",
        feature = "GpuPrimitiveTopology",
        feature = "GpuProgrammableStageDescriptor",
    ))]
    #[doc = "Construct a new `GpuRenderPipelineDescriptor`.\n\n*This API requires the following crate features to be activated: `GpuPipelineLayout`, `GpuPrimitiveTopology`, `GpuProgrammableStageDescriptor`, `GpuRenderPipelineDescriptor`*"]
    pub fn new(
        layout: &GpuPipelineLayout,
        color_states: &::wasm_bindgen::JsValue,
        primitive_topology: GpuPrimitiveTopology,
        vertex_stage: &GpuProgrammableStageDescriptor,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.layout(layout);
        ret.color_states(color_states);
        ret.primitive_topology(primitive_topology);
        ret.vertex_stage(vertex_stage);
        ret
    }
    #[doc = "Change the `label` field of this object.\n\n*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
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
    #[doc = "Change the `layout` field of this object.\n\n*This API requires the following crate features to be activated: `GpuPipelineLayout`, `GpuRenderPipelineDescriptor`*"]
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
    #[doc = "Change the `alphaToCoverageEnabled` field of this object.\n\n*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    pub fn alpha_to_coverage_enabled(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("alphaToCoverageEnabled"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `colorStates` field of this object.\n\n*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    pub fn color_states(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("colorStates"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "GpuDepthStencilStateDescriptor")]
    #[doc = "Change the `depthStencilState` field of this object.\n\n*This API requires the following crate features to be activated: `GpuDepthStencilStateDescriptor`, `GpuRenderPipelineDescriptor`*"]
    pub fn depth_stencil_state(&mut self, val: &GpuDepthStencilStateDescriptor) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("depthStencilState"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "GpuProgrammableStageDescriptor")]
    #[doc = "Change the `fragmentStage` field of this object.\n\n*This API requires the following crate features to be activated: `GpuProgrammableStageDescriptor`, `GpuRenderPipelineDescriptor`*"]
    pub fn fragment_stage(&mut self, val: &GpuProgrammableStageDescriptor) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("fragmentStage"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "GpuPrimitiveTopology")]
    #[doc = "Change the `primitiveTopology` field of this object.\n\n*This API requires the following crate features to be activated: `GpuPrimitiveTopology`, `GpuRenderPipelineDescriptor`*"]
    pub fn primitive_topology(&mut self, val: GpuPrimitiveTopology) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("primitiveTopology"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "GpuRasterizationStateDescriptor")]
    #[doc = "Change the `rasterizationState` field of this object.\n\n*This API requires the following crate features to be activated: `GpuRasterizationStateDescriptor`, `GpuRenderPipelineDescriptor`*"]
    pub fn rasterization_state(&mut self, val: &GpuRasterizationStateDescriptor) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("rasterizationState"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `sampleCount` field of this object.\n\n*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    pub fn sample_count(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("sampleCount"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `sampleMask` field of this object.\n\n*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    pub fn sample_mask(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("sampleMask"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "GpuProgrammableStageDescriptor")]
    #[doc = "Change the `vertexStage` field of this object.\n\n*This API requires the following crate features to be activated: `GpuProgrammableStageDescriptor`, `GpuRenderPipelineDescriptor`*"]
    pub fn vertex_stage(&mut self, val: &GpuProgrammableStageDescriptor) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("vertexStage"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "GpuVertexStateDescriptor")]
    #[doc = "Change the `vertexState` field of this object.\n\n*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`, `GpuVertexStateDescriptor`*"]
    pub fn vertex_state(&mut self, val: &GpuVertexStateDescriptor) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("vertexState"),
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
