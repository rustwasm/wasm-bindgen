use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WebGL2RenderingContext , typescript_name = WebGL2RenderingContext ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebGl2RenderingContext` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub type WebGl2RenderingContext;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebGL2RenderingContext" , js_name = canvas ) ]
    ///Getter for the `canvas` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/canvas)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn canvas(this: &WebGl2RenderingContext) -> Option<::js_sys::Object>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebGL2RenderingContext" , js_name = drawingBufferWidth ) ]
    ///Getter for the `drawingBufferWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawingBufferWidth)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn drawing_buffer_width(this: &WebGl2RenderingContext) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebGL2RenderingContext" , js_name = drawingBufferHeight ) ]
    ///Getter for the `drawingBufferHeight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawingBufferHeight)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn drawing_buffer_height(this: &WebGl2RenderingContext) -> i32;

    #[cfg(feature = "WebGlQuery")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = beginQuery ) ]
    ///The `beginQuery()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/beginQuery)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlQuery`*
    pub fn begin_query(this: &WebGl2RenderingContext, target: u32, query: &WebGlQuery);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = beginTransformFeedback ) ]
    ///The `beginTransformFeedback()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/beginTransformFeedback)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn begin_transform_feedback(this: &WebGl2RenderingContext, primitive_mode: u32);

    #[cfg(feature = "WebGlBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bindBufferBase ) ]
    ///The `bindBufferBase()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindBufferBase)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlBuffer`*
    pub fn bind_buffer_base(
        this: &WebGl2RenderingContext,
        target: u32,
        index: u32,
        buffer: Option<&WebGlBuffer>,
    );

    #[cfg(feature = "WebGlBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bindBufferRange ) ]
    ///The `bindBufferRange()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindBufferRange)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlBuffer`*
    pub fn bind_buffer_range_with_i32_and_i32(
        this: &WebGl2RenderingContext,
        target: u32,
        index: u32,
        buffer: Option<&WebGlBuffer>,
        offset: i32,
        size: i32,
    );

    #[cfg(feature = "WebGlBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bindBufferRange ) ]
    ///The `bindBufferRange()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindBufferRange)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlBuffer`*
    pub fn bind_buffer_range_with_f64_and_i32(
        this: &WebGl2RenderingContext,
        target: u32,
        index: u32,
        buffer: Option<&WebGlBuffer>,
        offset: f64,
        size: i32,
    );

    #[cfg(feature = "WebGlBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bindBufferRange ) ]
    ///The `bindBufferRange()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindBufferRange)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlBuffer`*
    pub fn bind_buffer_range_with_i32_and_f64(
        this: &WebGl2RenderingContext,
        target: u32,
        index: u32,
        buffer: Option<&WebGlBuffer>,
        offset: i32,
        size: f64,
    );

    #[cfg(feature = "WebGlBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bindBufferRange ) ]
    ///The `bindBufferRange()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindBufferRange)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlBuffer`*
    pub fn bind_buffer_range_with_f64_and_f64(
        this: &WebGl2RenderingContext,
        target: u32,
        index: u32,
        buffer: Option<&WebGlBuffer>,
        offset: f64,
        size: f64,
    );

    #[cfg(feature = "WebGlSampler")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bindSampler ) ]
    ///The `bindSampler()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindSampler)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSampler`*
    pub fn bind_sampler(this: &WebGl2RenderingContext, unit: u32, sampler: Option<&WebGlSampler>);

    #[cfg(feature = "WebGlTransformFeedback")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bindTransformFeedback ) ]
    ///The `bindTransformFeedback()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindTransformFeedback)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlTransformFeedback`*
    pub fn bind_transform_feedback(
        this: &WebGl2RenderingContext,
        target: u32,
        tf: Option<&WebGlTransformFeedback>,
    );

    #[cfg(feature = "WebGlVertexArrayObject")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bindVertexArray ) ]
    ///The `bindVertexArray()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindVertexArray)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlVertexArrayObject`*
    pub fn bind_vertex_array(this: &WebGl2RenderingContext, array: Option<&WebGlVertexArrayObject>);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = blitFramebuffer ) ]
    ///The `blitFramebuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/blitFramebuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn blit_framebuffer(
        this: &WebGl2RenderingContext,
        src_x0: i32,
        src_y0: i32,
        src_x1: i32,
        src_y1: i32,
        dst_x0: i32,
        dst_y0: i32,
        dst_x1: i32,
        dst_y1: i32,
        mask: u32,
        filter: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferData ) ]
    ///The `bufferData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_data_with_i32(this: &WebGl2RenderingContext, target: u32, size: i32, usage: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferData ) ]
    ///The `bufferData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_data_with_f64(this: &WebGl2RenderingContext, target: u32, size: f64, usage: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferData ) ]
    ///The `bufferData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_data_with_opt_array_buffer(
        this: &WebGl2RenderingContext,
        target: u32,
        src_data: Option<&::js_sys::ArrayBuffer>,
        usage: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferData ) ]
    ///The `bufferData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_data_with_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        src_data: &::js_sys::Object,
        usage: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferData ) ]
    ///The `bufferData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_data_with_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        src_data: &[u8],
        usage: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferData ) ]
    ///The `bufferData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_data_with_array_buffer_view_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        src_data: &::js_sys::Object,
        usage: u32,
        src_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferData ) ]
    ///The `bufferData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_data_with_u8_array_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        src_data: &[u8],
        usage: u32,
        src_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferData ) ]
    ///The `bufferData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_data_with_array_buffer_view_and_src_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        src_data: &::js_sys::Object,
        usage: u32,
        src_offset: u32,
        length: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferData ) ]
    ///The `bufferData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_data_with_u8_array_and_src_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        src_data: &[u8],
        usage: u32,
        src_offset: u32,
        length: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferSubData ) ]
    ///The `bufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_sub_data_with_i32_and_array_buffer(
        this: &WebGl2RenderingContext,
        target: u32,
        offset: i32,
        src_data: &::js_sys::ArrayBuffer,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferSubData ) ]
    ///The `bufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_sub_data_with_f64_and_array_buffer(
        this: &WebGl2RenderingContext,
        target: u32,
        offset: f64,
        src_data: &::js_sys::ArrayBuffer,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferSubData ) ]
    ///The `bufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_sub_data_with_i32_and_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        offset: i32,
        src_data: &::js_sys::Object,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferSubData ) ]
    ///The `bufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_sub_data_with_f64_and_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        offset: f64,
        src_data: &::js_sys::Object,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferSubData ) ]
    ///The `bufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_sub_data_with_i32_and_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        offset: i32,
        src_data: &[u8],
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferSubData ) ]
    ///The `bufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_sub_data_with_f64_and_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        offset: f64,
        src_data: &[u8],
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferSubData ) ]
    ///The `bufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_sub_data_with_i32_and_array_buffer_view_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        dst_byte_offset: i32,
        src_data: &::js_sys::Object,
        src_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferSubData ) ]
    ///The `bufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_sub_data_with_f64_and_array_buffer_view_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        dst_byte_offset: f64,
        src_data: &::js_sys::Object,
        src_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferSubData ) ]
    ///The `bufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_sub_data_with_i32_and_u8_array_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        dst_byte_offset: i32,
        src_data: &[u8],
        src_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferSubData ) ]
    ///The `bufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_sub_data_with_f64_and_u8_array_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        dst_byte_offset: f64,
        src_data: &[u8],
        src_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferSubData ) ]
    ///The `bufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_sub_data_with_i32_and_array_buffer_view_and_src_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        dst_byte_offset: i32,
        src_data: &::js_sys::Object,
        src_offset: u32,
        length: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferSubData ) ]
    ///The `bufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_sub_data_with_f64_and_array_buffer_view_and_src_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        dst_byte_offset: f64,
        src_data: &::js_sys::Object,
        src_offset: u32,
        length: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferSubData ) ]
    ///The `bufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_sub_data_with_i32_and_u8_array_and_src_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        dst_byte_offset: i32,
        src_data: &[u8],
        src_offset: u32,
        length: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bufferSubData ) ]
    ///The `bufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn buffer_sub_data_with_f64_and_u8_array_and_src_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        dst_byte_offset: f64,
        src_data: &[u8],
        src_offset: u32,
        length: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = clearBufferfi ) ]
    ///The `clearBufferfi()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferfi)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn clear_bufferfi(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        depth: f32,
        stencil: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = clearBufferfv ) ]
    ///The `clearBufferfv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferfv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn clear_bufferfv_with_f32_array(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &[f32],
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = clearBufferfv ) ]
    ///The `clearBufferfv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferfv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn clear_bufferfv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &::wasm_bindgen::JsValue,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = clearBufferfv ) ]
    ///The `clearBufferfv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferfv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn clear_bufferfv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &[f32],
        src_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = clearBufferfv ) ]
    ///The `clearBufferfv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferfv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn clear_bufferfv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = clearBufferiv ) ]
    ///The `clearBufferiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn clear_bufferiv_with_i32_array(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &[i32],
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = clearBufferiv ) ]
    ///The `clearBufferiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn clear_bufferiv_with_i32_sequence(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &::wasm_bindgen::JsValue,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = clearBufferiv ) ]
    ///The `clearBufferiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn clear_bufferiv_with_i32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &[i32],
        src_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = clearBufferiv ) ]
    ///The `clearBufferiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn clear_bufferiv_with_i32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = clearBufferuiv ) ]
    ///The `clearBufferuiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferuiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn clear_bufferuiv_with_u32_array(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &[u32],
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = clearBufferuiv ) ]
    ///The `clearBufferuiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferuiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn clear_bufferuiv_with_u32_sequence(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &::wasm_bindgen::JsValue,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = clearBufferuiv ) ]
    ///The `clearBufferuiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferuiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn clear_bufferuiv_with_u32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &[u32],
        src_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = clearBufferuiv ) ]
    ///The `clearBufferuiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferuiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn clear_bufferuiv_with_u32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlSync")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = clientWaitSync ) ]
    ///The `clientWaitSync()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clientWaitSync)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSync`*
    pub fn client_wait_sync_with_u32(
        this: &WebGl2RenderingContext,
        sync: &WebGlSync,
        flags: u32,
        timeout: u32,
    ) -> u32;

    #[cfg(feature = "WebGlSync")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = clientWaitSync ) ]
    ///The `clientWaitSync()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clientWaitSync)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSync`*
    pub fn client_wait_sync_with_f64(
        this: &WebGl2RenderingContext,
        sync: &WebGlSync,
        flags: u32,
        timeout: f64,
    ) -> u32;

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexImage2D ) ]
    ///The `compressedTexImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_image_2d_with_i32_and_i32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        image_size: i32,
        offset: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexImage2D ) ]
    ///The `compressedTexImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_image_2d_with_i32_and_f64(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        image_size: i32,
        offset: f64,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexImage2D ) ]
    ///The `compressedTexImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_image_2d_with_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        src_data: &::js_sys::Object,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexImage2D ) ]
    ///The `compressedTexImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_image_2d_with_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        src_data: &[u8],
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexImage2D ) ]
    ///The `compressedTexImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_image_2d_with_array_buffer_view_and_u32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        src_data: &::js_sys::Object,
        src_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexImage2D ) ]
    ///The `compressedTexImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_image_2d_with_u8_array_and_u32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        src_data: &[u8],
        src_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexImage2D ) ]
    ///The `compressedTexImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_image_2d_with_array_buffer_view_and_u32_and_src_length_override(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        src_data: &::js_sys::Object,
        src_offset: u32,
        src_length_override: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexImage2D ) ]
    ///The `compressedTexImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_image_2d_with_u8_array_and_u32_and_src_length_override(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        src_data: &[u8],
        src_offset: u32,
        src_length_override: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexImage3D ) ]
    ///The `compressedTexImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_image_3d_with_i32_and_i32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        image_size: i32,
        offset: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexImage3D ) ]
    ///The `compressedTexImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_image_3d_with_i32_and_f64(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        image_size: i32,
        offset: f64,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexImage3D ) ]
    ///The `compressedTexImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_image_3d_with_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        src_data: &::js_sys::Object,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexImage3D ) ]
    ///The `compressedTexImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_image_3d_with_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        src_data: &[u8],
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexImage3D ) ]
    ///The `compressedTexImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_image_3d_with_array_buffer_view_and_u32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        src_data: &::js_sys::Object,
        src_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexImage3D ) ]
    ///The `compressedTexImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_image_3d_with_u8_array_and_u32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        src_data: &[u8],
        src_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexImage3D ) ]
    ///The `compressedTexImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_image_3d_with_array_buffer_view_and_u32_and_src_length_override(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        src_data: &::js_sys::Object,
        src_offset: u32,
        src_length_override: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexImage3D ) ]
    ///The `compressedTexImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_image_3d_with_u8_array_and_u32_and_src_length_override(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        src_data: &[u8],
        src_offset: u32,
        src_length_override: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexSubImage2D ) ]
    ///The `compressedTexSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_sub_image_2d_with_i32_and_i32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        image_size: i32,
        offset: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexSubImage2D ) ]
    ///The `compressedTexSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_sub_image_2d_with_i32_and_f64(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        image_size: i32,
        offset: f64,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexSubImage2D ) ]
    ///The `compressedTexSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_sub_image_2d_with_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        src_data: &::js_sys::Object,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexSubImage2D ) ]
    ///The `compressedTexSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_sub_image_2d_with_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        src_data: &mut [u8],
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexSubImage2D ) ]
    ///The `compressedTexSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_sub_image_2d_with_array_buffer_view_and_u32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        src_data: &::js_sys::Object,
        src_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexSubImage2D ) ]
    ///The `compressedTexSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_sub_image_2d_with_u8_array_and_u32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        src_data: &mut [u8],
        src_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexSubImage2D ) ]
    ///The `compressedTexSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_sub_image_2d_with_array_buffer_view_and_u32_and_src_length_override(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        src_data: &::js_sys::Object,
        src_offset: u32,
        src_length_override: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexSubImage2D ) ]
    ///The `compressedTexSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_sub_image_2d_with_u8_array_and_u32_and_src_length_override(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        src_data: &mut [u8],
        src_offset: u32,
        src_length_override: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexSubImage3D ) ]
    ///The `compressedTexSubImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_sub_image_3d_with_i32_and_i32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        image_size: i32,
        offset: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexSubImage3D ) ]
    ///The `compressedTexSubImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_sub_image_3d_with_i32_and_f64(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        image_size: i32,
        offset: f64,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexSubImage3D ) ]
    ///The `compressedTexSubImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_sub_image_3d_with_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        src_data: &::js_sys::Object,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexSubImage3D ) ]
    ///The `compressedTexSubImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_sub_image_3d_with_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        src_data: &mut [u8],
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexSubImage3D ) ]
    ///The `compressedTexSubImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_sub_image_3d_with_array_buffer_view_and_u32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        src_data: &::js_sys::Object,
        src_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexSubImage3D ) ]
    ///The `compressedTexSubImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_sub_image_3d_with_u8_array_and_u32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        src_data: &mut [u8],
        src_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexSubImage3D ) ]
    ///The `compressedTexSubImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_sub_image_3d_with_array_buffer_view_and_u32_and_src_length_override(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        src_data: &::js_sys::Object,
        src_offset: u32,
        src_length_override: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compressedTexSubImage3D ) ]
    ///The `compressedTexSubImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn compressed_tex_sub_image_3d_with_u8_array_and_u32_and_src_length_override(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        src_data: &mut [u8],
        src_offset: u32,
        src_length_override: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = copyBufferSubData ) ]
    ///The `copyBufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyBufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn copy_buffer_sub_data_with_i32_and_i32_and_i32(
        this: &WebGl2RenderingContext,
        read_target: u32,
        write_target: u32,
        read_offset: i32,
        write_offset: i32,
        size: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = copyBufferSubData ) ]
    ///The `copyBufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyBufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn copy_buffer_sub_data_with_f64_and_i32_and_i32(
        this: &WebGl2RenderingContext,
        read_target: u32,
        write_target: u32,
        read_offset: f64,
        write_offset: i32,
        size: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = copyBufferSubData ) ]
    ///The `copyBufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyBufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn copy_buffer_sub_data_with_i32_and_f64_and_i32(
        this: &WebGl2RenderingContext,
        read_target: u32,
        write_target: u32,
        read_offset: i32,
        write_offset: f64,
        size: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = copyBufferSubData ) ]
    ///The `copyBufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyBufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn copy_buffer_sub_data_with_f64_and_f64_and_i32(
        this: &WebGl2RenderingContext,
        read_target: u32,
        write_target: u32,
        read_offset: f64,
        write_offset: f64,
        size: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = copyBufferSubData ) ]
    ///The `copyBufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyBufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn copy_buffer_sub_data_with_i32_and_i32_and_f64(
        this: &WebGl2RenderingContext,
        read_target: u32,
        write_target: u32,
        read_offset: i32,
        write_offset: i32,
        size: f64,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = copyBufferSubData ) ]
    ///The `copyBufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyBufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn copy_buffer_sub_data_with_f64_and_i32_and_f64(
        this: &WebGl2RenderingContext,
        read_target: u32,
        write_target: u32,
        read_offset: f64,
        write_offset: i32,
        size: f64,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = copyBufferSubData ) ]
    ///The `copyBufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyBufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn copy_buffer_sub_data_with_i32_and_f64_and_f64(
        this: &WebGl2RenderingContext,
        read_target: u32,
        write_target: u32,
        read_offset: i32,
        write_offset: f64,
        size: f64,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = copyBufferSubData ) ]
    ///The `copyBufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyBufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn copy_buffer_sub_data_with_f64_and_f64_and_f64(
        this: &WebGl2RenderingContext,
        read_target: u32,
        write_target: u32,
        read_offset: f64,
        write_offset: f64,
        size: f64,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = copyTexSubImage3D ) ]
    ///The `copyTexSubImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyTexSubImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn copy_tex_sub_image_3d(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    );

    #[cfg(feature = "WebGlQuery")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = createQuery ) ]
    ///The `createQuery()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/createQuery)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlQuery`*
    pub fn create_query(this: &WebGl2RenderingContext) -> Option<WebGlQuery>;

    #[cfg(feature = "WebGlSampler")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = createSampler ) ]
    ///The `createSampler()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/createSampler)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSampler`*
    pub fn create_sampler(this: &WebGl2RenderingContext) -> Option<WebGlSampler>;

    #[cfg(feature = "WebGlTransformFeedback")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = createTransformFeedback ) ]
    ///The `createTransformFeedback()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/createTransformFeedback)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlTransformFeedback`*
    pub fn create_transform_feedback(
        this: &WebGl2RenderingContext,
    ) -> Option<WebGlTransformFeedback>;

    #[cfg(feature = "WebGlVertexArrayObject")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = createVertexArray ) ]
    ///The `createVertexArray()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/createVertexArray)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlVertexArrayObject`*
    pub fn create_vertex_array(this: &WebGl2RenderingContext) -> Option<WebGlVertexArrayObject>;

    #[cfg(feature = "WebGlQuery")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = deleteQuery ) ]
    ///The `deleteQuery()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteQuery)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlQuery`*
    pub fn delete_query(this: &WebGl2RenderingContext, query: Option<&WebGlQuery>);

    #[cfg(feature = "WebGlSampler")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = deleteSampler ) ]
    ///The `deleteSampler()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteSampler)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSampler`*
    pub fn delete_sampler(this: &WebGl2RenderingContext, sampler: Option<&WebGlSampler>);

    #[cfg(feature = "WebGlSync")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = deleteSync ) ]
    ///The `deleteSync()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteSync)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSync`*
    pub fn delete_sync(this: &WebGl2RenderingContext, sync: Option<&WebGlSync>);

    #[cfg(feature = "WebGlTransformFeedback")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = deleteTransformFeedback ) ]
    ///The `deleteTransformFeedback()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteTransformFeedback)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlTransformFeedback`*
    pub fn delete_transform_feedback(
        this: &WebGl2RenderingContext,
        tf: Option<&WebGlTransformFeedback>,
    );

    #[cfg(feature = "WebGlVertexArrayObject")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = deleteVertexArray ) ]
    ///The `deleteVertexArray()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteVertexArray)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlVertexArrayObject`*
    pub fn delete_vertex_array(
        this: &WebGl2RenderingContext,
        vertex_array: Option<&WebGlVertexArrayObject>,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = drawArraysInstanced ) ]
    ///The `drawArraysInstanced()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawArraysInstanced)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn draw_arrays_instanced(
        this: &WebGl2RenderingContext,
        mode: u32,
        first: i32,
        count: i32,
        instance_count: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = drawBuffers ) ]
    ///The `drawBuffers()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawBuffers)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn draw_buffers(this: &WebGl2RenderingContext, buffers: &::wasm_bindgen::JsValue);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = drawElementsInstanced ) ]
    ///The `drawElementsInstanced()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawElementsInstanced)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn draw_elements_instanced_with_i32(
        this: &WebGl2RenderingContext,
        mode: u32,
        count: i32,
        type_: u32,
        offset: i32,
        instance_count: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = drawElementsInstanced ) ]
    ///The `drawElementsInstanced()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawElementsInstanced)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn draw_elements_instanced_with_f64(
        this: &WebGl2RenderingContext,
        mode: u32,
        count: i32,
        type_: u32,
        offset: f64,
        instance_count: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = drawRangeElements ) ]
    ///The `drawRangeElements()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawRangeElements)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn draw_range_elements_with_i32(
        this: &WebGl2RenderingContext,
        mode: u32,
        start: u32,
        end: u32,
        count: i32,
        type_: u32,
        offset: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = drawRangeElements ) ]
    ///The `drawRangeElements()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawRangeElements)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn draw_range_elements_with_f64(
        this: &WebGl2RenderingContext,
        mode: u32,
        start: u32,
        end: u32,
        count: i32,
        type_: u32,
        offset: f64,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = endQuery ) ]
    ///The `endQuery()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/endQuery)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn end_query(this: &WebGl2RenderingContext, target: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = endTransformFeedback ) ]
    ///The `endTransformFeedback()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/endTransformFeedback)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn end_transform_feedback(this: &WebGl2RenderingContext);

    #[cfg(feature = "WebGlSync")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = fenceSync ) ]
    ///The `fenceSync()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/fenceSync)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSync`*
    pub fn fence_sync(
        this: &WebGl2RenderingContext,
        condition: u32,
        flags: u32,
    ) -> Option<WebGlSync>;

    #[cfg(feature = "WebGlTexture")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = framebufferTextureLayer ) ]
    ///The `framebufferTextureLayer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/framebufferTextureLayer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlTexture`*
    pub fn framebuffer_texture_layer(
        this: &WebGl2RenderingContext,
        target: u32,
        attachment: u32,
        texture: Option<&WebGlTexture>,
        level: i32,
        layer: i32,
    );

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getActiveUniformBlockName ) ]
    ///The `getActiveUniformBlockName()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getActiveUniformBlockName)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*
    pub fn get_active_uniform_block_name(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        uniform_block_index: u32,
    ) -> Option<String>;

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = getActiveUniformBlockParameter ) ]
    ///The `getActiveUniformBlockParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getActiveUniformBlockParameter)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*
    pub fn get_active_uniform_block_parameter(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        uniform_block_index: u32,
        pname: u32,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getActiveUniforms ) ]
    ///The `getActiveUniforms()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getActiveUniforms)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*
    pub fn get_active_uniforms(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        uniform_indices: &::wasm_bindgen::JsValue,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getBufferSubData ) ]
    ///The `getBufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_buffer_sub_data_with_i32_and_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: i32,
        dst_data: &::js_sys::Object,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getBufferSubData ) ]
    ///The `getBufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_buffer_sub_data_with_f64_and_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: f64,
        dst_data: &::js_sys::Object,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getBufferSubData ) ]
    ///The `getBufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_buffer_sub_data_with_i32_and_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: i32,
        dst_data: &mut [u8],
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getBufferSubData ) ]
    ///The `getBufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_buffer_sub_data_with_f64_and_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: f64,
        dst_data: &mut [u8],
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getBufferSubData ) ]
    ///The `getBufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_buffer_sub_data_with_i32_and_array_buffer_view_and_dst_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: i32,
        dst_data: &::js_sys::Object,
        dst_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getBufferSubData ) ]
    ///The `getBufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_buffer_sub_data_with_f64_and_array_buffer_view_and_dst_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: f64,
        dst_data: &::js_sys::Object,
        dst_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getBufferSubData ) ]
    ///The `getBufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_buffer_sub_data_with_i32_and_u8_array_and_dst_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: i32,
        dst_data: &mut [u8],
        dst_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getBufferSubData ) ]
    ///The `getBufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_buffer_sub_data_with_f64_and_u8_array_and_dst_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: f64,
        dst_data: &mut [u8],
        dst_offset: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getBufferSubData ) ]
    ///The `getBufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_buffer_sub_data_with_i32_and_array_buffer_view_and_dst_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: i32,
        dst_data: &::js_sys::Object,
        dst_offset: u32,
        length: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getBufferSubData ) ]
    ///The `getBufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_buffer_sub_data_with_f64_and_array_buffer_view_and_dst_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: f64,
        dst_data: &::js_sys::Object,
        dst_offset: u32,
        length: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getBufferSubData ) ]
    ///The `getBufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_buffer_sub_data_with_i32_and_u8_array_and_dst_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: i32,
        dst_data: &mut [u8],
        dst_offset: u32,
        length: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getBufferSubData ) ]
    ///The `getBufferSubData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_buffer_sub_data_with_f64_and_u8_array_and_dst_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: f64,
        dst_data: &mut [u8],
        dst_offset: u32,
        length: u32,
    );

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getFragDataLocation ) ]
    ///The `getFragDataLocation()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getFragDataLocation)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*
    pub fn get_frag_data_location(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        name: &str,
    ) -> i32;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = getIndexedParameter ) ]
    ///The `getIndexedParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getIndexedParameter)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_indexed_parameter(
        this: &WebGl2RenderingContext,
        target: u32,
        index: u32,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = getInternalformatParameter ) ]
    ///The `getInternalformatParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getInternalformatParameter)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_internalformat_parameter(
        this: &WebGl2RenderingContext,
        target: u32,
        internalformat: u32,
        pname: u32,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getQuery ) ]
    ///The `getQuery()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getQuery)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_query(
        this: &WebGl2RenderingContext,
        target: u32,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;

    #[cfg(feature = "WebGlQuery")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getQueryParameter ) ]
    ///The `getQueryParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getQueryParameter)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlQuery`*
    pub fn get_query_parameter(
        this: &WebGl2RenderingContext,
        query: &WebGlQuery,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;

    #[cfg(feature = "WebGlSampler")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getSamplerParameter ) ]
    ///The `getSamplerParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getSamplerParameter)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSampler`*
    pub fn get_sampler_parameter(
        this: &WebGl2RenderingContext,
        sampler: &WebGlSampler,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;

    #[cfg(feature = "WebGlSync")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getSyncParameter ) ]
    ///The `getSyncParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getSyncParameter)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSync`*
    pub fn get_sync_parameter(
        this: &WebGl2RenderingContext,
        sync: &WebGlSync,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;

    #[cfg(all(feature = "WebGlActiveInfo", feature = "WebGlProgram",))]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getTransformFeedbackVarying ) ]
    ///The `getTransformFeedbackVarying()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getTransformFeedbackVarying)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlActiveInfo`, `WebGlProgram`*
    pub fn get_transform_feedback_varying(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        index: u32,
    ) -> Option<WebGlActiveInfo>;

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getUniformBlockIndex ) ]
    ///The `getUniformBlockIndex()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getUniformBlockIndex)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*
    pub fn get_uniform_block_index(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        uniform_block_name: &str,
    ) -> u32;

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getUniformIndices ) ]
    ///The `getUniformIndices()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getUniformIndices)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*
    pub fn get_uniform_indices(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        uniform_names: &::wasm_bindgen::JsValue,
    ) -> Option<::js_sys::Array>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = invalidateFramebuffer ) ]
    ///The `invalidateFramebuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/invalidateFramebuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn invalidate_framebuffer(
        this: &WebGl2RenderingContext,
        target: u32,
        attachments: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = invalidateSubFramebuffer ) ]
    ///The `invalidateSubFramebuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/invalidateSubFramebuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn invalidate_sub_framebuffer(
        this: &WebGl2RenderingContext,
        target: u32,
        attachments: &::wasm_bindgen::JsValue,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "WebGlQuery")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = isQuery ) ]
    ///The `isQuery()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isQuery)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlQuery`*
    pub fn is_query(this: &WebGl2RenderingContext, query: Option<&WebGlQuery>) -> bool;

    #[cfg(feature = "WebGlSampler")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = isSampler ) ]
    ///The `isSampler()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isSampler)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSampler`*
    pub fn is_sampler(this: &WebGl2RenderingContext, sampler: Option<&WebGlSampler>) -> bool;

    #[cfg(feature = "WebGlSync")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = isSync ) ]
    ///The `isSync()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isSync)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSync`*
    pub fn is_sync(this: &WebGl2RenderingContext, sync: Option<&WebGlSync>) -> bool;

    #[cfg(feature = "WebGlTransformFeedback")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = isTransformFeedback ) ]
    ///The `isTransformFeedback()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isTransformFeedback)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlTransformFeedback`*
    pub fn is_transform_feedback(
        this: &WebGl2RenderingContext,
        tf: Option<&WebGlTransformFeedback>,
    ) -> bool;

    #[cfg(feature = "WebGlVertexArrayObject")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = isVertexArray ) ]
    ///The `isVertexArray()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isVertexArray)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlVertexArrayObject`*
    pub fn is_vertex_array(
        this: &WebGl2RenderingContext,
        vertex_array: Option<&WebGlVertexArrayObject>,
    ) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = pauseTransformFeedback ) ]
    ///The `pauseTransformFeedback()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/pauseTransformFeedback)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn pause_transform_feedback(this: &WebGl2RenderingContext);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = readBuffer ) ]
    ///The `readBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/readBuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn read_buffer(this: &WebGl2RenderingContext, src: u32);

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = readPixels ) ]
    ///The `readPixels()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/readPixels)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn read_pixels_with_opt_array_buffer_view(
        this: &WebGl2RenderingContext,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        dst_data: Option<&::js_sys::Object>,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = readPixels ) ]
    ///The `readPixels()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/readPixels)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn read_pixels_with_opt_u8_array(
        this: &WebGl2RenderingContext,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        dst_data: Option<&mut [u8]>,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = readPixels ) ]
    ///The `readPixels()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/readPixels)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn read_pixels_with_i32(
        this: &WebGl2RenderingContext,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        offset: i32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = readPixels ) ]
    ///The `readPixels()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/readPixels)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn read_pixels_with_f64(
        this: &WebGl2RenderingContext,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        offset: f64,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = readPixels ) ]
    ///The `readPixels()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/readPixels)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn read_pixels_with_array_buffer_view_and_dst_offset(
        this: &WebGl2RenderingContext,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        dst_data: &::js_sys::Object,
        dst_offset: u32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = readPixels ) ]
    ///The `readPixels()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/readPixels)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn read_pixels_with_u8_array_and_dst_offset(
        this: &WebGl2RenderingContext,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        dst_data: &mut [u8],
        dst_offset: u32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = renderbufferStorageMultisample ) ]
    ///The `renderbufferStorageMultisample()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/renderbufferStorageMultisample)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn renderbuffer_storage_multisample(
        this: &WebGl2RenderingContext,
        target: u32,
        samples: i32,
        internalformat: u32,
        width: i32,
        height: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = resumeTransformFeedback ) ]
    ///The `resumeTransformFeedback()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/resumeTransformFeedback)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn resume_transform_feedback(this: &WebGl2RenderingContext);

    #[cfg(feature = "WebGlSampler")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = samplerParameterf ) ]
    ///The `samplerParameterf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/samplerParameterf)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSampler`*
    pub fn sampler_parameterf(
        this: &WebGl2RenderingContext,
        sampler: &WebGlSampler,
        pname: u32,
        param: f32,
    );

    #[cfg(feature = "WebGlSampler")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = samplerParameteri ) ]
    ///The `samplerParameteri()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/samplerParameteri)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSampler`*
    pub fn sampler_parameteri(
        this: &WebGl2RenderingContext,
        sampler: &WebGlSampler,
        pname: u32,
        param: i32,
    );

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
        this: &WebGl2RenderingContext,
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

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        this: &WebGl2RenderingContext,
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

    #[cfg(feature = "HtmlCanvasElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WebGl2RenderingContext`*
    pub fn tex_image_2d_with_u32_and_u32_and_html_canvas_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        source: &HtmlCanvasElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`, `WebGl2RenderingContext`*
    pub fn tex_image_2d_with_u32_and_u32_and_html_image_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        source: &HtmlImageElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlVideoElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlVideoElement`, `WebGl2RenderingContext`*
    pub fn tex_image_2d_with_u32_and_u32_and_html_video_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        source: &HtmlVideoElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmap`, `WebGl2RenderingContext`*
    pub fn tex_image_2d_with_u32_and_u32_and_image_bitmap(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        source: &ImageBitmap,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `ImageData`, `WebGl2RenderingContext`*
    pub fn tex_image_2d_with_u32_and_u32_and_image_data(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        source: &ImageData,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_i32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        pbo_offset: i32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_f64(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        pbo_offset: f64,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlCanvasElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WebGl2RenderingContext`*
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_html_canvas_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &HtmlCanvasElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`, `WebGl2RenderingContext`*
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_html_image_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &HtmlImageElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlVideoElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlVideoElement`, `WebGl2RenderingContext`*
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_html_video_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &HtmlVideoElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmap`, `WebGl2RenderingContext`*
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_image_bitmap(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &ImageBitmap,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `ImageData`, `WebGl2RenderingContext`*
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_image_data(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &ImageData,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_array_buffer_view_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        src_data: &::js_sys::Object,
        src_offset: u32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage2D ) ]
    ///The `texImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_u8_array_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        src_data: &[u8],
        src_offset: u32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage3D ) ]
    ///The `texImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_image_3d_with_i32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        pbo_offset: i32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage3D ) ]
    ///The `texImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_image_3d_with_f64(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        pbo_offset: f64,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlCanvasElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage3D ) ]
    ///The `texImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WebGl2RenderingContext`*
    pub fn tex_image_3d_with_html_canvas_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &HtmlCanvasElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage3D ) ]
    ///The `texImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`, `WebGl2RenderingContext`*
    pub fn tex_image_3d_with_html_image_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &HtmlImageElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlVideoElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage3D ) ]
    ///The `texImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlVideoElement`, `WebGl2RenderingContext`*
    pub fn tex_image_3d_with_html_video_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &HtmlVideoElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage3D ) ]
    ///The `texImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmap`, `WebGl2RenderingContext`*
    pub fn tex_image_3d_with_image_bitmap(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &ImageBitmap,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage3D ) ]
    ///The `texImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)
    ///
    ///*This API requires the following crate features to be activated: `ImageData`, `WebGl2RenderingContext`*
    pub fn tex_image_3d_with_image_data(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &ImageData,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage3D ) ]
    ///The `texImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_image_3d_with_opt_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        src_data: Option<&::js_sys::Object>,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage3D ) ]
    ///The `texImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_image_3d_with_opt_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        src_data: Option<&[u8]>,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage3D ) ]
    ///The `texImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_image_3d_with_array_buffer_view_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        src_data: &::js_sys::Object,
        src_offset: u32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texImage3D ) ]
    ///The `texImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_image_3d_with_u8_array_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        src_data: &[u8],
        src_offset: u32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = texStorage2D ) ]
    ///The `texStorage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texStorage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_storage_2d(
        this: &WebGl2RenderingContext,
        target: u32,
        levels: i32,
        internalformat: u32,
        width: i32,
        height: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = texStorage3D ) ]
    ///The `texStorage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texStorage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_storage_3d(
        this: &WebGl2RenderingContext,
        target: u32,
        levels: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        depth: i32,
    );

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_array_buffer_view(
        this: &WebGl2RenderingContext,
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

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(
        this: &WebGl2RenderingContext,
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

    #[cfg(feature = "HtmlCanvasElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WebGl2RenderingContext`*
    pub fn tex_sub_image_2d_with_u32_and_u32_and_html_canvas_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        format: u32,
        type_: u32,
        source: &HtmlCanvasElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`, `WebGl2RenderingContext`*
    pub fn tex_sub_image_2d_with_u32_and_u32_and_html_image_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        format: u32,
        type_: u32,
        source: &HtmlImageElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlVideoElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlVideoElement`, `WebGl2RenderingContext`*
    pub fn tex_sub_image_2d_with_u32_and_u32_and_html_video_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        format: u32,
        type_: u32,
        source: &HtmlVideoElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmap`, `WebGl2RenderingContext`*
    pub fn tex_sub_image_2d_with_u32_and_u32_and_image_bitmap(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        format: u32,
        type_: u32,
        source: &ImageBitmap,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `ImageData`, `WebGl2RenderingContext`*
    pub fn tex_sub_image_2d_with_u32_and_u32_and_image_data(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        format: u32,
        type_: u32,
        source: &ImageData,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_i32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        pbo_offset: i32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_f64(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        pbo_offset: f64,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlCanvasElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WebGl2RenderingContext`*
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_html_canvas_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        source: &HtmlCanvasElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`, `WebGl2RenderingContext`*
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_html_image_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        source: &HtmlImageElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlVideoElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlVideoElement`, `WebGl2RenderingContext`*
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_html_video_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        source: &HtmlVideoElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmap`, `WebGl2RenderingContext`*
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_image_bitmap(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        source: &ImageBitmap,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `ImageData`, `WebGl2RenderingContext`*
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_image_data(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        source: &ImageData,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_array_buffer_view_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        src_data: &::js_sys::Object,
        src_offset: u32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage2D ) ]
    ///The `texSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_u8_array_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        src_data: &[u8],
        src_offset: u32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage3D ) ]
    ///The `texSubImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_sub_image_3d_with_i32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        pbo_offset: i32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage3D ) ]
    ///The `texSubImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_sub_image_3d_with_f64(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        pbo_offset: f64,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlCanvasElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage3D ) ]
    ///The `texSubImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WebGl2RenderingContext`*
    pub fn tex_sub_image_3d_with_html_canvas_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        source: &HtmlCanvasElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage3D ) ]
    ///The `texSubImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`, `WebGl2RenderingContext`*
    pub fn tex_sub_image_3d_with_html_image_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        source: &HtmlImageElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlVideoElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage3D ) ]
    ///The `texSubImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)
    ///
    ///*This API requires the following crate features to be activated: `HtmlVideoElement`, `WebGl2RenderingContext`*
    pub fn tex_sub_image_3d_with_html_video_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        source: &HtmlVideoElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage3D ) ]
    ///The `texSubImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmap`, `WebGl2RenderingContext`*
    pub fn tex_sub_image_3d_with_image_bitmap(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        source: &ImageBitmap,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage3D ) ]
    ///The `texSubImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)
    ///
    ///*This API requires the following crate features to be activated: `ImageData`, `WebGl2RenderingContext`*
    pub fn tex_sub_image_3d_with_image_data(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        source: &ImageData,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage3D ) ]
    ///The `texSubImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_sub_image_3d_with_opt_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        src_data: Option<&::js_sys::Object>,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage3D ) ]
    ///The `texSubImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_sub_image_3d_with_opt_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        src_data: Option<&[u8]>,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage3D ) ]
    ///The `texSubImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_sub_image_3d_with_opt_array_buffer_view_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        src_data: Option<&::js_sys::Object>,
        src_offset: u32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = texSubImage3D ) ]
    ///The `texSubImage3D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_sub_image_3d_with_opt_u8_array_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        src_data: Option<&[u8]>,
        src_offset: u32,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = transformFeedbackVaryings ) ]
    ///The `transformFeedbackVaryings()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/transformFeedbackVaryings)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*
    pub fn transform_feedback_varyings(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        varyings: &::wasm_bindgen::JsValue,
        buffer_mode: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform1fv ) ]
    ///The `uniform1fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform1fv ) ]
    ///The `uniform1fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform1fv ) ]
    ///The `uniform1fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform1fv ) ]
    ///The `uniform1fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform1fv ) ]
    ///The `uniform1fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform1fv ) ]
    ///The `uniform1fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform1iv ) ]
    ///The `uniform1iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1iv_with_i32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform1iv ) ]
    ///The `uniform1iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1iv_with_i32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform1iv ) ]
    ///The `uniform1iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1iv_with_i32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform1iv ) ]
    ///The `uniform1iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1iv_with_i32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform1iv ) ]
    ///The `uniform1iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1iv_with_i32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform1iv ) ]
    ///The `uniform1iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1iv_with_i32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform1ui ) ]
    ///The `uniform1ui()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1ui)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1ui(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        v0: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform1uiv ) ]
    ///The `uniform1uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1uiv_with_u32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform1uiv ) ]
    ///The `uniform1uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1uiv_with_u32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform1uiv ) ]
    ///The `uniform1uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1uiv_with_u32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform1uiv ) ]
    ///The `uniform1uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1uiv_with_u32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform1uiv ) ]
    ///The `uniform1uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1uiv_with_u32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform1uiv ) ]
    ///The `uniform1uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1uiv_with_u32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform2fv ) ]
    ///The `uniform2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform2fv ) ]
    ///The `uniform2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform2fv ) ]
    ///The `uniform2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform2fv ) ]
    ///The `uniform2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform2fv ) ]
    ///The `uniform2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform2fv ) ]
    ///The `uniform2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform2iv ) ]
    ///The `uniform2iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2iv_with_i32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform2iv ) ]
    ///The `uniform2iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2iv_with_i32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform2iv ) ]
    ///The `uniform2iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2iv_with_i32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform2iv ) ]
    ///The `uniform2iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2iv_with_i32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform2iv ) ]
    ///The `uniform2iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2iv_with_i32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform2iv ) ]
    ///The `uniform2iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2iv_with_i32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform2ui ) ]
    ///The `uniform2ui()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2ui)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2ui(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        v0: u32,
        v1: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform2uiv ) ]
    ///The `uniform2uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2uiv_with_u32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform2uiv ) ]
    ///The `uniform2uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2uiv_with_u32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform2uiv ) ]
    ///The `uniform2uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2uiv_with_u32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform2uiv ) ]
    ///The `uniform2uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2uiv_with_u32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform2uiv ) ]
    ///The `uniform2uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2uiv_with_u32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform2uiv ) ]
    ///The `uniform2uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2uiv_with_u32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform3fv ) ]
    ///The `uniform3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform3fv ) ]
    ///The `uniform3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform3fv ) ]
    ///The `uniform3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform3fv ) ]
    ///The `uniform3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform3fv ) ]
    ///The `uniform3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform3fv ) ]
    ///The `uniform3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform3iv ) ]
    ///The `uniform3iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3iv_with_i32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform3iv ) ]
    ///The `uniform3iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3iv_with_i32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform3iv ) ]
    ///The `uniform3iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3iv_with_i32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform3iv ) ]
    ///The `uniform3iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3iv_with_i32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform3iv ) ]
    ///The `uniform3iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3iv_with_i32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform3iv ) ]
    ///The `uniform3iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3iv_with_i32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform3ui ) ]
    ///The `uniform3ui()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3ui)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3ui(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        v0: u32,
        v1: u32,
        v2: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform3uiv ) ]
    ///The `uniform3uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3uiv_with_u32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform3uiv ) ]
    ///The `uniform3uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3uiv_with_u32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform3uiv ) ]
    ///The `uniform3uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3uiv_with_u32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform3uiv ) ]
    ///The `uniform3uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3uiv_with_u32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform3uiv ) ]
    ///The `uniform3uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3uiv_with_u32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform3uiv ) ]
    ///The `uniform3uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3uiv_with_u32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform4fv ) ]
    ///The `uniform4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform4fv ) ]
    ///The `uniform4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform4fv ) ]
    ///The `uniform4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform4fv ) ]
    ///The `uniform4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform4fv ) ]
    ///The `uniform4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform4fv ) ]
    ///The `uniform4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform4iv ) ]
    ///The `uniform4iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4iv_with_i32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform4iv ) ]
    ///The `uniform4iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4iv_with_i32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform4iv ) ]
    ///The `uniform4iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4iv_with_i32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform4iv ) ]
    ///The `uniform4iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4iv_with_i32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform4iv ) ]
    ///The `uniform4iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4iv_with_i32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform4iv ) ]
    ///The `uniform4iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4iv_with_i32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform4ui ) ]
    ///The `uniform4ui()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4ui)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4ui(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        v0: u32,
        v1: u32,
        v2: u32,
        v3: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform4uiv ) ]
    ///The `uniform4uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4uiv_with_u32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform4uiv ) ]
    ///The `uniform4uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4uiv_with_u32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform4uiv ) ]
    ///The `uniform4uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4uiv_with_u32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform4uiv ) ]
    ///The `uniform4uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4uiv_with_u32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform4uiv ) ]
    ///The `uniform4uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4uiv_with_u32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform4uiv ) ]
    ///The `uniform4uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4uiv_with_u32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformBlockBinding ) ]
    ///The `uniformBlockBinding()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformBlockBinding)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*
    pub fn uniform_block_binding(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        uniform_block_index: u32,
        uniform_block_binding: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix2fv ) ]
    ///The `uniformMatrix2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix2fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix2fv ) ]
    ///The `uniformMatrix2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix2fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix2fv ) ]
    ///The `uniformMatrix2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix2fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix2fv ) ]
    ///The `uniformMatrix2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix2fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix2fv ) ]
    ///The `uniformMatrix2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix2fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix2fv ) ]
    ///The `uniformMatrix2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix2fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix2x3fv ) ]
    ///The `uniformMatrix2x3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix2x3fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix2x3fv ) ]
    ///The `uniformMatrix2x3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix2x3fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix2x3fv ) ]
    ///The `uniformMatrix2x3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix2x3fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix2x3fv ) ]
    ///The `uniformMatrix2x3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix2x3fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix2x3fv ) ]
    ///The `uniformMatrix2x3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix2x3fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix2x3fv ) ]
    ///The `uniformMatrix2x3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix2x3fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix2x4fv ) ]
    ///The `uniformMatrix2x4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix2x4fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix2x4fv ) ]
    ///The `uniformMatrix2x4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix2x4fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix2x4fv ) ]
    ///The `uniformMatrix2x4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix2x4fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix2x4fv ) ]
    ///The `uniformMatrix2x4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix2x4fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix2x4fv ) ]
    ///The `uniformMatrix2x4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix2x4fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix2x4fv ) ]
    ///The `uniformMatrix2x4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix2x4fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix3fv ) ]
    ///The `uniformMatrix3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix3fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix3fv ) ]
    ///The `uniformMatrix3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix3fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix3fv ) ]
    ///The `uniformMatrix3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix3fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix3fv ) ]
    ///The `uniformMatrix3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix3fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix3fv ) ]
    ///The `uniformMatrix3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix3fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix3fv ) ]
    ///The `uniformMatrix3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix3fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix3x2fv ) ]
    ///The `uniformMatrix3x2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix3x2fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix3x2fv ) ]
    ///The `uniformMatrix3x2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix3x2fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix3x2fv ) ]
    ///The `uniformMatrix3x2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix3x2fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix3x2fv ) ]
    ///The `uniformMatrix3x2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix3x2fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix3x2fv ) ]
    ///The `uniformMatrix3x2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix3x2fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix3x2fv ) ]
    ///The `uniformMatrix3x2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix3x2fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix3x4fv ) ]
    ///The `uniformMatrix3x4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix3x4fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix3x4fv ) ]
    ///The `uniformMatrix3x4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix3x4fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix3x4fv ) ]
    ///The `uniformMatrix3x4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix3x4fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix3x4fv ) ]
    ///The `uniformMatrix3x4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix3x4fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix3x4fv ) ]
    ///The `uniformMatrix3x4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix3x4fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix3x4fv ) ]
    ///The `uniformMatrix3x4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix3x4fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix4fv ) ]
    ///The `uniformMatrix4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix4fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix4fv ) ]
    ///The `uniformMatrix4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix4fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix4fv ) ]
    ///The `uniformMatrix4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix4fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix4fv ) ]
    ///The `uniformMatrix4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix4fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix4fv ) ]
    ///The `uniformMatrix4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix4fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix4fv ) ]
    ///The `uniformMatrix4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix4fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix4x2fv ) ]
    ///The `uniformMatrix4x2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix4x2fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix4x2fv ) ]
    ///The `uniformMatrix4x2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix4x2fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix4x2fv ) ]
    ///The `uniformMatrix4x2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix4x2fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix4x2fv ) ]
    ///The `uniformMatrix4x2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix4x2fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix4x2fv ) ]
    ///The `uniformMatrix4x2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix4x2fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix4x2fv ) ]
    ///The `uniformMatrix4x2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix4x2fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix4x3fv ) ]
    ///The `uniformMatrix4x3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix4x3fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix4x3fv ) ]
    ///The `uniformMatrix4x3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix4x3fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix4x3fv ) ]
    ///The `uniformMatrix4x3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix4x3fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix4x3fv ) ]
    ///The `uniformMatrix4x3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix4x3fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix4x3fv ) ]
    ///The `uniformMatrix4x3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix4x3fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniformMatrix4x3fv ) ]
    ///The `uniformMatrix4x3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform_matrix4x3fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttribDivisor ) ]
    ///The `vertexAttribDivisor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribDivisor)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib_divisor(this: &WebGl2RenderingContext, index: u32, divisor: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttribI4i ) ]
    ///The `vertexAttribI4i()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribI4i)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib_i4i(
        this: &WebGl2RenderingContext,
        index: u32,
        x: i32,
        y: i32,
        z: i32,
        w: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttribI4iv ) ]
    ///The `vertexAttribI4iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribI4iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib_i4iv_with_i32_array(
        this: &WebGl2RenderingContext,
        index: u32,
        values: &mut [i32],
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttribI4iv ) ]
    ///The `vertexAttribI4iv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribI4iv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib_i4iv_with_i32_sequence(
        this: &WebGl2RenderingContext,
        index: u32,
        values: &::wasm_bindgen::JsValue,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttribI4ui ) ]
    ///The `vertexAttribI4ui()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribI4ui)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib_i4ui(
        this: &WebGl2RenderingContext,
        index: u32,
        x: u32,
        y: u32,
        z: u32,
        w: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttribI4uiv ) ]
    ///The `vertexAttribI4uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribI4uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib_i4uiv_with_u32_array(
        this: &WebGl2RenderingContext,
        index: u32,
        values: &mut [u32],
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttribI4uiv ) ]
    ///The `vertexAttribI4uiv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribI4uiv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib_i4uiv_with_u32_sequence(
        this: &WebGl2RenderingContext,
        index: u32,
        values: &::wasm_bindgen::JsValue,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttribIPointer ) ]
    ///The `vertexAttribIPointer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribIPointer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib_i_pointer_with_i32(
        this: &WebGl2RenderingContext,
        index: u32,
        size: i32,
        type_: u32,
        stride: i32,
        offset: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttribIPointer ) ]
    ///The `vertexAttribIPointer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribIPointer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib_i_pointer_with_f64(
        this: &WebGl2RenderingContext,
        index: u32,
        size: i32,
        type_: u32,
        stride: i32,
        offset: f64,
    );

    #[cfg(feature = "WebGlSync")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = waitSync ) ]
    ///The `waitSync()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/waitSync)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSync`*
    pub fn wait_sync_with_i32(
        this: &WebGl2RenderingContext,
        sync: &WebGlSync,
        flags: u32,
        timeout: i32,
    );

    #[cfg(feature = "WebGlSync")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = waitSync ) ]
    ///The `waitSync()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/waitSync)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSync`*
    pub fn wait_sync_with_f64(
        this: &WebGl2RenderingContext,
        sync: &WebGlSync,
        flags: u32,
        timeout: f64,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = activeTexture ) ]
    ///The `activeTexture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/activeTexture)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn active_texture(this: &WebGl2RenderingContext, texture: u32);

    #[cfg(all(feature = "WebGlProgram", feature = "WebGlShader",))]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = attachShader ) ]
    ///The `attachShader()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/attachShader)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`, `WebGlShader`*
    pub fn attach_shader(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        shader: &WebGlShader,
    );

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bindAttribLocation ) ]
    ///The `bindAttribLocation()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindAttribLocation)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*
    pub fn bind_attrib_location(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        index: u32,
        name: &str,
    );

    #[cfg(feature = "WebGlBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bindBuffer ) ]
    ///The `bindBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindBuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlBuffer`*
    pub fn bind_buffer(this: &WebGl2RenderingContext, target: u32, buffer: Option<&WebGlBuffer>);

    #[cfg(feature = "WebGlFramebuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bindFramebuffer ) ]
    ///The `bindFramebuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindFramebuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlFramebuffer`*
    pub fn bind_framebuffer(
        this: &WebGl2RenderingContext,
        target: u32,
        framebuffer: Option<&WebGlFramebuffer>,
    );

    #[cfg(feature = "WebGlRenderbuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bindRenderbuffer ) ]
    ///The `bindRenderbuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindRenderbuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlRenderbuffer`*
    pub fn bind_renderbuffer(
        this: &WebGl2RenderingContext,
        target: u32,
        renderbuffer: Option<&WebGlRenderbuffer>,
    );

    #[cfg(feature = "WebGlTexture")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = bindTexture ) ]
    ///The `bindTexture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindTexture)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlTexture`*
    pub fn bind_texture(this: &WebGl2RenderingContext, target: u32, texture: Option<&WebGlTexture>);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = blendColor ) ]
    ///The `blendColor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/blendColor)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn blend_color(this: &WebGl2RenderingContext, red: f32, green: f32, blue: f32, alpha: f32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = blendEquation ) ]
    ///The `blendEquation()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/blendEquation)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn blend_equation(this: &WebGl2RenderingContext, mode: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = blendEquationSeparate ) ]
    ///The `blendEquationSeparate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/blendEquationSeparate)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn blend_equation_separate(this: &WebGl2RenderingContext, mode_rgb: u32, mode_alpha: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = blendFunc ) ]
    ///The `blendFunc()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/blendFunc)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn blend_func(this: &WebGl2RenderingContext, sfactor: u32, dfactor: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = blendFuncSeparate ) ]
    ///The `blendFuncSeparate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/blendFuncSeparate)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn blend_func_separate(
        this: &WebGl2RenderingContext,
        src_rgb: u32,
        dst_rgb: u32,
        src_alpha: u32,
        dst_alpha: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = checkFramebufferStatus ) ]
    ///The `checkFramebufferStatus()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/checkFramebufferStatus)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn check_framebuffer_status(this: &WebGl2RenderingContext, target: u32) -> u32;

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = clear ) ]
    ///The `clear()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clear)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn clear(this: &WebGl2RenderingContext, mask: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = clearColor ) ]
    ///The `clearColor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearColor)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn clear_color(this: &WebGl2RenderingContext, red: f32, green: f32, blue: f32, alpha: f32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = clearDepth ) ]
    ///The `clearDepth()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearDepth)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn clear_depth(this: &WebGl2RenderingContext, depth: f32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = clearStencil ) ]
    ///The `clearStencil()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearStencil)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn clear_stencil(this: &WebGl2RenderingContext, s: i32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = colorMask ) ]
    ///The `colorMask()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/colorMask)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn color_mask(
        this: &WebGl2RenderingContext,
        red: bool,
        green: bool,
        blue: bool,
        alpha: bool,
    );

    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = compileShader ) ]
    ///The `compileShader()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compileShader)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlShader`*
    pub fn compile_shader(this: &WebGl2RenderingContext, shader: &WebGlShader);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = copyTexImage2D ) ]
    ///The `copyTexImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyTexImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn copy_tex_image_2d(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        border: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = copyTexSubImage2D ) ]
    ///The `copyTexSubImage2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyTexSubImage2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn copy_tex_sub_image_2d(
        this: &WebGl2RenderingContext,
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
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = createBuffer ) ]
    ///The `createBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/createBuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlBuffer`*
    pub fn create_buffer(this: &WebGl2RenderingContext) -> Option<WebGlBuffer>;

    #[cfg(feature = "WebGlFramebuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = createFramebuffer ) ]
    ///The `createFramebuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/createFramebuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlFramebuffer`*
    pub fn create_framebuffer(this: &WebGl2RenderingContext) -> Option<WebGlFramebuffer>;

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = createProgram ) ]
    ///The `createProgram()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/createProgram)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*
    pub fn create_program(this: &WebGl2RenderingContext) -> Option<WebGlProgram>;

    #[cfg(feature = "WebGlRenderbuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = createRenderbuffer ) ]
    ///The `createRenderbuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/createRenderbuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlRenderbuffer`*
    pub fn create_renderbuffer(this: &WebGl2RenderingContext) -> Option<WebGlRenderbuffer>;

    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = createShader ) ]
    ///The `createShader()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/createShader)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlShader`*
    pub fn create_shader(this: &WebGl2RenderingContext, type_: u32) -> Option<WebGlShader>;

    #[cfg(feature = "WebGlTexture")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = createTexture ) ]
    ///The `createTexture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/createTexture)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlTexture`*
    pub fn create_texture(this: &WebGl2RenderingContext) -> Option<WebGlTexture>;

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = cullFace ) ]
    ///The `cullFace()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/cullFace)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn cull_face(this: &WebGl2RenderingContext, mode: u32);

    #[cfg(feature = "WebGlBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = deleteBuffer ) ]
    ///The `deleteBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteBuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlBuffer`*
    pub fn delete_buffer(this: &WebGl2RenderingContext, buffer: Option<&WebGlBuffer>);

    #[cfg(feature = "WebGlFramebuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = deleteFramebuffer ) ]
    ///The `deleteFramebuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteFramebuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlFramebuffer`*
    pub fn delete_framebuffer(
        this: &WebGl2RenderingContext,
        framebuffer: Option<&WebGlFramebuffer>,
    );

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = deleteProgram ) ]
    ///The `deleteProgram()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteProgram)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*
    pub fn delete_program(this: &WebGl2RenderingContext, program: Option<&WebGlProgram>);

    #[cfg(feature = "WebGlRenderbuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = deleteRenderbuffer ) ]
    ///The `deleteRenderbuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteRenderbuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlRenderbuffer`*
    pub fn delete_renderbuffer(
        this: &WebGl2RenderingContext,
        renderbuffer: Option<&WebGlRenderbuffer>,
    );

    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = deleteShader ) ]
    ///The `deleteShader()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteShader)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlShader`*
    pub fn delete_shader(this: &WebGl2RenderingContext, shader: Option<&WebGlShader>);

    #[cfg(feature = "WebGlTexture")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = deleteTexture ) ]
    ///The `deleteTexture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteTexture)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlTexture`*
    pub fn delete_texture(this: &WebGl2RenderingContext, texture: Option<&WebGlTexture>);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = depthFunc ) ]
    ///The `depthFunc()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/depthFunc)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn depth_func(this: &WebGl2RenderingContext, func: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = depthMask ) ]
    ///The `depthMask()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/depthMask)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn depth_mask(this: &WebGl2RenderingContext, flag: bool);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = depthRange ) ]
    ///The `depthRange()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/depthRange)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn depth_range(this: &WebGl2RenderingContext, z_near: f32, z_far: f32);

    #[cfg(all(feature = "WebGlProgram", feature = "WebGlShader",))]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = detachShader ) ]
    ///The `detachShader()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/detachShader)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`, `WebGlShader`*
    pub fn detach_shader(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        shader: &WebGlShader,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = disable ) ]
    ///The `disable()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/disable)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn disable(this: &WebGl2RenderingContext, cap: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = disableVertexAttribArray ) ]
    ///The `disableVertexAttribArray()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/disableVertexAttribArray)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn disable_vertex_attrib_array(this: &WebGl2RenderingContext, index: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = drawArrays ) ]
    ///The `drawArrays()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawArrays)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn draw_arrays(this: &WebGl2RenderingContext, mode: u32, first: i32, count: i32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = drawElements ) ]
    ///The `drawElements()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawElements)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn draw_elements_with_i32(
        this: &WebGl2RenderingContext,
        mode: u32,
        count: i32,
        type_: u32,
        offset: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = drawElements ) ]
    ///The `drawElements()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawElements)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn draw_elements_with_f64(
        this: &WebGl2RenderingContext,
        mode: u32,
        count: i32,
        type_: u32,
        offset: f64,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = enable ) ]
    ///The `enable()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/enable)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn enable(this: &WebGl2RenderingContext, cap: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = enableVertexAttribArray ) ]
    ///The `enableVertexAttribArray()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/enableVertexAttribArray)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn enable_vertex_attrib_array(this: &WebGl2RenderingContext, index: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = finish ) ]
    ///The `finish()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/finish)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn finish(this: &WebGl2RenderingContext);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = flush ) ]
    ///The `flush()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/flush)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn flush(this: &WebGl2RenderingContext);

    #[cfg(feature = "WebGlRenderbuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = framebufferRenderbuffer ) ]
    ///The `framebufferRenderbuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/framebufferRenderbuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlRenderbuffer`*
    pub fn framebuffer_renderbuffer(
        this: &WebGl2RenderingContext,
        target: u32,
        attachment: u32,
        renderbuffertarget: u32,
        renderbuffer: Option<&WebGlRenderbuffer>,
    );

    #[cfg(feature = "WebGlTexture")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = framebufferTexture2D ) ]
    ///The `framebufferTexture2D()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/framebufferTexture2D)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlTexture`*
    pub fn framebuffer_texture_2d(
        this: &WebGl2RenderingContext,
        target: u32,
        attachment: u32,
        textarget: u32,
        texture: Option<&WebGlTexture>,
        level: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = frontFace ) ]
    ///The `frontFace()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/frontFace)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn front_face(this: &WebGl2RenderingContext, mode: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = generateMipmap ) ]
    ///The `generateMipmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/generateMipmap)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn generate_mipmap(this: &WebGl2RenderingContext, target: u32);

    #[cfg(all(feature = "WebGlActiveInfo", feature = "WebGlProgram",))]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getActiveAttrib ) ]
    ///The `getActiveAttrib()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getActiveAttrib)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlActiveInfo`, `WebGlProgram`*
    pub fn get_active_attrib(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        index: u32,
    ) -> Option<WebGlActiveInfo>;

    #[cfg(all(feature = "WebGlActiveInfo", feature = "WebGlProgram",))]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getActiveUniform ) ]
    ///The `getActiveUniform()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getActiveUniform)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlActiveInfo`, `WebGlProgram`*
    pub fn get_active_uniform(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        index: u32,
    ) -> Option<WebGlActiveInfo>;

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getAttachedShaders ) ]
    ///The `getAttachedShaders()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getAttachedShaders)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*
    pub fn get_attached_shaders(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
    ) -> Option<::js_sys::Array>;

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getAttribLocation ) ]
    ///The `getAttribLocation()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getAttribLocation)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*
    pub fn get_attrib_location(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        name: &str,
    ) -> i32;

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getBufferParameter ) ]
    ///The `getBufferParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferParameter)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_buffer_parameter(
        this: &WebGl2RenderingContext,
        target: u32,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;

    #[cfg(feature = "WebGlContextAttributes")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getContextAttributes ) ]
    ///The `getContextAttributes()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getContextAttributes)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlContextAttributes`*
    pub fn get_context_attributes(this: &WebGl2RenderingContext) -> Option<WebGlContextAttributes>;

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getError ) ]
    ///The `getError()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getError)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_error(this: &WebGl2RenderingContext) -> u32;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = getExtension ) ]
    ///The `getExtension()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getExtension)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_extension(
        this: &WebGl2RenderingContext,
        name: &str,
    ) -> Result<Option<::js_sys::Object>, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = getFramebufferAttachmentParameter ) ]
    ///The `getFramebufferAttachmentParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getFramebufferAttachmentParameter)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_framebuffer_attachment_parameter(
        this: &WebGl2RenderingContext,
        target: u32,
        attachment: u32,
        pname: u32,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = getParameter ) ]
    ///The `getParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getParameter)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_parameter(
        this: &WebGl2RenderingContext,
        pname: u32,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getProgramInfoLog ) ]
    ///The `getProgramInfoLog()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getProgramInfoLog)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*
    pub fn get_program_info_log(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
    ) -> Option<String>;

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getProgramParameter ) ]
    ///The `getProgramParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getProgramParameter)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*
    pub fn get_program_parameter(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getRenderbufferParameter ) ]
    ///The `getRenderbufferParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getRenderbufferParameter)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_renderbuffer_parameter(
        this: &WebGl2RenderingContext,
        target: u32,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;

    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getShaderInfoLog ) ]
    ///The `getShaderInfoLog()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getShaderInfoLog)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlShader`*
    pub fn get_shader_info_log(
        this: &WebGl2RenderingContext,
        shader: &WebGlShader,
    ) -> Option<String>;

    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getShaderParameter ) ]
    ///The `getShaderParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getShaderParameter)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlShader`*
    pub fn get_shader_parameter(
        this: &WebGl2RenderingContext,
        shader: &WebGlShader,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;

    #[cfg(feature = "WebGlShaderPrecisionFormat")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getShaderPrecisionFormat ) ]
    ///The `getShaderPrecisionFormat()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getShaderPrecisionFormat)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlShaderPrecisionFormat`*
    pub fn get_shader_precision_format(
        this: &WebGl2RenderingContext,
        shadertype: u32,
        precisiontype: u32,
    ) -> Option<WebGlShaderPrecisionFormat>;

    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getShaderSource ) ]
    ///The `getShaderSource()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getShaderSource)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlShader`*
    pub fn get_shader_source(this: &WebGl2RenderingContext, shader: &WebGlShader)
        -> Option<String>;

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getSupportedExtensions ) ]
    ///The `getSupportedExtensions()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getSupportedExtensions)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_supported_extensions(this: &WebGl2RenderingContext) -> Option<::js_sys::Array>;

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getTexParameter ) ]
    ///The `getTexParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getTexParameter)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_tex_parameter(
        this: &WebGl2RenderingContext,
        target: u32,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;

    #[cfg(all(feature = "WebGlProgram", feature = "WebGlUniformLocation",))]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getUniform ) ]
    ///The `getUniform()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getUniform)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`, `WebGlUniformLocation`*
    pub fn get_uniform(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        location: &WebGlUniformLocation,
    ) -> ::wasm_bindgen::JsValue;

    #[cfg(all(feature = "WebGlProgram", feature = "WebGlUniformLocation",))]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getUniformLocation ) ]
    ///The `getUniformLocation()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getUniformLocation)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`, `WebGlUniformLocation`*
    pub fn get_uniform_location(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        name: &str,
    ) -> Option<WebGlUniformLocation>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebGL2RenderingContext" , js_name = getVertexAttrib ) ]
    ///The `getVertexAttrib()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getVertexAttrib)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_vertex_attrib(
        this: &WebGl2RenderingContext,
        index: u32,
        pname: u32,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = getVertexAttribOffset ) ]
    ///The `getVertexAttribOffset()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getVertexAttribOffset)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn get_vertex_attrib_offset(this: &WebGl2RenderingContext, index: u32, pname: u32) -> f64;

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = hint ) ]
    ///The `hint()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/hint)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn hint(this: &WebGl2RenderingContext, target: u32, mode: u32);

    #[cfg(feature = "WebGlBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = isBuffer ) ]
    ///The `isBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isBuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlBuffer`*
    pub fn is_buffer(this: &WebGl2RenderingContext, buffer: Option<&WebGlBuffer>) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = isContextLost ) ]
    ///The `isContextLost()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isContextLost)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn is_context_lost(this: &WebGl2RenderingContext) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = isEnabled ) ]
    ///The `isEnabled()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isEnabled)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn is_enabled(this: &WebGl2RenderingContext, cap: u32) -> bool;

    #[cfg(feature = "WebGlFramebuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = isFramebuffer ) ]
    ///The `isFramebuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isFramebuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlFramebuffer`*
    pub fn is_framebuffer(
        this: &WebGl2RenderingContext,
        framebuffer: Option<&WebGlFramebuffer>,
    ) -> bool;

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = isProgram ) ]
    ///The `isProgram()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isProgram)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*
    pub fn is_program(this: &WebGl2RenderingContext, program: Option<&WebGlProgram>) -> bool;

    #[cfg(feature = "WebGlRenderbuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = isRenderbuffer ) ]
    ///The `isRenderbuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isRenderbuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlRenderbuffer`*
    pub fn is_renderbuffer(
        this: &WebGl2RenderingContext,
        renderbuffer: Option<&WebGlRenderbuffer>,
    ) -> bool;

    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = isShader ) ]
    ///The `isShader()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isShader)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlShader`*
    pub fn is_shader(this: &WebGl2RenderingContext, shader: Option<&WebGlShader>) -> bool;

    #[cfg(feature = "WebGlTexture")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = isTexture ) ]
    ///The `isTexture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isTexture)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlTexture`*
    pub fn is_texture(this: &WebGl2RenderingContext, texture: Option<&WebGlTexture>) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = lineWidth ) ]
    ///The `lineWidth()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/lineWidth)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn line_width(this: &WebGl2RenderingContext, width: f32);

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = linkProgram ) ]
    ///The `linkProgram()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/linkProgram)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*
    pub fn link_program(this: &WebGl2RenderingContext, program: &WebGlProgram);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = pixelStorei ) ]
    ///The `pixelStorei()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/pixelStorei)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn pixel_storei(this: &WebGl2RenderingContext, pname: u32, param: i32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = polygonOffset ) ]
    ///The `polygonOffset()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/polygonOffset)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn polygon_offset(this: &WebGl2RenderingContext, factor: f32, units: f32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = renderbufferStorage ) ]
    ///The `renderbufferStorage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/renderbufferStorage)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn renderbuffer_storage(
        this: &WebGl2RenderingContext,
        target: u32,
        internalformat: u32,
        width: i32,
        height: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = sampleCoverage ) ]
    ///The `sampleCoverage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/sampleCoverage)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn sample_coverage(this: &WebGl2RenderingContext, value: f32, invert: bool);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = scissor ) ]
    ///The `scissor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/scissor)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn scissor(this: &WebGl2RenderingContext, x: i32, y: i32, width: i32, height: i32);

    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = shaderSource ) ]
    ///The `shaderSource()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/shaderSource)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlShader`*
    pub fn shader_source(this: &WebGl2RenderingContext, shader: &WebGlShader, source: &str);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = stencilFunc ) ]
    ///The `stencilFunc()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/stencilFunc)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn stencil_func(this: &WebGl2RenderingContext, func: u32, ref_: i32, mask: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = stencilFuncSeparate ) ]
    ///The `stencilFuncSeparate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/stencilFuncSeparate)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn stencil_func_separate(
        this: &WebGl2RenderingContext,
        face: u32,
        func: u32,
        ref_: i32,
        mask: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = stencilMask ) ]
    ///The `stencilMask()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/stencilMask)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn stencil_mask(this: &WebGl2RenderingContext, mask: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = stencilMaskSeparate ) ]
    ///The `stencilMaskSeparate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/stencilMaskSeparate)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn stencil_mask_separate(this: &WebGl2RenderingContext, face: u32, mask: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = stencilOp ) ]
    ///The `stencilOp()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/stencilOp)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn stencil_op(this: &WebGl2RenderingContext, fail: u32, zfail: u32, zpass: u32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = stencilOpSeparate ) ]
    ///The `stencilOpSeparate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/stencilOpSeparate)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn stencil_op_separate(
        this: &WebGl2RenderingContext,
        face: u32,
        fail: u32,
        zfail: u32,
        zpass: u32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = texParameterf ) ]
    ///The `texParameterf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texParameterf)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_parameterf(this: &WebGl2RenderingContext, target: u32, pname: u32, param: f32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = texParameteri ) ]
    ///The `texParameteri()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texParameteri)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn tex_parameteri(this: &WebGl2RenderingContext, target: u32, pname: u32, param: i32);

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform1f ) ]
    ///The `uniform1f()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1f)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1f(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: f32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform1i ) ]
    ///The `uniform1i()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1i)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform1i(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: i32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform2f ) ]
    ///The `uniform2f()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2f)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2f(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: f32,
        y: f32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform2i ) ]
    ///The `uniform2i()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2i)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform2i(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: i32,
        y: i32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform3f ) ]
    ///The `uniform3f()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3f)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3f(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: f32,
        y: f32,
        z: f32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform3i ) ]
    ///The `uniform3i()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3i)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform3i(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: i32,
        y: i32,
        z: i32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform4f ) ]
    ///The `uniform4f()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4f)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4f(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    );

    #[cfg(feature = "WebGlUniformLocation")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = uniform4i ) ]
    ///The `uniform4i()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4i)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*
    pub fn uniform4i(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: i32,
        y: i32,
        z: i32,
        w: i32,
    );

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = useProgram ) ]
    ///The `useProgram()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/useProgram)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*
    pub fn use_program(this: &WebGl2RenderingContext, program: Option<&WebGlProgram>);

    #[cfg(feature = "WebGlProgram")]
    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = validateProgram ) ]
    ///The `validateProgram()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/validateProgram)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*
    pub fn validate_program(this: &WebGl2RenderingContext, program: &WebGlProgram);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttrib1f ) ]
    ///The `vertexAttrib1f()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib1f)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib1f(this: &WebGl2RenderingContext, indx: u32, x: f32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttrib1fv ) ]
    ///The `vertexAttrib1fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib1fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib1fv_with_f32_array(
        this: &WebGl2RenderingContext,
        indx: u32,
        values: &[f32],
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttrib1fv ) ]
    ///The `vertexAttrib1fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib1fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib1fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        indx: u32,
        values: &::wasm_bindgen::JsValue,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttrib2f ) ]
    ///The `vertexAttrib2f()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib2f)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib2f(this: &WebGl2RenderingContext, indx: u32, x: f32, y: f32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttrib2fv ) ]
    ///The `vertexAttrib2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib2fv_with_f32_array(
        this: &WebGl2RenderingContext,
        indx: u32,
        values: &[f32],
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttrib2fv ) ]
    ///The `vertexAttrib2fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib2fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib2fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        indx: u32,
        values: &::wasm_bindgen::JsValue,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttrib3f ) ]
    ///The `vertexAttrib3f()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib3f)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib3f(this: &WebGl2RenderingContext, indx: u32, x: f32, y: f32, z: f32);

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttrib3fv ) ]
    ///The `vertexAttrib3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib3fv_with_f32_array(
        this: &WebGl2RenderingContext,
        indx: u32,
        values: &[f32],
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttrib3fv ) ]
    ///The `vertexAttrib3fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib3fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib3fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        indx: u32,
        values: &::wasm_bindgen::JsValue,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttrib4f ) ]
    ///The `vertexAttrib4f()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib4f)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib4f(
        this: &WebGl2RenderingContext,
        indx: u32,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttrib4fv ) ]
    ///The `vertexAttrib4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib4fv_with_f32_array(
        this: &WebGl2RenderingContext,
        indx: u32,
        values: &[f32],
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttrib4fv ) ]
    ///The `vertexAttrib4fv()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib4fv)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib4fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        indx: u32,
        values: &::wasm_bindgen::JsValue,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttribPointer ) ]
    ///The `vertexAttribPointer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribPointer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib_pointer_with_i32(
        this: &WebGl2RenderingContext,
        indx: u32,
        size: i32,
        type_: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = vertexAttribPointer ) ]
    ///The `vertexAttribPointer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribPointer)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn vertex_attrib_pointer_with_f64(
        this: &WebGl2RenderingContext,
        indx: u32,
        size: i32,
        type_: u32,
        normalized: bool,
        stride: i32,
        offset: f64,
    );

    # [ wasm_bindgen ( method , structural , js_class = "WebGL2RenderingContext" , js_name = viewport ) ]
    ///The `viewport()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/viewport)
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*
    pub fn viewport(this: &WebGl2RenderingContext, x: i32, y: i32, width: i32, height: i32);

}

