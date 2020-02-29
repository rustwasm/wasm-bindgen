use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WebGLRenderingContext , typescript_type = "WebGLRenderingContext" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebGlRenderingContext` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub type WebGlRenderingContext;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebGLRenderingContext" , js_name = canvas ) ]
    ///Getter for the `canvas` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/canvas)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn canvas(this: &WebGlRenderingContext) -> Option<::js_sys::Object>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebGLRenderingContext" , js_name = drawingBufferWidth ) ]
    ///Getter for the `drawingBufferWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawingBufferWidth)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn drawing_buffer_width(this: &WebGlRenderingContext) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebGLRenderingContext" , js_name = drawingBufferHeight ) ]
    ///Getter for the `drawingBufferHeight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawingBufferHeight)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn drawing_buffer_height(this: &WebGlRenderingContext) -> i32;

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = bufferData ) ]
    ///The `bufferData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn buffer_data_with_i32(this: &WebGlRenderingContext, target: u32, size: i32, usage: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = bufferData ) ]
    ///The `bufferData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn buffer_data_with_f64(this: &WebGlRenderingContext, target: u32, size: f64, usage: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = bufferData ) ]
    ///The `bufferData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn buffer_data_with_opt_array_buffer(
        this: &WebGlRenderingContext,
        target: u32,
        data: Option<&::js_sys::ArrayBuffer>,
        usage: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = bufferData ) ]
    ///The `bufferData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn buffer_data_with_array_buffer_view(
        this: &WebGlRenderingContext,
        target: u32,
        data: &::js_sys::Object,
        usage: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = bufferData ) ]
    ///The `bufferData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn buffer_data_with_u8_array(
        this: &WebGlRenderingContext,
        target: u32,
        data: &[u8],
        usage: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = bufferSubData ) ]
    ///The `bufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn buffer_sub_data_with_i32_and_array_buffer(
        this: &WebGlRenderingContext,
        target: u32,
        offset: i32,
        data: &::js_sys::ArrayBuffer,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = bufferSubData ) ]
    ///The `bufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn buffer_sub_data_with_f64_and_array_buffer(
        this: &WebGlRenderingContext,
        target: u32,
        offset: f64,
        data: &::js_sys::ArrayBuffer,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = bufferSubData ) ]
    ///The `bufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn buffer_sub_data_with_i32_and_array_buffer_view(
        this: &WebGlRenderingContext,
        target: u32,
        offset: i32,
        data: &::js_sys::Object,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = bufferSubData ) ]
    ///The `bufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn buffer_sub_data_with_f64_and_array_buffer_view(
        this: &WebGlRenderingContext,
        target: u32,
        offset: f64,
        data: &::js_sys::Object,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = bufferSubData ) ]
    ///The `bufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn buffer_sub_data_with_i32_and_u8_array(
        this: &WebGlRenderingContext,
        target: u32,
        offset: i32,
        data: &[u8],
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = bufferSubData ) ]
    ///The `bufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn buffer_sub_data_with_f64_and_u8_array(
        this: &WebGlRenderingContext,
        target: u32,
        offset: f64,
        data: &[u8],
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = commit ) ]
    ///The `commit()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/commit)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn commit(this: &WebGlRenderingContext);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = compressedTexImage2D ) ]
    ///The `compressedTexImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compressedTexImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn compressed_tex_image_2d_with_array_buffer_view(
        this: &WebGlRenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        data: &::js_sys::Object,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = compressedTexImage2D ) ]
    ///The `compressedTexImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compressedTexImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn compressed_tex_image_2d_with_u8_array(
        this: &WebGlRenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        data: &[u8],
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = compressedTexSubImage2D ) ]
    ///The `compressedTexSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compressedTexSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn compressed_tex_sub_image_2d_with_array_buffer_view(
        this: &WebGlRenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        data: &::js_sys::Object,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = compressedTexSubImage2D ) ]
    ///The `compressedTexSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compressedTexSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn compressed_tex_sub_image_2d_with_u8_array(
        this: &WebGlRenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        data: &mut [u8],
    );

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGLRenderingContext" , js_name = readPixels ) ]
    ///The `readPixels()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/readPixels)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn read_pixels_with_opt_array_buffer_view(
        this: &WebGlRenderingContext,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        pixels: Option<&::js_sys::Object>,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGLRenderingContext" , js_name = readPixels ) ]
    ///The `readPixels()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/readPixels)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn read_pixels_with_opt_u8_array(
        this: &WebGlRenderingContext,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        pixels: Option<&mut [u8]>,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGLRenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
        this: &WebGlRenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        pixels: Option<&::js_sys::Object>,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGLRenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        this: &WebGlRenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        pixels: Option<&[u8]>,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGLRenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmap`, `WebGlRenderingContext`*
    pub fn tex_image_2d_with_u32_and_u32_and_image_bitmap(
        this: &WebGlRenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        pixels: &ImageBitmap,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGLRenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `ImageData`, `WebGlRenderingContext`*
    pub fn tex_image_2d_with_u32_and_u32_and_image_data(
        this: &WebGlRenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        pixels: &ImageData,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGLRenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`, `WebGlRenderingContext`*
    pub fn tex_image_2d_with_u32_and_u32_and_image(
        this: &WebGlRenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        image: &HtmlImageElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlCanvasElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGLRenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WebGlRenderingContext`*
    pub fn tex_image_2d_with_u32_and_u32_and_canvas(
        this: &WebGlRenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        canvas: &HtmlCanvasElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlVideoElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGLRenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlVideoElement`, `WebGlRenderingContext`*
    pub fn tex_image_2d_with_u32_and_u32_and_video(
        this: &WebGlRenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        video: &HtmlVideoElement,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGLRenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_array_buffer_view(
        this: &WebGlRenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        pixels: Option<&::js_sys::Object>,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGLRenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(
        this: &WebGlRenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        pixels: Option<&[u8]>,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGLRenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmap`, `WebGlRenderingContext`*
    pub fn tex_sub_image_2d_with_u32_and_u32_and_image_bitmap(
        this: &WebGlRenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        format: u32,
        type_: u32,
        pixels: &ImageBitmap,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGLRenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `ImageData`, `WebGlRenderingContext`*
    pub fn tex_sub_image_2d_with_u32_and_u32_and_image_data(
        this: &WebGlRenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        format: u32,
        type_: u32,
        pixels: &ImageData,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGLRenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`, `WebGlRenderingContext`*
    pub fn tex_sub_image_2d_with_u32_and_u32_and_image(
        this: &WebGlRenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        format: u32,
        type_: u32,
        image: &HtmlImageElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlCanvasElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGLRenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WebGlRenderingContext`*
    pub fn tex_sub_image_2d_with_u32_and_u32_and_canvas(
        this: &WebGlRenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        format: u32,
        type_: u32,
        canvas: &HtmlCanvasElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlVideoElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGLRenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlVideoElement`, `WebGlRenderingContext`*
    pub fn tex_sub_image_2d_with_u32_and_u32_and_video(
        this: &WebGlRenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        format: u32,
        type_: u32,
        video: &HtmlVideoElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform1fv ) ]
    ///The `uniform1fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1fv_with_f32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform1fv ) ]
    ///The `uniform1fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform1iv ) ]
    ///The `uniform1iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1iv_with_i32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform1iv ) ]
    ///The `uniform1iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1iv_with_i32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform2fv ) ]
    ///The `uniform2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2fv_with_f32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform2fv ) ]
    ///The `uniform2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform2iv ) ]
    ///The `uniform2iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2iv_with_i32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform2iv ) ]
    ///The `uniform2iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2iv_with_i32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform3fv ) ]
    ///The `uniform3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3fv_with_f32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform3fv ) ]
    ///The `uniform3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform3iv ) ]
    ///The `uniform3iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3iv_with_i32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform3iv ) ]
    ///The `uniform3iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3iv_with_i32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform4fv ) ]
    ///The `uniform4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4fv_with_f32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform4fv ) ]
    ///The `uniform4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform4iv ) ]
    ///The `uniform4iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4iv_with_i32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform4iv ) ]
    ///The `uniform4iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4iv_with_i32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniformMatrix2fv ) ]
    ///The `uniformMatrix2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix2fv_with_f32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniformMatrix2fv ) ]
    ///The `uniformMatrix2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix2fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniformMatrix3fv ) ]
    ///The `uniformMatrix3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix3fv_with_f32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniformMatrix3fv ) ]
    ///The `uniformMatrix3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix3fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniformMatrix4fv ) ]
    ///The `uniformMatrix4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix4fv_with_f32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniformMatrix4fv ) ]
    ///The `uniformMatrix4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix4fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = activeTexture ) ]
    ///The `activeTexture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/activeTexture)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn active_texture(this: &WebGlRenderingContext, texture: u32);

    #[cfg(all(feature = "WebGlProgram", feature = "WebGlShader",))]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = attachShader ) ]
    ///The `attachShader()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/attachShader)
    ///
    ///*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`, `WebGlShader`*
    pub fn attach_shader(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
        shader: &WebGlShader,
    );

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = bindAttribLocation ) ]
    ///The `bindAttribLocation()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bindAttribLocation)
    ///
    ///*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*
    pub fn bind_attrib_location(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
        index: u32,
        name: &str,
    );

    #[cfg(feature = "WebGlBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = bindBuffer ) ]
    ///The `bindBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bindBuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGlBuffer`, `WebGlRenderingContext`*
    pub fn bind_buffer(this: &WebGlRenderingContext, target: u32, buffer: Option<&WebGlBuffer>);

    #[cfg(feature = "WebGlFramebuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = bindFramebuffer ) ]
    ///The `bindFramebuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bindFramebuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGlFramebuffer`, `WebGlRenderingContext`*
    pub fn bind_framebuffer(
        this: &WebGlRenderingContext,
        target: u32,
        framebuffer: Option<&WebGlFramebuffer>,
    );

    #[cfg(feature = "WebGlRenderbuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = bindRenderbuffer ) ]
    ///The `bindRenderbuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bindRenderbuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderbuffer`, `WebGlRenderingContext`*
    pub fn bind_renderbuffer(
        this: &WebGlRenderingContext,
        target: u32,
        renderbuffer: Option<&WebGlRenderbuffer>,
    );

    #[cfg(feature = "WebGlTexture")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = bindTexture ) ]
    ///The `bindTexture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bindTexture)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlTexture`*
    pub fn bind_texture(this: &WebGlRenderingContext, target: u32, texture: Option<&WebGlTexture>);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = blendColor ) ]
    ///The `blendColor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendColor)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn blend_color(this: &WebGlRenderingContext, red: f32, green: f32, blue: f32, alpha: f32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = blendEquation ) ]
    ///The `blendEquation()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendEquation)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn blend_equation(this: &WebGlRenderingContext, mode: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = blendEquationSeparate ) ]
    ///The `blendEquationSeparate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendEquationSeparate)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn blend_equation_separate(this: &WebGlRenderingContext, mode_rgb: u32, mode_alpha: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = blendFunc ) ]
    ///The `blendFunc()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendFunc)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn blend_func(this: &WebGlRenderingContext, sfactor: u32, dfactor: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = blendFuncSeparate ) ]
    ///The `blendFuncSeparate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendFuncSeparate)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn blend_func_separate(
        this: &WebGlRenderingContext,
        src_rgb: u32,
        dst_rgb: u32,
        src_alpha: u32,
        dst_alpha: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = checkFramebufferStatus ) ]
    ///The `checkFramebufferStatus()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/checkFramebufferStatus)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn check_framebuffer_status(this: &WebGlRenderingContext, target: u32) -> u32;

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = clear ) ]
    ///The `clear()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/clear)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn clear(this: &WebGlRenderingContext, mask: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = clearColor ) ]
    ///The `clearColor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/clearColor)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn clear_color(this: &WebGlRenderingContext, red: f32, green: f32, blue: f32, alpha: f32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = clearDepth ) ]
    ///The `clearDepth()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/clearDepth)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn clear_depth(this: &WebGlRenderingContext, depth: f32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = clearStencil ) ]
    ///The `clearStencil()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/clearStencil)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn clear_stencil(this: &WebGlRenderingContext, s: i32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = colorMask ) ]
    ///The `colorMask()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/colorMask)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn color_mask(
        this: &WebGlRenderingContext,
        red: bool,
        green: bool,
        blue: bool,
        alpha: bool,
    );

    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = compileShader ) ]
    ///The `compileShader()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compileShader)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*
    pub fn compile_shader(this: &WebGlRenderingContext, shader: &WebGlShader);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = copyTexImage2D ) ]
    ///The `copyTexImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/copyTexImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn copy_tex_image_2d(
        this: &WebGlRenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        border: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = copyTexSubImage2D ) ]
    ///The `copyTexSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/copyTexSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn copy_tex_sub_image_2d(
        this: &WebGlRenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    );

    #[cfg(feature = "WebGlBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = createBuffer ) ]
    ///The `createBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createBuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGlBuffer`, `WebGlRenderingContext`*
    pub fn create_buffer(this: &WebGlRenderingContext) -> Option<WebGlBuffer>;

    #[cfg(feature = "WebGlFramebuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = createFramebuffer ) ]
    ///The `createFramebuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createFramebuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGlFramebuffer`, `WebGlRenderingContext`*
    pub fn create_framebuffer(this: &WebGlRenderingContext) -> Option<WebGlFramebuffer>;

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = createProgram ) ]
    ///The `createProgram()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createProgram)
    ///
    ///*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*
    pub fn create_program(this: &WebGlRenderingContext) -> Option<WebGlProgram>;

    #[cfg(feature = "WebGlRenderbuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = createRenderbuffer ) ]
    ///The `createRenderbuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createRenderbuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderbuffer`, `WebGlRenderingContext`*
    pub fn create_renderbuffer(this: &WebGlRenderingContext) -> Option<WebGlRenderbuffer>;

    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = createShader ) ]
    ///The `createShader()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createShader)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*
    pub fn create_shader(this: &WebGlRenderingContext, type_: u32) -> Option<WebGlShader>;

    #[cfg(feature = "WebGlTexture")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = createTexture ) ]
    ///The `createTexture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createTexture)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlTexture`*
    pub fn create_texture(this: &WebGlRenderingContext) -> Option<WebGlTexture>;

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = cullFace ) ]
    ///The `cullFace()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/cullFace)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn cull_face(this: &WebGlRenderingContext, mode: u32);

    #[cfg(feature = "WebGlBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = deleteBuffer ) ]
    ///The `deleteBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteBuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGlBuffer`, `WebGlRenderingContext`*
    pub fn delete_buffer(this: &WebGlRenderingContext, buffer: Option<&WebGlBuffer>);

    #[cfg(feature = "WebGlFramebuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = deleteFramebuffer ) ]
    ///The `deleteFramebuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteFramebuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGlFramebuffer`, `WebGlRenderingContext`*
    pub fn delete_framebuffer(this: &WebGlRenderingContext, framebuffer: Option<&WebGlFramebuffer>);

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = deleteProgram ) ]
    ///The `deleteProgram()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteProgram)
    ///
    ///*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*
    pub fn delete_program(this: &WebGlRenderingContext, program: Option<&WebGlProgram>);

    #[cfg(feature = "WebGlRenderbuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = deleteRenderbuffer ) ]
    ///The `deleteRenderbuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteRenderbuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderbuffer`, `WebGlRenderingContext`*
    pub fn delete_renderbuffer(
        this: &WebGlRenderingContext,
        renderbuffer: Option<&WebGlRenderbuffer>,
    );

    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = deleteShader ) ]
    ///The `deleteShader()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteShader)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*
    pub fn delete_shader(this: &WebGlRenderingContext, shader: Option<&WebGlShader>);

    #[cfg(feature = "WebGlTexture")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = deleteTexture ) ]
    ///The `deleteTexture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteTexture)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlTexture`*
    pub fn delete_texture(this: &WebGlRenderingContext, texture: Option<&WebGlTexture>);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = depthFunc ) ]
    ///The `depthFunc()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/depthFunc)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn depth_func(this: &WebGlRenderingContext, func: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = depthMask ) ]
    ///The `depthMask()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/depthMask)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn depth_mask(this: &WebGlRenderingContext, flag: bool);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = depthRange ) ]
    ///The `depthRange()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/depthRange)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn depth_range(this: &WebGlRenderingContext, z_near: f32, z_far: f32);

    #[cfg(all(feature = "WebGlProgram", feature = "WebGlShader",))]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = detachShader ) ]
    ///The `detachShader()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/detachShader)
    ///
    ///*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`, `WebGlShader`*
    pub fn detach_shader(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
        shader: &WebGlShader,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = disable ) ]
    ///The `disable()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/disable)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn disable(this: &WebGlRenderingContext, cap: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = disableVertexAttribArray ) ]
    ///The `disableVertexAttribArray()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/disableVertexAttribArray)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn disable_vertex_attrib_array(this: &WebGlRenderingContext, index: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = drawArrays ) ]
    ///The `drawArrays()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawArrays)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn draw_arrays(this: &WebGlRenderingContext, mode: u32, first: i32, count: i32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = drawElements ) ]
    ///The `drawElements()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawElements)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn draw_elements_with_i32(
        this: &WebGlRenderingContext,
        mode: u32,
        count: i32,
        type_: u32,
        offset: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = drawElements ) ]
    ///The `drawElements()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawElements)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn draw_elements_with_f64(
        this: &WebGlRenderingContext,
        mode: u32,
        count: i32,
        type_: u32,
        offset: f64,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = enable ) ]
    ///The `enable()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/enable)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn enable(this: &WebGlRenderingContext, cap: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = enableVertexAttribArray ) ]
    ///The `enableVertexAttribArray()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/enableVertexAttribArray)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn enable_vertex_attrib_array(this: &WebGlRenderingContext, index: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = finish ) ]
    ///The `finish()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/finish)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn finish(this: &WebGlRenderingContext);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = flush ) ]
    ///The `flush()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/flush)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn flush(this: &WebGlRenderingContext);

    #[cfg(feature = "WebGlRenderbuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = framebufferRenderbuffer ) ]
    ///The `framebufferRenderbuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/framebufferRenderbuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderbuffer`, `WebGlRenderingContext`*
    pub fn framebuffer_renderbuffer(
        this: &WebGlRenderingContext,
        target: u32,
        attachment: u32,
        renderbuffertarget: u32,
        renderbuffer: Option<&WebGlRenderbuffer>,
    );

    #[cfg(feature = "WebGlTexture")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = framebufferTexture2D ) ]
    ///The `framebufferTexture2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/framebufferTexture2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlTexture`*
    pub fn framebuffer_texture_2d(
        this: &WebGlRenderingContext,
        target: u32,
        attachment: u32,
        textarget: u32,
        texture: Option<&WebGlTexture>,
        level: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = frontFace ) ]
    ///The `frontFace()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/frontFace)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn front_face(this: &WebGlRenderingContext, mode: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = generateMipmap ) ]
    ///The `generateMipmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/generateMipmap)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn generate_mipmap(this: &WebGlRenderingContext, target: u32);

    #[cfg(all(feature = "WebGlActiveInfo", feature = "WebGlProgram",))]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = getActiveAttrib ) ]
    ///The `getActiveAttrib()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getActiveAttrib)
    ///
    ///*This API requires the following crate features to be activated: `WebGlActiveInfo`, `WebGlProgram`, `WebGlRenderingContext`*
    pub fn get_active_attrib(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
        index: u32,
    ) -> Option<WebGlActiveInfo>;

    #[cfg(all(feature = "WebGlActiveInfo", feature = "WebGlProgram",))]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = getActiveUniform ) ]
    ///The `getActiveUniform()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getActiveUniform)
    ///
    ///*This API requires the following crate features to be activated: `WebGlActiveInfo`, `WebGlProgram`, `WebGlRenderingContext`*
    pub fn get_active_uniform(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
        index: u32,
    ) -> Option<WebGlActiveInfo>;

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = getAttachedShaders ) ]
    ///The `getAttachedShaders()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getAttachedShaders)
    ///
    ///*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*
    pub fn get_attached_shaders(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
    ) -> Option<::js_sys::Array>;

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = getAttribLocation ) ]
    ///The `getAttribLocation()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getAttribLocation)
    ///
    ///*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*
    pub fn get_attrib_location(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
        name: &str,
    ) -> i32;

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = getBufferParameter ) ]
    ///The `getBufferParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getBufferParameter)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn get_buffer_parameter(
        this: &WebGlRenderingContext,
        target: u32,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;

    #[cfg(feature = "WebGlContextAttributes")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = getContextAttributes ) ]
    ///The `getContextAttributes()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getContextAttributes)
    ///
    ///*This API requires the following crate features to be activated: `WebGlContextAttributes`, `WebGlRenderingContext`*
    pub fn get_context_attributes(this: &WebGlRenderingContext) -> Option<WebGlContextAttributes>;

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = getError ) ]
    ///The `getError()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getError)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn get_error(this: &WebGlRenderingContext) -> u32;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGLRenderingContext" , js_name = getExtension ) ]
    ///The `getExtension()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getExtension)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn get_extension(
        this: &WebGlRenderingContext,
        name: &str,
    ) -> Result<Option<::js_sys::Object>, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGLRenderingContext" , js_name = getFramebufferAttachmentParameter ) ]
    ///The `getFramebufferAttachmentParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getFramebufferAttachmentParameter)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn get_framebuffer_attachment_parameter(
        this: &WebGlRenderingContext,
        target: u32,
        attachment: u32,
        pname: u32,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGLRenderingContext" , js_name = getParameter ) ]
    ///The `getParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getParameter)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn get_parameter(
        this: &WebGlRenderingContext,
        pname: u32,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = getProgramInfoLog ) ]
    ///The `getProgramInfoLog()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getProgramInfoLog)
    ///
    ///*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*
    pub fn get_program_info_log(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
    ) -> Option<String>;

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = getProgramParameter ) ]
    ///The `getProgramParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getProgramParameter)
    ///
    ///*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*
    pub fn get_program_parameter(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = getRenderbufferParameter ) ]
    ///The `getRenderbufferParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getRenderbufferParameter)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn get_renderbuffer_parameter(
        this: &WebGlRenderingContext,
        target: u32,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;

    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = getShaderInfoLog ) ]
    ///The `getShaderInfoLog()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getShaderInfoLog)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*
    pub fn get_shader_info_log(
        this: &WebGlRenderingContext,
        shader: &WebGlShader,
    ) -> Option<String>;

    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = getShaderParameter ) ]
    ///The `getShaderParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getShaderParameter)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*
    pub fn get_shader_parameter(
        this: &WebGlRenderingContext,
        shader: &WebGlShader,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;

    #[cfg(feature = "WebGlShaderPrecisionFormat")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = getShaderPrecisionFormat ) ]
    ///The `getShaderPrecisionFormat()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getShaderPrecisionFormat)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShaderPrecisionFormat`*
    pub fn get_shader_precision_format(
        this: &WebGlRenderingContext,
        shadertype: u32,
        precisiontype: u32,
    ) -> Option<WebGlShaderPrecisionFormat>;

    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = getShaderSource ) ]
    ///The `getShaderSource()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getShaderSource)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*
    pub fn get_shader_source(this: &WebGlRenderingContext, shader: &WebGlShader) -> Option<String>;

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = getSupportedExtensions ) ]
    ///The `getSupportedExtensions()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getSupportedExtensions)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn get_supported_extensions(this: &WebGlRenderingContext) -> Option<::js_sys::Array>;

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = getTexParameter ) ]
    ///The `getTexParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getTexParameter)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn get_tex_parameter(
        this: &WebGlRenderingContext,
        target: u32,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;

    #[cfg(all(feature = "WebGlProgram", feature = "WebGlUniformLocation",))]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = getUniform ) ]
    ///The `getUniform()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getUniform)
    ///
    ///*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn get_uniform(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
        location: &WebGlUniformLocation,
    ) -> ::wasm_bindgen::JsValue;

    #[cfg(all(feature = "WebGlProgram", feature = "WebGlUniformLocation",))]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = getUniformLocation ) ]
    ///The `getUniformLocation()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getUniformLocation)
    ///
    ///*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn get_uniform_location(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
        name: &str,
    ) -> Option<WebGlUniformLocation>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGLRenderingContext" , js_name = getVertexAttrib ) ]
    ///The `getVertexAttrib()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getVertexAttrib)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn get_vertex_attrib(
        this: &WebGlRenderingContext,
        index: u32,
        pname: u32,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = getVertexAttribOffset ) ]
    ///The `getVertexAttribOffset()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getVertexAttribOffset)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn get_vertex_attrib_offset(this: &WebGlRenderingContext, index: u32, pname: u32) -> f64;

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = hint ) ]
    ///The `hint()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/hint)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn hint(this: &WebGlRenderingContext, target: u32, mode: u32);

    #[cfg(feature = "WebGlBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = isBuffer ) ]
    ///The `isBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isBuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGlBuffer`, `WebGlRenderingContext`*
    pub fn is_buffer(this: &WebGlRenderingContext, buffer: Option<&WebGlBuffer>) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = isContextLost ) ]
    ///The `isContextLost()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isContextLost)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn is_context_lost(this: &WebGlRenderingContext) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = isEnabled ) ]
    ///The `isEnabled()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isEnabled)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn is_enabled(this: &WebGlRenderingContext, cap: u32) -> bool;

    #[cfg(feature = "WebGlFramebuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = isFramebuffer ) ]
    ///The `isFramebuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isFramebuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGlFramebuffer`, `WebGlRenderingContext`*
    pub fn is_framebuffer(
        this: &WebGlRenderingContext,
        framebuffer: Option<&WebGlFramebuffer>,
    ) -> bool;

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = isProgram ) ]
    ///The `isProgram()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isProgram)
    ///
    ///*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*
    pub fn is_program(this: &WebGlRenderingContext, program: Option<&WebGlProgram>) -> bool;

    #[cfg(feature = "WebGlRenderbuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = isRenderbuffer ) ]
    ///The `isRenderbuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isRenderbuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderbuffer`, `WebGlRenderingContext`*
    pub fn is_renderbuffer(
        this: &WebGlRenderingContext,
        renderbuffer: Option<&WebGlRenderbuffer>,
    ) -> bool;

    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = isShader ) ]
    ///The `isShader()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isShader)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*
    pub fn is_shader(this: &WebGlRenderingContext, shader: Option<&WebGlShader>) -> bool;

    #[cfg(feature = "WebGlTexture")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = isTexture ) ]
    ///The `isTexture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isTexture)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlTexture`*
    pub fn is_texture(this: &WebGlRenderingContext, texture: Option<&WebGlTexture>) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = lineWidth ) ]
    ///The `lineWidth()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/lineWidth)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn line_width(this: &WebGlRenderingContext, width: f32);

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = linkProgram ) ]
    ///The `linkProgram()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/linkProgram)
    ///
    ///*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*
    pub fn link_program(this: &WebGlRenderingContext, program: &WebGlProgram);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = pixelStorei ) ]
    ///The `pixelStorei()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/pixelStorei)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn pixel_storei(this: &WebGlRenderingContext, pname: u32, param: i32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = polygonOffset ) ]
    ///The `polygonOffset()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/polygonOffset)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn polygon_offset(this: &WebGlRenderingContext, factor: f32, units: f32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = renderbufferStorage ) ]
    ///The `renderbufferStorage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/renderbufferStorage)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn renderbuffer_storage(
        this: &WebGlRenderingContext,
        target: u32,
        internalformat: u32,
        width: i32,
        height: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = sampleCoverage ) ]
    ///The `sampleCoverage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/sampleCoverage)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn sample_coverage(this: &WebGlRenderingContext, value: f32, invert: bool);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = scissor ) ]
    ///The `scissor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/scissor)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn scissor(this: &WebGlRenderingContext, x: i32, y: i32, width: i32, height: i32);

    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = shaderSource ) ]
    ///The `shaderSource()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/shaderSource)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*
    pub fn shader_source(this: &WebGlRenderingContext, shader: &WebGlShader, source: &str);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = stencilFunc ) ]
    ///The `stencilFunc()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilFunc)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn stencil_func(this: &WebGlRenderingContext, func: u32, ref_: i32, mask: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = stencilFuncSeparate ) ]
    ///The `stencilFuncSeparate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilFuncSeparate)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn stencil_func_separate(
        this: &WebGlRenderingContext,
        face: u32,
        func: u32,
        ref_: i32,
        mask: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = stencilMask ) ]
    ///The `stencilMask()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilMask)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn stencil_mask(this: &WebGlRenderingContext, mask: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = stencilMaskSeparate ) ]
    ///The `stencilMaskSeparate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilMaskSeparate)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn stencil_mask_separate(this: &WebGlRenderingContext, face: u32, mask: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = stencilOp ) ]
    ///The `stencilOp()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilOp)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn stencil_op(this: &WebGlRenderingContext, fail: u32, zfail: u32, zpass: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = stencilOpSeparate ) ]
    ///The `stencilOpSeparate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilOpSeparate)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn stencil_op_separate(
        this: &WebGlRenderingContext,
        face: u32,
        fail: u32,
        zfail: u32,
        zpass: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = texParameterf ) ]
    ///The `texParameterf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texParameterf)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn tex_parameterf(this: &WebGlRenderingContext, target: u32, pname: u32, param: f32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = texParameteri ) ]
    ///The `texParameteri()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texParameteri)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn tex_parameteri(this: &WebGlRenderingContext, target: u32, pname: u32, param: i32);

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform1f ) ]
    ///The `uniform1f()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1f)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1f(this: &WebGlRenderingContext, location: Option<&WebGlUniformLocation>, x: f32);

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform1i ) ]
    ///The `uniform1i()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1i)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1i(this: &WebGlRenderingContext, location: Option<&WebGlUniformLocation>, x: i32);

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform2f ) ]
    ///The `uniform2f()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2f)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2f(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: f32,
        y: f32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform2i ) ]
    ///The `uniform2i()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2i)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2i(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: i32,
        y: i32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform3f ) ]
    ///The `uniform3f()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3f)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3f(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: f32,
        y: f32,
        z: f32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform3i ) ]
    ///The `uniform3i()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3i)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3i(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: i32,
        y: i32,
        z: i32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform4f ) ]
    ///The `uniform4f()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4f)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4f(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = uniform4i ) ]
    ///The `uniform4i()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4i)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4i(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: i32,
        y: i32,
        z: i32,
        w: i32,
    );

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = useProgram ) ]
    ///The `useProgram()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/useProgram)
    ///
    ///*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*
    pub fn use_program(this: &WebGlRenderingContext, program: Option<&WebGlProgram>);

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = validateProgram ) ]
    ///The `validateProgram()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/validateProgram)
    ///
    ///*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*
    pub fn validate_program(this: &WebGlRenderingContext, program: &WebGlProgram);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = vertexAttrib1f ) ]
    ///The `vertexAttrib1f()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib1f)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn vertex_attrib1f(this: &WebGlRenderingContext, indx: u32, x: f32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = vertexAttrib1fv ) ]
    ///The `vertexAttrib1fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib1fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn vertex_attrib1fv_with_f32_array(this: &WebGlRenderingContext, indx: u32, values: &[f32]);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = vertexAttrib1fv ) ]
    ///The `vertexAttrib1fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib1fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn vertex_attrib1fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        indx: u32,
        values: &::wasm_bindgen::JsValue,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = vertexAttrib2f ) ]
    ///The `vertexAttrib2f()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib2f)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn vertex_attrib2f(this: &WebGlRenderingContext, indx: u32, x: f32, y: f32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = vertexAttrib2fv ) ]
    ///The `vertexAttrib2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn vertex_attrib2fv_with_f32_array(this: &WebGlRenderingContext, indx: u32, values: &[f32]);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = vertexAttrib2fv ) ]
    ///The `vertexAttrib2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn vertex_attrib2fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        indx: u32,
        values: &::wasm_bindgen::JsValue,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = vertexAttrib3f ) ]
    ///The `vertexAttrib3f()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib3f)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn vertex_attrib3f(this: &WebGlRenderingContext, indx: u32, x: f32, y: f32, z: f32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = vertexAttrib3fv ) ]
    ///The `vertexAttrib3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn vertex_attrib3fv_with_f32_array(this: &WebGlRenderingContext, indx: u32, values: &[f32]);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = vertexAttrib3fv ) ]
    ///The `vertexAttrib3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn vertex_attrib3fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        indx: u32,
        values: &::wasm_bindgen::JsValue,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = vertexAttrib4f ) ]
    ///The `vertexAttrib4f()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib4f)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn vertex_attrib4f(this: &WebGlRenderingContext, indx: u32, x: f32, y: f32, z: f32, w: f32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = vertexAttrib4fv ) ]
    ///The `vertexAttrib4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn vertex_attrib4fv_with_f32_array(this: &WebGlRenderingContext, indx: u32, values: &[f32]);

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = vertexAttrib4fv ) ]
    ///The `vertexAttrib4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn vertex_attrib4fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        indx: u32,
        values: &::wasm_bindgen::JsValue,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = vertexAttribPointer ) ]
    ///The `vertexAttribPointer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttribPointer)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn vertex_attrib_pointer_with_i32(
        this: &WebGlRenderingContext,
        indx: u32,
        size: i32,
        type_: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = vertexAttribPointer ) ]
    ///The `vertexAttribPointer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttribPointer)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn vertex_attrib_pointer_with_f64(
        this: &WebGlRenderingContext,
        indx: u32,
        size: i32,
        type_: u32,
        normalized: bool,
        stride: i32,
        offset: f64,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGLRenderingContext" , js_name = viewport ) ]
    ///The `viewport()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/viewport)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*
    pub fn viewport(this: &WebGlRenderingContext, x: i32, y: i32, width: i32, height: i32);

}

