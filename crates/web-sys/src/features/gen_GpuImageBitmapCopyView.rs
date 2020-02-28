use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUImageBitmapCopyView ) ]
    #[doc = "The `GpuImageBitmapCopyView` dictionary.\n\n*This API requires the following crate features to be activated: `GpuImageBitmapCopyView`, `ImageBitmap`*"]
    pub type GpuImageBitmapCopyView;
}
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuImageBitmapCopyView {
    #[cfg(feature = "ImageBitmap")]
    #[doc = "Construct a new `GpuImageBitmapCopyView`.\n\n*This API requires the following crate features to be activated: `GpuImageBitmapCopyView`, `ImageBitmap`*"]
    pub fn new(image_bitmap: &ImageBitmap) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.image_bitmap(image_bitmap);
        ret
    }
    #[cfg(feature = "ImageBitmap")]
    #[doc = "Change the `imageBitmap` field of this object.\n\n*This API requires the following crate features to be activated: `GpuImageBitmapCopyView`, `ImageBitmap`*"]
    pub fn image_bitmap(&mut self, val: &ImageBitmap) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("imageBitmap"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `origin` field of this object.\n\n*This API requires the following crate features to be activated: `GpuImageBitmapCopyView`*"]
    pub fn origin(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("origin"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
