use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = VideoTrackList , typescript_type = "VideoTrackList" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `VideoTrackList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList)
    ///
    ///*This API requires the following crate features to be activated: `VideoTrackList`*
    pub type VideoTrackList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VideoTrackList" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/length)
    ///
    ///*This API requires the following crate features to be activated: `VideoTrackList`*
    pub fn length(this: &VideoTrackList) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VideoTrackList" , js_name = selectedIndex ) ]
    ///Getter for the `selectedIndex` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/selectedIndex)
    ///
    ///*This API requires the following crate features to be activated: `VideoTrackList`*
    pub fn selected_index(this: &VideoTrackList) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VideoTrackList" , js_name = onchange ) ]
    ///Getter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onchange)
    ///
    ///*This API requires the following crate features to be activated: `VideoTrackList`*
    pub fn onchange(this: &VideoTrackList) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "VideoTrackList" , js_name = onchange ) ]
    ///Setter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onchange)
    ///
    ///*This API requires the following crate features to be activated: `VideoTrackList`*
    pub fn set_onchange(this: &VideoTrackList, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "VideoTrackList" , js_name = onaddtrack ) ]
    ///Getter for the `onaddtrack` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onaddtrack)
    ///
    ///*This API requires the following crate features to be activated: `VideoTrackList`*
    pub fn onaddtrack(this: &VideoTrackList) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "VideoTrackList" , js_name = onaddtrack ) ]
    ///Setter for the `onaddtrack` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onaddtrack)
    ///
    ///*This API requires the following crate features to be activated: `VideoTrackList`*
    pub fn set_onaddtrack(this: &VideoTrackList, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "VideoTrackList" , js_name = onremovetrack ) ]
    ///Getter for the `onremovetrack` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onremovetrack)
    ///
    ///*This API requires the following crate features to be activated: `VideoTrackList`*
    pub fn onremovetrack(this: &VideoTrackList) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "VideoTrackList" , js_name = onremovetrack ) ]
    ///Setter for the `onremovetrack` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onremovetrack)
    ///
    ///*This API requires the following crate features to be activated: `VideoTrackList`*
    pub fn set_onremovetrack(this: &VideoTrackList, value: Option<&::js_sys::Function>);

    #[cfg(feature = "VideoTrack")]
    # [ wasm_bindgen ( method , structural , js_class = "VideoTrackList" , js_name = getTrackById ) ]
    ///The `getTrackById()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/getTrackById)
    ///
    ///*This API requires the following crate features to be activated: `VideoTrack`, `VideoTrackList`*
    pub fn get_track_by_id(this: &VideoTrackList, id: &str) -> Option<VideoTrack>;

    #[cfg(feature = "VideoTrack")]
    #[wasm_bindgen(method, structural, js_class = "VideoTrackList", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `VideoTrack`, `VideoTrackList`*
    pub fn get(this: &VideoTrackList, index: u32) -> Option<VideoTrack>;

}
