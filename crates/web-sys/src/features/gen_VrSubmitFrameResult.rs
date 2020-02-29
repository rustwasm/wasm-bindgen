use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = VRSubmitFrameResult , typescript_type = "VRSubmitFrameResult" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `VrSubmitFrameResult` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRSubmitFrameResult)
    ///
    ///*This API requires the following crate features to be activated: `VrSubmitFrameResult`*
    pub type VrSubmitFrameResult;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VRSubmitFrameResult" , js_name = frameNum ) ]
    ///Getter for the `frameNum` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRSubmitFrameResult/frameNum)
    ///
    ///*This API requires the following crate features to be activated: `VrSubmitFrameResult`*
    pub fn frame_num(this: &VrSubmitFrameResult) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VRSubmitFrameResult" , js_name = base64Image ) ]
    ///Getter for the `base64Image` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRSubmitFrameResult/base64Image)
    ///
    ///*This API requires the following crate features to be activated: `VrSubmitFrameResult`*
    pub fn base64_image(this: &VrSubmitFrameResult) -> Option<String>;

    #[wasm_bindgen(catch, constructor, js_class = "VRSubmitFrameResult")]
    ///The `new VrSubmitFrameResult(..)` constructor, creating a new instance of `VrSubmitFrameResult`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRSubmitFrameResult/VRSubmitFrameResult)
    ///
    ///*This API requires the following crate features to be activated: `VrSubmitFrameResult`*
    pub fn new() -> Result<VrSubmitFrameResult, JsValue>;

}