impl WebGlRenderingContext {
    ///The `WebGLRenderingContext.DEPTH_BUFFER_BIT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const DEPTH_BUFFER_BIT: u32 = 256u64 as u32;

    ///The `WebGLRenderingContext.STENCIL_BUFFER_BIT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STENCIL_BUFFER_BIT: u32 = 1024u64 as u32;

    ///The `WebGLRenderingContext.COLOR_BUFFER_BIT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const COLOR_BUFFER_BIT: u32 = 16384u64 as u32;

    ///The `WebGLRenderingContext.POINTS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const POINTS: u32 = 0u64 as u32;

    ///The `WebGLRenderingContext.LINES` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const LINES: u32 = 1u64 as u32;

    ///The `WebGLRenderingContext.LINE_LOOP` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const LINE_LOOP: u32 = 2u64 as u32;

    ///The `WebGLRenderingContext.LINE_STRIP` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const LINE_STRIP: u32 = 3u64 as u32;

    ///The `WebGLRenderingContext.TRIANGLES` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TRIANGLES: u32 = 4u64 as u32;

    ///The `WebGLRenderingContext.TRIANGLE_STRIP` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TRIANGLE_STRIP: u32 = 5u64 as u32;

    ///The `WebGLRenderingContext.TRIANGLE_FAN` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TRIANGLE_FAN: u32 = 6u64 as u32;