impl WebGl2RenderingContext {
    ///The `WebGL2RenderingContext.READ_BUFFER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const READ_BUFFER: u32 = 3074u64 as u32;

    ///The `WebGL2RenderingContext.UNPACK_ROW_LENGTH` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNPACK_ROW_LENGTH: u32 = 3314u64 as u32;

    ///The `WebGL2RenderingContext.UNPACK_SKIP_ROWS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNPACK_SKIP_ROWS: u32 = 3315u64 as u32;

    ///The `WebGL2RenderingContext.UNPACK_SKIP_PIXELS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNPACK_SKIP_PIXELS: u32 = 3316u64 as u32;

    ///The `WebGL2RenderingContext.PACK_ROW_LENGTH` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const PACK_ROW_LENGTH: u32 = 3330u64 as u32;

    ///The `WebGL2RenderingContext.PACK_SKIP_ROWS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const PACK_SKIP_ROWS: u32 = 3331u64 as u32;

    ///The `WebGL2RenderingContext.PACK_SKIP_PIXELS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const PACK_SKIP_PIXELS: u32 = 3332u64 as u32;

    ///The `WebGL2RenderingContext.COLOR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COLOR: u32 = 6144u64 as u32;

    ///The `WebGL2RenderingContext.DEPTH` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DEPTH: u32 = 6145u64 as u32;

