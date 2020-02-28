use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = ""]
#[doc = ""]
#[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
#[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUPipelineDescriptorBase ) ]
    #[doc = "The `GpuPipelineDescriptorBase` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineDescriptorBase`, `GpuPipelineLayout`*"]
    pub type GpuPipelineDescriptorBase;
}
#[cfg(web_sys_unstable_apis)]
#[doc = ""]
#[doc = ""]
#[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
#[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuPipelineDescriptorBase {
    #[cfg(feature = "GpuPipelineLayout")]
    #[doc = "Construct a new `GpuPipelineDescriptorBase`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineDescriptorBase`, `GpuPipelineLayout`*"]
    pub fn new(layout: &GpuPipelineLayout) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.layout(layout);
        ret
    }
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineDescriptorBase`*"]
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
    #[doc = "Change the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineDescriptorBase`, `GpuPipelineLayout`*"]
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
}
