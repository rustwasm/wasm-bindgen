use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = TextTrackList , typescript_name = TextTrackList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `TextTrackList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackList`*
    pub type TextTrackList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TextTrackList" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/length)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackList`*
    pub fn length(this: &TextTrackList) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TextTrackList" , js_name = onchange ) ]
    ///Getter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onchange)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackList`*
    pub fn onchange(this: &TextTrackList) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "TextTrackList" , js_name = onchange ) ]
    ///Setter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onchange)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackList`*
    pub fn set_onchange(this: &TextTrackList, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "TextTrackList" , js_name = onaddtrack ) ]
    ///Getter for the `onaddtrack` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onaddtrack)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackList`*
    pub fn onaddtrack(this: &TextTrackList) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "TextTrackList" , js_name = onaddtrack ) ]
    ///Setter for the `onaddtrack` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onaddtrack)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackList`*
    pub fn set_onaddtrack(this: &TextTrackList, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "TextTrackList" , js_name = onremovetrack ) ]
    ///Getter for the `onremovetrack` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onremovetrack)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackList`*
    pub fn onremovetrack(this: &TextTrackList) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "TextTrackList" , js_name = onremovetrack ) ]
    ///Setter for the `onremovetrack` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onremovetrack)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackList`*
    pub fn set_onremovetrack(this: &TextTrackList, value: Option<&::js_sys::Function>);

    #[cfg(feature = "TextTrack")]
    # [ wasm_bindgen ( method , structural , js_class = "TextTrackList" , js_name = getTrackById ) ]
    ///The `getTrackById()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/getTrackById)
    ///
    ///*This API requires the following crate features to be activated: `TextTrack`, `TextTrackList`*
    pub fn get_track_by_id(this: &TextTrackList, id: &str) -> Option<TextTrack>;

    #[cfg(feature = "TextTrack")]
    #[wasm_bindgen(method, structural, js_class = "TextTrackList", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `TextTrack`, `TextTrackList`*
    pub fn get(this: &TextTrackList, index: u32) -> Option<TextTrack>;

}