    ///The `WebGL2RenderingContext.STENCIL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STENCIL: u32 = 6146u64 as u32;

    ///The `WebGL2RenderingContext.RED` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RED: u32 = 6403u64 as u32;

    ///The `WebGL2RenderingContext.RGB8` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGB8: u32 = 32849u64 as u32;

    ///The `WebGL2RenderingContext.RGBA8` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGBA8: u32 = 32856u64 as u32;

    ///The `WebGL2RenderingContext.RGB10_A2` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGB10_A2: u32 = 32857u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_BINDING_3D` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_BINDING_3D: u32 = 32874u64 as u32;

    ///The `WebGL2RenderingContext.UNPACK_SKIP_IMAGES` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNPACK_SKIP_IMAGES: u32 = 32877u64 as u32;

    ///The `WebGL2RenderingContext.UNPACK_IMAGE_HEIGHT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNPACK_IMAGE_HEIGHT: u32 = 32878u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_3D` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_3D: u32 = 32879u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_WRAP_R` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_WRAP_R: u32 = 32882u64 as u32;

    ///The `WebGL2RenderingContext.MAX_3D_TEXTURE_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_3D_TEXTURE_SIZE: u32 = 32883u64 as u32;

    ///The `WebGL2RenderingContext.UNSIGNED_INT_2_10_10_10_REV` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNSIGNED_INT_2_10_10_10_REV: u32 = 33640u64 as u32;