    ///The `WebGLRenderingContext.ZERO` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const ZERO: u32 = 0i64 as u32;

    ///The `WebGLRenderingContext.ONE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const ONE: u32 = 1u64 as u32;

    ///The `WebGLRenderingContext.SRC_COLOR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const SRC_COLOR: u32 = 768u64 as u32;

    ///The `WebGLRenderingContext.ONE_MINUS_SRC_COLOR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const ONE_MINUS_SRC_COLOR: u32 = 769u64 as u32;

    ///The `WebGLRenderingContext.SRC_ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const SRC_ALPHA: u32 = 770u64 as u32;

    ///The `WebGLRenderingContext.ONE_MINUS_SRC_ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const ONE_MINUS_SRC_ALPHA: u32 = 771u64 as u32;

    ///The `WebGLRenderingContext.DST_ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const DST_ALPHA: u32 = 772u64 as u32;

    ///The `WebGLRenderingContext.ONE_MINUS_DST_ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const ONE_MINUS_DST_ALPHA: u32 = 773u64 as u32;

    ///The `WebGLRenderingContext.DST_COLOR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const DST_COLOR: u32 = 774u64 as u32;

    ///The `WebGLRenderingContext.ONE_MINUS_DST_COLOR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const ONE_MINUS_DST_COLOR: u32 = 775u64 as u32;

