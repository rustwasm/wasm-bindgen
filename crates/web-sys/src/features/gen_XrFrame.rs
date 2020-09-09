#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = XRFrame , typescript_type = "XRFrame")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrFrame` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrFrame;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSession")]
    # [wasm_bindgen (structural , method , getter , js_class = "XRFrame" , js_name = session)]
    #[doc = "Getter for the `session` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/session)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`, `XrSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn session(this: &XrFrame) -> XrSession;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "XrPose", feature = "XrSpace",))]
    # [wasm_bindgen (method , structural , js_class = "XRFrame" , js_name = getPose)]
    #[doc = "The `getPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/getPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`, `XrPose`, `XrSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_pose(this: &XrFrame, space: &XrSpace, base_space: &XrSpace) -> Option<XrPose>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "XrReferenceSpace", feature = "XrViewerPose",))]
    # [wasm_bindgen (method , structural , js_class = "XRFrame" , js_name = getViewerPose)]
    #[doc = "The `getViewerPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/getViewerPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`, `XrReferenceSpace`, `XrViewerPose`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_viewer_pose(
        this: &XrFrame,
        reference_space: &XrReferenceSpace,
    ) -> Option<XrViewerPose>;
}