    ///The `WebGL2RenderingContext.MAX_ELEMENTS_VERTICES` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_ELEMENTS_VERTICES: u32 = 33000u64 as u32;

    ///The `WebGL2RenderingContext.MAX_ELEMENTS_INDICES` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_ELEMENTS_INDICES: u32 = 33001u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_MIN_LOD` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_MIN_LOD: u32 = 33082u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_MAX_LOD` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_MAX_LOD: u32 = 33083u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_BASE_LEVEL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_BASE_LEVEL: u32 = 33084u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_MAX_LEVEL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_MAX_LEVEL: u32 = 33085u64 as u32;

    ///The `WebGL2RenderingContext.MIN` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MIN: u32 = 32775u64 as u32;

    ///The `WebGL2RenderingContext.MAX` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX: u32 = 32776u64 as u32;

    ///The `WebGL2RenderingContext.DEPTH_COMPONENT24` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DEPTH_COMPONENT24: u32 = 33190u64 as u32;

    ///The `WebGL2RenderingContext.MAX_TEXTURE_LOD_BIAS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_TEXTURE_LOD_BIAS: u32 = 34045u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_COMPARE_MODE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_COMPARE_MODE: u32 = 34892u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_COMPARE_FUNC` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_COMPARE_FUNC: u32 = 34893u64 as u32;