    ///The `WebGLRenderingContext.SRC_ALPHA_SATURATE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const SRC_ALPHA_SATURATE: u32 = 776u64 as u32;

    ///The `WebGLRenderingContext.FUNC_ADD` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FUNC_ADD: u32 = 32774u64 as u32;

    ///The `WebGLRenderingContext.BLEND_EQUATION` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const BLEND_EQUATION: u32 = 32777u64 as u32;

    ///The `WebGLRenderingContext.BLEND_EQUATION_RGB` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const BLEND_EQUATION_RGB: u32 = 32777u64 as u32;

    ///The `WebGLRenderingContext.BLEND_EQUATION_ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const BLEND_EQUATION_ALPHA: u32 = 34877u64 as u32;

    ///The `WebGLRenderingContext.FUNC_SUBTRACT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FUNC_SUBTRACT: u32 = 32778u64 as u32;

    ///The `WebGLRenderingContext.FUNC_REVERSE_SUBTRACT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FUNC_REVERSE_SUBTRACT: u32 = 32779u64 as u32;

    ///The `WebGLRenderingContext.BLEND_DST_RGB` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const BLEND_DST_RGB: u32 = 32968u64 as u32;

    ///The `WebGLRenderingContext.BLEND_SRC_RGB` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const BLEND_SRC_RGB: u32 = 32969u64 as u32;

