use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = TextTrackCueList , typescript_name = TextTrackCueList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `TextTrackCueList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCueList)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackCueList`*
    pub type TextTrackCueList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TextTrackCueList" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCueList/length)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackCueList`*
    pub fn length(this: &TextTrackCueList) -> u32;

    #[cfg(feature = "VttCue")]
    # [ wasm_bindgen ( method , structural , js_class = "TextTrackCueList" , js_name = getCueById ) ]
    ///The `getCueById()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCueList/getCueById)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackCueList`, `VttCue`*
    pub fn get_cue_by_id(this: &TextTrackCueList, id: &str) -> Option<VttCue>;

    #[cfg(feature = "VttCue")]
    #[wasm_bindgen(method, structural, js_class = "TextTrackCueList", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `TextTrackCueList`, `VttCue`*
    pub fn get(this: &TextTrackCueList, index: u32) -> Option<VttCue>;

}
