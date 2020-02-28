use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WebGLRenderingContext , typescript_name = WebGLRenderingContext ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGlRenderingContext` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub type WebGlRenderingContext;
    # [ wasm_bindgen ( structural , method , getter , js_name = canvas ) ]
    #[doc = "Getter for the `canvas` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/canvas)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn canvas(this: &WebGlRenderingContext) -> Option<::js_sys::Object>;
    # [ wasm_bindgen ( structural , method , getter , js_name = drawingBufferWidth ) ]
    #[doc = "Getter for the `drawingBufferWidth` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawingBufferWidth)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn drawing_buffer_width(this: &WebGlRenderingContext) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_name = drawingBufferHeight ) ]
    #[doc = "Getter for the `drawingBufferHeight` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawingBufferHeight)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn drawing_buffer_height(this: &WebGlRenderingContext) -> i32;
    # [ wasm_bindgen ( method , structural , js_name = bufferData ) ]
    #[doc = "The `bufferData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn buffer_data_with_i32(this: &WebGlRenderingContext, target: u32, size: i32, usage: u32);
    # [ wasm_bindgen ( method , structural , js_name = bufferData ) ]
    #[doc = "The `bufferData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn buffer_data_with_f64(this: &WebGlRenderingContext, target: u32, size: f64, usage: u32);
    # [ wasm_bindgen ( method , structural , js_name = bufferData ) ]
    #[doc = "The `bufferData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn buffer_data_with_opt_array_buffer(
        this: &WebGlRenderingContext,
        target: u32,
        data: Option<&::js_sys::ArrayBuffer>,
        usage: u32,
    );
    # [ wasm_bindgen ( method , structural , js_name = bufferData ) ]
    #[doc = "The `bufferData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn buffer_data_with_array_buffer_view(
        this: &WebGlRenderingContext,
        target: u32,
        data: &::js_sys::Object,
        usage: u32,
    );
    # [ wasm_bindgen ( method , structural , js_name = bufferData ) ]
    #[doc = "The `bufferData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn buffer_data_with_u8_array(
        this: &WebGlRenderingContext,
        target: u32,
        data: &[u8],
        usage: u32,
    );
    # [ wasm_bindgen ( method , structural , js_name = bufferSubData ) ]
    #[doc = "The `bufferSubData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn buffer_sub_data_with_i32_and_array_buffer(
        this: &WebGlRenderingContext,
        target: u32,
        offset: i32,
        data: &::js_sys::ArrayBuffer,
    );
    # [ wasm_bindgen ( method , structural , js_name = bufferSubData ) ]
    #[doc = "The `bufferSubData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn buffer_sub_data_with_f64_and_array_buffer(
        this: &WebGlRenderingContext,
        target: u32,
        offset: f64,
        data: &::js_sys::ArrayBuffer,
    );
    # [ wasm_bindgen ( method , structural , js_name = bufferSubData ) ]
    #[doc = "The `bufferSubData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn buffer_sub_data_with_i32_and_array_buffer_view(
        this: &WebGlRenderingContext,
        target: u32,
        offset: i32,
        data: &::js_sys::Object,
    );
    # [ wasm_bindgen ( method , structural , js_name = bufferSubData ) ]
    #[doc = "The `bufferSubData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn buffer_sub_data_with_f64_and_array_buffer_view(
        this: &WebGlRenderingContext,
        target: u32,
        offset: f64,
        data: &::js_sys::Object,
    );
    # [ wasm_bindgen ( method , structural , js_name = bufferSubData ) ]
    #[doc = "The `bufferSubData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn buffer_sub_data_with_i32_and_u8_array(
        this: &WebGlRenderingContext,
        target: u32,
        offset: i32,
        data: &[u8],
    );
    # [ wasm_bindgen ( method , structural , js_name = bufferSubData ) ]
    #[doc = "The `bufferSubData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn buffer_sub_data_with_f64_and_u8_array(
        this: &WebGlRenderingContext,
        target: u32,
        offset: f64,
        data: &[u8],
    );
    # [ wasm_bindgen ( method , structural , js_name = commit ) ]
    #[doc = "The `commit()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/commit)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn commit(this: &WebGlRenderingContext);
    # [ wasm_bindgen ( method , structural , js_name = compressedTexImage2D ) ]
    #[doc = "The `compressedTexImage2D()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compressedTexImage2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
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
    # [ wasm_bindgen ( method , structural , js_name = compressedTexImage2D ) ]
    #[doc = "The `compressedTexImage2D()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compressedTexImage2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
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
    # [ wasm_bindgen ( method , structural , js_name = compressedTexSubImage2D ) ]
    #[doc = "The `compressedTexSubImage2D()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compressedTexSubImage2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
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
    # [ wasm_bindgen ( method , structural , js_name = compressedTexSubImage2D ) ]
    #[doc = "The `compressedTexSubImage2D()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compressedTexSubImage2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = readPixels ) ]
    #[doc = "The `readPixels()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/readPixels)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = readPixels ) ]
    #[doc = "The `readPixels()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/readPixels)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = texImage2D ) ]
    #[doc = "The `texImage2D()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = texImage2D ) ]
    #[doc = "The `texImage2D()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = texImage2D ) ]
    #[doc = "The `texImage2D()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)\n\n*This API requires the following crate features to be activated: `ImageBitmap`, `WebGlRenderingContext`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = texImage2D ) ]
    #[doc = "The `texImage2D()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)\n\n*This API requires the following crate features to be activated: `ImageData`, `WebGlRenderingContext`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = texImage2D ) ]
    #[doc = "The `texImage2D()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`, `WebGlRenderingContext`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = texImage2D ) ]
    #[doc = "The `texImage2D()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WebGlRenderingContext`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = texImage2D ) ]
    #[doc = "The `texImage2D()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)\n\n*This API requires the following crate features to be activated: `HtmlVideoElement`, `WebGlRenderingContext`*"]
    pub fn tex_image_2d_with_u32_and_u32_and_video(
        this: &WebGlRenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        video: &HtmlVideoElement,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = texSubImage2D ) ]
    #[doc = "The `texSubImage2D()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = texSubImage2D ) ]
    #[doc = "The `texSubImage2D()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = texSubImage2D ) ]
    #[doc = "The `texSubImage2D()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)\n\n*This API requires the following crate features to be activated: `ImageBitmap`, `WebGlRenderingContext`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = texSubImage2D ) ]
    #[doc = "The `texSubImage2D()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)\n\n*This API requires the following crate features to be activated: `ImageData`, `WebGlRenderingContext`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = texSubImage2D ) ]
    #[doc = "The `texSubImage2D()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`, `WebGlRenderingContext`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = texSubImage2D ) ]
    #[doc = "The `texSubImage2D()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WebGlRenderingContext`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = texSubImage2D ) ]
    #[doc = "The `texSubImage2D()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)\n\n*This API requires the following crate features to be activated: `HtmlVideoElement`, `WebGlRenderingContext`*"]
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
    # [ wasm_bindgen ( method , structural , js_name = uniform1fv ) ]
    #[doc = "The `uniform1fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1fv_with_f32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform1fv ) ]
    #[doc = "The `uniform1fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform1iv ) ]
    #[doc = "The `uniform1iv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1iv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1iv_with_i32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform1iv ) ]
    #[doc = "The `uniform1iv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1iv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1iv_with_i32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform2fv ) ]
    #[doc = "The `uniform2fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2fv_with_f32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform2fv ) ]
    #[doc = "The `uniform2fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform2iv ) ]
    #[doc = "The `uniform2iv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2iv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2iv_with_i32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform2iv ) ]
    #[doc = "The `uniform2iv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2iv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2iv_with_i32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform3fv ) ]
    #[doc = "The `uniform3fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3fv_with_f32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform3fv ) ]
    #[doc = "The `uniform3fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform3iv ) ]
    #[doc = "The `uniform3iv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3iv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3iv_with_i32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform3iv ) ]
    #[doc = "The `uniform3iv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3iv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3iv_with_i32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform4fv ) ]
    #[doc = "The `uniform4fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4fv_with_f32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform4fv ) ]
    #[doc = "The `uniform4fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform4iv ) ]
    #[doc = "The `uniform4iv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4iv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4iv_with_i32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform4iv ) ]
    #[doc = "The `uniform4iv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4iv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4iv_with_i32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniformMatrix2fv ) ]
    #[doc = "The `uniformMatrix2fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix2fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2fv_with_f32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniformMatrix2fv ) ]
    #[doc = "The `uniformMatrix2fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix2fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniformMatrix3fv ) ]
    #[doc = "The `uniformMatrix3fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix3fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3fv_with_f32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniformMatrix3fv ) ]
    #[doc = "The `uniformMatrix3fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix3fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniformMatrix4fv ) ]
    #[doc = "The `uniformMatrix4fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix4fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4fv_with_f32_array(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniformMatrix4fv ) ]
    #[doc = "The `uniformMatrix4fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix4fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );
    # [ wasm_bindgen ( method , structural , js_name = activeTexture ) ]
    #[doc = "The `activeTexture()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/activeTexture)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn active_texture(this: &WebGlRenderingContext, texture: u32);
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlShader",))]
    # [ wasm_bindgen ( method , structural , js_name = attachShader ) ]
    #[doc = "The `attachShader()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/attachShader)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`, `WebGlShader`*"]
    pub fn attach_shader(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
        shader: &WebGlShader,
    );
    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_name = bindAttribLocation ) ]
    #[doc = "The `bindAttribLocation()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bindAttribLocation)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    pub fn bind_attrib_location(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
        index: u32,
        name: &str,
    );
    #[cfg(feature = "WebGlBuffer")]
    # [ wasm_bindgen ( method , structural , js_name = bindBuffer ) ]
    #[doc = "The `bindBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bindBuffer)\n\n*This API requires the following crate features to be activated: `WebGlBuffer`, `WebGlRenderingContext`*"]
    pub fn bind_buffer(this: &WebGlRenderingContext, target: u32, buffer: Option<&WebGlBuffer>);
    #[cfg(feature = "WebGlFramebuffer")]
    # [ wasm_bindgen ( method , structural , js_name = bindFramebuffer ) ]
    #[doc = "The `bindFramebuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bindFramebuffer)\n\n*This API requires the following crate features to be activated: `WebGlFramebuffer`, `WebGlRenderingContext`*"]
    pub fn bind_framebuffer(
        this: &WebGlRenderingContext,
        target: u32,
        framebuffer: Option<&WebGlFramebuffer>,
    );
    #[cfg(feature = "WebGlRenderbuffer")]
    # [ wasm_bindgen ( method , structural , js_name = bindRenderbuffer ) ]
    #[doc = "The `bindRenderbuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bindRenderbuffer)\n\n*This API requires the following crate features to be activated: `WebGlRenderbuffer`, `WebGlRenderingContext`*"]
    pub fn bind_renderbuffer(
        this: &WebGlRenderingContext,
        target: u32,
        renderbuffer: Option<&WebGlRenderbuffer>,
    );
    #[cfg(feature = "WebGlTexture")]
    # [ wasm_bindgen ( method , structural , js_name = bindTexture ) ]
    #[doc = "The `bindTexture()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bindTexture)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlTexture`*"]
    pub fn bind_texture(this: &WebGlRenderingContext, target: u32, texture: Option<&WebGlTexture>);
    # [ wasm_bindgen ( method , structural , js_name = blendColor ) ]
    #[doc = "The `blendColor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendColor)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn blend_color(this: &WebGlRenderingContext, red: f32, green: f32, blue: f32, alpha: f32);
    # [ wasm_bindgen ( method , structural , js_name = blendEquation ) ]
    #[doc = "The `blendEquation()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendEquation)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn blend_equation(this: &WebGlRenderingContext, mode: u32);
    # [ wasm_bindgen ( method , structural , js_name = blendEquationSeparate ) ]
    #[doc = "The `blendEquationSeparate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendEquationSeparate)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn blend_equation_separate(this: &WebGlRenderingContext, mode_rgb: u32, mode_alpha: u32);
    # [ wasm_bindgen ( method , structural , js_name = blendFunc ) ]
    #[doc = "The `blendFunc()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendFunc)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn blend_func(this: &WebGlRenderingContext, sfactor: u32, dfactor: u32);
    # [ wasm_bindgen ( method , structural , js_name = blendFuncSeparate ) ]
    #[doc = "The `blendFuncSeparate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendFuncSeparate)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn blend_func_separate(
        this: &WebGlRenderingContext,
        src_rgb: u32,
        dst_rgb: u32,
        src_alpha: u32,
        dst_alpha: u32,
    );
    # [ wasm_bindgen ( method , structural , js_name = checkFramebufferStatus ) ]
    #[doc = "The `checkFramebufferStatus()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/checkFramebufferStatus)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn check_framebuffer_status(this: &WebGlRenderingContext, target: u32) -> u32;
    # [ wasm_bindgen ( method , structural , js_name = clear ) ]
    #[doc = "The `clear()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/clear)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn clear(this: &WebGlRenderingContext, mask: u32);
    # [ wasm_bindgen ( method , structural , js_name = clearColor ) ]
    #[doc = "The `clearColor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/clearColor)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn clear_color(this: &WebGlRenderingContext, red: f32, green: f32, blue: f32, alpha: f32);
    # [ wasm_bindgen ( method , structural , js_name = clearDepth ) ]
    #[doc = "The `clearDepth()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/clearDepth)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn clear_depth(this: &WebGlRenderingContext, depth: f32);
    # [ wasm_bindgen ( method , structural , js_name = clearStencil ) ]
    #[doc = "The `clearStencil()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/clearStencil)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn clear_stencil(this: &WebGlRenderingContext, s: i32);
    # [ wasm_bindgen ( method , structural , js_name = colorMask ) ]
    #[doc = "The `colorMask()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/colorMask)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn color_mask(
        this: &WebGlRenderingContext,
        red: bool,
        green: bool,
        blue: bool,
        alpha: bool,
    );
    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_name = compileShader ) ]
    #[doc = "The `compileShader()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compileShader)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
    pub fn compile_shader(this: &WebGlRenderingContext, shader: &WebGlShader);
    # [ wasm_bindgen ( method , structural , js_name = copyTexImage2D ) ]
    #[doc = "The `copyTexImage2D()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/copyTexImage2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
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
    # [ wasm_bindgen ( method , structural , js_name = copyTexSubImage2D ) ]
    #[doc = "The `copyTexSubImage2D()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/copyTexSubImage2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
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
    # [ wasm_bindgen ( method , structural , js_name = createBuffer ) ]
    #[doc = "The `createBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createBuffer)\n\n*This API requires the following crate features to be activated: `WebGlBuffer`, `WebGlRenderingContext`*"]
    pub fn create_buffer(this: &WebGlRenderingContext) -> Option<WebGlBuffer>;
    #[cfg(feature = "WebGlFramebuffer")]
    # [ wasm_bindgen ( method , structural , js_name = createFramebuffer ) ]
    #[doc = "The `createFramebuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createFramebuffer)\n\n*This API requires the following crate features to be activated: `WebGlFramebuffer`, `WebGlRenderingContext`*"]
    pub fn create_framebuffer(this: &WebGlRenderingContext) -> Option<WebGlFramebuffer>;
    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_name = createProgram ) ]
    #[doc = "The `createProgram()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createProgram)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    pub fn create_program(this: &WebGlRenderingContext) -> Option<WebGlProgram>;
    #[cfg(feature = "WebGlRenderbuffer")]
    # [ wasm_bindgen ( method , structural , js_name = createRenderbuffer ) ]
    #[doc = "The `createRenderbuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createRenderbuffer)\n\n*This API requires the following crate features to be activated: `WebGlRenderbuffer`, `WebGlRenderingContext`*"]
    pub fn create_renderbuffer(this: &WebGlRenderingContext) -> Option<WebGlRenderbuffer>;
    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_name = createShader ) ]
    #[doc = "The `createShader()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createShader)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
    pub fn create_shader(this: &WebGlRenderingContext, type_: u32) -> Option<WebGlShader>;
    #[cfg(feature = "WebGlTexture")]
    # [ wasm_bindgen ( method , structural , js_name = createTexture ) ]
    #[doc = "The `createTexture()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createTexture)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlTexture`*"]
    pub fn create_texture(this: &WebGlRenderingContext) -> Option<WebGlTexture>;
    # [ wasm_bindgen ( method , structural , js_name = cullFace ) ]
    #[doc = "The `cullFace()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/cullFace)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn cull_face(this: &WebGlRenderingContext, mode: u32);
    #[cfg(feature = "WebGlBuffer")]
    # [ wasm_bindgen ( method , structural , js_name = deleteBuffer ) ]
    #[doc = "The `deleteBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteBuffer)\n\n*This API requires the following crate features to be activated: `WebGlBuffer`, `WebGlRenderingContext`*"]
    pub fn delete_buffer(this: &WebGlRenderingContext, buffer: Option<&WebGlBuffer>);
    #[cfg(feature = "WebGlFramebuffer")]
    # [ wasm_bindgen ( method , structural , js_name = deleteFramebuffer ) ]
    #[doc = "The `deleteFramebuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteFramebuffer)\n\n*This API requires the following crate features to be activated: `WebGlFramebuffer`, `WebGlRenderingContext`*"]
    pub fn delete_framebuffer(this: &WebGlRenderingContext, framebuffer: Option<&WebGlFramebuffer>);
    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_name = deleteProgram ) ]
    #[doc = "The `deleteProgram()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteProgram)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    pub fn delete_program(this: &WebGlRenderingContext, program: Option<&WebGlProgram>);
    #[cfg(feature = "WebGlRenderbuffer")]
    # [ wasm_bindgen ( method , structural , js_name = deleteRenderbuffer ) ]
    #[doc = "The `deleteRenderbuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteRenderbuffer)\n\n*This API requires the following crate features to be activated: `WebGlRenderbuffer`, `WebGlRenderingContext`*"]
    pub fn delete_renderbuffer(
        this: &WebGlRenderingContext,
        renderbuffer: Option<&WebGlRenderbuffer>,
    );
    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_name = deleteShader ) ]
    #[doc = "The `deleteShader()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteShader)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
    pub fn delete_shader(this: &WebGlRenderingContext, shader: Option<&WebGlShader>);
    #[cfg(feature = "WebGlTexture")]
    # [ wasm_bindgen ( method , structural , js_name = deleteTexture ) ]
    #[doc = "The `deleteTexture()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteTexture)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlTexture`*"]
    pub fn delete_texture(this: &WebGlRenderingContext, texture: Option<&WebGlTexture>);
    # [ wasm_bindgen ( method , structural , js_name = depthFunc ) ]
    #[doc = "The `depthFunc()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/depthFunc)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn depth_func(this: &WebGlRenderingContext, func: u32);
    # [ wasm_bindgen ( method , structural , js_name = depthMask ) ]
    #[doc = "The `depthMask()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/depthMask)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn depth_mask(this: &WebGlRenderingContext, flag: bool);
    # [ wasm_bindgen ( method , structural , js_name = depthRange ) ]
    #[doc = "The `depthRange()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/depthRange)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn depth_range(this: &WebGlRenderingContext, z_near: f32, z_far: f32);
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlShader",))]
    # [ wasm_bindgen ( method , structural , js_name = detachShader ) ]
    #[doc = "The `detachShader()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/detachShader)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`, `WebGlShader`*"]
    pub fn detach_shader(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
        shader: &WebGlShader,
    );
    # [ wasm_bindgen ( method , structural , js_name = disable ) ]
    #[doc = "The `disable()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/disable)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn disable(this: &WebGlRenderingContext, cap: u32);
    # [ wasm_bindgen ( method , structural , js_name = disableVertexAttribArray ) ]
    #[doc = "The `disableVertexAttribArray()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/disableVertexAttribArray)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn disable_vertex_attrib_array(this: &WebGlRenderingContext, index: u32);
    # [ wasm_bindgen ( method , structural , js_name = drawArrays ) ]
    #[doc = "The `drawArrays()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawArrays)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn draw_arrays(this: &WebGlRenderingContext, mode: u32, first: i32, count: i32);
    # [ wasm_bindgen ( method , structural , js_name = drawElements ) ]
    #[doc = "The `drawElements()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawElements)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn draw_elements_with_i32(
        this: &WebGlRenderingContext,
        mode: u32,
        count: i32,
        type_: u32,
        offset: i32,
    );
    # [ wasm_bindgen ( method , structural , js_name = drawElements ) ]
    #[doc = "The `drawElements()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawElements)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn draw_elements_with_f64(
        this: &WebGlRenderingContext,
        mode: u32,
        count: i32,
        type_: u32,
        offset: f64,
    );
    # [ wasm_bindgen ( method , structural , js_name = enable ) ]
    #[doc = "The `enable()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/enable)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn enable(this: &WebGlRenderingContext, cap: u32);
    # [ wasm_bindgen ( method , structural , js_name = enableVertexAttribArray ) ]
    #[doc = "The `enableVertexAttribArray()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/enableVertexAttribArray)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn enable_vertex_attrib_array(this: &WebGlRenderingContext, index: u32);
    # [ wasm_bindgen ( method , structural , js_name = finish ) ]
    #[doc = "The `finish()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/finish)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn finish(this: &WebGlRenderingContext);
    # [ wasm_bindgen ( method , structural , js_name = flush ) ]
    #[doc = "The `flush()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/flush)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn flush(this: &WebGlRenderingContext);
    #[cfg(feature = "WebGlRenderbuffer")]
    # [ wasm_bindgen ( method , structural , js_name = framebufferRenderbuffer ) ]
    #[doc = "The `framebufferRenderbuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/framebufferRenderbuffer)\n\n*This API requires the following crate features to be activated: `WebGlRenderbuffer`, `WebGlRenderingContext`*"]
    pub fn framebuffer_renderbuffer(
        this: &WebGlRenderingContext,
        target: u32,
        attachment: u32,
        renderbuffertarget: u32,
        renderbuffer: Option<&WebGlRenderbuffer>,
    );
    #[cfg(feature = "WebGlTexture")]
    # [ wasm_bindgen ( method , structural , js_name = framebufferTexture2D ) ]
    #[doc = "The `framebufferTexture2D()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/framebufferTexture2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlTexture`*"]
    pub fn framebuffer_texture_2d(
        this: &WebGlRenderingContext,
        target: u32,
        attachment: u32,
        textarget: u32,
        texture: Option<&WebGlTexture>,
        level: i32,
    );
    # [ wasm_bindgen ( method , structural , js_name = frontFace ) ]
    #[doc = "The `frontFace()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/frontFace)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn front_face(this: &WebGlRenderingContext, mode: u32);
    # [ wasm_bindgen ( method , structural , js_name = generateMipmap ) ]
    #[doc = "The `generateMipmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/generateMipmap)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn generate_mipmap(this: &WebGlRenderingContext, target: u32);
    #[cfg(all(feature = "WebGlActiveInfo", feature = "WebGlProgram",))]
    # [ wasm_bindgen ( method , structural , js_name = getActiveAttrib ) ]
    #[doc = "The `getActiveAttrib()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getActiveAttrib)\n\n*This API requires the following crate features to be activated: `WebGlActiveInfo`, `WebGlProgram`, `WebGlRenderingContext`*"]
    pub fn get_active_attrib(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
        index: u32,
    ) -> Option<WebGlActiveInfo>;
    #[cfg(all(feature = "WebGlActiveInfo", feature = "WebGlProgram",))]
    # [ wasm_bindgen ( method , structural , js_name = getActiveUniform ) ]
    #[doc = "The `getActiveUniform()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getActiveUniform)\n\n*This API requires the following crate features to be activated: `WebGlActiveInfo`, `WebGlProgram`, `WebGlRenderingContext`*"]
    pub fn get_active_uniform(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
        index: u32,
    ) -> Option<WebGlActiveInfo>;
    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_name = getAttachedShaders ) ]
    #[doc = "The `getAttachedShaders()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getAttachedShaders)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    pub fn get_attached_shaders(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
    ) -> Option<::js_sys::Array>;
    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_name = getAttribLocation ) ]
    #[doc = "The `getAttribLocation()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getAttribLocation)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    pub fn get_attrib_location(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
        name: &str,
    ) -> i32;
    # [ wasm_bindgen ( method , structural , js_name = getBufferParameter ) ]
    #[doc = "The `getBufferParameter()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getBufferParameter)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn get_buffer_parameter(
        this: &WebGlRenderingContext,
        target: u32,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;
    #[cfg(feature = "WebGlContextAttributes")]
    # [ wasm_bindgen ( method , structural , js_name = getContextAttributes ) ]
    #[doc = "The `getContextAttributes()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getContextAttributes)\n\n*This API requires the following crate features to be activated: `WebGlContextAttributes`, `WebGlRenderingContext`*"]
    pub fn get_context_attributes(this: &WebGlRenderingContext) -> Option<WebGlContextAttributes>;
    # [ wasm_bindgen ( method , structural , js_name = getError ) ]
    #[doc = "The `getError()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getError)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn get_error(this: &WebGlRenderingContext) -> u32;
    # [ wasm_bindgen ( catch , method , structural , js_name = getExtension ) ]
    #[doc = "The `getExtension()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getExtension)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn get_extension(
        this: &WebGlRenderingContext,
        name: &str,
    ) -> Result<Option<::js_sys::Object>, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = getFramebufferAttachmentParameter ) ]
    #[doc = "The `getFramebufferAttachmentParameter()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getFramebufferAttachmentParameter)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn get_framebuffer_attachment_parameter(
        this: &WebGlRenderingContext,
        target: u32,
        attachment: u32,
        pname: u32,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = getParameter ) ]
    #[doc = "The `getParameter()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getParameter)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn get_parameter(
        this: &WebGlRenderingContext,
        pname: u32,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_name = getProgramInfoLog ) ]
    #[doc = "The `getProgramInfoLog()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getProgramInfoLog)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    pub fn get_program_info_log(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
    ) -> Option<String>;
    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_name = getProgramParameter ) ]
    #[doc = "The `getProgramParameter()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getProgramParameter)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    pub fn get_program_parameter(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;
    # [ wasm_bindgen ( method , structural , js_name = getRenderbufferParameter ) ]
    #[doc = "The `getRenderbufferParameter()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getRenderbufferParameter)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn get_renderbuffer_parameter(
        this: &WebGlRenderingContext,
        target: u32,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;
    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_name = getShaderInfoLog ) ]
    #[doc = "The `getShaderInfoLog()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getShaderInfoLog)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
    pub fn get_shader_info_log(
        this: &WebGlRenderingContext,
        shader: &WebGlShader,
    ) -> Option<String>;
    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_name = getShaderParameter ) ]
    #[doc = "The `getShaderParameter()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getShaderParameter)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
    pub fn get_shader_parameter(
        this: &WebGlRenderingContext,
        shader: &WebGlShader,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;
    #[cfg(feature = "WebGlShaderPrecisionFormat")]
    # [ wasm_bindgen ( method , structural , js_name = getShaderPrecisionFormat ) ]
    #[doc = "The `getShaderPrecisionFormat()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getShaderPrecisionFormat)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShaderPrecisionFormat`*"]
    pub fn get_shader_precision_format(
        this: &WebGlRenderingContext,
        shadertype: u32,
        precisiontype: u32,
    ) -> Option<WebGlShaderPrecisionFormat>;
    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_name = getShaderSource ) ]
    #[doc = "The `getShaderSource()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getShaderSource)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
    pub fn get_shader_source(this: &WebGlRenderingContext, shader: &WebGlShader) -> Option<String>;
    # [ wasm_bindgen ( method , structural , js_name = getSupportedExtensions ) ]
    #[doc = "The `getSupportedExtensions()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getSupportedExtensions)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn get_supported_extensions(this: &WebGlRenderingContext) -> Option<::js_sys::Array>;
    # [ wasm_bindgen ( method , structural , js_name = getTexParameter ) ]
    #[doc = "The `getTexParameter()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getTexParameter)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn get_tex_parameter(
        this: &WebGlRenderingContext,
        target: u32,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlUniformLocation",))]
    # [ wasm_bindgen ( method , structural , js_name = getUniform ) ]
    #[doc = "The `getUniform()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getUniform)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn get_uniform(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
        location: &WebGlUniformLocation,
    ) -> ::wasm_bindgen::JsValue;
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlUniformLocation",))]
    # [ wasm_bindgen ( method , structural , js_name = getUniformLocation ) ]
    #[doc = "The `getUniformLocation()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getUniformLocation)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn get_uniform_location(
        this: &WebGlRenderingContext,
        program: &WebGlProgram,
        name: &str,
    ) -> Option<WebGlUniformLocation>;
    # [ wasm_bindgen ( catch , method , structural , js_name = getVertexAttrib ) ]
    #[doc = "The `getVertexAttrib()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getVertexAttrib)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn get_vertex_attrib(
        this: &WebGlRenderingContext,
        index: u32,
        pname: u32,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = getVertexAttribOffset ) ]
    #[doc = "The `getVertexAttribOffset()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getVertexAttribOffset)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn get_vertex_attrib_offset(this: &WebGlRenderingContext, index: u32, pname: u32) -> f64;
    # [ wasm_bindgen ( method , structural , js_name = hint ) ]
    #[doc = "The `hint()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/hint)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn hint(this: &WebGlRenderingContext, target: u32, mode: u32);
    #[cfg(feature = "WebGlBuffer")]
    # [ wasm_bindgen ( method , structural , js_name = isBuffer ) ]
    #[doc = "The `isBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isBuffer)\n\n*This API requires the following crate features to be activated: `WebGlBuffer`, `WebGlRenderingContext`*"]
    pub fn is_buffer(this: &WebGlRenderingContext, buffer: Option<&WebGlBuffer>) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = isContextLost ) ]
    #[doc = "The `isContextLost()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isContextLost)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn is_context_lost(this: &WebGlRenderingContext) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = isEnabled ) ]
    #[doc = "The `isEnabled()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isEnabled)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn is_enabled(this: &WebGlRenderingContext, cap: u32) -> bool;
    #[cfg(feature = "WebGlFramebuffer")]
    # [ wasm_bindgen ( method , structural , js_name = isFramebuffer ) ]
    #[doc = "The `isFramebuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isFramebuffer)\n\n*This API requires the following crate features to be activated: `WebGlFramebuffer`, `WebGlRenderingContext`*"]
    pub fn is_framebuffer(
        this: &WebGlRenderingContext,
        framebuffer: Option<&WebGlFramebuffer>,
    ) -> bool;
    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_name = isProgram ) ]
    #[doc = "The `isProgram()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isProgram)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    pub fn is_program(this: &WebGlRenderingContext, program: Option<&WebGlProgram>) -> bool;
    #[cfg(feature = "WebGlRenderbuffer")]
    # [ wasm_bindgen ( method , structural , js_name = isRenderbuffer ) ]
    #[doc = "The `isRenderbuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isRenderbuffer)\n\n*This API requires the following crate features to be activated: `WebGlRenderbuffer`, `WebGlRenderingContext`*"]
    pub fn is_renderbuffer(
        this: &WebGlRenderingContext,
        renderbuffer: Option<&WebGlRenderbuffer>,
    ) -> bool;
    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_name = isShader ) ]
    #[doc = "The `isShader()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isShader)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
    pub fn is_shader(this: &WebGlRenderingContext, shader: Option<&WebGlShader>) -> bool;
    #[cfg(feature = "WebGlTexture")]
    # [ wasm_bindgen ( method , structural , js_name = isTexture ) ]
    #[doc = "The `isTexture()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isTexture)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlTexture`*"]
    pub fn is_texture(this: &WebGlRenderingContext, texture: Option<&WebGlTexture>) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = lineWidth ) ]
    #[doc = "The `lineWidth()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/lineWidth)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn line_width(this: &WebGlRenderingContext, width: f32);
    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_name = linkProgram ) ]
    #[doc = "The `linkProgram()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/linkProgram)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    pub fn link_program(this: &WebGlRenderingContext, program: &WebGlProgram);
    # [ wasm_bindgen ( method , structural , js_name = pixelStorei ) ]
    #[doc = "The `pixelStorei()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/pixelStorei)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn pixel_storei(this: &WebGlRenderingContext, pname: u32, param: i32);
    # [ wasm_bindgen ( method , structural , js_name = polygonOffset ) ]
    #[doc = "The `polygonOffset()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/polygonOffset)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn polygon_offset(this: &WebGlRenderingContext, factor: f32, units: f32);
    # [ wasm_bindgen ( method , structural , js_name = renderbufferStorage ) ]
    #[doc = "The `renderbufferStorage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/renderbufferStorage)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn renderbuffer_storage(
        this: &WebGlRenderingContext,
        target: u32,
        internalformat: u32,
        width: i32,
        height: i32,
    );
    # [ wasm_bindgen ( method , structural , js_name = sampleCoverage ) ]
    #[doc = "The `sampleCoverage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/sampleCoverage)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn sample_coverage(this: &WebGlRenderingContext, value: f32, invert: bool);
    # [ wasm_bindgen ( method , structural , js_name = scissor ) ]
    #[doc = "The `scissor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/scissor)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn scissor(this: &WebGlRenderingContext, x: i32, y: i32, width: i32, height: i32);
    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_name = shaderSource ) ]
    #[doc = "The `shaderSource()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/shaderSource)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
    pub fn shader_source(this: &WebGlRenderingContext, shader: &WebGlShader, source: &str);
    # [ wasm_bindgen ( method , structural , js_name = stencilFunc ) ]
    #[doc = "The `stencilFunc()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilFunc)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn stencil_func(this: &WebGlRenderingContext, func: u32, ref_: i32, mask: u32);
    # [ wasm_bindgen ( method , structural , js_name = stencilFuncSeparate ) ]
    #[doc = "The `stencilFuncSeparate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilFuncSeparate)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn stencil_func_separate(
        this: &WebGlRenderingContext,
        face: u32,
        func: u32,
        ref_: i32,
        mask: u32,
    );
    # [ wasm_bindgen ( method , structural , js_name = stencilMask ) ]
    #[doc = "The `stencilMask()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilMask)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn stencil_mask(this: &WebGlRenderingContext, mask: u32);
    # [ wasm_bindgen ( method , structural , js_name = stencilMaskSeparate ) ]
    #[doc = "The `stencilMaskSeparate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilMaskSeparate)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn stencil_mask_separate(this: &WebGlRenderingContext, face: u32, mask: u32);
    # [ wasm_bindgen ( method , structural , js_name = stencilOp ) ]
    #[doc = "The `stencilOp()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilOp)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn stencil_op(this: &WebGlRenderingContext, fail: u32, zfail: u32, zpass: u32);
    # [ wasm_bindgen ( method , structural , js_name = stencilOpSeparate ) ]
    #[doc = "The `stencilOpSeparate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilOpSeparate)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn stencil_op_separate(
        this: &WebGlRenderingContext,
        face: u32,
        fail: u32,
        zfail: u32,
        zpass: u32,
    );
    # [ wasm_bindgen ( method , structural , js_name = texParameterf ) ]
    #[doc = "The `texParameterf()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texParameterf)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn tex_parameterf(this: &WebGlRenderingContext, target: u32, pname: u32, param: f32);
    # [ wasm_bindgen ( method , structural , js_name = texParameteri ) ]
    #[doc = "The `texParameteri()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texParameteri)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn tex_parameteri(this: &WebGlRenderingContext, target: u32, pname: u32, param: i32);
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform1f ) ]
    #[doc = "The `uniform1f()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1f)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1f(this: &WebGlRenderingContext, location: Option<&WebGlUniformLocation>, x: f32);
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform1i ) ]
    #[doc = "The `uniform1i()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1i)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1i(this: &WebGlRenderingContext, location: Option<&WebGlUniformLocation>, x: i32);
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform2f ) ]
    #[doc = "The `uniform2f()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2f)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2f(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: f32,
        y: f32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform2i ) ]
    #[doc = "The `uniform2i()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2i)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2i(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: i32,
        y: i32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform3f ) ]
    #[doc = "The `uniform3f()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3f)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3f(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: f32,
        y: f32,
        z: f32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform3i ) ]
    #[doc = "The `uniform3i()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3i)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3i(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: i32,
        y: i32,
        z: i32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform4f ) ]
    #[doc = "The `uniform4f()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4f)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4f(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_name = uniform4i ) ]
    #[doc = "The `uniform4i()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4i)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4i(
        this: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: i32,
        y: i32,
        z: i32,
        w: i32,
    );
    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_name = useProgram ) ]
    #[doc = "The `useProgram()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/useProgram)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    pub fn use_program(this: &WebGlRenderingContext, program: Option<&WebGlProgram>);
    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_name = validateProgram ) ]
    #[doc = "The `validateProgram()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/validateProgram)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    pub fn validate_program(this: &WebGlRenderingContext, program: &WebGlProgram);
    # [ wasm_bindgen ( method , structural , js_name = vertexAttrib1f ) ]
    #[doc = "The `vertexAttrib1f()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib1f)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn vertex_attrib1f(this: &WebGlRenderingContext, indx: u32, x: f32);
    # [ wasm_bindgen ( method , structural , js_name = vertexAttrib1fv ) ]
    #[doc = "The `vertexAttrib1fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib1fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn vertex_attrib1fv_with_f32_array(this: &WebGlRenderingContext, indx: u32, values: &[f32]);
    # [ wasm_bindgen ( method , structural , js_name = vertexAttrib1fv ) ]
    #[doc = "The `vertexAttrib1fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib1fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn vertex_attrib1fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        indx: u32,
        values: &::wasm_bindgen::JsValue,
    );
    # [ wasm_bindgen ( method , structural , js_name = vertexAttrib2f ) ]
    #[doc = "The `vertexAttrib2f()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib2f)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn vertex_attrib2f(this: &WebGlRenderingContext, indx: u32, x: f32, y: f32);
    # [ wasm_bindgen ( method , structural , js_name = vertexAttrib2fv ) ]
    #[doc = "The `vertexAttrib2fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib2fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn vertex_attrib2fv_with_f32_array(this: &WebGlRenderingContext, indx: u32, values: &[f32]);
    # [ wasm_bindgen ( method , structural , js_name = vertexAttrib2fv ) ]
    #[doc = "The `vertexAttrib2fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib2fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn vertex_attrib2fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        indx: u32,
        values: &::wasm_bindgen::JsValue,
    );
    # [ wasm_bindgen ( method , structural , js_name = vertexAttrib3f ) ]
    #[doc = "The `vertexAttrib3f()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib3f)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn vertex_attrib3f(this: &WebGlRenderingContext, indx: u32, x: f32, y: f32, z: f32);
    # [ wasm_bindgen ( method , structural , js_name = vertexAttrib3fv ) ]
    #[doc = "The `vertexAttrib3fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib3fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn vertex_attrib3fv_with_f32_array(this: &WebGlRenderingContext, indx: u32, values: &[f32]);
    # [ wasm_bindgen ( method , structural , js_name = vertexAttrib3fv ) ]
    #[doc = "The `vertexAttrib3fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib3fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn vertex_attrib3fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        indx: u32,
        values: &::wasm_bindgen::JsValue,
    );
    # [ wasm_bindgen ( method , structural , js_name = vertexAttrib4f ) ]
    #[doc = "The `vertexAttrib4f()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib4f)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn vertex_attrib4f(this: &WebGlRenderingContext, indx: u32, x: f32, y: f32, z: f32, w: f32);
    # [ wasm_bindgen ( method , structural , js_name = vertexAttrib4fv ) ]
    #[doc = "The `vertexAttrib4fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib4fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn vertex_attrib4fv_with_f32_array(this: &WebGlRenderingContext, indx: u32, values: &[f32]);
    # [ wasm_bindgen ( method , structural , js_name = vertexAttrib4fv ) ]
    #[doc = "The `vertexAttrib4fv()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib4fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn vertex_attrib4fv_with_f32_sequence(
        this: &WebGlRenderingContext,
        indx: u32,
        values: &::wasm_bindgen::JsValue,
    );
    # [ wasm_bindgen ( method , structural , js_name = vertexAttribPointer ) ]
    #[doc = "The `vertexAttribPointer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttribPointer)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn vertex_attrib_pointer_with_i32(
        this: &WebGlRenderingContext,
        indx: u32,
        size: i32,
        type_: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    );
    # [ wasm_bindgen ( method , structural , js_name = vertexAttribPointer ) ]
    #[doc = "The `vertexAttribPointer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttribPointer)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn vertex_attrib_pointer_with_f64(
        this: &WebGlRenderingContext,
        indx: u32,
        size: i32,
        type_: u32,
        normalized: bool,
        stride: i32,
        offset: f64,
    );
    # [ wasm_bindgen ( method , structural , js_name = viewport ) ]
    #[doc = "The `viewport()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/viewport)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub fn viewport(this: &WebGlRenderingContext, x: i32, y: i32, width: i32, height: i32);
}
impl WebGlRenderingContext {
    pub const DEPTH_BUFFER_BIT: u32 = 256u64 as u32;
    pub const STENCIL_BUFFER_BIT: u32 = 1024u64 as u32;
    pub const COLOR_BUFFER_BIT: u32 = 16384u64 as u32;
    pub const POINTS: u32 = 0u64 as u32;
    pub const LINES: u32 = 1u64 as u32;
    pub const LINE_LOOP: u32 = 2u64 as u32;
    pub const LINE_STRIP: u32 = 3u64 as u32;
    pub const TRIANGLES: u32 = 4u64 as u32;
    pub const TRIANGLE_STRIP: u32 = 5u64 as u32;
    pub const TRIANGLE_FAN: u32 = 6u64 as u32;
    pub const ZERO: u32 = 0i64 as u32;
    pub const ONE: u32 = 1u64 as u32;
    pub const SRC_COLOR: u32 = 768u64 as u32;
    pub const ONE_MINUS_SRC_COLOR: u32 = 769u64 as u32;
    pub const SRC_ALPHA: u32 = 770u64 as u32;
    pub const ONE_MINUS_SRC_ALPHA: u32 = 771u64 as u32;
    pub const DST_ALPHA: u32 = 772u64 as u32;
    pub const ONE_MINUS_DST_ALPHA: u32 = 773u64 as u32;
    pub const DST_COLOR: u32 = 774u64 as u32;
    pub const ONE_MINUS_DST_COLOR: u32 = 775u64 as u32;
    pub const SRC_ALPHA_SATURATE: u32 = 776u64 as u32;
    pub const FUNC_ADD: u32 = 32774u64 as u32;
    pub const BLEND_EQUATION: u32 = 32777u64 as u32;
    pub const BLEND_EQUATION_RGB: u32 = 32777u64 as u32;
    pub const BLEND_EQUATION_ALPHA: u32 = 34877u64 as u32;
    pub const FUNC_SUBTRACT: u32 = 32778u64 as u32;
    pub const FUNC_REVERSE_SUBTRACT: u32 = 32779u64 as u32;
    pub const BLEND_DST_RGB: u32 = 32968u64 as u32;
    pub const BLEND_SRC_RGB: u32 = 32969u64 as u32;
    pub const BLEND_DST_ALPHA: u32 = 32970u64 as u32;
    pub const BLEND_SRC_ALPHA: u32 = 32971u64 as u32;
    pub const CONSTANT_COLOR: u32 = 32769u64 as u32;
    pub const ONE_MINUS_CONSTANT_COLOR: u32 = 32770u64 as u32;
    pub const CONSTANT_ALPHA: u32 = 32771u64 as u32;
    pub const ONE_MINUS_CONSTANT_ALPHA: u32 = 32772u64 as u32;
    pub const BLEND_COLOR: u32 = 32773u64 as u32;
    pub const ARRAY_BUFFER: u32 = 34962u64 as u32;
    pub const ELEMENT_ARRAY_BUFFER: u32 = 34963u64 as u32;
    pub const ARRAY_BUFFER_BINDING: u32 = 34964u64 as u32;
    pub const ELEMENT_ARRAY_BUFFER_BINDING: u32 = 34965u64 as u32;
    pub const STREAM_DRAW: u32 = 35040u64 as u32;
    pub const STATIC_DRAW: u32 = 35044u64 as u32;
    pub const DYNAMIC_DRAW: u32 = 35048u64 as u32;
    pub const BUFFER_SIZE: u32 = 34660u64 as u32;
    pub const BUFFER_USAGE: u32 = 34661u64 as u32;
    pub const CURRENT_VERTEX_ATTRIB: u32 = 34342u64 as u32;
    pub const FRONT: u32 = 1028u64 as u32;
    pub const BACK: u32 = 1029u64 as u32;
    pub const FRONT_AND_BACK: u32 = 1032u64 as u32;
    pub const CULL_FACE: u32 = 2884u64 as u32;
    pub const BLEND: u32 = 3042u64 as u32;
    pub const DITHER: u32 = 3024u64 as u32;
    pub const STENCIL_TEST: u32 = 2960u64 as u32;
    pub const DEPTH_TEST: u32 = 2929u64 as u32;
    pub const SCISSOR_TEST: u32 = 3089u64 as u32;
    pub const POLYGON_OFFSET_FILL: u32 = 32823u64 as u32;
    pub const SAMPLE_ALPHA_TO_COVERAGE: u32 = 32926u64 as u32;
    pub const SAMPLE_COVERAGE: u32 = 32928u64 as u32;
    pub const NO_ERROR: u32 = 0i64 as u32;
    pub const INVALID_ENUM: u32 = 1280u64 as u32;
    pub const INVALID_VALUE: u32 = 1281u64 as u32;
    pub const INVALID_OPERATION: u32 = 1282u64 as u32;
    pub const OUT_OF_MEMORY: u32 = 1285u64 as u32;
    pub const CW: u32 = 2304u64 as u32;
    pub const CCW: u32 = 2305u64 as u32;
    pub const LINE_WIDTH: u32 = 2849u64 as u32;
    pub const ALIASED_POINT_SIZE_RANGE: u32 = 33901u64 as u32;
    pub const ALIASED_LINE_WIDTH_RANGE: u32 = 33902u64 as u32;
    pub const CULL_FACE_MODE: u32 = 2885u64 as u32;
    pub const FRONT_FACE: u32 = 2886u64 as u32;
    pub const DEPTH_RANGE: u32 = 2928u64 as u32;
    pub const DEPTH_WRITEMASK: u32 = 2930u64 as u32;
    pub const DEPTH_CLEAR_VALUE: u32 = 2931u64 as u32;
    pub const DEPTH_FUNC: u32 = 2932u64 as u32;
    pub const STENCIL_CLEAR_VALUE: u32 = 2961u64 as u32;
    pub const STENCIL_FUNC: u32 = 2962u64 as u32;
    pub const STENCIL_FAIL: u32 = 2964u64 as u32;
    pub const STENCIL_PASS_DEPTH_FAIL: u32 = 2965u64 as u32;
    pub const STENCIL_PASS_DEPTH_PASS: u32 = 2966u64 as u32;
    pub const STENCIL_REF: u32 = 2967u64 as u32;
    pub const STENCIL_VALUE_MASK: u32 = 2963u64 as u32;
    pub const STENCIL_WRITEMASK: u32 = 2968u64 as u32;
    pub const STENCIL_BACK_FUNC: u32 = 34816u64 as u32;
    pub const STENCIL_BACK_FAIL: u32 = 34817u64 as u32;
    pub const STENCIL_BACK_PASS_DEPTH_FAIL: u32 = 34818u64 as u32;
    pub const STENCIL_BACK_PASS_DEPTH_PASS: u32 = 34819u64 as u32;
    pub const STENCIL_BACK_REF: u32 = 36003u64 as u32;
    pub const STENCIL_BACK_VALUE_MASK: u32 = 36004u64 as u32;
    pub const STENCIL_BACK_WRITEMASK: u32 = 36005u64 as u32;
    pub const VIEWPORT: u32 = 2978u64 as u32;
    pub const SCISSOR_BOX: u32 = 3088u64 as u32;
    pub const COLOR_CLEAR_VALUE: u32 = 3106u64 as u32;
    pub const COLOR_WRITEMASK: u32 = 3107u64 as u32;
    pub const UNPACK_ALIGNMENT: u32 = 3317u64 as u32;
    pub const PACK_ALIGNMENT: u32 = 3333u64 as u32;
    pub const MAX_TEXTURE_SIZE: u32 = 3379u64 as u32;
    pub const MAX_VIEWPORT_DIMS: u32 = 3386u64 as u32;
    pub const SUBPIXEL_BITS: u32 = 3408u64 as u32;
    pub const RED_BITS: u32 = 3410u64 as u32;
    pub const GREEN_BITS: u32 = 3411u64 as u32;
    pub const BLUE_BITS: u32 = 3412u64 as u32;
    pub const ALPHA_BITS: u32 = 3413u64 as u32;
    pub const DEPTH_BITS: u32 = 3414u64 as u32;
    pub const STENCIL_BITS: u32 = 3415u64 as u32;
    pub const POLYGON_OFFSET_UNITS: u32 = 10752u64 as u32;
    pub const POLYGON_OFFSET_FACTOR: u32 = 32824u64 as u32;
    pub const TEXTURE_BINDING_2D: u32 = 32873u64 as u32;
    pub const SAMPLE_BUFFERS: u32 = 32936u64 as u32;
    pub const SAMPLES: u32 = 32937u64 as u32;
    pub const SAMPLE_COVERAGE_VALUE: u32 = 32938u64 as u32;
    pub const SAMPLE_COVERAGE_INVERT: u32 = 32939u64 as u32;
    pub const COMPRESSED_TEXTURE_FORMATS: u32 = 34467u64 as u32;
    pub const DONT_CARE: u32 = 4352u64 as u32;
    pub const FASTEST: u32 = 4353u64 as u32;
    pub const NICEST: u32 = 4354u64 as u32;
    pub const GENERATE_MIPMAP_HINT: u32 = 33170u64 as u32;
    pub const BYTE: u32 = 5120u64 as u32;
    pub const UNSIGNED_BYTE: u32 = 5121u64 as u32;
    pub const SHORT: u32 = 5122u64 as u32;
    pub const UNSIGNED_SHORT: u32 = 5123u64 as u32;
    pub const INT: u32 = 5124u64 as u32;
    pub const UNSIGNED_INT: u32 = 5125u64 as u32;
    pub const FLOAT: u32 = 5126u64 as u32;
    pub const DEPTH_COMPONENT: u32 = 6402u64 as u32;
    pub const ALPHA: u32 = 6406u64 as u32;
    pub const RGB: u32 = 6407u64 as u32;
    pub const RGBA: u32 = 6408u64 as u32;
    pub const LUMINANCE: u32 = 6409u64 as u32;
    pub const LUMINANCE_ALPHA: u32 = 6410u64 as u32;
    pub const UNSIGNED_SHORT_4_4_4_4: u32 = 32819u64 as u32;
    pub const UNSIGNED_SHORT_5_5_5_1: u32 = 32820u64 as u32;
    pub const UNSIGNED_SHORT_5_6_5: u32 = 33635u64 as u32;
    pub const FRAGMENT_SHADER: u32 = 35632u64 as u32;
    pub const VERTEX_SHADER: u32 = 35633u64 as u32;
    pub const MAX_VERTEX_ATTRIBS: u32 = 34921u64 as u32;
    pub const MAX_VERTEX_UNIFORM_VECTORS: u32 = 36347u64 as u32;
    pub const MAX_VARYING_VECTORS: u32 = 36348u64 as u32;
    pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: u32 = 35661u64 as u32;
    pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: u32 = 35660u64 as u32;
    pub const MAX_TEXTURE_IMAGE_UNITS: u32 = 34930u64 as u32;
    pub const MAX_FRAGMENT_UNIFORM_VECTORS: u32 = 36349u64 as u32;
    pub const SHADER_TYPE: u32 = 35663u64 as u32;
    pub const DELETE_STATUS: u32 = 35712u64 as u32;
    pub const LINK_STATUS: u32 = 35714u64 as u32;
    pub const VALIDATE_STATUS: u32 = 35715u64 as u32;
    pub const ATTACHED_SHADERS: u32 = 35717u64 as u32;
    pub const ACTIVE_UNIFORMS: u32 = 35718u64 as u32;
    pub const ACTIVE_ATTRIBUTES: u32 = 35721u64 as u32;
    pub const SHADING_LANGUAGE_VERSION: u32 = 35724u64 as u32;
    pub const CURRENT_PROGRAM: u32 = 35725u64 as u32;
    pub const NEVER: u32 = 512u64 as u32;
    pub const LESS: u32 = 513u64 as u32;
    pub const EQUAL: u32 = 514u64 as u32;
    pub const LEQUAL: u32 = 515u64 as u32;
    pub const GREATER: u32 = 516u64 as u32;
    pub const NOTEQUAL: u32 = 517u64 as u32;
    pub const GEQUAL: u32 = 518u64 as u32;
    pub const ALWAYS: u32 = 519u64 as u32;
    pub const KEEP: u32 = 7680u64 as u32;
    pub const REPLACE: u32 = 7681u64 as u32;
    pub const INCR: u32 = 7682u64 as u32;
    pub const DECR: u32 = 7683u64 as u32;
    pub const INVERT: u32 = 5386u64 as u32;
    pub const INCR_WRAP: u32 = 34055u64 as u32;
    pub const DECR_WRAP: u32 = 34056u64 as u32;
    pub const VENDOR: u32 = 7936u64 as u32;
    pub const RENDERER: u32 = 7937u64 as u32;
    pub const VERSION: u32 = 7938u64 as u32;
    pub const NEAREST: u32 = 9728u64 as u32;
    pub const LINEAR: u32 = 9729u64 as u32;
    pub const NEAREST_MIPMAP_NEAREST: u32 = 9984u64 as u32;
    pub const LINEAR_MIPMAP_NEAREST: u32 = 9985u64 as u32;
    pub const NEAREST_MIPMAP_LINEAR: u32 = 9986u64 as u32;
    pub const LINEAR_MIPMAP_LINEAR: u32 = 9987u64 as u32;
    pub const TEXTURE_MAG_FILTER: u32 = 10240u64 as u32;
    pub const TEXTURE_MIN_FILTER: u32 = 10241u64 as u32;
    pub const TEXTURE_WRAP_S: u32 = 10242u64 as u32;
    pub const TEXTURE_WRAP_T: u32 = 10243u64 as u32;
    pub const TEXTURE_2D: u32 = 3553u64 as u32;
    pub const TEXTURE: u32 = 5890u64 as u32;
    pub const TEXTURE_CUBE_MAP: u32 = 34067u64 as u32;
    pub const TEXTURE_BINDING_CUBE_MAP: u32 = 34068u64 as u32;
    pub const TEXTURE_CUBE_MAP_POSITIVE_X: u32 = 34069u64 as u32;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_X: u32 = 34070u64 as u32;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Y: u32 = 34071u64 as u32;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: u32 = 34072u64 as u32;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Z: u32 = 34073u64 as u32;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: u32 = 34074u64 as u32;
    pub const MAX_CUBE_MAP_TEXTURE_SIZE: u32 = 34076u64 as u32;
    pub const TEXTURE0: u32 = 33984u64 as u32;
    pub const TEXTURE1: u32 = 33985u64 as u32;
    pub const TEXTURE2: u32 = 33986u64 as u32;
    pub const TEXTURE3: u32 = 33987u64 as u32;
    pub const TEXTURE4: u32 = 33988u64 as u32;
    pub const TEXTURE5: u32 = 33989u64 as u32;
    pub const TEXTURE6: u32 = 33990u64 as u32;
    pub const TEXTURE7: u32 = 33991u64 as u32;
    pub const TEXTURE8: u32 = 33992u64 as u32;
    pub const TEXTURE9: u32 = 33993u64 as u32;
    pub const TEXTURE10: u32 = 33994u64 as u32;
    pub const TEXTURE11: u32 = 33995u64 as u32;
    pub const TEXTURE12: u32 = 33996u64 as u32;
    pub const TEXTURE13: u32 = 33997u64 as u32;
    pub const TEXTURE14: u32 = 33998u64 as u32;
    pub const TEXTURE15: u32 = 33999u64 as u32;
    pub const TEXTURE16: u32 = 34000u64 as u32;
    pub const TEXTURE17: u32 = 34001u64 as u32;
    pub const TEXTURE18: u32 = 34002u64 as u32;
    pub const TEXTURE19: u32 = 34003u64 as u32;
    pub const TEXTURE20: u32 = 34004u64 as u32;
    pub const TEXTURE21: u32 = 34005u64 as u32;
    pub const TEXTURE22: u32 = 34006u64 as u32;
    pub const TEXTURE23: u32 = 34007u64 as u32;
    pub const TEXTURE24: u32 = 34008u64 as u32;
    pub const TEXTURE25: u32 = 34009u64 as u32;
    pub const TEXTURE26: u32 = 34010u64 as u32;
    pub const TEXTURE27: u32 = 34011u64 as u32;
    pub const TEXTURE28: u32 = 34012u64 as u32;
    pub const TEXTURE29: u32 = 34013u64 as u32;
    pub const TEXTURE30: u32 = 34014u64 as u32;
    pub const TEXTURE31: u32 = 34015u64 as u32;
    pub const ACTIVE_TEXTURE: u32 = 34016u64 as u32;
    pub const REPEAT: u32 = 10497u64 as u32;
    pub const CLAMP_TO_EDGE: u32 = 33071u64 as u32;
    pub const MIRRORED_REPEAT: u32 = 33648u64 as u32;
    pub const FLOAT_VEC2: u32 = 35664u64 as u32;
    pub const FLOAT_VEC3: u32 = 35665u64 as u32;
    pub const FLOAT_VEC4: u32 = 35666u64 as u32;
    pub const INT_VEC2: u32 = 35667u64 as u32;
    pub const INT_VEC3: u32 = 35668u64 as u32;
    pub const INT_VEC4: u32 = 35669u64 as u32;
    pub const BOOL: u32 = 35670u64 as u32;
    pub const BOOL_VEC2: u32 = 35671u64 as u32;
    pub const BOOL_VEC3: u32 = 35672u64 as u32;
    pub const BOOL_VEC4: u32 = 35673u64 as u32;
    pub const FLOAT_MAT2: u32 = 35674u64 as u32;
    pub const FLOAT_MAT3: u32 = 35675u64 as u32;
    pub const FLOAT_MAT4: u32 = 35676u64 as u32;
    pub const SAMPLER_2D: u32 = 35678u64 as u32;
    pub const SAMPLER_CUBE: u32 = 35680u64 as u32;
    pub const VERTEX_ATTRIB_ARRAY_ENABLED: u32 = 34338u64 as u32;
    pub const VERTEX_ATTRIB_ARRAY_SIZE: u32 = 34339u64 as u32;
    pub const VERTEX_ATTRIB_ARRAY_STRIDE: u32 = 34340u64 as u32;
    pub const VERTEX_ATTRIB_ARRAY_TYPE: u32 = 34341u64 as u32;
    pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: u32 = 34922u64 as u32;
    pub const VERTEX_ATTRIB_ARRAY_POINTER: u32 = 34373u64 as u32;
    pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: u32 = 34975u64 as u32;
    pub const IMPLEMENTATION_COLOR_READ_TYPE: u32 = 35738u64 as u32;
    pub const IMPLEMENTATION_COLOR_READ_FORMAT: u32 = 35739u64 as u32;
    pub const COMPILE_STATUS: u32 = 35713u64 as u32;
    pub const LOW_FLOAT: u32 = 36336u64 as u32;
    pub const MEDIUM_FLOAT: u32 = 36337u64 as u32;
    pub const HIGH_FLOAT: u32 = 36338u64 as u32;
    pub const LOW_INT: u32 = 36339u64 as u32;
    pub const MEDIUM_INT: u32 = 36340u64 as u32;
    pub const HIGH_INT: u32 = 36341u64 as u32;
    pub const FRAMEBUFFER: u32 = 36160u64 as u32;
    pub const RENDERBUFFER: u32 = 36161u64 as u32;
    pub const RGBA4: u32 = 32854u64 as u32;
    pub const RGB5_A1: u32 = 32855u64 as u32;
    pub const RGB565: u32 = 36194u64 as u32;
    pub const DEPTH_COMPONENT16: u32 = 33189u64 as u32;
    pub const STENCIL_INDEX8: u32 = 36168u64 as u32;
    pub const DEPTH_STENCIL: u32 = 34041u64 as u32;
    pub const RENDERBUFFER_WIDTH: u32 = 36162u64 as u32;
    pub const RENDERBUFFER_HEIGHT: u32 = 36163u64 as u32;
    pub const RENDERBUFFER_INTERNAL_FORMAT: u32 = 36164u64 as u32;
    pub const RENDERBUFFER_RED_SIZE: u32 = 36176u64 as u32;
    pub const RENDERBUFFER_GREEN_SIZE: u32 = 36177u64 as u32;
    pub const RENDERBUFFER_BLUE_SIZE: u32 = 36178u64 as u32;
    pub const RENDERBUFFER_ALPHA_SIZE: u32 = 36179u64 as u32;
    pub const RENDERBUFFER_DEPTH_SIZE: u32 = 36180u64 as u32;
    pub const RENDERBUFFER_STENCIL_SIZE: u32 = 36181u64 as u32;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: u32 = 36048u64 as u32;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: u32 = 36049u64 as u32;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: u32 = 36050u64 as u32;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: u32 = 36051u64 as u32;
    pub const COLOR_ATTACHMENT0: u32 = 36064u64 as u32;
    pub const DEPTH_ATTACHMENT: u32 = 36096u64 as u32;
    pub const STENCIL_ATTACHMENT: u32 = 36128u64 as u32;
    pub const DEPTH_STENCIL_ATTACHMENT: u32 = 33306u64 as u32;
    pub const NONE: u32 = 0i64 as u32;
    pub const FRAMEBUFFER_COMPLETE: u32 = 36053u64 as u32;
    pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: u32 = 36054u64 as u32;
    pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: u32 = 36055u64 as u32;
    pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: u32 = 36057u64 as u32;
    pub const FRAMEBUFFER_UNSUPPORTED: u32 = 36061u64 as u32;
    pub const FRAMEBUFFER_BINDING: u32 = 36006u64 as u32;
    pub const RENDERBUFFER_BINDING: u32 = 36007u64 as u32;
    pub const MAX_RENDERBUFFER_SIZE: u32 = 34024u64 as u32;
    pub const INVALID_FRAMEBUFFER_OPERATION: u32 = 1286u64 as u32;
    pub const UNPACK_FLIP_Y_WEBGL: u32 = 37440u64 as u32;
    pub const UNPACK_PREMULTIPLY_ALPHA_WEBGL: u32 = 37441u64 as u32;
    pub const CONTEXT_LOST_WEBGL: u32 = 37442u64 as u32;
    pub const UNPACK_COLORSPACE_CONVERSION_WEBGL: u32 = 37443u64 as u32;
    pub const BROWSER_DEFAULT_WEBGL: u32 = 37444u64 as u32;
}