    ///The `WebGLRenderingContext.BLEND_DST_ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const BLEND_DST_ALPHA: u32 = 32970u64 as u32;

    ///The `WebGLRenderingContext.BLEND_SRC_ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const BLEND_SRC_ALPHA: u32 = 32971u64 as u32;

    ///The `WebGLRenderingContext.CONSTANT_COLOR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const CONSTANT_COLOR: u32 = 32769u64 as u32;

    ///The `WebGLRenderingContext.ONE_MINUS_CONSTANT_COLOR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const ONE_MINUS_CONSTANT_COLOR: u32 = 32770u64 as u32;

    ///The `WebGLRenderingContext.CONSTANT_ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const CONSTANT_ALPHA: u32 = 32771u64 as u32;

    ///The `WebGLRenderingContext.ONE_MINUS_CONSTANT_ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const ONE_MINUS_CONSTANT_ALPHA: u32 = 32772u64 as u32;

    ///The `WebGLRenderingContext.BLEND_COLOR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const BLEND_COLOR: u32 = 32773u64 as u32;

    ///The `WebGLRenderingContext.ARRAY_BUFFER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const ARRAY_BUFFER: u32 = 34962u64 as u32;

    ///The `WebGLRenderingContext.ELEMENT_ARRAY_BUFFER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const ELEMENT_ARRAY_BUFFER: u32 = 34963u64 as u32;

    ///The `WebGLRenderingContext.ARRAY_BUFFER_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const ARRAY_BUFFER_BINDING: u32 = 34964u64 as u32;

    ///The `WebGLRenderingContext.ELEMENT_ARRAY_BUFFER_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const ELEMENT_ARRAY_BUFFER_BINDING: u32 = 34965u64 as u32;

    ///The `WebGLRenderingContext.STREAM_DRAW` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STREAM_DRAW: u32 = 35040u64 as u32;

    ///The `WebGLRenderingContext.STATIC_DRAW` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STATIC_DRAW: u32 = 35044u64 as u32;

    ///The `WebGLRenderingContext.DYNAMIC_DRAW` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const DYNAMIC_DRAW: u32 = 35048u64 as u32;

    ///The `WebGLRenderingContext.BUFFER_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const BUFFER_SIZE: u32 = 34660u64 as u32;

    ///The `WebGLRenderingContext.BUFFER_USAGE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const BUFFER_USAGE: u32 = 34661u64 as u32;

    ///The `WebGLRenderingContext.CURRENT_VERTEX_ATTRIB` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const CURRENT_VERTEX_ATTRIB: u32 = 34342u64 as u32;

    ///The `WebGLRenderingContext.FRONT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FRONT: u32 = 1028u64 as u32;

    ///The `WebGLRenderingContext.BACK` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const BACK: u32 = 1029u64 as u32;

    ///The `WebGLRenderingContext.FRONT_AND_BACK` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FRONT_AND_BACK: u32 = 1032u64 as u32;

    ///The `WebGLRenderingContext.CULL_FACE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const CULL_FACE: u32 = 2884u64 as u32;

    ///The `WebGLRenderingContext.BLEND` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const BLEND: u32 = 3042u64 as u32;

    ///The `WebGLRenderingContext.DITHER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const DITHER: u32 = 3024u64 as u32;

    ///The `WebGLRenderingContext.STENCIL_TEST` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STENCIL_TEST: u32 = 2960u64 as u32;

    ///The `WebGLRenderingContext.DEPTH_TEST` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const DEPTH_TEST: u32 = 2929u64 as u32;

    ///The `WebGLRenderingContext.SCISSOR_TEST` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const SCISSOR_TEST: u32 = 3089u64 as u32;

    ///The `WebGLRenderingContext.POLYGON_OFFSET_FILL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const POLYGON_OFFSET_FILL: u32 = 32823u64 as u32;

    ///The `WebGLRenderingContext.SAMPLE_ALPHA_TO_COVERAGE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const SAMPLE_ALPHA_TO_COVERAGE: u32 = 32926u64 as u32;

    ///The `WebGLRenderingContext.SAMPLE_COVERAGE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const SAMPLE_COVERAGE: u32 = 32928u64 as u32;

    ///The `WebGLRenderingContext.NO_ERROR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const NO_ERROR: u32 = 0i64 as u32;

    ///The `WebGLRenderingContext.INVALID_ENUM` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const INVALID_ENUM: u32 = 1280u64 as u32;

    ///The `WebGLRenderingContext.INVALID_VALUE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const INVALID_VALUE: u32 = 1281u64 as u32;

    ///The `WebGLRenderingContext.INVALID_OPERATION` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const INVALID_OPERATION: u32 = 1282u64 as u32;

    ///The `WebGLRenderingContext.OUT_OF_MEMORY` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const OUT_OF_MEMORY: u32 = 1285u64 as u32;

    ///The `WebGLRenderingContext.CW` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const CW: u32 = 2304u64 as u32;

    ///The `WebGLRenderingContext.CCW` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const CCW: u32 = 2305u64 as u32;

    ///The `WebGLRenderingContext.LINE_WIDTH` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const LINE_WIDTH: u32 = 2849u64 as u32;

    ///The `WebGLRenderingContext.ALIASED_POINT_SIZE_RANGE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const ALIASED_POINT_SIZE_RANGE: u32 = 33901u64 as u32;

    ///The `WebGLRenderingContext.ALIASED_LINE_WIDTH_RANGE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const ALIASED_LINE_WIDTH_RANGE: u32 = 33902u64 as u32;

    ///The `WebGLRenderingContext.CULL_FACE_MODE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const CULL_FACE_MODE: u32 = 2885u64 as u32;

    ///The `WebGLRenderingContext.FRONT_FACE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FRONT_FACE: u32 = 2886u64 as u32;

    ///The `WebGLRenderingContext.DEPTH_RANGE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const DEPTH_RANGE: u32 = 2928u64 as u32;

    ///The `WebGLRenderingContext.DEPTH_WRITEMASK` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const DEPTH_WRITEMASK: u32 = 2930u64 as u32;

    ///The `WebGLRenderingContext.DEPTH_CLEAR_VALUE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const DEPTH_CLEAR_VALUE: u32 = 2931u64 as u32;

    ///The `WebGLRenderingContext.DEPTH_FUNC` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const DEPTH_FUNC: u32 = 2932u64 as u32;

    ///The `WebGLRenderingContext.STENCIL_CLEAR_VALUE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STENCIL_CLEAR_VALUE: u32 = 2961u64 as u32;

    ///The `WebGLRenderingContext.STENCIL_FUNC` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STENCIL_FUNC: u32 = 2962u64 as u32;

    ///The `WebGLRenderingContext.STENCIL_FAIL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STENCIL_FAIL: u32 = 2964u64 as u32;

    ///The `WebGLRenderingContext.STENCIL_PASS_DEPTH_FAIL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STENCIL_PASS_DEPTH_FAIL: u32 = 2965u64 as u32;