    ///The `WebGL2RenderingContext.CURRENT_QUERY` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const CURRENT_QUERY: u32 = 34917u64 as u32;

    ///The `WebGL2RenderingContext.QUERY_RESULT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const QUERY_RESULT: u32 = 34918u64 as u32;

    ///The `WebGL2RenderingContext.QUERY_RESULT_AVAILABLE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const QUERY_RESULT_AVAILABLE: u32 = 34919u64 as u32;

    ///The `WebGL2RenderingContext.STREAM_READ` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STREAM_READ: u32 = 35041u64 as u32;

    ///The `WebGL2RenderingContext.STREAM_COPY` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STREAM_COPY: u32 = 35042u64 as u32;

    ///The `WebGL2RenderingContext.STATIC_READ` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STATIC_READ: u32 = 35045u64 as u32;

    ///The `WebGL2RenderingContext.STATIC_COPY` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STATIC_COPY: u32 = 35046u64 as u32;

    ///The `WebGL2RenderingContext.DYNAMIC_READ` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DYNAMIC_READ: u32 = 35049u64 as u32;

    ///The `WebGL2RenderingContext.DYNAMIC_COPY` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DYNAMIC_COPY: u32 = 35050u64 as u32;

    ///The `WebGL2RenderingContext.MAX_DRAW_BUFFERS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_DRAW_BUFFERS: u32 = 34852u64 as u32;

    ///The `WebGL2RenderingContext.DRAW_BUFFER0` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DRAW_BUFFER0: u32 = 34853u64 as u32;

    ///The `WebGL2RenderingContext.DRAW_BUFFER1` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DRAW_BUFFER1: u32 = 34854u64 as u32;

    ///The `WebGL2RenderingContext.DRAW_BUFFER2` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DRAW_BUFFER2: u32 = 34855u64 as u32;

    ///The `WebGL2RenderingContext.DRAW_BUFFER3` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DRAW_BUFFER3: u32 = 34856u64 as u32;

    ///The `WebGL2RenderingContext.DRAW_BUFFER4` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DRAW_BUFFER4: u32 = 34857u64 as u32;

    ///The `WebGL2RenderingContext.DRAW_BUFFER5` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DRAW_BUFFER5: u32 = 34858u64 as u32;

    ///The `WebGL2RenderingContext.DRAW_BUFFER6` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DRAW_BUFFER6: u32 = 34859u64 as u32;

    ///The `WebGL2RenderingContext.DRAW_BUFFER7` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DRAW_BUFFER7: u32 = 34860u64 as u32;

    ///The `WebGL2RenderingContext.DRAW_BUFFER8` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DRAW_BUFFER8: u32 = 34861u64 as u32;

    ///The `WebGL2RenderingContext.DRAW_BUFFER9` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DRAW_BUFFER9: u32 = 34862u64 as u32;

    ///The `WebGL2RenderingContext.DRAW_BUFFER10` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DRAW_BUFFER10: u32 = 34863u64 as u32;

    ///The `WebGL2RenderingContext.DRAW_BUFFER11` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DRAW_BUFFER11: u32 = 34864u64 as u32;

    ///The `WebGL2RenderingContext.DRAW_BUFFER12` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DRAW_BUFFER12: u32 = 34865u64 as u32;

    ///The `WebGL2RenderingContext.DRAW_BUFFER13` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DRAW_BUFFER13: u32 = 34866u64 as u32;

    ///The `WebGL2RenderingContext.DRAW_BUFFER14` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DRAW_BUFFER14: u32 = 34867u64 as u32;

    ///The `WebGL2RenderingContext.DRAW_BUFFER15` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DRAW_BUFFER15: u32 = 34868u64 as u32;

    ///The `WebGL2RenderingContext.MAX_FRAGMENT_UNIFORM_COMPONENTS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: u32 = 35657u64 as u32;

    ///The `WebGL2RenderingContext.MAX_VERTEX_UNIFORM_COMPONENTS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_VERTEX_UNIFORM_COMPONENTS: u32 = 35658u64 as u32;

    ///The `WebGL2RenderingContext.SAMPLER_3D` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SAMPLER_3D: u32 = 35679u64 as u32;

    ///The `WebGL2RenderingContext.SAMPLER_2D_SHADOW` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SAMPLER_2D_SHADOW: u32 = 35682u64 as u32;

    ///The `WebGL2RenderingContext.FRAGMENT_SHADER_DERIVATIVE_HINT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAGMENT_SHADER_DERIVATIVE_HINT: u32 = 35723u64 as u32;

    ///The `WebGL2RenderingContext.PIXEL_PACK_BUFFER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const PIXEL_PACK_BUFFER: u32 = 35051u64 as u32;

    ///The `WebGL2RenderingContext.PIXEL_UNPACK_BUFFER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const PIXEL_UNPACK_BUFFER: u32 = 35052u64 as u32;

    ///The `WebGL2RenderingContext.PIXEL_PACK_BUFFER_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const PIXEL_PACK_BUFFER_BINDING: u32 = 35053u64 as u32;

    ///The `WebGL2RenderingContext.PIXEL_UNPACK_BUFFER_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const PIXEL_UNPACK_BUFFER_BINDING: u32 = 35055u64 as u32;

    ///The `WebGL2RenderingContext.FLOAT_MAT2x3` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FLOAT_MAT2X3: u32 = 35685u64 as u32;

    ///The `WebGL2RenderingContext.FLOAT_MAT2x4` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FLOAT_MAT2X4: u32 = 35686u64 as u32;

    ///The `WebGL2RenderingContext.FLOAT_MAT3x2` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FLOAT_MAT3X2: u32 = 35687u64 as u32;

    ///The `WebGL2RenderingContext.FLOAT_MAT3x4` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FLOAT_MAT3X4: u32 = 35688u64 as u32;

    ///The `WebGL2RenderingContext.FLOAT_MAT4x2` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FLOAT_MAT4X2: u32 = 35689u64 as u32;

    ///The `WebGL2RenderingContext.FLOAT_MAT4x3` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FLOAT_MAT4X3: u32 = 35690u64 as u32;

    ///The `WebGL2RenderingContext.SRGB` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SRGB: u32 = 35904u64 as u32;

    ///The `WebGL2RenderingContext.SRGB8` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SRGB8: u32 = 35905u64 as u32;

    ///The `WebGL2RenderingContext.SRGB8_ALPHA8` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SRGB8_ALPHA8: u32 = 35907u64 as u32;

    ///The `WebGL2RenderingContext.COMPARE_REF_TO_TEXTURE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COMPARE_REF_TO_TEXTURE: u32 = 34894u64 as u32;

    ///The `WebGL2RenderingContext.RGBA32F` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGBA32F: u32 = 34836u64 as u32;

    ///The `WebGL2RenderingContext.RGB32F` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGB32F: u32 = 34837u64 as u32;

    ///The `WebGL2RenderingContext.RGBA16F` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGBA16F: u32 = 34842u64 as u32;

    ///The `WebGL2RenderingContext.RGB16F` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGB16F: u32 = 34843u64 as u32;

    ///The `WebGL2RenderingContext.VERTEX_ATTRIB_ARRAY_INTEGER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const VERTEX_ATTRIB_ARRAY_INTEGER: u32 = 35069u64 as u32;

    ///The `WebGL2RenderingContext.MAX_ARRAY_TEXTURE_LAYERS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_ARRAY_TEXTURE_LAYERS: u32 = 35071u64 as u32;

    ///The `WebGL2RenderingContext.MIN_PROGRAM_TEXEL_OFFSET` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MIN_PROGRAM_TEXEL_OFFSET: u32 = 35076u64 as u32;

    ///The `WebGL2RenderingContext.MAX_PROGRAM_TEXEL_OFFSET` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_PROGRAM_TEXEL_OFFSET: u32 = 35077u64 as u32;

    ///The `WebGL2RenderingContext.MAX_VARYING_COMPONENTS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_VARYING_COMPONENTS: u32 = 35659u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_2D_ARRAY` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_2D_ARRAY: u32 = 35866u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_BINDING_2D_ARRAY` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_BINDING_2D_ARRAY: u32 = 35869u64 as u32;

    ///The `WebGL2RenderingContext.R11F_G11F_B10F` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const R11F_G11F_B10F: u32 = 35898u64 as u32;

    ///The `WebGL2RenderingContext.UNSIGNED_INT_10F_11F_11F_REV` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNSIGNED_INT_10F_11F_11F_REV: u32 = 35899u64 as u32;

    ///The `WebGL2RenderingContext.RGB9_E5` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGB9_E5: u32 = 35901u64 as u32;

    ///The `WebGL2RenderingContext.UNSIGNED_INT_5_9_9_9_REV` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNSIGNED_INT_5_9_9_9_REV: u32 = 35902u64 as u32;

    ///The `WebGL2RenderingContext.TRANSFORM_FEEDBACK_BUFFER_MODE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TRANSFORM_FEEDBACK_BUFFER_MODE: u32 = 35967u64 as u32;

    ///The `WebGL2RenderingContext.MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: u32 = 35968u64 as u32;

    ///The `WebGL2RenderingContext.TRANSFORM_FEEDBACK_VARYINGS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TRANSFORM_FEEDBACK_VARYINGS: u32 = 35971u64 as u32;

    ///The `WebGL2RenderingContext.TRANSFORM_FEEDBACK_BUFFER_START` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TRANSFORM_FEEDBACK_BUFFER_START: u32 = 35972u64 as u32;

    ///The `WebGL2RenderingContext.TRANSFORM_FEEDBACK_BUFFER_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: u32 = 35973u64 as u32;

    ///The `WebGL2RenderingContext.TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: u32 = 35976u64 as u32;

    ///The `WebGL2RenderingContext.RASTERIZER_DISCARD` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RASTERIZER_DISCARD: u32 = 35977u64 as u32;

    ///The `WebGL2RenderingContext.MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: u32 = 35978u64 as u32;

    ///The `WebGL2RenderingContext.MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: u32 = 35979u64 as u32;

    ///The `WebGL2RenderingContext.INTERLEAVED_ATTRIBS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const INTERLEAVED_ATTRIBS: u32 = 35980u64 as u32;

    ///The `WebGL2RenderingContext.SEPARATE_ATTRIBS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SEPARATE_ATTRIBS: u32 = 35981u64 as u32;

    ///The `WebGL2RenderingContext.TRANSFORM_FEEDBACK_BUFFER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TRANSFORM_FEEDBACK_BUFFER: u32 = 35982u64 as u32;

    ///The `WebGL2RenderingContext.TRANSFORM_FEEDBACK_BUFFER_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: u32 = 35983u64 as u32;

    ///The `WebGL2RenderingContext.RGBA32UI` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGBA32UI: u32 = 36208u64 as u32;

    ///The `WebGL2RenderingContext.RGB32UI` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGB32UI: u32 = 36209u64 as u32;

    ///The `WebGL2RenderingContext.RGBA16UI` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGBA16UI: u32 = 36214u64 as u32;

    ///The `WebGL2RenderingContext.RGB16UI` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGB16UI: u32 = 36215u64 as u32;

    ///The `WebGL2RenderingContext.RGBA8UI` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGBA8UI: u32 = 36220u64 as u32;

    ///The `WebGL2RenderingContext.RGB8UI` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGB8UI: u32 = 36221u64 as u32;

    ///The `WebGL2RenderingContext.RGBA32I` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGBA32I: u32 = 36226u64 as u32;

    ///The `WebGL2RenderingContext.RGB32I` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGB32I: u32 = 36227u64 as u32;

    ///The `WebGL2RenderingContext.RGBA16I` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGBA16I: u32 = 36232u64 as u32;

    ///The `WebGL2RenderingContext.RGB16I` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGB16I: u32 = 36233u64 as u32;

    ///The `WebGL2RenderingContext.RGBA8I` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGBA8I: u32 = 36238u64 as u32;

    ///The `WebGL2RenderingContext.RGB8I` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGB8I: u32 = 36239u64 as u32;

    ///The `WebGL2RenderingContext.RED_INTEGER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RED_INTEGER: u32 = 36244u64 as u32;

    ///The `WebGL2RenderingContext.RGB_INTEGER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGB_INTEGER: u32 = 36248u64 as u32;

    ///The `WebGL2RenderingContext.RGBA_INTEGER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGBA_INTEGER: u32 = 36249u64 as u32;

    ///The `WebGL2RenderingContext.SAMPLER_2D_ARRAY` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SAMPLER_2D_ARRAY: u32 = 36289u64 as u32;

    ///The `WebGL2RenderingContext.SAMPLER_2D_ARRAY_SHADOW` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SAMPLER_2D_ARRAY_SHADOW: u32 = 36292u64 as u32;

