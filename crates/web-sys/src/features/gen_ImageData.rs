use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ImageData , typescript_name = ImageData ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageData` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData)\n\n*This API requires the following crate features to be activated: `ImageData`*"]
    pub type ImageData;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ImageData" , js_name = width ) ]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/width)\n\n*This API requires the following crate features to be activated: `ImageData`*"]
    pub fn width(this: &ImageData) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ImageData" , js_name = height ) ]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/height)\n\n*This API requires the following crate features to be activated: `ImageData`*"]
    pub fn height(this: &ImageData) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ImageData" , js_name = data ) ]
    #[doc = "Getter for the `data` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/data)\n\n*This API requires the following crate features to be activated: `ImageData`*"]
    pub fn data(this: &ImageData) -> ::wasm_bindgen::Clamped<Vec<u8>>;
    #[wasm_bindgen(catch, js_class = "ImageData", constructor)]
    #[doc = "The `new ImageData(..)` constructor, creating a new instance of `ImageData`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)\n\n*This API requires the following crate features to be activated: `ImageData`*"]
    pub fn new_with_sw(this: &ImageData, sw: u32, sh: u32) -> Result<ImageData, JsValue>;
    #[wasm_bindgen(catch, js_class = "ImageData", constructor)]
    #[doc = "The `new ImageData(..)` constructor, creating a new instance of `ImageData`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)\n\n*This API requires the following crate features to be activated: `ImageData`*"]
    pub fn new_with_u8_clamped_array(
        this: &ImageData,
        data: ::wasm_bindgen::Clamped<&mut [u8]>,
        sw: u32,
    ) -> Result<ImageData, JsValue>;
    #[wasm_bindgen(catch, js_class = "ImageData", constructor)]
    #[doc = "The `new ImageData(..)` constructor, creating a new instance of `ImageData`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)\n\n*This API requires the following crate features to be activated: `ImageData`*"]
    pub fn new_with_u8_clamped_array_and_sh(
        this: &ImageData,
        data: ::wasm_bindgen::Clamped<&mut [u8]>,
        sw: u32,
        sh: u32,
    ) -> Result<ImageData, JsValue>;
}