    ///The `WebGLRenderingContext.STENCIL_PASS_DEPTH_PASS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STENCIL_PASS_DEPTH_PASS: u32 = 2966u64 as u32;

    ///The `WebGLRenderingContext.STENCIL_REF` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STENCIL_REF: u32 = 2967u64 as u32;

    ///The `WebGLRenderingContext.STENCIL_VALUE_MASK` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STENCIL_VALUE_MASK: u32 = 2963u64 as u32;

    ///The `WebGLRenderingContext.STENCIL_WRITEMASK` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STENCIL_WRITEMASK: u32 = 2968u64 as u32;

    ///The `WebGLRenderingContext.STENCIL_BACK_FUNC` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STENCIL_BACK_FUNC: u32 = 34816u64 as u32;

    ///The `WebGLRenderingContext.STENCIL_BACK_FAIL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STENCIL_BACK_FAIL: u32 = 34817u64 as u32;

    ///The `WebGLRenderingContext.STENCIL_BACK_PASS_DEPTH_FAIL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STENCIL_BACK_PASS_DEPTH_FAIL: u32 = 34818u64 as u32;

    ///The `WebGLRenderingContext.STENCIL_BACK_PASS_DEPTH_PASS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STENCIL_BACK_PASS_DEPTH_PASS: u32 = 34819u64 as u32;

    ///The `WebGLRenderingContext.STENCIL_BACK_REF` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STENCIL_BACK_REF: u32 = 36003u64 as u32;

    ///The `WebGLRenderingContext.STENCIL_BACK_VALUE_MASK` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STENCIL_BACK_VALUE_MASK: u32 = 36004u64 as u32;

    ///The `WebGLRenderingContext.STENCIL_BACK_WRITEMASK` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STENCIL_BACK_WRITEMASK: u32 = 36005u64 as u32;

    ///The `WebGLRenderingContext.VIEWPORT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const VIEWPORT: u32 = 2978u64 as u32;

    ///The `WebGLRenderingContext.SCISSOR_BOX` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const SCISSOR_BOX: u32 = 3088u64 as u32;

    ///The `WebGLRenderingContext.COLOR_CLEAR_VALUE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const COLOR_CLEAR_VALUE: u32 = 3106u64 as u32;

    ///The `WebGLRenderingContext.COLOR_WRITEMASK` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const COLOR_WRITEMASK: u32 = 3107u64 as u32;

    ///The `WebGLRenderingContext.UNPACK_ALIGNMENT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const UNPACK_ALIGNMENT: u32 = 3317u64 as u32;

    ///The `WebGLRenderingContext.PACK_ALIGNMENT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const PACK_ALIGNMENT: u32 = 3333u64 as u32;

    ///The `WebGLRenderingContext.MAX_TEXTURE_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const MAX_TEXTURE_SIZE: u32 = 3379u64 as u32;

    ///The `WebGLRenderingContext.MAX_VIEWPORT_DIMS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const MAX_VIEWPORT_DIMS: u32 = 3386u64 as u32;

    ///The `WebGLRenderingContext.SUBPIXEL_BITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const SUBPIXEL_BITS: u32 = 3408u64 as u32;

    ///The `WebGLRenderingContext.RED_BITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const RED_BITS: u32 = 3410u64 as u32;

    ///The `WebGLRenderingContext.GREEN_BITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const GREEN_BITS: u32 = 3411u64 as u32;

    ///The `WebGLRenderingContext.BLUE_BITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const BLUE_BITS: u32 = 3412u64 as u32;

    ///The `WebGLRenderingContext.ALPHA_BITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const ALPHA_BITS: u32 = 3413u64 as u32;

    ///The `WebGLRenderingContext.DEPTH_BITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const DEPTH_BITS: u32 = 3414u64 as u32;

    ///The `WebGLRenderingContext.STENCIL_BITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STENCIL_BITS: u32 = 3415u64 as u32;

    ///The `WebGLRenderingContext.POLYGON_OFFSET_UNITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const POLYGON_OFFSET_UNITS: u32 = 10752u64 as u32;

    ///The `WebGLRenderingContext.POLYGON_OFFSET_FACTOR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const POLYGON_OFFSET_FACTOR: u32 = 32824u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE_BINDING_2D` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE_BINDING_2D: u32 = 32873u64 as u32;

    ///The `WebGLRenderingContext.SAMPLE_BUFFERS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const SAMPLE_BUFFERS: u32 = 32936u64 as u32;

    ///The `WebGLRenderingContext.SAMPLES` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const SAMPLES: u32 = 32937u64 as u32;

    ///The `WebGLRenderingContext.SAMPLE_COVERAGE_VALUE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const SAMPLE_COVERAGE_VALUE: u32 = 32938u64 as u32;

    ///The `WebGLRenderingContext.SAMPLE_COVERAGE_INVERT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const SAMPLE_COVERAGE_INVERT: u32 = 32939u64 as u32;

    ///The `WebGLRenderingContext.COMPRESSED_TEXTURE_FORMATS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const COMPRESSED_TEXTURE_FORMATS: u32 = 34467u64 as u32;

    ///The `WebGLRenderingContext.DONT_CARE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const DONT_CARE: u32 = 4352u64 as u32;

    ///The `WebGLRenderingContext.FASTEST` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FASTEST: u32 = 4353u64 as u32;

    ///The `WebGLRenderingContext.NICEST` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const NICEST: u32 = 4354u64 as u32;

    ///The `WebGLRenderingContext.GENERATE_MIPMAP_HINT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const GENERATE_MIPMAP_HINT: u32 = 33170u64 as u32;

    ///The `WebGLRenderingContext.BYTE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const BYTE: u32 = 5120u64 as u32;

    ///The `WebGLRenderingContext.UNSIGNED_BYTE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const UNSIGNED_BYTE: u32 = 5121u64 as u32;

    ///The `WebGLRenderingContext.SHORT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const SHORT: u32 = 5122u64 as u32;

    ///The `WebGLRenderingContext.UNSIGNED_SHORT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const UNSIGNED_SHORT: u32 = 5123u64 as u32;

    ///The `WebGLRenderingContext.INT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const INT: u32 = 5124u64 as u32;

    ///The `WebGLRenderingContext.UNSIGNED_INT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const UNSIGNED_INT: u32 = 5125u64 as u32;

    ///The `WebGLRenderingContext.FLOAT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FLOAT: u32 = 5126u64 as u32;

    ///The `WebGLRenderingContext.DEPTH_COMPONENT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const DEPTH_COMPONENT: u32 = 6402u64 as u32;

    ///The `WebGLRenderingContext.ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const ALPHA: u32 = 6406u64 as u32;

    ///The `WebGLRenderingContext.RGB` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const RGB: u32 = 6407u64 as u32;

    ///The `WebGLRenderingContext.RGBA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const RGBA: u32 = 6408u64 as u32;

    ///The `WebGLRenderingContext.LUMINANCE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const LUMINANCE: u32 = 6409u64 as u32;

    ///The `WebGLRenderingContext.LUMINANCE_ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const LUMINANCE_ALPHA: u32 = 6410u64 as u32;

    ///The `WebGLRenderingContext.UNSIGNED_SHORT_4_4_4_4` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const UNSIGNED_SHORT_4_4_4_4: u32 = 32819u64 as u32;

    ///The `WebGLRenderingContext.UNSIGNED_SHORT_5_5_5_1` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const UNSIGNED_SHORT_5_5_5_1: u32 = 32820u64 as u32;

    ///The `WebGLRenderingContext.UNSIGNED_SHORT_5_6_5` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const UNSIGNED_SHORT_5_6_5: u32 = 33635u64 as u32;

    ///The `WebGLRenderingContext.FRAGMENT_SHADER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FRAGMENT_SHADER: u32 = 35632u64 as u32;

    ///The `WebGLRenderingContext.VERTEX_SHADER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const VERTEX_SHADER: u32 = 35633u64 as u32;

    ///The `WebGLRenderingContext.MAX_VERTEX_ATTRIBS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const MAX_VERTEX_ATTRIBS: u32 = 34921u64 as u32;

    ///The `WebGLRenderingContext.MAX_VERTEX_UNIFORM_VECTORS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const MAX_VERTEX_UNIFORM_VECTORS: u32 = 36347u64 as u32;

    ///The `WebGLRenderingContext.MAX_VARYING_VECTORS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const MAX_VARYING_VECTORS: u32 = 36348u64 as u32;

    ///The `WebGLRenderingContext.MAX_COMBINED_TEXTURE_IMAGE_UNITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: u32 = 35661u64 as u32;

    ///The `WebGLRenderingContext.MAX_VERTEX_TEXTURE_IMAGE_UNITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: u32 = 35660u64 as u32;

    ///The `WebGLRenderingContext.MAX_TEXTURE_IMAGE_UNITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const MAX_TEXTURE_IMAGE_UNITS: u32 = 34930u64 as u32;

    ///The `WebGLRenderingContext.MAX_FRAGMENT_UNIFORM_VECTORS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const MAX_FRAGMENT_UNIFORM_VECTORS: u32 = 36349u64 as u32;