    ///The `WebGL2RenderingContext.SAMPLER_CUBE_SHADOW` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SAMPLER_CUBE_SHADOW: u32 = 36293u64 as u32;

    ///The `WebGL2RenderingContext.UNSIGNED_INT_VEC2` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNSIGNED_INT_VEC2: u32 = 36294u64 as u32;

    ///The `WebGL2RenderingContext.UNSIGNED_INT_VEC3` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNSIGNED_INT_VEC3: u32 = 36295u64 as u32;

    ///The `WebGL2RenderingContext.UNSIGNED_INT_VEC4` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNSIGNED_INT_VEC4: u32 = 36296u64 as u32;

    ///The `WebGL2RenderingContext.INT_SAMPLER_2D` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const INT_SAMPLER_2D: u32 = 36298u64 as u32;

    ///The `WebGL2RenderingContext.INT_SAMPLER_3D` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const INT_SAMPLER_3D: u32 = 36299u64 as u32;

    ///The `WebGL2RenderingContext.INT_SAMPLER_CUBE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const INT_SAMPLER_CUBE: u32 = 36300u64 as u32;

    ///The `WebGL2RenderingContext.INT_SAMPLER_2D_ARRAY` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const INT_SAMPLER_2D_ARRAY: u32 = 36303u64 as u32;

    ///The `WebGL2RenderingContext.UNSIGNED_INT_SAMPLER_2D` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNSIGNED_INT_SAMPLER_2D: u32 = 36306u64 as u32;

    ///The `WebGL2RenderingContext.UNSIGNED_INT_SAMPLER_3D` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNSIGNED_INT_SAMPLER_3D: u32 = 36307u64 as u32;

    ///The `WebGL2RenderingContext.UNSIGNED_INT_SAMPLER_CUBE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNSIGNED_INT_SAMPLER_CUBE: u32 = 36308u64 as u32;

    ///The `WebGL2RenderingContext.UNSIGNED_INT_SAMPLER_2D_ARRAY` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: u32 = 36311u64 as u32;

    ///The `WebGL2RenderingContext.DEPTH_COMPONENT32F` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DEPTH_COMPONENT32F: u32 = 36012u64 as u32;

    ///The `WebGL2RenderingContext.DEPTH32F_STENCIL8` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DEPTH32F_STENCIL8: u32 = 36013u64 as u32;

    ///The `WebGL2RenderingContext.FLOAT_32_UNSIGNED_INT_24_8_REV` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FLOAT_32_UNSIGNED_INT_24_8_REV: u32 = 36269u64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: u32 = 33296u64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: u32 = 33297u64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_RED_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: u32 = 33298u64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_GREEN_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: u32 = 33299u64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_BLUE_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: u32 = 33300u64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: u32 = 33301u64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: u32 = 33302u64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: u32 = 33303u64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER_DEFAULT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER_DEFAULT: u32 = 33304u64 as u32;

    ///The `WebGL2RenderingContext.UNSIGNED_INT_24_8` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNSIGNED_INT_24_8: u32 = 34042u64 as u32;

    ///The `WebGL2RenderingContext.DEPTH24_STENCIL8` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DEPTH24_STENCIL8: u32 = 35056u64 as u32;

    ///The `WebGL2RenderingContext.UNSIGNED_NORMALIZED` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNSIGNED_NORMALIZED: u32 = 35863u64 as u32;

    ///The `WebGL2RenderingContext.DRAW_FRAMEBUFFER_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DRAW_FRAMEBUFFER_BINDING: u32 = 36006u64 as u32;

    ///The `WebGL2RenderingContext.READ_FRAMEBUFFER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const READ_FRAMEBUFFER: u32 = 36008u64 as u32;

    ///The `WebGL2RenderingContext.DRAW_FRAMEBUFFER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DRAW_FRAMEBUFFER: u32 = 36009u64 as u32;

    ///The `WebGL2RenderingContext.READ_FRAMEBUFFER_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const READ_FRAMEBUFFER_BINDING: u32 = 36010u64 as u32;

    ///The `WebGL2RenderingContext.RENDERBUFFER_SAMPLES` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RENDERBUFFER_SAMPLES: u32 = 36011u64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: u32 = 36052u64 as u32;

    ///The `WebGL2RenderingContext.MAX_COLOR_ATTACHMENTS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_COLOR_ATTACHMENTS: u32 = 36063u64 as u32;

    ///The `WebGL2RenderingContext.COLOR_ATTACHMENT1` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COLOR_ATTACHMENT1: u32 = 36065u64 as u32;

    ///The `WebGL2RenderingContext.COLOR_ATTACHMENT2` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COLOR_ATTACHMENT2: u32 = 36066u64 as u32;

    ///The `WebGL2RenderingContext.COLOR_ATTACHMENT3` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COLOR_ATTACHMENT3: u32 = 36067u64 as u32;

    ///The `WebGL2RenderingContext.COLOR_ATTACHMENT4` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COLOR_ATTACHMENT4: u32 = 36068u64 as u32;

    ///The `WebGL2RenderingContext.COLOR_ATTACHMENT5` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COLOR_ATTACHMENT5: u32 = 36069u64 as u32;

    ///The `WebGL2RenderingContext.COLOR_ATTACHMENT6` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COLOR_ATTACHMENT6: u32 = 36070u64 as u32;

    ///The `WebGL2RenderingContext.COLOR_ATTACHMENT7` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COLOR_ATTACHMENT7: u32 = 36071u64 as u32;

    ///The `WebGL2RenderingContext.COLOR_ATTACHMENT8` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COLOR_ATTACHMENT8: u32 = 36072u64 as u32;

    ///The `WebGL2RenderingContext.COLOR_ATTACHMENT9` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COLOR_ATTACHMENT9: u32 = 36073u64 as u32;

    ///The `WebGL2RenderingContext.COLOR_ATTACHMENT10` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COLOR_ATTACHMENT10: u32 = 36074u64 as u32;

    ///The `WebGL2RenderingContext.COLOR_ATTACHMENT11` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COLOR_ATTACHMENT11: u32 = 36075u64 as u32;

    ///The `WebGL2RenderingContext.COLOR_ATTACHMENT12` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COLOR_ATTACHMENT12: u32 = 36076u64 as u32;

    ///The `WebGL2RenderingContext.COLOR_ATTACHMENT13` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COLOR_ATTACHMENT13: u32 = 36077u64 as u32;

    ///The `WebGL2RenderingContext.COLOR_ATTACHMENT14` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COLOR_ATTACHMENT14: u32 = 36078u64 as u32;

    ///The `WebGL2RenderingContext.COLOR_ATTACHMENT15` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COLOR_ATTACHMENT15: u32 = 36079u64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER_INCOMPLETE_MULTISAMPLE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: u32 = 36182u64 as u32;

    ///The `WebGL2RenderingContext.MAX_SAMPLES` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_SAMPLES: u32 = 36183u64 as u32;

    ///The `WebGL2RenderingContext.HALF_FLOAT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const HALF_FLOAT: u32 = 5131u64 as u32;

    ///The `WebGL2RenderingContext.RG` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RG: u32 = 33319u64 as u32;

    ///The `WebGL2RenderingContext.RG_INTEGER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RG_INTEGER: u32 = 33320u64 as u32;

    ///The `WebGL2RenderingContext.R8` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const R8: u32 = 33321u64 as u32;

    ///The `WebGL2RenderingContext.RG8` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RG8: u32 = 33323u64 as u32;

    ///The `WebGL2RenderingContext.R16F` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const R16F: u32 = 33325u64 as u32;

    ///The `WebGL2RenderingContext.R32F` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const R32F: u32 = 33326u64 as u32;

    ///The `WebGL2RenderingContext.RG16F` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RG16F: u32 = 33327u64 as u32;

    ///The `WebGL2RenderingContext.RG32F` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RG32F: u32 = 33328u64 as u32;

    ///The `WebGL2RenderingContext.R8I` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const R8I: u32 = 33329u64 as u32;

    ///The `WebGL2RenderingContext.R8UI` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const R8UI: u32 = 33330u64 as u32;

    ///The `WebGL2RenderingContext.R16I` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const R16I: u32 = 33331u64 as u32;

    ///The `WebGL2RenderingContext.R16UI` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const R16UI: u32 = 33332u64 as u32;

    ///The `WebGL2RenderingContext.R32I` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const R32I: u32 = 33333u64 as u32;

    ///The `WebGL2RenderingContext.R32UI` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const R32UI: u32 = 33334u64 as u32;

    ///The `WebGL2RenderingContext.RG8I` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RG8I: u32 = 33335u64 as u32;

    ///The `WebGL2RenderingContext.RG8UI` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RG8UI: u32 = 33336u64 as u32;

    ///The `WebGL2RenderingContext.RG16I` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RG16I: u32 = 33337u64 as u32;

    ///The `WebGL2RenderingContext.RG16UI` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RG16UI: u32 = 33338u64 as u32;

    ///The `WebGL2RenderingContext.RG32I` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RG32I: u32 = 33339u64 as u32;

    ///The `WebGL2RenderingContext.RG32UI` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RG32UI: u32 = 33340u64 as u32;

    ///The `WebGL2RenderingContext.VERTEX_ARRAY_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const VERTEX_ARRAY_BINDING: u32 = 34229u64 as u32;

    ///The `WebGL2RenderingContext.R8_SNORM` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const R8_SNORM: u32 = 36756u64 as u32;

    ///The `WebGL2RenderingContext.RG8_SNORM` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RG8_SNORM: u32 = 36757u64 as u32;

    ///The `WebGL2RenderingContext.RGB8_SNORM` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGB8_SNORM: u32 = 36758u64 as u32;

    ///The `WebGL2RenderingContext.RGBA8_SNORM` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGBA8_SNORM: u32 = 36759u64 as u32;

    ///The `WebGL2RenderingContext.SIGNED_NORMALIZED` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SIGNED_NORMALIZED: u32 = 36764u64 as u32;

    ///The `WebGL2RenderingContext.COPY_READ_BUFFER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COPY_READ_BUFFER: u32 = 36662u64 as u32;

    ///The `WebGL2RenderingContext.COPY_WRITE_BUFFER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COPY_WRITE_BUFFER: u32 = 36663u64 as u32;

    ///The `WebGL2RenderingContext.COPY_READ_BUFFER_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COPY_READ_BUFFER_BINDING: u32 = 36662u64 as u32;

    ///The `WebGL2RenderingContext.COPY_WRITE_BUFFER_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COPY_WRITE_BUFFER_BINDING: u32 = 36663u64 as u32;

    ///The `WebGL2RenderingContext.UNIFORM_BUFFER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNIFORM_BUFFER: u32 = 35345u64 as u32;

    ///The `WebGL2RenderingContext.UNIFORM_BUFFER_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNIFORM_BUFFER_BINDING: u32 = 35368u64 as u32;

    ///The `WebGL2RenderingContext.UNIFORM_BUFFER_START` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNIFORM_BUFFER_START: u32 = 35369u64 as u32;

    ///The `WebGL2RenderingContext.UNIFORM_BUFFER_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNIFORM_BUFFER_SIZE: u32 = 35370u64 as u32;

    ///The `WebGL2RenderingContext.MAX_VERTEX_UNIFORM_BLOCKS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_VERTEX_UNIFORM_BLOCKS: u32 = 35371u64 as u32;

    ///The `WebGL2RenderingContext.MAX_FRAGMENT_UNIFORM_BLOCKS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_FRAGMENT_UNIFORM_BLOCKS: u32 = 35373u64 as u32;

    ///The `WebGL2RenderingContext.MAX_COMBINED_UNIFORM_BLOCKS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_COMBINED_UNIFORM_BLOCKS: u32 = 35374u64 as u32;

    ///The `WebGL2RenderingContext.MAX_UNIFORM_BUFFER_BINDINGS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_UNIFORM_BUFFER_BINDINGS: u32 = 35375u64 as u32;

    ///The `WebGL2RenderingContext.MAX_UNIFORM_BLOCK_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_UNIFORM_BLOCK_SIZE: u32 = 35376u64 as u32;

    ///The `WebGL2RenderingContext.MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: u32 = 35377u64 as u32;

    ///The `WebGL2RenderingContext.MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: u32 = 35379u64 as u32;

    ///The `WebGL2RenderingContext.UNIFORM_BUFFER_OFFSET_ALIGNMENT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: u32 = 35380u64 as u32;

    ///The `WebGL2RenderingContext.ACTIVE_UNIFORM_BLOCKS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ACTIVE_UNIFORM_BLOCKS: u32 = 35382u64 as u32;

    ///The `WebGL2RenderingContext.UNIFORM_TYPE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNIFORM_TYPE: u32 = 35383u64 as u32;

    ///The `WebGL2RenderingContext.UNIFORM_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNIFORM_SIZE: u32 = 35384u64 as u32;

    ///The `WebGL2RenderingContext.UNIFORM_BLOCK_INDEX` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNIFORM_BLOCK_INDEX: u32 = 35386u64 as u32;

    ///The `WebGL2RenderingContext.UNIFORM_OFFSET` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNIFORM_OFFSET: u32 = 35387u64 as u32;

    ///The `WebGL2RenderingContext.UNIFORM_ARRAY_STRIDE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNIFORM_ARRAY_STRIDE: u32 = 35388u64 as u32;

    ///The `WebGL2RenderingContext.UNIFORM_MATRIX_STRIDE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNIFORM_MATRIX_STRIDE: u32 = 35389u64 as u32;

    ///The `WebGL2RenderingContext.UNIFORM_IS_ROW_MAJOR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNIFORM_IS_ROW_MAJOR: u32 = 35390u64 as u32;

    ///The `WebGL2RenderingContext.UNIFORM_BLOCK_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNIFORM_BLOCK_BINDING: u32 = 35391u64 as u32;

    ///The `WebGL2RenderingContext.UNIFORM_BLOCK_DATA_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNIFORM_BLOCK_DATA_SIZE: u32 = 35392u64 as u32;

    ///The `WebGL2RenderingContext.UNIFORM_BLOCK_ACTIVE_UNIFORMS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: u32 = 35394u64 as u32;

    ///The `WebGL2RenderingContext.UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: u32 = 35395u64 as u32;

    ///The `WebGL2RenderingContext.UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: u32 = 35396u64 as u32;

    ///The `WebGL2RenderingContext.UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: u32 = 35398u64 as u32;

    ///The `WebGL2RenderingContext.INVALID_INDEX` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const INVALID_INDEX: u32 = 4294967295u64 as u32;

    ///The `WebGL2RenderingContext.MAX_VERTEX_OUTPUT_COMPONENTS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_VERTEX_OUTPUT_COMPONENTS: u32 = 37154u64 as u32;

    ///The `WebGL2RenderingContext.MAX_FRAGMENT_INPUT_COMPONENTS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_FRAGMENT_INPUT_COMPONENTS: u32 = 37157u64 as u32;

    ///The `WebGL2RenderingContext.MAX_SERVER_WAIT_TIMEOUT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_SERVER_WAIT_TIMEOUT: u32 = 37137u64 as u32;

    ///The `WebGL2RenderingContext.OBJECT_TYPE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const OBJECT_TYPE: u32 = 37138u64 as u32;

    ///The `WebGL2RenderingContext.SYNC_CONDITION` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SYNC_CONDITION: u32 = 37139u64 as u32;

    ///The `WebGL2RenderingContext.SYNC_STATUS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SYNC_STATUS: u32 = 37140u64 as u32;

    ///The `WebGL2RenderingContext.SYNC_FLAGS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SYNC_FLAGS: u32 = 37141u64 as u32;

    ///The `WebGL2RenderingContext.SYNC_FENCE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SYNC_FENCE: u32 = 37142u64 as u32;

    ///The `WebGL2RenderingContext.SYNC_GPU_COMMANDS_COMPLETE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SYNC_GPU_COMMANDS_COMPLETE: u32 = 37143u64 as u32;

    ///The `WebGL2RenderingContext.UNSIGNALED` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNSIGNALED: u32 = 37144u64 as u32;

    ///The `WebGL2RenderingContext.SIGNALED` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SIGNALED: u32 = 37145u64 as u32;

    ///The `WebGL2RenderingContext.ALREADY_SIGNALED` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ALREADY_SIGNALED: u32 = 37146u64 as u32;

    ///The `WebGL2RenderingContext.TIMEOUT_EXPIRED` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TIMEOUT_EXPIRED: u32 = 37147u64 as u32;

    ///The `WebGL2RenderingContext.CONDITION_SATISFIED` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const CONDITION_SATISFIED: u32 = 37148u64 as u32;

    ///The `WebGL2RenderingContext.WAIT_FAILED` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const WAIT_FAILED: u32 = 37149u64 as u32;

    ///The `WebGL2RenderingContext.SYNC_FLUSH_COMMANDS_BIT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SYNC_FLUSH_COMMANDS_BIT: u32 = 1u64 as u32;

    ///The `WebGL2RenderingContext.VERTEX_ATTRIB_ARRAY_DIVISOR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const VERTEX_ATTRIB_ARRAY_DIVISOR: u32 = 35070u64 as u32;

    ///The `WebGL2RenderingContext.ANY_SAMPLES_PASSED` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ANY_SAMPLES_PASSED: u32 = 35887u64 as u32;

    ///The `WebGL2RenderingContext.ANY_SAMPLES_PASSED_CONSERVATIVE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ANY_SAMPLES_PASSED_CONSERVATIVE: u32 = 36202u64 as u32;

    ///The `WebGL2RenderingContext.SAMPLER_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SAMPLER_BINDING: u32 = 35097u64 as u32;

    ///The `WebGL2RenderingContext.RGB10_A2UI` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGB10_A2UI: u32 = 36975u64 as u32;

    ///The `WebGL2RenderingContext.INT_2_10_10_10_REV` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const INT_2_10_10_10_REV: u32 = 36255u64 as u32;

    ///The `WebGL2RenderingContext.TRANSFORM_FEEDBACK` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TRANSFORM_FEEDBACK: u32 = 36386u64 as u32;

    ///The `WebGL2RenderingContext.TRANSFORM_FEEDBACK_PAUSED` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TRANSFORM_FEEDBACK_PAUSED: u32 = 36387u64 as u32;

    ///The `WebGL2RenderingContext.TRANSFORM_FEEDBACK_ACTIVE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TRANSFORM_FEEDBACK_ACTIVE: u32 = 36388u64 as u32;

    ///The `WebGL2RenderingContext.TRANSFORM_FEEDBACK_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TRANSFORM_FEEDBACK_BINDING: u32 = 36389u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_IMMUTABLE_FORMAT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_IMMUTABLE_FORMAT: u32 = 37167u64 as u32;

    ///The `WebGL2RenderingContext.MAX_ELEMENT_INDEX` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_ELEMENT_INDEX: u32 = 36203u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_IMMUTABLE_LEVELS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_IMMUTABLE_LEVELS: u32 = 33503u64 as u32;

