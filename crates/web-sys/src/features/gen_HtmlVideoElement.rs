use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlMediaElement , extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLVideoElement , typescript_type = "HTMLVideoElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlVideoElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlVideoElement`*
    pub type HtmlVideoElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLVideoElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlVideoElement`*
    pub fn width(this: &HtmlVideoElement) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLVideoElement" , js_name = width ) ]
    ///Setter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlVideoElement`*
    pub fn set_width(this: &HtmlVideoElement, value: u32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLVideoElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/height)
    ///
    ///*This API requires the following crate features to be activated: `HtmlVideoElement`*
    pub fn height(this: &HtmlVideoElement) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLVideoElement" , js_name = height ) ]
    ///Setter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/height)
    ///
    ///*This API requires the following crate features to be activated: `HtmlVideoElement`*
    pub fn set_height(this: &HtmlVideoElement, value: u32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLVideoElement" , js_name = videoWidth ) ]
    ///Getter for the `videoWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/videoWidth)
    ///
    ///*This API requires the following crate features to be activated: `HtmlVideoElement`*
    pub fn video_width(this: &HtmlVideoElement) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLVideoElement" , js_name = videoHeight ) ]
    ///Getter for the `videoHeight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/videoHeight)
    ///
    ///*This API requires the following crate features to be activated: `HtmlVideoElement`*
    pub fn video_height(this: &HtmlVideoElement) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLVideoElement" , js_name = poster ) ]
    ///Getter for the `poster` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/poster)
    ///
    ///*This API requires the following crate features to be activated: `HtmlVideoElement`*
    pub fn poster(this: &HtmlVideoElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLVideoElement" , js_name = poster ) ]
    ///Setter for the `poster` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/poster)
    ///
    ///*This API requires the following crate features to be activated: `HtmlVideoElement`*
    pub fn set_poster(this: &HtmlVideoElement, value: &str);

    #[cfg(feature = "VideoPlaybackQuality")]
    # [ wasm_bindgen ( method , structural , js_class = "HTMLVideoElement" , js_name = getVideoPlaybackQuality ) ]
    ///The `getVideoPlaybackQuality()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/getVideoPlaybackQuality)
    ///
    ///*This API requires the following crate features to be activated: `HtmlVideoElement`, `VideoPlaybackQuality`*
    pub fn get_video_playback_quality(this: &HtmlVideoElement) -> VideoPlaybackQuality;

}
