use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = TextTrackCueList , typescript_name = TextTrackCueList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextTrackCueList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCueList)\n\n*This API requires the following crate features to be activated: `TextTrackCueList`*"]
    pub type TextTrackCueList;
    # [ wasm_bindgen ( structural , method , getter , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCueList/length)\n\n*This API requires the following crate features to be activated: `TextTrackCueList`*"]
    pub fn length(this: &TextTrackCueList) -> u32;
    #[cfg(feature = "VttCue")]
    # [ wasm_bindgen ( method , structural , js_name = getCueById ) ]
    #[doc = "The `getCueById()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCueList/getCueById)\n\n*This API requires the following crate features to be activated: `TextTrackCueList`, `VttCue`*"]
    pub fn get_cue_by_id(this: &TextTrackCueList, id: &str) -> Option<VttCue>;
    #[cfg(feature = "VttCue")]
    #[wasm_bindgen(method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `TextTrackCueList`, `VttCue`*"]
    pub fn get(this: &TextTrackCueList, index: u32) -> Option<VttCue>;
}
