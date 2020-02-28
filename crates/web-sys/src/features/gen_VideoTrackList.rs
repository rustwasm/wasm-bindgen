use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = VideoTrackList , typescript_name = VideoTrackList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoTrackList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList)\n\n*This API requires the following crate features to be activated: `VideoTrackList`*"]
    pub type VideoTrackList;
    # [ wasm_bindgen ( structural , method , getter , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/length)\n\n*This API requires the following crate features to be activated: `VideoTrackList`*"]
    pub fn length(this: &VideoTrackList) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = selectedIndex ) ]
    #[doc = "Getter for the `selectedIndex` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/selectedIndex)\n\n*This API requires the following crate features to be activated: `VideoTrackList`*"]
    pub fn selected_index(this: &VideoTrackList) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_name = onchange ) ]
    #[doc = "Getter for the `onchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onchange)\n\n*This API requires the following crate features to be activated: `VideoTrackList`*"]
    pub fn onchange(this: &VideoTrackList) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onchange ) ]
    #[doc = "Setter for the `onchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onchange)\n\n*This API requires the following crate features to be activated: `VideoTrackList`*"]
    pub fn set_onchange(this: &VideoTrackList, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onaddtrack ) ]
    #[doc = "Getter for the `onaddtrack` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onaddtrack)\n\n*This API requires the following crate features to be activated: `VideoTrackList`*"]
    pub fn onaddtrack(this: &VideoTrackList) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onaddtrack ) ]
    #[doc = "Setter for the `onaddtrack` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onaddtrack)\n\n*This API requires the following crate features to be activated: `VideoTrackList`*"]
    pub fn set_onaddtrack(this: &VideoTrackList, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onremovetrack ) ]
    #[doc = "Getter for the `onremovetrack` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onremovetrack)\n\n*This API requires the following crate features to be activated: `VideoTrackList`*"]
    pub fn onremovetrack(this: &VideoTrackList) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onremovetrack ) ]
    #[doc = "Setter for the `onremovetrack` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onremovetrack)\n\n*This API requires the following crate features to be activated: `VideoTrackList`*"]
    pub fn set_onremovetrack(this: &VideoTrackList, value: Option<::js_sys::Function>);
    #[cfg(feature = "VideoTrack")]
    # [ wasm_bindgen ( method , structural , js_name = getTrackById ) ]
    #[doc = "The `getTrackById()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/getTrackById)\n\n*This API requires the following crate features to be activated: `VideoTrack`, `VideoTrackList`*"]
    pub fn get_track_by_id(this: &VideoTrackList, id: &str) -> Option<VideoTrack>;
    #[cfg(feature = "VideoTrack")]
    #[wasm_bindgen(method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `VideoTrack`, `VideoTrackList`*"]
    pub fn get(this: &VideoTrackList, index: u32) -> Option<VideoTrack>;
}
