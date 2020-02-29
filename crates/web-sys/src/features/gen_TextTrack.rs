use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = TextTrack , typescript_name = TextTrack ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `TextTrack` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack)
    ///
    ///*This API requires the following crate features to be activated: `TextTrack`*
    pub type TextTrack;

    #[cfg(feature = "TextTrackKind")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "TextTrack" , js_name = kind ) ]
    ///Getter for the `kind` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/kind)
    ///
    ///*This API requires the following crate features to be activated: `TextTrack`, `TextTrackKind`*
    pub fn kind(this: &TextTrack) -> TextTrackKind;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TextTrack" , js_name = label ) ]
    ///Getter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/label)
    ///
    ///*This API requires the following crate features to be activated: `TextTrack`*
    pub fn label(this: &TextTrack) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TextTrack" , js_name = language ) ]
    ///Getter for the `language` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/language)
    ///
    ///*This API requires the following crate features to be activated: `TextTrack`*
    pub fn language(this: &TextTrack) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TextTrack" , js_name = id ) ]
    ///Getter for the `id` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/id)
    ///
    ///*This API requires the following crate features to be activated: `TextTrack`*
    pub fn id(this: &TextTrack) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TextTrack" , js_name = inBandMetadataTrackDispatchType ) ]
    ///Getter for the `inBandMetadataTrackDispatchType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/inBandMetadataTrackDispatchType)
    ///
    ///*This API requires the following crate features to be activated: `TextTrack`*
    pub fn in_band_metadata_track_dispatch_type(this: &TextTrack) -> String;

    #[cfg(feature = "TextTrackMode")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "TextTrack" , js_name = mode ) ]
    ///Getter for the `mode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/mode)
    ///
    ///*This API requires the following crate features to be activated: `TextTrack`, `TextTrackMode`*
    pub fn mode(this: &TextTrack) -> TextTrackMode;

    #[cfg(feature = "TextTrackMode")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "TextTrack" , js_name = mode ) ]
    ///Setter for the `mode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/mode)
    ///
    ///*This API requires the following crate features to be activated: `TextTrack`, `TextTrackMode`*
    pub fn set_mode(this: &TextTrack, value: TextTrackMode);

    #[cfg(feature = "TextTrackCueList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "TextTrack" , js_name = cues ) ]
    ///Getter for the `cues` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/cues)
    ///
    ///*This API requires the following crate features to be activated: `TextTrack`, `TextTrackCueList`*
    pub fn cues(this: &TextTrack) -> Option<TextTrackCueList>;

    #[cfg(feature = "TextTrackCueList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "TextTrack" , js_name = activeCues ) ]
    ///Getter for the `activeCues` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/activeCues)
    ///
    ///*This API requires the following crate features to be activated: `TextTrack`, `TextTrackCueList`*
    pub fn active_cues(this: &TextTrack) -> Option<TextTrackCueList>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TextTrack" , js_name = oncuechange ) ]
    ///Getter for the `oncuechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/oncuechange)
    ///
    ///*This API requires the following crate features to be activated: `TextTrack`*
    pub fn oncuechange(this: &TextTrack) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "TextTrack" , js_name = oncuechange ) ]
    ///Setter for the `oncuechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/oncuechange)
    ///
    ///*This API requires the following crate features to be activated: `TextTrack`*
    pub fn set_oncuechange(this: &TextTrack, value: Option<&::js_sys::Function>);

    #[cfg(feature = "VttCue")]
    # [ wasm_bindgen ( method , structural , js_class = "TextTrack" , js_name = addCue ) ]
    ///The `addCue()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/addCue)
    ///
    ///*This API requires the following crate features to be activated: `TextTrack`, `VttCue`*
    pub fn add_cue(this: &TextTrack, cue: &VttCue);

    #[cfg(feature = "VttCue")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "TextTrack" , js_name = removeCue ) ]
    ///The `removeCue()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/removeCue)
    ///
    ///*This API requires the following crate features to be activated: `TextTrack`, `VttCue`*
    pub fn remove_cue(this: &TextTrack, cue: &VttCue) -> Result<(), JsValue>;

}
