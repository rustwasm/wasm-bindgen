use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ImageBitmapRenderingContext , typescript_name = ImageBitmapRenderingContext ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ImageBitmapRenderingContext` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmapRenderingContext)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmapRenderingContext`*
    pub type ImageBitmapRenderingContext;

    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( method , structural , js_class = "ImageBitmapRenderingContext" , js_name = transferFromImageBitmap ) ]
    ///The `transferFromImageBitmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmapRenderingContext/transferFromImageBitmap)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapRenderingContext`*
    pub fn transfer_from_image_bitmap(this: &ImageBitmapRenderingContext, bitmap: &ImageBitmap);

    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( method , structural , js_class = "ImageBitmapRenderingContext" , js_name = transferImageBitmap ) ]
    ///The `transferImageBitmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmapRenderingContext/transferImageBitmap)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapRenderingContext`*
    pub fn transfer_image_bitmap(this: &ImageBitmapRenderingContext, bitmap: &ImageBitmap);

}
