use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = TextTrackCue , typescript_type = "TextTrackCue" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `TextTrackCue` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackCue`*
    pub type TextTrackCue;

    #[cfg(feature = "TextTrack")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "TextTrackCue" , js_name = track ) ]
    ///Getter for the `track` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/track)
    ///
    ///*This API requires the following crate features to be activated: `TextTrack`, `TextTrackCue`*
    pub fn track(this: &TextTrackCue) -> Option<TextTrack>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TextTrackCue" , js_name = id ) ]
    ///Getter for the `id` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/id)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackCue`*
    pub fn id(this: &TextTrackCue) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "TextTrackCue" , js_name = id ) ]
    ///Setter for the `id` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/id)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackCue`*
    pub fn set_id(this: &TextTrackCue, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "TextTrackCue" , js_name = startTime ) ]
    ///Getter for the `startTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/startTime)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackCue`*
    pub fn start_time(this: &TextTrackCue) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "TextTrackCue" , js_name = startTime ) ]
    ///Setter for the `startTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/startTime)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackCue`*
    pub fn set_start_time(this: &TextTrackCue, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "TextTrackCue" , js_name = endTime ) ]
    ///Getter for the `endTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/endTime)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackCue`*
    pub fn end_time(this: &TextTrackCue) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "TextTrackCue" , js_name = endTime ) ]
    ///Setter for the `endTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/endTime)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackCue`*
    pub fn set_end_time(this: &TextTrackCue, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "TextTrackCue" , js_name = pauseOnExit ) ]
    ///Getter for the `pauseOnExit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/pauseOnExit)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackCue`*
    pub fn pause_on_exit(this: &TextTrackCue) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "TextTrackCue" , js_name = pauseOnExit ) ]
    ///Setter for the `pauseOnExit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/pauseOnExit)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackCue`*
    pub fn set_pause_on_exit(this: &TextTrackCue, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "TextTrackCue" , js_name = onenter ) ]
    ///Getter for the `onenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onenter)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackCue`*
    pub fn onenter(this: &TextTrackCue) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "TextTrackCue" , js_name = onenter ) ]
    ///Setter for the `onenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onenter)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackCue`*
    pub fn set_onenter(this: &TextTrackCue, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "TextTrackCue" , js_name = onexit ) ]
    ///Getter for the `onexit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onexit)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackCue`*
    pub fn onexit(this: &TextTrackCue) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "TextTrackCue" , js_name = onexit ) ]
    ///Setter for the `onexit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onexit)
    ///
    ///*This API requires the following crate features to be activated: `TextTrackCue`*
    pub fn set_onexit(this: &TextTrackCue, value: Option<&::js_sys::Function>);

}
