use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = VRSubmitFrameResult , typescript_name = VRSubmitFrameResult ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrSubmitFrameResult` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRSubmitFrameResult)\n\n*This API requires the following crate features to be activated: `VrSubmitFrameResult`*"]
    pub type VrSubmitFrameResult;
    # [ wasm_bindgen ( structural , method , getter , js_class = "VRSubmitFrameResult" , js_name = frameNum ) ]
    #[doc = "Getter for the `frameNum` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRSubmitFrameResult/frameNum)\n\n*This API requires the following crate features to be activated: `VrSubmitFrameResult`*"]
    pub fn frame_num(this: &VrSubmitFrameResult) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "VRSubmitFrameResult" , js_name = base64Image ) ]
    #[doc = "Getter for the `base64Image` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRSubmitFrameResult/base64Image)\n\n*This API requires the following crate features to be activated: `VrSubmitFrameResult`*"]
    pub fn base64_image(this: &VrSubmitFrameResult) -> Option<String>;
    #[wasm_bindgen(catch, js_class = "VRSubmitFrameResult", constructor)]
    #[doc = "The `new VrSubmitFrameResult(..)` constructor, creating a new instance of `VrSubmitFrameResult`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRSubmitFrameResult/VRSubmitFrameResult)\n\n*This API requires the following crate features to be activated: `VrSubmitFrameResult`*"]
    pub fn new(this: &VrSubmitFrameResult) -> Result<VrSubmitFrameResult, JsValue>;
}
