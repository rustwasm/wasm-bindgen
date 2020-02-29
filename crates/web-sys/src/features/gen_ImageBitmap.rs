use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ImageBitmap , typescript_name = ImageBitmap ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ImageBitmap` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmap`*
    pub type ImageBitmap;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ImageBitmap" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/width)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmap`*
    pub fn width(this: &ImageBitmap) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ImageBitmap" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/height)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmap`*
    pub fn height(this: &ImageBitmap) -> u32;

    # [ wasm_bindgen ( method , structural , js_class = "ImageBitmap" , js_name = close ) ]
    ///The `close()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/close)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmap`*
    pub fn close(this: &ImageBitmap);

    #[cfg(feature = "ImageBitmapFormat")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "ImageBitmap" , js_name = findOptimalFormat ) ]
    ///The `findOptimalFormat()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/findOptimalFormat)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapFormat`*
    pub fn find_optimal_format(this: &ImageBitmap) -> Result<ImageBitmapFormat, JsValue>;

    #[cfg(feature = "ImageBitmapFormat")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "ImageBitmap" , js_name = findOptimalFormat ) ]
    ///The `findOptimalFormat()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/findOptimalFormat)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapFormat`*
    pub fn find_optimal_format_with_a_possible_formats(
        this: &ImageBitmap,
        a_possible_formats: &::wasm_bindgen::JsValue,
    ) -> Result<ImageBitmapFormat, JsValue>;

    #[cfg(feature = "ImageBitmapFormat")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "ImageBitmap" , js_name = mapDataInto ) ]
    ///The `mapDataInto()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/mapDataInto)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapFormat`*
    pub fn map_data_into_with_buffer_source(
        this: &ImageBitmap,
        a_format: ImageBitmapFormat,
        a_buffer: &::js_sys::Object,
        a_offset: i32,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "ImageBitmapFormat")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "ImageBitmap" , js_name = mapDataInto ) ]
    ///The `mapDataInto()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/mapDataInto)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapFormat`*
    pub fn map_data_into_with_u8_array(
        this: &ImageBitmap,
        a_format: ImageBitmapFormat,
        a_buffer: &mut [u8],
        a_offset: i32,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "ImageBitmapFormat")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "ImageBitmap" , js_name = mappedDataLength ) ]
    ///The `mappedDataLength()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/mappedDataLength)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapFormat`*
    pub fn mapped_data_length(
        this: &ImageBitmap,
        a_format: ImageBitmapFormat,
    ) -> Result<i32, JsValue>;

}