    ///The `WebGLRenderingContext.SHADER_TYPE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const SHADER_TYPE: u32 = 35663u64 as u32;

    ///The `WebGLRenderingContext.DELETE_STATUS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const DELETE_STATUS: u32 = 35712u64 as u32;

    ///The `WebGLRenderingContext.LINK_STATUS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const LINK_STATUS: u32 = 35714u64 as u32;

    ///The `WebGLRenderingContext.VALIDATE_STATUS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const VALIDATE_STATUS: u32 = 35715u64 as u32;

    ///The `WebGLRenderingContext.ATTACHED_SHADERS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const ATTACHED_SHADERS: u32 = 35717u64 as u32;

    ///The `WebGLRenderingContext.ACTIVE_UNIFORMS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const ACTIVE_UNIFORMS: u32 = 35718u64 as u32;

    ///The `WebGLRenderingContext.ACTIVE_ATTRIBUTES` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const ACTIVE_ATTRIBUTES: u32 = 35721u64 as u32;

    ///The `WebGLRenderingContext.SHADING_LANGUAGE_VERSION` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const SHADING_LANGUAGE_VERSION: u32 = 35724u64 as u32;

    ///The `WebGLRenderingContext.CURRENT_PROGRAM` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const CURRENT_PROGRAM: u32 = 35725u64 as u32;

    ///The `WebGLRenderingContext.NEVER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const NEVER: u32 = 512u64 as u32;

    ///The `WebGLRenderingContext.LESS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const LESS: u32 = 513u64 as u32;

    ///The `WebGLRenderingContext.EQUAL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const EQUAL: u32 = 514u64 as u32;

    ///The `WebGLRenderingContext.LEQUAL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const LEQUAL: u32 = 515u64 as u32;

    ///The `WebGLRenderingContext.GREATER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const GREATER: u32 = 516u64 as u32;

    ///The `WebGLRenderingContext.NOTEQUAL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const NOTEQUAL: u32 = 517u64 as u32;

    ///The `WebGLRenderingContext.GEQUAL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const GEQUAL: u32 = 518u64 as u32;

    ///The `WebGLRenderingContext.ALWAYS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const ALWAYS: u32 = 519u64 as u32;

    ///The `WebGLRenderingContext.KEEP` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const KEEP: u32 = 7680u64 as u32;

    ///The `WebGLRenderingContext.REPLACE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const REPLACE: u32 = 7681u64 as u32;

    ///The `WebGLRenderingContext.INCR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const INCR: u32 = 7682u64 as u32;

    ///The `WebGLRenderingContext.DECR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const DECR: u32 = 7683u64 as u32;

    ///The `WebGLRenderingContext.INVERT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const INVERT: u32 = 5386u64 as u32;

    ///The `WebGLRenderingContext.INCR_WRAP` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const INCR_WRAP: u32 = 34055u64 as u32;

    ///The `WebGLRenderingContext.DECR_WRAP` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const DECR_WRAP: u32 = 34056u64 as u32;

    ///The `WebGLRenderingContext.VENDOR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const VENDOR: u32 = 7936u64 as u32;

    ///The `WebGLRenderingContext.RENDERER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const RENDERER: u32 = 7937u64 as u32;

    ///The `WebGLRenderingContext.VERSION` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const VERSION: u32 = 7938u64 as u32;

    ///The `WebGLRenderingContext.NEAREST` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const NEAREST: u32 = 9728u64 as u32;

    ///The `WebGLRenderingContext.LINEAR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const LINEAR: u32 = 9729u64 as u32;

    ///The `WebGLRenderingContext.NEAREST_MIPMAP_NEAREST` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const NEAREST_MIPMAP_NEAREST: u32 = 9984u64 as u32;

    ///The `WebGLRenderingContext.LINEAR_MIPMAP_NEAREST` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const LINEAR_MIPMAP_NEAREST: u32 = 9985u64 as u32;

    ///The `WebGLRenderingContext.NEAREST_MIPMAP_LINEAR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const NEAREST_MIPMAP_LINEAR: u32 = 9986u64 as u32;

    ///The `WebGLRenderingContext.LINEAR_MIPMAP_LINEAR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const LINEAR_MIPMAP_LINEAR: u32 = 9987u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE_MAG_FILTER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE_MAG_FILTER: u32 = 10240u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE_MIN_FILTER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE_MIN_FILTER: u32 = 10241u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE_WRAP_S` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE_WRAP_S: u32 = 10242u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE_WRAP_T` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE_WRAP_T: u32 = 10243u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE_2D` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE_2D: u32 = 3553u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE: u32 = 5890u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE_CUBE_MAP` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE_CUBE_MAP: u32 = 34067u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE_BINDING_CUBE_MAP` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE_BINDING_CUBE_MAP: u32 = 34068u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE_CUBE_MAP_POSITIVE_X` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE_CUBE_MAP_POSITIVE_X: u32 = 34069u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE_CUBE_MAP_NEGATIVE_X` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE_CUBE_MAP_NEGATIVE_X: u32 = 34070u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE_CUBE_MAP_POSITIVE_Y` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE_CUBE_MAP_POSITIVE_Y: u32 = 34071u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE_CUBE_MAP_NEGATIVE_Y` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: u32 = 34072u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE_CUBE_MAP_POSITIVE_Z` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE_CUBE_MAP_POSITIVE_Z: u32 = 34073u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE_CUBE_MAP_NEGATIVE_Z` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: u32 = 34074u64 as u32;

    ///The `WebGLRenderingContext.MAX_CUBE_MAP_TEXTURE_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const MAX_CUBE_MAP_TEXTURE_SIZE: u32 = 34076u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE0` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE0: u32 = 33984u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE1` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE1: u32 = 33985u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE2` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE2: u32 = 33986u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE3` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE3: u32 = 33987u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE4` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE4: u32 = 33988u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE5` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE5: u32 = 33989u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE6` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE6: u32 = 33990u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE7` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE7: u32 = 33991u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE8` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE8: u32 = 33992u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE9` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE9: u32 = 33993u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE10` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE10: u32 = 33994u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE11` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE11: u32 = 33995u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE12` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE12: u32 = 33996u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE13` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE13: u32 = 33997u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE14` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE14: u32 = 33998u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE15` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE15: u32 = 33999u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE16` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE16: u32 = 34000u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE17` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE17: u32 = 34001u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE18` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE18: u32 = 34002u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE19` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE19: u32 = 34003u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE20` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE20: u32 = 34004u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE21` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE21: u32 = 34005u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE22` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE22: u32 = 34006u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE23` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE23: u32 = 34007u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE24` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE24: u32 = 34008u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE25` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE25: u32 = 34009u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE26` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE26: u32 = 34010u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE27` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE27: u32 = 34011u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE28` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE28: u32 = 34012u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE29` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE29: u32 = 34013u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE30` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE30: u32 = 34014u64 as u32;

    ///The `WebGLRenderingContext.TEXTURE31` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const TEXTURE31: u32 = 34015u64 as u32;

    ///The `WebGLRenderingContext.ACTIVE_TEXTURE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const ACTIVE_TEXTURE: u32 = 34016u64 as u32;

    ///The `WebGLRenderingContext.REPEAT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const REPEAT: u32 = 10497u64 as u32;

    ///The `WebGLRenderingContext.CLAMP_TO_EDGE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const CLAMP_TO_EDGE: u32 = 33071u64 as u32;

    ///The `WebGLRenderingContext.MIRRORED_REPEAT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const MIRRORED_REPEAT: u32 = 33648u64 as u32;

    ///The `WebGLRenderingContext.FLOAT_VEC2` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FLOAT_VEC2: u32 = 35664u64 as u32;

    ///The `WebGLRenderingContext.FLOAT_VEC3` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FLOAT_VEC3: u32 = 35665u64 as u32;

    ///The `WebGLRenderingContext.FLOAT_VEC4` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FLOAT_VEC4: u32 = 35666u64 as u32;

    ///The `WebGLRenderingContext.INT_VEC2` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const INT_VEC2: u32 = 35667u64 as u32;

    ///The `WebGLRenderingContext.INT_VEC3` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const INT_VEC3: u32 = 35668u64 as u32;

    ///The `WebGLRenderingContext.INT_VEC4` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const INT_VEC4: u32 = 35669u64 as u32;

    ///The `WebGLRenderingContext.BOOL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const BOOL: u32 = 35670u64 as u32;

    ///The `WebGLRenderingContext.BOOL_VEC2` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const BOOL_VEC2: u32 = 35671u64 as u32;

    ///The `WebGLRenderingContext.BOOL_VEC3` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const BOOL_VEC3: u32 = 35672u64 as u32;

    ///The `WebGLRenderingContext.BOOL_VEC4` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const BOOL_VEC4: u32 = 35673u64 as u32;

    ///The `WebGLRenderingContext.FLOAT_MAT2` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FLOAT_MAT2: u32 = 35674u64 as u32;

    ///The `WebGLRenderingContext.FLOAT_MAT3` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FLOAT_MAT3: u32 = 35675u64 as u32;

