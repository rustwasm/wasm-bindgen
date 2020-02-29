use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ImageData , typescript_type = "ImageData" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ImageData` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData)
    ///
    ///*This API requires the following crate features to be activated: `ImageData`*
    pub type ImageData;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ImageData" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/width)
    ///
    ///*This API requires the following crate features to be activated: `ImageData`*
    pub fn width(this: &ImageData) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ImageData" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/height)
    ///
    ///*This API requires the following crate features to be activated: `ImageData`*
    pub fn height(this: &ImageData) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ImageData" , js_name = data ) ]
    ///Getter for the `data` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/data)
    ///
    ///*This API requires the following crate features to be activated: `ImageData`*
    pub fn data(this: &ImageData) -> ::wasm_bindgen::Clamped<Vec<u8>>;

    #[wasm_bindgen(catch, constructor, js_class = "ImageData")]
    ///The `new ImageData(..)` constructor, creating a new instance of `ImageData`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)
    ///
    ///*This API requires the following crate features to be activated: `ImageData`*
    pub fn new_with_sw(sw: u32, sh: u32) -> Result<ImageData, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "ImageData")]
    ///The `new ImageData(..)` constructor, creating a new instance of `ImageData`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)
    ///
    ///*This API requires the following crate features to be activated: `ImageData`*
    pub fn new_with_u8_clamped_array(
        data: ::wasm_bindgen::Clamped<&mut [u8]>,
        sw: u32,
    ) -> Result<ImageData, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "ImageData")]
    ///The `new ImageData(..)` constructor, creating a new instance of `ImageData`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)
    ///
    ///*This API requires the following crate features to be activated: `ImageData`*
    pub fn new_with_u8_clamped_array_and_sh(
        data: ::wasm_bindgen::Clamped<&mut [u8]>,
        sw: u32,
        sh: u32,
    ) -> Result<ImageData, JsValue>;

}
