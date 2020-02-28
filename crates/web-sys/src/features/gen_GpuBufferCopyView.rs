use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUBufferCopyView ) ]
    #[doc = "The `GpuBufferCopyView` dictionary.\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuBufferCopyView`*"]
    pub type GpuBufferCopyView;
}
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuBufferCopyView {
    #[cfg(feature = "GpuBuffer")]
    #[doc = "Construct a new `GpuBufferCopyView`.\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuBufferCopyView`*"]
    pub fn new(buffer: &GpuBuffer, image_height: u32, row_pitch: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.buffer(buffer);
        ret.image_height(image_height);
        ret.row_pitch(row_pitch);
        ret
    }
    #[cfg(feature = "GpuBuffer")]
    #[doc = "Change the `buffer` field of this object.\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuBufferCopyView`*"]
    pub fn buffer(&mut self, val: &GpuBuffer) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("buffer"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `imageHeight` field of this object.\n\n*This API requires the following crate features to be activated: `GpuBufferCopyView`*"]
    pub fn image_height(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("imageHeight"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `offset` field of this object.\n\n*This API requires the following crate features to be activated: `GpuBufferCopyView`*"]
    pub fn offset(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("offset"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `rowPitch` field of this object.\n\n*This API requires the following crate features to be activated: `GpuBufferCopyView`*"]
    pub fn row_pitch(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("rowPitch"),
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