    ///The `WebGL2RenderingContext.TIMEOUT_IGNORED` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TIMEOUT_IGNORED: f64 = -1i64 as f64;

    ///The `WebGL2RenderingContext.MAX_CLIENT_WAIT_TIMEOUT_WEBGL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_CLIENT_WAIT_TIMEOUT_WEBGL: u32 = 37447u64 as u32;

    ///The `WebGL2RenderingContext.DEPTH_BUFFER_BIT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DEPTH_BUFFER_BIT: u32 = 256u64 as u32;

    ///The `WebGL2RenderingContext.STENCIL_BUFFER_BIT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STENCIL_BUFFER_BIT: u32 = 1024u64 as u32;

    ///The `WebGL2RenderingContext.COLOR_BUFFER_BIT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COLOR_BUFFER_BIT: u32 = 16384u64 as u32;

    ///The `WebGL2RenderingContext.POINTS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const POINTS: u32 = 0u64 as u32;

    ///The `WebGL2RenderingContext.LINES` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const LINES: u32 = 1u64 as u32;

    ///The `WebGL2RenderingContext.LINE_LOOP` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const LINE_LOOP: u32 = 2u64 as u32;

    ///The `WebGL2RenderingContext.LINE_STRIP` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const LINE_STRIP: u32 = 3u64 as u32;

    ///The `WebGL2RenderingContext.TRIANGLES` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TRIANGLES: u32 = 4u64 as u32;

    ///The `WebGL2RenderingContext.TRIANGLE_STRIP` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TRIANGLE_STRIP: u32 = 5u64 as u32;

    ///The `WebGL2RenderingContext.TRIANGLE_FAN` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TRIANGLE_FAN: u32 = 6u64 as u32;

    ///The `WebGL2RenderingContext.ZERO` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ZERO: u32 = 0i64 as u32;

    ///The `WebGL2RenderingContext.ONE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ONE: u32 = 1u64 as u32;

    ///The `WebGL2RenderingContext.SRC_COLOR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SRC_COLOR: u32 = 768u64 as u32;

    ///The `WebGL2RenderingContext.ONE_MINUS_SRC_COLOR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ONE_MINUS_SRC_COLOR: u32 = 769u64 as u32;

    ///The `WebGL2RenderingContext.SRC_ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SRC_ALPHA: u32 = 770u64 as u32;

    ///The `WebGL2RenderingContext.ONE_MINUS_SRC_ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ONE_MINUS_SRC_ALPHA: u32 = 771u64 as u32;

    ///The `WebGL2RenderingContext.DST_ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DST_ALPHA: u32 = 772u64 as u32;

    ///The `WebGL2RenderingContext.ONE_MINUS_DST_ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ONE_MINUS_DST_ALPHA: u32 = 773u64 as u32;

    ///The `WebGL2RenderingContext.DST_COLOR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DST_COLOR: u32 = 774u64 as u32;

    ///The `WebGL2RenderingContext.ONE_MINUS_DST_COLOR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ONE_MINUS_DST_COLOR: u32 = 775u64 as u32;

    ///The `WebGL2RenderingContext.SRC_ALPHA_SATURATE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SRC_ALPHA_SATURATE: u32 = 776u64 as u32;

    ///The `WebGL2RenderingContext.FUNC_ADD` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FUNC_ADD: u32 = 32774u64 as u32;

    ///The `WebGL2RenderingContext.BLEND_EQUATION` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const BLEND_EQUATION: u32 = 32777u64 as u32;

    ///The `WebGL2RenderingContext.BLEND_EQUATION_RGB` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const BLEND_EQUATION_RGB: u32 = 32777u64 as u32;

    ///The `WebGL2RenderingContext.BLEND_EQUATION_ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const BLEND_EQUATION_ALPHA: u32 = 34877u64 as u32;

    ///The `WebGL2RenderingContext.FUNC_SUBTRACT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FUNC_SUBTRACT: u32 = 32778u64 as u32;

    ///The `WebGL2RenderingContext.FUNC_REVERSE_SUBTRACT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FUNC_REVERSE_SUBTRACT: u32 = 32779u64 as u32;

    ///The `WebGL2RenderingContext.BLEND_DST_RGB` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const BLEND_DST_RGB: u32 = 32968u64 as u32;

    ///The `WebGL2RenderingContext.BLEND_SRC_RGB` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const BLEND_SRC_RGB: u32 = 32969u64 as u32;

    ///The `WebGL2RenderingContext.BLEND_DST_ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const BLEND_DST_ALPHA: u32 = 32970u64 as u32;

    ///The `WebGL2RenderingContext.BLEND_SRC_ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const BLEND_SRC_ALPHA: u32 = 32971u64 as u32;

    ///The `WebGL2RenderingContext.CONSTANT_COLOR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const CONSTANT_COLOR: u32 = 32769u64 as u32;

    ///The `WebGL2RenderingContext.ONE_MINUS_CONSTANT_COLOR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ONE_MINUS_CONSTANT_COLOR: u32 = 32770u64 as u32;

    ///The `WebGL2RenderingContext.CONSTANT_ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const CONSTANT_ALPHA: u32 = 32771u64 as u32;

    ///The `WebGL2RenderingContext.ONE_MINUS_CONSTANT_ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ONE_MINUS_CONSTANT_ALPHA: u32 = 32772u64 as u32;

    ///The `WebGL2RenderingContext.BLEND_COLOR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const BLEND_COLOR: u32 = 32773u64 as u32;

    ///The `WebGL2RenderingContext.ARRAY_BUFFER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ARRAY_BUFFER: u32 = 34962u64 as u32;

    ///The `WebGL2RenderingContext.ELEMENT_ARRAY_BUFFER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ELEMENT_ARRAY_BUFFER: u32 = 34963u64 as u32;

    ///The `WebGL2RenderingContext.ARRAY_BUFFER_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ARRAY_BUFFER_BINDING: u32 = 34964u64 as u32;

    ///The `WebGL2RenderingContext.ELEMENT_ARRAY_BUFFER_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ELEMENT_ARRAY_BUFFER_BINDING: u32 = 34965u64 as u32;

    ///The `WebGL2RenderingContext.STREAM_DRAW` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STREAM_DRAW: u32 = 35040u64 as u32;

    ///The `WebGL2RenderingContext.STATIC_DRAW` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STATIC_DRAW: u32 = 35044u64 as u32;

    ///The `WebGL2RenderingContext.DYNAMIC_DRAW` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DYNAMIC_DRAW: u32 = 35048u64 as u32;

    ///The `WebGL2RenderingContext.BUFFER_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const BUFFER_SIZE: u32 = 34660u64 as u32;

    ///The `WebGL2RenderingContext.BUFFER_USAGE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const BUFFER_USAGE: u32 = 34661u64 as u32;

    ///The `WebGL2RenderingContext.CURRENT_VERTEX_ATTRIB` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const CURRENT_VERTEX_ATTRIB: u32 = 34342u64 as u32;

    ///The `WebGL2RenderingContext.FRONT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRONT: u32 = 1028u64 as u32;

    ///The `WebGL2RenderingContext.BACK` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const BACK: u32 = 1029u64 as u32;

    ///The `WebGL2RenderingContext.FRONT_AND_BACK` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRONT_AND_BACK: u32 = 1032u64 as u32;

    ///The `WebGL2RenderingContext.CULL_FACE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const CULL_FACE: u32 = 2884u64 as u32;

    ///The `WebGL2RenderingContext.BLEND` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const BLEND: u32 = 3042u64 as u32;

    ///The `WebGL2RenderingContext.DITHER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DITHER: u32 = 3024u64 as u32;

    ///The `WebGL2RenderingContext.STENCIL_TEST` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STENCIL_TEST: u32 = 2960u64 as u32;

    ///The `WebGL2RenderingContext.DEPTH_TEST` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DEPTH_TEST: u32 = 2929u64 as u32;

    ///The `WebGL2RenderingContext.SCISSOR_TEST` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SCISSOR_TEST: u32 = 3089u64 as u32;

    ///The `WebGL2RenderingContext.POLYGON_OFFSET_FILL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const POLYGON_OFFSET_FILL: u32 = 32823u64 as u32;

    ///The `WebGL2RenderingContext.SAMPLE_ALPHA_TO_COVERAGE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SAMPLE_ALPHA_TO_COVERAGE: u32 = 32926u64 as u32;

    ///The `WebGL2RenderingContext.SAMPLE_COVERAGE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SAMPLE_COVERAGE: u32 = 32928u64 as u32;

    ///The `WebGL2RenderingContext.NO_ERROR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const NO_ERROR: u32 = 0i64 as u32;

    ///The `WebGL2RenderingContext.INVALID_ENUM` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const INVALID_ENUM: u32 = 1280u64 as u32;

    ///The `WebGL2RenderingContext.INVALID_VALUE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const INVALID_VALUE: u32 = 1281u64 as u32;

    ///The `WebGL2RenderingContext.INVALID_OPERATION` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const INVALID_OPERATION: u32 = 1282u64 as u32;

    ///The `WebGL2RenderingContext.OUT_OF_MEMORY` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const OUT_OF_MEMORY: u32 = 1285u64 as u32;

    ///The `WebGL2RenderingContext.CW` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const CW: u32 = 2304u64 as u32;

    ///The `WebGL2RenderingContext.CCW` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const CCW: u32 = 2305u64 as u32;

    ///The `WebGL2RenderingContext.LINE_WIDTH` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const LINE_WIDTH: u32 = 2849u64 as u32;

    ///The `WebGL2RenderingContext.ALIASED_POINT_SIZE_RANGE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ALIASED_POINT_SIZE_RANGE: u32 = 33901u64 as u32;

    ///The `WebGL2RenderingContext.ALIASED_LINE_WIDTH_RANGE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ALIASED_LINE_WIDTH_RANGE: u32 = 33902u64 as u32;

    ///The `WebGL2RenderingContext.CULL_FACE_MODE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const CULL_FACE_MODE: u32 = 2885u64 as u32;

    ///The `WebGL2RenderingContext.FRONT_FACE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRONT_FACE: u32 = 2886u64 as u32;

    ///The `WebGL2RenderingContext.DEPTH_RANGE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DEPTH_RANGE: u32 = 2928u64 as u32;

    ///The `WebGL2RenderingContext.DEPTH_WRITEMASK` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DEPTH_WRITEMASK: u32 = 2930u64 as u32;

    ///The `WebGL2RenderingContext.DEPTH_CLEAR_VALUE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DEPTH_CLEAR_VALUE: u32 = 2931u64 as u32;

    ///The `WebGL2RenderingContext.DEPTH_FUNC` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DEPTH_FUNC: u32 = 2932u64 as u32;

    ///The `WebGL2RenderingContext.STENCIL_CLEAR_VALUE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STENCIL_CLEAR_VALUE: u32 = 2961u64 as u32;

    ///The `WebGL2RenderingContext.STENCIL_FUNC` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STENCIL_FUNC: u32 = 2962u64 as u32;

    ///The `WebGL2RenderingContext.STENCIL_FAIL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STENCIL_FAIL: u32 = 2964u64 as u32;

    ///The `WebGL2RenderingContext.STENCIL_PASS_DEPTH_FAIL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STENCIL_PASS_DEPTH_FAIL: u32 = 2965u64 as u32;

    ///The `WebGL2RenderingContext.STENCIL_PASS_DEPTH_PASS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STENCIL_PASS_DEPTH_PASS: u32 = 2966u64 as u32;

    ///The `WebGL2RenderingContext.STENCIL_REF` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STENCIL_REF: u32 = 2967u64 as u32;

    ///The `WebGL2RenderingContext.STENCIL_VALUE_MASK` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STENCIL_VALUE_MASK: u32 = 2963u64 as u32;

    ///The `WebGL2RenderingContext.STENCIL_WRITEMASK` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STENCIL_WRITEMASK: u32 = 2968u64 as u32;

    ///The `WebGL2RenderingContext.STENCIL_BACK_FUNC` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STENCIL_BACK_FUNC: u32 = 34816u64 as u32;

    ///The `WebGL2RenderingContext.STENCIL_BACK_FAIL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STENCIL_BACK_FAIL: u32 = 34817u64 as u32;

    ///The `WebGL2RenderingContext.STENCIL_BACK_PASS_DEPTH_FAIL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STENCIL_BACK_PASS_DEPTH_FAIL: u32 = 34818u64 as u32;

    ///The `WebGL2RenderingContext.STENCIL_BACK_PASS_DEPTH_PASS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STENCIL_BACK_PASS_DEPTH_PASS: u32 = 34819u64 as u32;

    ///The `WebGL2RenderingContext.STENCIL_BACK_REF` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STENCIL_BACK_REF: u32 = 36003u64 as u32;

    ///The `WebGL2RenderingContext.STENCIL_BACK_VALUE_MASK` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STENCIL_BACK_VALUE_MASK: u32 = 36004u64 as u32;

    ///The `WebGL2RenderingContext.STENCIL_BACK_WRITEMASK` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STENCIL_BACK_WRITEMASK: u32 = 36005u64 as u32;

    ///The `WebGL2RenderingContext.VIEWPORT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const VIEWPORT: u32 = 2978u64 as u32;

    ///The `WebGL2RenderingContext.SCISSOR_BOX` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SCISSOR_BOX: u32 = 3088u64 as u32;

    ///The `WebGL2RenderingContext.COLOR_CLEAR_VALUE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COLOR_CLEAR_VALUE: u32 = 3106u64 as u32;

    ///The `WebGL2RenderingContext.COLOR_WRITEMASK` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COLOR_WRITEMASK: u32 = 3107u64 as u32;

    ///The `WebGL2RenderingContext.UNPACK_ALIGNMENT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNPACK_ALIGNMENT: u32 = 3317u64 as u32;

    ///The `WebGL2RenderingContext.PACK_ALIGNMENT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const PACK_ALIGNMENT: u32 = 3333u64 as u32;

    ///The `WebGL2RenderingContext.MAX_TEXTURE_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_TEXTURE_SIZE: u32 = 3379u64 as u32;

    ///The `WebGL2RenderingContext.MAX_VIEWPORT_DIMS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_VIEWPORT_DIMS: u32 = 3386u64 as u32;

    ///The `WebGL2RenderingContext.SUBPIXEL_BITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SUBPIXEL_BITS: u32 = 3408u64 as u32;

    ///The `WebGL2RenderingContext.RED_BITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RED_BITS: u32 = 3410u64 as u32;

    ///The `WebGL2RenderingContext.GREEN_BITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const GREEN_BITS: u32 = 3411u64 as u32;

    ///The `WebGL2RenderingContext.BLUE_BITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const BLUE_BITS: u32 = 3412u64 as u32;

    ///The `WebGL2RenderingContext.ALPHA_BITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ALPHA_BITS: u32 = 3413u64 as u32;

    ///The `WebGL2RenderingContext.DEPTH_BITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DEPTH_BITS: u32 = 3414u64 as u32;

    ///The `WebGL2RenderingContext.STENCIL_BITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STENCIL_BITS: u32 = 3415u64 as u32;

    ///The `WebGL2RenderingContext.POLYGON_OFFSET_UNITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const POLYGON_OFFSET_UNITS: u32 = 10752u64 as u32;

    ///The `WebGL2RenderingContext.POLYGON_OFFSET_FACTOR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const POLYGON_OFFSET_FACTOR: u32 = 32824u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_BINDING_2D` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_BINDING_2D: u32 = 32873u64 as u32;

    ///The `WebGL2RenderingContext.SAMPLE_BUFFERS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SAMPLE_BUFFERS: u32 = 32936u64 as u32;

    ///The `WebGL2RenderingContext.SAMPLES` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SAMPLES: u32 = 32937u64 as u32;

    ///The `WebGL2RenderingContext.SAMPLE_COVERAGE_VALUE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SAMPLE_COVERAGE_VALUE: u32 = 32938u64 as u32;

    ///The `WebGL2RenderingContext.SAMPLE_COVERAGE_INVERT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SAMPLE_COVERAGE_INVERT: u32 = 32939u64 as u32;

    ///The `WebGL2RenderingContext.COMPRESSED_TEXTURE_FORMATS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COMPRESSED_TEXTURE_FORMATS: u32 = 34467u64 as u32;

    ///The `WebGL2RenderingContext.DONT_CARE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DONT_CARE: u32 = 4352u64 as u32;

    ///The `WebGL2RenderingContext.FASTEST` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FASTEST: u32 = 4353u64 as u32;

    ///The `WebGL2RenderingContext.NICEST` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const NICEST: u32 = 4354u64 as u32;

    ///The `WebGL2RenderingContext.GENERATE_MIPMAP_HINT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const GENERATE_MIPMAP_HINT: u32 = 33170u64 as u32;

    ///The `WebGL2RenderingContext.BYTE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const BYTE: u32 = 5120u64 as u32;

    ///The `WebGL2RenderingContext.UNSIGNED_BYTE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNSIGNED_BYTE: u32 = 5121u64 as u32;

    ///The `WebGL2RenderingContext.SHORT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SHORT: u32 = 5122u64 as u32;

    ///The `WebGL2RenderingContext.UNSIGNED_SHORT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNSIGNED_SHORT: u32 = 5123u64 as u32;

    ///The `WebGL2RenderingContext.INT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const INT: u32 = 5124u64 as u32;

    ///The `WebGL2RenderingContext.UNSIGNED_INT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNSIGNED_INT: u32 = 5125u64 as u32;

    ///The `WebGL2RenderingContext.FLOAT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FLOAT: u32 = 5126u64 as u32;

    ///The `WebGL2RenderingContext.DEPTH_COMPONENT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DEPTH_COMPONENT: u32 = 6402u64 as u32;

    ///The `WebGL2RenderingContext.ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ALPHA: u32 = 6406u64 as u32;

    ///The `WebGL2RenderingContext.RGB` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGB: u32 = 6407u64 as u32;

    ///The `WebGL2RenderingContext.RGBA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGBA: u32 = 6408u64 as u32;

    ///The `WebGL2RenderingContext.LUMINANCE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const LUMINANCE: u32 = 6409u64 as u32;

    ///The `WebGL2RenderingContext.LUMINANCE_ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const LUMINANCE_ALPHA: u32 = 6410u64 as u32;

    ///The `WebGL2RenderingContext.UNSIGNED_SHORT_4_4_4_4` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNSIGNED_SHORT_4_4_4_4: u32 = 32819u64 as u32;

    ///The `WebGL2RenderingContext.UNSIGNED_SHORT_5_5_5_1` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNSIGNED_SHORT_5_5_5_1: u32 = 32820u64 as u32;

    ///The `WebGL2RenderingContext.UNSIGNED_SHORT_5_6_5` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNSIGNED_SHORT_5_6_5: u32 = 33635u64 as u32;

