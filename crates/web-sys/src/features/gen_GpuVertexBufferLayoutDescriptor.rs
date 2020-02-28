use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUVertexBufferLayoutDescriptor ) ]
    #[doc = "The `GpuVertexBufferLayoutDescriptor` dictionary.\n\n*This API requires the following crate features to be activated: `GpuVertexBufferLayoutDescriptor`*"]
    pub type GpuVertexBufferLayoutDescriptor;
}
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuVertexBufferLayoutDescriptor {
    #[doc = "Construct a new `GpuVertexBufferLayoutDescriptor`.\n\n*This API requires the following crate features to be activated: `GpuVertexBufferLayoutDescriptor`*"]
    pub fn new(array_stride: f64, attributes: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.array_stride(array_stride);
        ret.attributes(attributes);
        ret
    }
    #[doc = "Change the `arrayStride` field of this object.\n\n*This API requires the following crate features to be activated: `GpuVertexBufferLayoutDescriptor`*"]
    pub fn array_stride(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("arrayStride"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `attributes` field of this object.\n\n*This API requires the following crate features to be activated: `GpuVertexBufferLayoutDescriptor`*"]
    pub fn attributes(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("attributes"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "GpuInputStepMode")]
    #[doc = "Change the `stepMode` field of this object.\n\n*This API requires the following crate features to be activated: `GpuInputStepMode`, `GpuVertexBufferLayoutDescriptor`*"]
    pub fn step_mode(&mut self, val: GpuInputStepMode) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("stepMode"),
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
