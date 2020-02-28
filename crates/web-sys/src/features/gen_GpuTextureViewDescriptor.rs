use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUTextureViewDescriptor ) ]
    #[doc = "The `GpuTextureViewDescriptor` dictionary.\n\n*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`*"]
    pub type GpuTextureViewDescriptor;
}
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuTextureViewDescriptor {
    #[doc = "Construct a new `GpuTextureViewDescriptor`.\n\n*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `label` field of this object.\n\n*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`*"]
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
    #[doc = "Change the `arrayLayerCount` field of this object.\n\n*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`*"]
    pub fn array_layer_count(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("arrayLayerCount"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "GpuTextureAspect")]
    #[doc = "Change the `aspect` field of this object.\n\n*This API requires the following crate features to be activated: `GpuTextureAspect`, `GpuTextureViewDescriptor`*"]
    pub fn aspect(&mut self, val: GpuTextureAspect) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("aspect"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `baseArrayLayer` field of this object.\n\n*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`*"]
    pub fn base_array_layer(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("baseArrayLayer"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `baseMipLevel` field of this object.\n\n*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`*"]
    pub fn base_mip_level(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("baseMipLevel"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "GpuTextureViewDimension")]
    #[doc = "Change the `dimension` field of this object.\n\n*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`, `GpuTextureViewDimension`*"]
    pub fn dimension(&mut self, val: GpuTextureViewDimension) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("dimension"),
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
    #[doc = "Change the `format` field of this object.\n\n*This API requires the following crate features to be activated: `GpuTextureFormat`, `GpuTextureViewDescriptor`*"]
    pub fn format(&mut self, val: GpuTextureFormat) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("format"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `mipLevelCount` field of this object.\n\n*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`*"]
    pub fn mip_level_count(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("mipLevelCount"),
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