    ///The `WebGL2RenderingContext.FRAGMENT_SHADER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAGMENT_SHADER: u32 = 35632u64 as u32;

    ///The `WebGL2RenderingContext.VERTEX_SHADER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const VERTEX_SHADER: u32 = 35633u64 as u32;

    ///The `WebGL2RenderingContext.MAX_VERTEX_ATTRIBS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_VERTEX_ATTRIBS: u32 = 34921u64 as u32;

    ///The `WebGL2RenderingContext.MAX_VERTEX_UNIFORM_VECTORS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_VERTEX_UNIFORM_VECTORS: u32 = 36347u64 as u32;

    ///The `WebGL2RenderingContext.MAX_VARYING_VECTORS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_VARYING_VECTORS: u32 = 36348u64 as u32;

    ///The `WebGL2RenderingContext.MAX_COMBINED_TEXTURE_IMAGE_UNITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: u32 = 35661u64 as u32;

    ///The `WebGL2RenderingContext.MAX_VERTEX_TEXTURE_IMAGE_UNITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: u32 = 35660u64 as u32;

    ///The `WebGL2RenderingContext.MAX_TEXTURE_IMAGE_UNITS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_TEXTURE_IMAGE_UNITS: u32 = 34930u64 as u32;

    ///The `WebGL2RenderingContext.MAX_FRAGMENT_UNIFORM_VECTORS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_FRAGMENT_UNIFORM_VECTORS: u32 = 36349u64 as u32;

    ///The `WebGL2RenderingContext.SHADER_TYPE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SHADER_TYPE: u32 = 35663u64 as u32;

    ///The `WebGL2RenderingContext.DELETE_STATUS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DELETE_STATUS: u32 = 35712u64 as u32;

    ///The `WebGL2RenderingContext.LINK_STATUS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const LINK_STATUS: u32 = 35714u64 as u32;

    ///The `WebGL2RenderingContext.VALIDATE_STATUS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const VALIDATE_STATUS: u32 = 35715u64 as u32;

    ///The `WebGL2RenderingContext.ATTACHED_SHADERS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ATTACHED_SHADERS: u32 = 35717u64 as u32;

    ///The `WebGL2RenderingContext.ACTIVE_UNIFORMS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ACTIVE_UNIFORMS: u32 = 35718u64 as u32;

    ///The `WebGL2RenderingContext.ACTIVE_ATTRIBUTES` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ACTIVE_ATTRIBUTES: u32 = 35721u64 as u32;

    ///The `WebGL2RenderingContext.SHADING_LANGUAGE_VERSION` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SHADING_LANGUAGE_VERSION: u32 = 35724u64 as u32;

    ///The `WebGL2RenderingContext.CURRENT_PROGRAM` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const CURRENT_PROGRAM: u32 = 35725u64 as u32;

    ///The `WebGL2RenderingContext.NEVER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const NEVER: u32 = 512u64 as u32;

    ///The `WebGL2RenderingContext.LESS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const LESS: u32 = 513u64 as u32;

    ///The `WebGL2RenderingContext.EQUAL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const EQUAL: u32 = 514u64 as u32;

    ///The `WebGL2RenderingContext.LEQUAL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const LEQUAL: u32 = 515u64 as u32;

    ///The `WebGL2RenderingContext.GREATER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const GREATER: u32 = 516u64 as u32;

    ///The `WebGL2RenderingContext.NOTEQUAL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const NOTEQUAL: u32 = 517u64 as u32;

    ///The `WebGL2RenderingContext.GEQUAL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const GEQUAL: u32 = 518u64 as u32;

    ///The `WebGL2RenderingContext.ALWAYS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ALWAYS: u32 = 519u64 as u32;

    ///The `WebGL2RenderingContext.KEEP` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const KEEP: u32 = 7680u64 as u32;

    ///The `WebGL2RenderingContext.REPLACE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const REPLACE: u32 = 7681u64 as u32;

    ///The `WebGL2RenderingContext.INCR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const INCR: u32 = 7682u64 as u32;

    ///The `WebGL2RenderingContext.DECR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DECR: u32 = 7683u64 as u32;

    ///The `WebGL2RenderingContext.INVERT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const INVERT: u32 = 5386u64 as u32;

    ///The `WebGL2RenderingContext.INCR_WRAP` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const INCR_WRAP: u32 = 34055u64 as u32;

    ///The `WebGL2RenderingContext.DECR_WRAP` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DECR_WRAP: u32 = 34056u64 as u32;

    ///The `WebGL2RenderingContext.VENDOR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const VENDOR: u32 = 7936u64 as u32;

    ///The `WebGL2RenderingContext.RENDERER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RENDERER: u32 = 7937u64 as u32;

    ///The `WebGL2RenderingContext.VERSION` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const VERSION: u32 = 7938u64 as u32;

    ///The `WebGL2RenderingContext.NEAREST` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const NEAREST: u32 = 9728u64 as u32;

    ///The `WebGL2RenderingContext.LINEAR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const LINEAR: u32 = 9729u64 as u32;

    ///The `WebGL2RenderingContext.NEAREST_MIPMAP_NEAREST` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const NEAREST_MIPMAP_NEAREST: u32 = 9984u64 as u32;

    ///The `WebGL2RenderingContext.LINEAR_MIPMAP_NEAREST` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const LINEAR_MIPMAP_NEAREST: u32 = 9985u64 as u32;

    ///The `WebGL2RenderingContext.NEAREST_MIPMAP_LINEAR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const NEAREST_MIPMAP_LINEAR: u32 = 9986u64 as u32;

    ///The `WebGL2RenderingContext.LINEAR_MIPMAP_LINEAR` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const LINEAR_MIPMAP_LINEAR: u32 = 9987u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_MAG_FILTER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_MAG_FILTER: u32 = 10240u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_MIN_FILTER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_MIN_FILTER: u32 = 10241u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_WRAP_S` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_WRAP_S: u32 = 10242u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_WRAP_T` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_WRAP_T: u32 = 10243u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_2D` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_2D: u32 = 3553u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE: u32 = 5890u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_CUBE_MAP` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_CUBE_MAP: u32 = 34067u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_BINDING_CUBE_MAP` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_BINDING_CUBE_MAP: u32 = 34068u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_CUBE_MAP_POSITIVE_X` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_CUBE_MAP_POSITIVE_X: u32 = 34069u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_CUBE_MAP_NEGATIVE_X` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_CUBE_MAP_NEGATIVE_X: u32 = 34070u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_CUBE_MAP_POSITIVE_Y` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_CUBE_MAP_POSITIVE_Y: u32 = 34071u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_CUBE_MAP_NEGATIVE_Y` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: u32 = 34072u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_CUBE_MAP_POSITIVE_Z` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_CUBE_MAP_POSITIVE_Z: u32 = 34073u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE_CUBE_MAP_NEGATIVE_Z` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: u32 = 34074u64 as u32;

    ///The `WebGL2RenderingContext.MAX_CUBE_MAP_TEXTURE_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_CUBE_MAP_TEXTURE_SIZE: u32 = 34076u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE0` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE0: u32 = 33984u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE1` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE1: u32 = 33985u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE2` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE2: u32 = 33986u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE3` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE3: u32 = 33987u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE4` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE4: u32 = 33988u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE5` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE5: u32 = 33989u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE6` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE6: u32 = 33990u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE7` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE7: u32 = 33991u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE8` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE8: u32 = 33992u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE9` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE9: u32 = 33993u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE10` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE10: u32 = 33994u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE11` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE11: u32 = 33995u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE12` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE12: u32 = 33996u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE13` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE13: u32 = 33997u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE14` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE14: u32 = 33998u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE15` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE15: u32 = 33999u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE16` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE16: u32 = 34000u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE17` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE17: u32 = 34001u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE18` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE18: u32 = 34002u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE19` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE19: u32 = 34003u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE20` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE20: u32 = 34004u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE21` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE21: u32 = 34005u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE22` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE22: u32 = 34006u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE23` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE23: u32 = 34007u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE24` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE24: u32 = 34008u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE25` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE25: u32 = 34009u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE26` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE26: u32 = 34010u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE27` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE27: u32 = 34011u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE28` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE28: u32 = 34012u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE29` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE29: u32 = 34013u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE30` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE30: u32 = 34014u64 as u32;

    ///The `WebGL2RenderingContext.TEXTURE31` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const TEXTURE31: u32 = 34015u64 as u32;

    ///The `WebGL2RenderingContext.ACTIVE_TEXTURE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const ACTIVE_TEXTURE: u32 = 34016u64 as u32;

    ///The `WebGL2RenderingContext.REPEAT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const REPEAT: u32 = 10497u64 as u32;

    ///The `WebGL2RenderingContext.CLAMP_TO_EDGE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const CLAMP_TO_EDGE: u32 = 33071u64 as u32;

    ///The `WebGL2RenderingContext.MIRRORED_REPEAT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MIRRORED_REPEAT: u32 = 33648u64 as u32;

    ///The `WebGL2RenderingContext.FLOAT_VEC2` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FLOAT_VEC2: u32 = 35664u64 as u32;

    ///The `WebGL2RenderingContext.FLOAT_VEC3` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FLOAT_VEC3: u32 = 35665u64 as u32;

    ///The `WebGL2RenderingContext.FLOAT_VEC4` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FLOAT_VEC4: u32 = 35666u64 as u32;

    ///The `WebGL2RenderingContext.INT_VEC2` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const INT_VEC2: u32 = 35667u64 as u32;

    ///The `WebGL2RenderingContext.INT_VEC3` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const INT_VEC3: u32 = 35668u64 as u32;

    ///The `WebGL2RenderingContext.INT_VEC4` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const INT_VEC4: u32 = 35669u64 as u32;

    ///The `WebGL2RenderingContext.BOOL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const BOOL: u32 = 35670u64 as u32;

    ///The `WebGL2RenderingContext.BOOL_VEC2` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const BOOL_VEC2: u32 = 35671u64 as u32;

    ///The `WebGL2RenderingContext.BOOL_VEC3` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const BOOL_VEC3: u32 = 35672u64 as u32;

    ///The `WebGL2RenderingContext.BOOL_VEC4` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const BOOL_VEC4: u32 = 35673u64 as u32;

    ///The `WebGL2RenderingContext.FLOAT_MAT2` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FLOAT_MAT2: u32 = 35674u64 as u32;

    ///The `WebGL2RenderingContext.FLOAT_MAT3` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FLOAT_MAT3: u32 = 35675u64 as u32;

    ///The `WebGL2RenderingContext.FLOAT_MAT4` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FLOAT_MAT4: u32 = 35676u64 as u32;

    ///The `WebGL2RenderingContext.SAMPLER_2D` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SAMPLER_2D: u32 = 35678u64 as u32;

    ///The `WebGL2RenderingContext.SAMPLER_CUBE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const SAMPLER_CUBE: u32 = 35680u64 as u32;

    ///The `WebGL2RenderingContext.VERTEX_ATTRIB_ARRAY_ENABLED` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const VERTEX_ATTRIB_ARRAY_ENABLED: u32 = 34338u64 as u32;

    ///The `WebGL2RenderingContext.VERTEX_ATTRIB_ARRAY_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const VERTEX_ATTRIB_ARRAY_SIZE: u32 = 34339u64 as u32;

    ///The `WebGL2RenderingContext.VERTEX_ATTRIB_ARRAY_STRIDE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const VERTEX_ATTRIB_ARRAY_STRIDE: u32 = 34340u64 as u32;

    ///The `WebGL2RenderingContext.VERTEX_ATTRIB_ARRAY_TYPE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const VERTEX_ATTRIB_ARRAY_TYPE: u32 = 34341u64 as u32;

    ///The `WebGL2RenderingContext.VERTEX_ATTRIB_ARRAY_NORMALIZED` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: u32 = 34922u64 as u32;

    ///The `WebGL2RenderingContext.VERTEX_ATTRIB_ARRAY_POINTER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const VERTEX_ATTRIB_ARRAY_POINTER: u32 = 34373u64 as u32;

    ///The `WebGL2RenderingContext.VERTEX_ATTRIB_ARRAY_BUFFER_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: u32 = 34975u64 as u32;

    ///The `WebGL2RenderingContext.IMPLEMENTATION_COLOR_READ_TYPE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const IMPLEMENTATION_COLOR_READ_TYPE: u32 = 35738u64 as u32;

    ///The `WebGL2RenderingContext.IMPLEMENTATION_COLOR_READ_FORMAT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const IMPLEMENTATION_COLOR_READ_FORMAT: u32 = 35739u64 as u32;

    ///The `WebGL2RenderingContext.COMPILE_STATUS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COMPILE_STATUS: u32 = 35713u64 as u32;

    ///The `WebGL2RenderingContext.LOW_FLOAT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const LOW_FLOAT: u32 = 36336u64 as u32;

    ///The `WebGL2RenderingContext.MEDIUM_FLOAT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MEDIUM_FLOAT: u32 = 36337u64 as u32;

    ///The `WebGL2RenderingContext.HIGH_FLOAT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const HIGH_FLOAT: u32 = 36338u64 as u32;

    ///The `WebGL2RenderingContext.LOW_INT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const LOW_INT: u32 = 36339u64 as u32;

    ///The `WebGL2RenderingContext.MEDIUM_INT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MEDIUM_INT: u32 = 36340u64 as u32;

    ///The `WebGL2RenderingContext.HIGH_INT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const HIGH_INT: u32 = 36341u64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER: u32 = 36160u64 as u32;

    ///The `WebGL2RenderingContext.RENDERBUFFER` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RENDERBUFFER: u32 = 36161u64 as u32;

    ///The `WebGL2RenderingContext.RGBA4` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGBA4: u32 = 32854u64 as u32;

    ///The `WebGL2RenderingContext.RGB5_A1` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGB5_A1: u32 = 32855u64 as u32;

    ///The `WebGL2RenderingContext.RGB565` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RGB565: u32 = 36194u64 as u32;

    ///The `WebGL2RenderingContext.DEPTH_COMPONENT16` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DEPTH_COMPONENT16: u32 = 33189u64 as u32;

    ///The `WebGL2RenderingContext.STENCIL_INDEX8` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STENCIL_INDEX8: u32 = 36168u64 as u32;

    ///The `WebGL2RenderingContext.DEPTH_STENCIL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DEPTH_STENCIL: u32 = 34041u64 as u32;

    ///The `WebGL2RenderingContext.RENDERBUFFER_WIDTH` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RENDERBUFFER_WIDTH: u32 = 36162u64 as u32;

    ///The `WebGL2RenderingContext.RENDERBUFFER_HEIGHT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RENDERBUFFER_HEIGHT: u32 = 36163u64 as u32;

    ///The `WebGL2RenderingContext.RENDERBUFFER_INTERNAL_FORMAT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RENDERBUFFER_INTERNAL_FORMAT: u32 = 36164u64 as u32;

    ///The `WebGL2RenderingContext.RENDERBUFFER_RED_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RENDERBUFFER_RED_SIZE: u32 = 36176u64 as u32;

    ///The `WebGL2RenderingContext.RENDERBUFFER_GREEN_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RENDERBUFFER_GREEN_SIZE: u32 = 36177u64 as u32;

    ///The `WebGL2RenderingContext.RENDERBUFFER_BLUE_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RENDERBUFFER_BLUE_SIZE: u32 = 36178u64 as u32;

    ///The `WebGL2RenderingContext.RENDERBUFFER_ALPHA_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RENDERBUFFER_ALPHA_SIZE: u32 = 36179u64 as u32;

    ///The `WebGL2RenderingContext.RENDERBUFFER_DEPTH_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RENDERBUFFER_DEPTH_SIZE: u32 = 36180u64 as u32;

    ///The `WebGL2RenderingContext.RENDERBUFFER_STENCIL_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RENDERBUFFER_STENCIL_SIZE: u32 = 36181u64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: u32 = 36048u64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_OBJECT_NAME` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: u32 = 36049u64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: u32 = 36050u64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: u32 = 36051u64 as u32;

    ///The `WebGL2RenderingContext.COLOR_ATTACHMENT0` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const COLOR_ATTACHMENT0: u32 = 36064u64 as u32;

    ///The `WebGL2RenderingContext.DEPTH_ATTACHMENT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DEPTH_ATTACHMENT: u32 = 36096u64 as u32;

    ///The `WebGL2RenderingContext.STENCIL_ATTACHMENT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const STENCIL_ATTACHMENT: u32 = 36128u64 as u32;

    ///The `WebGL2RenderingContext.DEPTH_STENCIL_ATTACHMENT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const DEPTH_STENCIL_ATTACHMENT: u32 = 33306u64 as u32;

    ///The `WebGL2RenderingContext.NONE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const NONE: u32 = 0i64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER_COMPLETE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER_COMPLETE: u32 = 36053u64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER_INCOMPLETE_ATTACHMENT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: u32 = 36054u64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: u32 = 36055u64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER_INCOMPLETE_DIMENSIONS` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: u32 = 36057u64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER_UNSUPPORTED` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER_UNSUPPORTED: u32 = 36061u64 as u32;

    ///The `WebGL2RenderingContext.FRAMEBUFFER_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const FRAMEBUFFER_BINDING: u32 = 36006u64 as u32;

    ///The `WebGL2RenderingContext.RENDERBUFFER_BINDING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const RENDERBUFFER_BINDING: u32 = 36007u64 as u32;

    ///The `WebGL2RenderingContext.MAX_RENDERBUFFER_SIZE` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const MAX_RENDERBUFFER_SIZE: u32 = 34024u64 as u32;

    ///The `WebGL2RenderingContext.INVALID_FRAMEBUFFER_OPERATION` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const INVALID_FRAMEBUFFER_OPERATION: u32 = 1286u64 as u32;

    ///The `WebGL2RenderingContext.UNPACK_FLIP_Y_WEBGL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNPACK_FLIP_Y_WEBGL: u32 = 37440u64 as u32;

    ///The `WebGL2RenderingContext.UNPACK_PREMULTIPLY_ALPHA_WEBGL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNPACK_PREMULTIPLY_ALPHA_WEBGL: u32 = 37441u64 as u32;

    ///The `WebGL2RenderingContext.CONTEXT_LOST_WEBGL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const CONTEXT_LOST_WEBGL: u32 = 37442u64 as u32;

    ///The `WebGL2RenderingContext.UNPACK_COLORSPACE_CONVERSION_WEBGL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const UNPACK_COLORSPACE_CONVERSION_WEBGL: u32 = 37443u64 as u32;

    ///The `WebGL2RenderingContext.BROWSER_DEFAULT_WEBGL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebGl2RenderingContext`*

    pub const BROWSER_DEFAULT_WEBGL: u32 = 37444u64 as u32;
}
