use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = ""]
#[doc = ""]
#[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
#[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPURenderBundleEncoderDescriptor ) ]
    #[doc = "The `GpuRenderBundleEncoderDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderBundleEncoderDescriptor`*"]
    pub type GpuRenderBundleEncoderDescriptor;
}
#[cfg(web_sys_unstable_apis)]
#[doc = ""]
#[doc = ""]
#[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
#[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderBundleEncoderDescriptor {
    #[doc = "Construct a new `GpuRenderBundleEncoderDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderBundleEncoderDescriptor`*"]
    pub fn new(color_formats: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.color_formats(color_formats);
        ret
    }
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderBundleEncoderDescriptor`*"]
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
    #[doc = "Change the `colorFormats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderBundleEncoderDescriptor`*"]
    pub fn color_formats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("colorFormats"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Change the `depthStencilFormat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderBundleEncoderDescriptor`, `GpuTextureFormat`*"]
    pub fn depth_stencil_format(&mut self, val: GpuTextureFormat) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("depthStencilFormat"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `sampleCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderBundleEncoderDescriptor`*"]
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
}
