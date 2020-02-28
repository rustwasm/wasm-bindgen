use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = WEBGL_draw_buffers , typescript_name = WEBGL_draw_buffers ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebglDrawBuffers` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_draw_buffers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglDrawBuffers`*"]
    pub type WebglDrawBuffers;
    # [ wasm_bindgen ( method , structural , js_class = "WEBGL_draw_buffers" , js_name = drawBuffersWEBGL ) ]
    #[doc = "The `drawBuffersWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_draw_buffers/drawBuffersWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglDrawBuffers`*"]
    pub fn draw_buffers_webgl(this: &WebglDrawBuffers, buffers: &::wasm_bindgen::JsValue);
}
impl WebglDrawBuffers {
    pub const COLOR_ATTACHMENT0_WEBGL: u32 = 36064u64 as u32;
    pub const COLOR_ATTACHMENT1_WEBGL: u32 = 36065u64 as u32;
    pub const COLOR_ATTACHMENT2_WEBGL: u32 = 36066u64 as u32;
    pub const COLOR_ATTACHMENT3_WEBGL: u32 = 36067u64 as u32;
    pub const COLOR_ATTACHMENT4_WEBGL: u32 = 36068u64 as u32;
    pub const COLOR_ATTACHMENT5_WEBGL: u32 = 36069u64 as u32;
    pub const COLOR_ATTACHMENT6_WEBGL: u32 = 36070u64 as u32;
    pub const COLOR_ATTACHMENT7_WEBGL: u32 = 36071u64 as u32;
    pub const COLOR_ATTACHMENT8_WEBGL: u32 = 36072u64 as u32;
    pub const COLOR_ATTACHMENT9_WEBGL: u32 = 36073u64 as u32;
    pub const COLOR_ATTACHMENT10_WEBGL: u32 = 36074u64 as u32;
    pub const COLOR_ATTACHMENT11_WEBGL: u32 = 36075u64 as u32;
    pub const COLOR_ATTACHMENT12_WEBGL: u32 = 36076u64 as u32;
    pub const COLOR_ATTACHMENT13_WEBGL: u32 = 36077u64 as u32;
    pub const COLOR_ATTACHMENT14_WEBGL: u32 = 36078u64 as u32;
    pub const COLOR_ATTACHMENT15_WEBGL: u32 = 36079u64 as u32;
    pub const DRAW_BUFFER0_WEBGL: u32 = 34853u64 as u32;
    pub const DRAW_BUFFER1_WEBGL: u32 = 34854u64 as u32;
    pub const DRAW_BUFFER2_WEBGL: u32 = 34855u64 as u32;
    pub const DRAW_BUFFER3_WEBGL: u32 = 34856u64 as u32;
    pub const DRAW_BUFFER4_WEBGL: u32 = 34857u64 as u32;
    pub const DRAW_BUFFER5_WEBGL: u32 = 34858u64 as u32;
    pub const DRAW_BUFFER6_WEBGL: u32 = 34859u64 as u32;
    pub const DRAW_BUFFER7_WEBGL: u32 = 34860u64 as u32;
    pub const DRAW_BUFFER8_WEBGL: u32 = 34861u64 as u32;
    pub const DRAW_BUFFER9_WEBGL: u32 = 34862u64 as u32;
    pub const DRAW_BUFFER10_WEBGL: u32 = 34863u64 as u32;
    pub const DRAW_BUFFER11_WEBGL: u32 = 34864u64 as u32;
    pub const DRAW_BUFFER12_WEBGL: u32 = 34865u64 as u32;
    pub const DRAW_BUFFER13_WEBGL: u32 = 34866u64 as u32;
    pub const DRAW_BUFFER14_WEBGL: u32 = 34867u64 as u32;
    pub const DRAW_BUFFER15_WEBGL: u32 = 34868u64 as u32;
    pub const MAX_COLOR_ATTACHMENTS_WEBGL: u32 = 36063u64 as u32;
    pub const MAX_DRAW_BUFFERS_WEBGL: u32 = 34852u64 as u32;
}
