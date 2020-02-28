use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ImageBitmapRenderingContext , typescript_name = ImageBitmapRenderingContext ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageBitmapRenderingContext` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmapRenderingContext)\n\n*This API requires the following crate features to be activated: `ImageBitmapRenderingContext`*"]
    pub type ImageBitmapRenderingContext;
    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( method , structural , js_name = transferFromImageBitmap ) ]
    #[doc = "The `transferFromImageBitmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmapRenderingContext/transferFromImageBitmap)\n\n*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapRenderingContext`*"]
    pub fn transfer_from_image_bitmap(this: &ImageBitmapRenderingContext, bitmap: &ImageBitmap);
    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( method , structural , js_name = transferImageBitmap ) ]
    #[doc = "The `transferImageBitmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmapRenderingContext/transferImageBitmap)\n\n*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapRenderingContext`*"]
    pub fn transfer_image_bitmap(this: &ImageBitmapRenderingContext, bitmap: &ImageBitmap);
}
