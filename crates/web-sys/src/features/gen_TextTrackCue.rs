use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = TextTrackCue , typescript_name = TextTrackCue ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextTrackCue` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub type TextTrackCue;
    # [ wasm_bindgen ( structural , method , getter , js_name = track ) ]
    #[cfg(feature = "TextTrack")]
    #[doc = "Getter for the `track` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/track)\n\n*This API requires the following crate features to be activated: `TextTrack`, `TextTrackCue`*"]
    pub fn track(this: &TextTrackCue) -> Option<TextTrack>;
    # [ wasm_bindgen ( structural , method , getter , js_name = id ) ]
    #[doc = "Getter for the `id` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/id)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn id(this: &TextTrackCue) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = id ) ]
    #[doc = "Setter for the `id` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/id)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn set_id(this: &TextTrackCue, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = startTime ) ]
    #[doc = "Getter for the `startTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/startTime)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn start_time(this: &TextTrackCue) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = startTime ) ]
    #[doc = "Setter for the `startTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/startTime)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn set_start_time(this: &TextTrackCue, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = endTime ) ]
    #[doc = "Getter for the `endTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/endTime)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn end_time(this: &TextTrackCue) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = endTime ) ]
    #[doc = "Setter for the `endTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/endTime)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn set_end_time(this: &TextTrackCue, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = pauseOnExit ) ]
    #[doc = "Getter for the `pauseOnExit` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/pauseOnExit)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn pause_on_exit(this: &TextTrackCue) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = pauseOnExit ) ]
    #[doc = "Setter for the `pauseOnExit` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/pauseOnExit)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn set_pause_on_exit(this: &TextTrackCue, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = onenter ) ]
    #[doc = "Getter for the `onenter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onenter)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn onenter(this: &TextTrackCue) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onenter ) ]
    #[doc = "Setter for the `onenter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onenter)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn set_onenter(this: &TextTrackCue, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onexit ) ]
    #[doc = "Getter for the `onexit` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onexit)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn onexit(this: &TextTrackCue) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onexit ) ]
    #[doc = "Setter for the `onexit` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onexit)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn set_onexit(this: &TextTrackCue, value: Option<::js_sys::Function>);
}