    ///The `WebGLRenderingContext.FLOAT_MAT4` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FLOAT_MAT4: u32 = 35676u64 as u32;

    ///The `WebGLRenderingContext.SAMPLER_2D` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const SAMPLER_2D: u32 = 35678u64 as u32;

    ///The `WebGLRenderingContext.SAMPLER_CUBE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const SAMPLER_CUBE: u32 = 35680u64 as u32;

    ///The `WebGLRenderingContext.VERTEX_ATTRIB_ARRAY_ENABLED` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const VERTEX_ATTRIB_ARRAY_ENABLED: u32 = 34338u64 as u32;

    ///The `WebGLRenderingContext.VERTEX_ATTRIB_ARRAY_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const VERTEX_ATTRIB_ARRAY_SIZE: u32 = 34339u64 as u32;

    ///The `WebGLRenderingContext.VERTEX_ATTRIB_ARRAY_STRIDE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const VERTEX_ATTRIB_ARRAY_STRIDE: u32 = 34340u64 as u32;

    ///The `WebGLRenderingContext.VERTEX_ATTRIB_ARRAY_TYPE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const VERTEX_ATTRIB_ARRAY_TYPE: u32 = 34341u64 as u32;

    ///The `WebGLRenderingContext.VERTEX_ATTRIB_ARRAY_NORMALIZED` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: u32 = 34922u64 as u32;

    ///The `WebGLRenderingContext.VERTEX_ATTRIB_ARRAY_POINTER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const VERTEX_ATTRIB_ARRAY_POINTER: u32 = 34373u64 as u32;

    ///The `WebGLRenderingContext.VERTEX_ATTRIB_ARRAY_BUFFER_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: u32 = 34975u64 as u32;

    ///The `WebGLRenderingContext.IMPLEMENTATION_COLOR_READ_TYPE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const IMPLEMENTATION_COLOR_READ_TYPE: u32 = 35738u64 as u32;

    ///The `WebGLRenderingContext.IMPLEMENTATION_COLOR_READ_FORMAT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const IMPLEMENTATION_COLOR_READ_FORMAT: u32 = 35739u64 as u32;

    ///The `WebGLRenderingContext.COMPILE_STATUS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const COMPILE_STATUS: u32 = 35713u64 as u32;

    ///The `WebGLRenderingContext.LOW_FLOAT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const LOW_FLOAT: u32 = 36336u64 as u32;

    ///The `WebGLRenderingContext.MEDIUM_FLOAT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const MEDIUM_FLOAT: u32 = 36337u64 as u32;

    ///The `WebGLRenderingContext.HIGH_FLOAT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const HIGH_FLOAT: u32 = 36338u64 as u32;

    ///The `WebGLRenderingContext.LOW_INT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const LOW_INT: u32 = 36339u64 as u32;

    ///The `WebGLRenderingContext.MEDIUM_INT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const MEDIUM_INT: u32 = 36340u64 as u32;

    ///The `WebGLRenderingContext.HIGH_INT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const HIGH_INT: u32 = 36341u64 as u32;

    ///The `WebGLRenderingContext.FRAMEBUFFER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FRAMEBUFFER: u32 = 36160u64 as u32;

    ///The `WebGLRenderingContext.RENDERBUFFER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const RENDERBUFFER: u32 = 36161u64 as u32;

    ///The `WebGLRenderingContext.RGBA4` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const RGBA4: u32 = 32854u64 as u32;

    ///The `WebGLRenderingContext.RGB5_A1` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const RGB5_A1: u32 = 32855u64 as u32;

    ///The `WebGLRenderingContext.RGB565` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const RGB565: u32 = 36194u64 as u32;

    ///The `WebGLRenderingContext.DEPTH_COMPONENT16` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const DEPTH_COMPONENT16: u32 = 33189u64 as u32;

    ///The `WebGLRenderingContext.STENCIL_INDEX8` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STENCIL_INDEX8: u32 = 36168u64 as u32;

    ///The `WebGLRenderingContext.DEPTH_STENCIL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const DEPTH_STENCIL: u32 = 34041u64 as u32;

    ///The `WebGLRenderingContext.RENDERBUFFER_WIDTH` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const RENDERBUFFER_WIDTH: u32 = 36162u64 as u32;

    ///The `WebGLRenderingContext.RENDERBUFFER_HEIGHT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const RENDERBUFFER_HEIGHT: u32 = 36163u64 as u32;

    ///The `WebGLRenderingContext.RENDERBUFFER_INTERNAL_FORMAT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const RENDERBUFFER_INTERNAL_FORMAT: u32 = 36164u64 as u32;

    ///The `WebGLRenderingContext.RENDERBUFFER_RED_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const RENDERBUFFER_RED_SIZE: u32 = 36176u64 as u32;

    ///The `WebGLRenderingContext.RENDERBUFFER_GREEN_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const RENDERBUFFER_GREEN_SIZE: u32 = 36177u64 as u32;

    ///The `WebGLRenderingContext.RENDERBUFFER_BLUE_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const RENDERBUFFER_BLUE_SIZE: u32 = 36178u64 as u32;

    ///The `WebGLRenderingContext.RENDERBUFFER_ALPHA_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const RENDERBUFFER_ALPHA_SIZE: u32 = 36179u64 as u32;

    ///The `WebGLRenderingContext.RENDERBUFFER_DEPTH_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const RENDERBUFFER_DEPTH_SIZE: u32 = 36180u64 as u32;

    ///The `WebGLRenderingContext.RENDERBUFFER_STENCIL_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const RENDERBUFFER_STENCIL_SIZE: u32 = 36181u64 as u32;

    ///The `WebGLRenderingContext.FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: u32 = 36048u64 as u32;

    ///The `WebGLRenderingContext.FRAMEBUFFER_ATTACHMENT_OBJECT_NAME` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: u32 = 36049u64 as u32;

    ///The `WebGLRenderingContext.FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: u32 = 36050u64 as u32;

    ///The `WebGLRenderingContext.FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: u32 = 36051u64 as u32;

    ///The `WebGLRenderingContext.COLOR_ATTACHMENT0` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const COLOR_ATTACHMENT0: u32 = 36064u64 as u32;

    ///The `WebGLRenderingContext.DEPTH_ATTACHMENT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const DEPTH_ATTACHMENT: u32 = 36096u64 as u32;

    ///The `WebGLRenderingContext.STENCIL_ATTACHMENT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const STENCIL_ATTACHMENT: u32 = 36128u64 as u32;

    ///The `WebGLRenderingContext.DEPTH_STENCIL_ATTACHMENT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const DEPTH_STENCIL_ATTACHMENT: u32 = 33306u64 as u32;

    ///The `WebGLRenderingContext.NONE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const NONE: u32 = 0i64 as u32;

    ///The `WebGLRenderingContext.FRAMEBUFFER_COMPLETE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FRAMEBUFFER_COMPLETE: u32 = 36053u64 as u32;

    ///The `WebGLRenderingContext.FRAMEBUFFER_INCOMPLETE_ATTACHMENT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: u32 = 36054u64 as u32;

    ///The `WebGLRenderingContext.FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: u32 = 36055u64 as u32;

    ///The `WebGLRenderingContext.FRAMEBUFFER_INCOMPLETE_DIMENSIONS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: u32 = 36057u64 as u32;

    ///The `WebGLRenderingContext.FRAMEBUFFER_UNSUPPORTED` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FRAMEBUFFER_UNSUPPORTED: u32 = 36061u64 as u32;

    ///The `WebGLRenderingContext.FRAMEBUFFER_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const FRAMEBUFFER_BINDING: u32 = 36006u64 as u32;

    ///The `WebGLRenderingContext.RENDERBUFFER_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const RENDERBUFFER_BINDING: u32 = 36007u64 as u32;

    ///The `WebGLRenderingContext.MAX_RENDERBUFFER_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const MAX_RENDERBUFFER_SIZE: u32 = 34024u64 as u32;

    ///The `WebGLRenderingContext.INVALID_FRAMEBUFFER_OPERATION` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const INVALID_FRAMEBUFFER_OPERATION: u32 = 1286u64 as u32;

    ///The `WebGLRenderingContext.UNPACK_FLIP_Y_WEBGL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const UNPACK_FLIP_Y_WEBGL: u32 = 37440u64 as u32;

    ///The `WebGLRenderingContext.UNPACK_PREMULTIPLY_ALPHA_WEBGL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const UNPACK_PREMULTIPLY_ALPHA_WEBGL: u32 = 37441u64 as u32;

    ///The `WebGLRenderingContext.CONTEXT_LOST_WEBGL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const CONTEXT_LOST_WEBGL: u32 = 37442u64 as u32;

    ///The `WebGLRenderingContext.UNPACK_COLORSPACE_CONVERSION_WEBGL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const UNPACK_COLORSPACE_CONVERSION_WEBGL: u32 = 37443u64 as u32;

    ///The `WebGLRenderingContext.BROWSER_DEFAULT_WEBGL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderingContext`*

    pub const BROWSER_DEFAULT_WEBGL: u32 = 37444u64 as u32;
}
