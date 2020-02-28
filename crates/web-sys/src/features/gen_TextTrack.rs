use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = TextTrack , typescript_name = TextTrack ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextTrack` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack)\n\n*This API requires the following crate features to be activated: `TextTrack`*"]
    pub type TextTrack;
    # [ wasm_bindgen ( structural , method , getter , js_name = kind ) ]
    #[cfg(feature = "TextTrackKind")]
    #[doc = "Getter for the `kind` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/kind)\n\n*This API requires the following crate features to be activated: `TextTrack`, `TextTrackKind`*"]
    pub fn kind(this: &TextTrack) -> TextTrackKind;
    # [ wasm_bindgen ( structural , method , getter , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/label)\n\n*This API requires the following crate features to be activated: `TextTrack`*"]
    pub fn label(this: &TextTrack) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = language ) ]
    #[doc = "Getter for the `language` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/language)\n\n*This API requires the following crate features to be activated: `TextTrack`*"]
    pub fn language(this: &TextTrack) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = id ) ]
    #[doc = "Getter for the `id` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/id)\n\n*This API requires the following crate features to be activated: `TextTrack`*"]
    pub fn id(this: &TextTrack) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = inBandMetadataTrackDispatchType ) ]
    #[doc = "Getter for the `inBandMetadataTrackDispatchType` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/inBandMetadataTrackDispatchType)\n\n*This API requires the following crate features to be activated: `TextTrack`*"]
    pub fn in_band_metadata_track_dispatch_type(this: &TextTrack) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = mode ) ]
    #[cfg(feature = "TextTrackMode")]
    #[doc = "Getter for the `mode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/mode)\n\n*This API requires the following crate features to be activated: `TextTrack`, `TextTrackMode`*"]
    pub fn mode(this: &TextTrack) -> TextTrackMode;
    # [ wasm_bindgen ( structural , method , setter , js_name = mode ) ]
    #[cfg(feature = "TextTrackMode")]
    #[doc = "Setter for the `mode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/mode)\n\n*This API requires the following crate features to be activated: `TextTrack`, `TextTrackMode`*"]
    pub fn set_mode(this: &TextTrack, value: TextTrackMode);
    # [ wasm_bindgen ( structural , method , getter , js_name = cues ) ]
    #[cfg(feature = "TextTrackCueList")]
    #[doc = "Getter for the `cues` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/cues)\n\n*This API requires the following crate features to be activated: `TextTrack`, `TextTrackCueList`*"]
    pub fn cues(this: &TextTrack) -> Option<TextTrackCueList>;
    # [ wasm_bindgen ( structural , method , getter , js_name = activeCues ) ]
    #[cfg(feature = "TextTrackCueList")]
    #[doc = "Getter for the `activeCues` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/activeCues)\n\n*This API requires the following crate features to be activated: `TextTrack`, `TextTrackCueList`*"]
    pub fn active_cues(this: &TextTrack) -> Option<TextTrackCueList>;
    # [ wasm_bindgen ( structural , method , getter , js_name = oncuechange ) ]
    #[doc = "Getter for the `oncuechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/oncuechange)\n\n*This API requires the following crate features to be activated: `TextTrack`*"]
    pub fn oncuechange(this: &TextTrack) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = oncuechange ) ]
    #[doc = "Setter for the `oncuechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/oncuechange)\n\n*This API requires the following crate features to be activated: `TextTrack`*"]
    pub fn set_oncuechange(this: &TextTrack, value: Option<::js_sys::Function>);
    #[cfg(feature = "VttCue")]
    # [ wasm_bindgen ( method , structural , js_name = addCue ) ]
    #[doc = "The `addCue()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/addCue)\n\n*This API requires the following crate features to be activated: `TextTrack`, `VttCue`*"]
    pub fn add_cue(this: &TextTrack, cue: &VttCue);
    #[cfg(feature = "VttCue")]
    # [ wasm_bindgen ( catch , method , structural , js_name = removeCue ) ]
    #[doc = "The `removeCue()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/removeCue)\n\n*This API requires the following crate features to be activated: `TextTrack`, `VttCue`*"]
    pub fn remove_cue(this: &TextTrack, cue: &VttCue) -> Result<(), JsValue>;
}
