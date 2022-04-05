#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = HtmlMediaElement , extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLVideoElement , typescript_type = "HTMLVideoElement")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlVideoElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    pub type HtmlVideoElement;
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLVideoElement" , js_name = width)]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    pub fn width(this: &HtmlVideoElement) -> u32;
    # [wasm_bindgen (structural , catch , method , setter , js_class = "HTMLVideoElement" , js_name = width)]
    #[doc = "Setter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    pub fn set_width(this: &HtmlVideoElement, value: u32) -> Result<(), JsValue>;
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLVideoElement" , js_name = height)]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    pub fn height(this: &HtmlVideoElement) -> u32;
    # [wasm_bindgen (structural , catch , method , setter , js_class = "HTMLVideoElement" , js_name = height)]
    #[doc = "Setter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    pub fn set_height(this: &HtmlVideoElement, value: u32) -> Result<(), JsValue>;
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLVideoElement" , js_name = videoWidth)]
    #[doc = "Getter for the `videoWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/videoWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    pub fn video_width(this: &HtmlVideoElement) -> u32;
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLVideoElement" , js_name = videoHeight)]
    #[doc = "Getter for the `videoHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/videoHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    pub fn video_height(this: &HtmlVideoElement) -> u32;
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLVideoElement" , js_name = poster)]
    #[doc = "Getter for the `poster` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/poster)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    pub fn poster(this: &HtmlVideoElement) -> String;
    # [wasm_bindgen (structural , catch , method , setter , js_class = "HTMLVideoElement" , js_name = poster)]
    #[doc = "Setter for the `poster` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/poster)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    pub fn set_poster(this: &HtmlVideoElement, value: &str) -> Result<(), JsValue>;
    #[cfg(feature = "VideoPlaybackQuality")]
    # [wasm_bindgen (method , structural , js_class = "HTMLVideoElement" , js_name = getVideoPlaybackQuality)]
    #[doc = "The `getVideoPlaybackQuality()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/getVideoPlaybackQuality)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `VideoPlaybackQuality`*"]
    pub fn get_video_playback_quality(this: &HtmlVideoElement) -> VideoPlaybackQuality;
}
