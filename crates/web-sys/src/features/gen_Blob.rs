use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Blob , typescript_type = "Blob" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Blob` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob)
    ///
    ///*This API requires the following crate features to be activated: `Blob`*
    pub type Blob;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Blob" , js_name = size ) ]
    ///Getter for the `size` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/size)
    ///
    ///*This API requires the following crate features to be activated: `Blob`*
    pub fn size(this: &Blob) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Blob" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/type)
    ///
    ///*This API requires the following crate features to be activated: `Blob`*
    pub fn type_(this: &Blob) -> String;

    #[wasm_bindgen(catch, constructor, js_class = "Blob")]
    ///The `new Blob(..)` constructor, creating a new instance of `Blob`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)
    ///
    ///*This API requires the following crate features to be activated: `Blob`*
    pub fn new() -> Result<Blob, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "Blob")]
    ///The `new Blob(..)` constructor, creating a new instance of `Blob`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)
    ///
    ///*This API requires the following crate features to be activated: `Blob`*
    pub fn new_with_buffer_source_sequence(
        blob_parts: &::wasm_bindgen::JsValue,
    ) -> Result<Blob, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "Blob")]
    ///The `new Blob(..)` constructor, creating a new instance of `Blob`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)
    ///
    ///*This API requires the following crate features to be activated: `Blob`*
    pub fn new_with_u8_array_sequence(
        blob_parts: &::wasm_bindgen::JsValue,
    ) -> Result<Blob, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "Blob")]
    ///The `new Blob(..)` constructor, creating a new instance of `Blob`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)
    ///
    ///*This API requires the following crate features to be activated: `Blob`*
    pub fn new_with_blob_sequence(blob_parts: &::wasm_bindgen::JsValue) -> Result<Blob, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "Blob")]
    ///The `new Blob(..)` constructor, creating a new instance of `Blob`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)
    ///
    ///*This API requires the following crate features to be activated: `Blob`*
    pub fn new_with_str_sequence(blob_parts: &::wasm_bindgen::JsValue) -> Result<Blob, JsValue>;

    #[cfg(feature = "BlobPropertyBag")]
    #[wasm_bindgen(catch, constructor, js_class = "Blob")]
    ///The `new Blob(..)` constructor, creating a new instance of `Blob`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `BlobPropertyBag`*
    pub fn new_with_buffer_source_sequence_and_options(
        blob_parts: &::wasm_bindgen::JsValue,
        options: &BlobPropertyBag,
    ) -> Result<Blob, JsValue>;

    #[cfg(feature = "BlobPropertyBag")]
    #[wasm_bindgen(catch, constructor, js_class = "Blob")]
    ///The `new Blob(..)` constructor, creating a new instance of `Blob`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `BlobPropertyBag`*
    pub fn new_with_u8_array_sequence_and_options(
        blob_parts: &::wasm_bindgen::JsValue,
        options: &BlobPropertyBag,
    ) -> Result<Blob, JsValue>;

    #[cfg(feature = "BlobPropertyBag")]
    #[wasm_bindgen(catch, constructor, js_class = "Blob")]
    ///The `new Blob(..)` constructor, creating a new instance of `Blob`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `BlobPropertyBag`*
    pub fn new_with_blob_sequence_and_options(
        blob_parts: &::wasm_bindgen::JsValue,
        options: &BlobPropertyBag,
    ) -> Result<Blob, JsValue>;

    #[cfg(feature = "BlobPropertyBag")]
    #[wasm_bindgen(catch, constructor, js_class = "Blob")]
    ///The `new Blob(..)` constructor, creating a new instance of `Blob`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `BlobPropertyBag`*
    pub fn new_with_str_sequence_and_options(
        blob_parts: &::wasm_bindgen::JsValue,
        options: &BlobPropertyBag,
    ) -> Result<Blob, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Blob" , js_name = arrayBuffer ) ]
    ///The `arrayBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/arrayBuffer)
    ///
    ///*This API requires the following crate features to be activated: `Blob`*
    pub fn array_buffer(this: &Blob) -> ::js_sys::Promise;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Blob" , js_name = slice ) ]
    ///The `slice()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)
    ///
    ///*This API requires the following crate features to be activated: `Blob`*
    pub fn slice(this: &Blob) -> Result<Blob, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Blob" , js_name = slice ) ]
    ///The `slice()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)
    ///
    ///*This API requires the following crate features to be activated: `Blob`*
    pub fn slice_with_i32(this: &Blob, start: i32) -> Result<Blob, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Blob" , js_name = slice ) ]
    ///The `slice()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)
    ///
    ///*This API requires the following crate features to be activated: `Blob`*
    pub fn slice_with_f64(this: &Blob, start: f64) -> Result<Blob, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Blob" , js_name = slice ) ]
    ///The `slice()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)
    ///
    ///*This API requires the following crate features to be activated: `Blob`*
    pub fn slice_with_i32_and_i32(this: &Blob, start: i32, end: i32) -> Result<Blob, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Blob" , js_name = slice ) ]
    ///The `slice()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)
    ///
    ///*This API requires the following crate features to be activated: `Blob`*
    pub fn slice_with_f64_and_i32(this: &Blob, start: f64, end: i32) -> Result<Blob, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Blob" , js_name = slice ) ]
    ///The `slice()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)
    ///
    ///*This API requires the following crate features to be activated: `Blob`*
    pub fn slice_with_i32_and_f64(this: &Blob, start: i32, end: f64) -> Result<Blob, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Blob" , js_name = slice ) ]
    ///The `slice()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)
    ///
    ///*This API requires the following crate features to be activated: `Blob`*
    pub fn slice_with_f64_and_f64(this: &Blob, start: f64, end: f64) -> Result<Blob, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Blob" , js_name = slice ) ]
    ///The `slice()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)
    ///
    ///*This API requires the following crate features to be activated: `Blob`*
    pub fn slice_with_i32_and_i32_and_content_type(
        this: &Blob,
        start: i32,
        end: i32,
        content_type: &str,
    ) -> Result<Blob, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Blob" , js_name = slice ) ]
    ///The `slice()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)
    ///
    ///*This API requires the following crate features to be activated: `Blob`*
    pub fn slice_with_f64_and_i32_and_content_type(
        this: &Blob,
        start: f64,
        end: i32,
        content_type: &str,
    ) -> Result<Blob, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Blob" , js_name = slice ) ]
    ///The `slice()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)
    ///
    ///*This API requires the following crate features to be activated: `Blob`*
    pub fn slice_with_i32_and_f64_and_content_type(
        this: &Blob,
        start: i32,
        end: f64,
        content_type: &str,
    ) -> Result<Blob, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Blob" , js_name = slice ) ]
    ///The `slice()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)
    ///
    ///*This API requires the following crate features to be activated: `Blob`*
    pub fn slice_with_f64_and_f64_and_content_type(
        this: &Blob,
        start: f64,
        end: f64,
        content_type: &str,
    ) -> Result<Blob, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Blob" , js_name = text ) ]
    ///The `text()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/text)
    ///
    ///*This API requires the following crate features to be activated: `Blob`*
    pub fn text(this: &Blob) -> ::js_sys::Promise;

}
