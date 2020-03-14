#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = XrPose , extends = :: js_sys :: Object , js_name = XRViewerPose , typescript_type = "XRViewerPose" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrViewerPose` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRViewerPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrViewerPose`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrViewerPose;
    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , getter , js_class = "XRViewerPose" , js_name = views ) ]
    #[doc = "Getter for the `views` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRViewerPose/views)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrViewerPose`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn views(this: &XrViewerPose) -> ::js_sys::Array;
}
