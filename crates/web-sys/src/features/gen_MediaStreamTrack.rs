use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = MediaStreamTrack , typescript_type = "MediaStreamTrack" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaStreamTrack` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`*
    pub type MediaStreamTrack;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStreamTrack" , js_name = kind ) ]
    ///Getter for the `kind` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/kind)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`*
    pub fn kind(this: &MediaStreamTrack) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStreamTrack" , js_name = id ) ]
    ///Getter for the `id` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/id)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`*
    pub fn id(this: &MediaStreamTrack) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStreamTrack" , js_name = label ) ]
    ///Getter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/label)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`*
    pub fn label(this: &MediaStreamTrack) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStreamTrack" , js_name = enabled ) ]
    ///Getter for the `enabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/enabled)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`*
    pub fn enabled(this: &MediaStreamTrack) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaStreamTrack" , js_name = enabled ) ]
    ///Setter for the `enabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/enabled)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`*
    pub fn set_enabled(this: &MediaStreamTrack, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStreamTrack" , js_name = muted ) ]
    ///Getter for the `muted` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/muted)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`*
    pub fn muted(this: &MediaStreamTrack) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStreamTrack" , js_name = onmute ) ]
    ///Getter for the `onmute` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onmute)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`*
    pub fn onmute(this: &MediaStreamTrack) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaStreamTrack" , js_name = onmute ) ]
    ///Setter for the `onmute` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onmute)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`*
    pub fn set_onmute(this: &MediaStreamTrack, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStreamTrack" , js_name = onunmute ) ]
    ///Getter for the `onunmute` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onunmute)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`*
    pub fn onunmute(this: &MediaStreamTrack) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaStreamTrack" , js_name = onunmute ) ]
    ///Setter for the `onunmute` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onunmute)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`*
    pub fn set_onunmute(this: &MediaStreamTrack, value: Option<&::js_sys::Function>);

    #[cfg(feature = "MediaStreamTrackState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStreamTrack" , js_name = readyState ) ]
    ///Getter for the `readyState` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/readyState)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaStreamTrackState`*
    pub fn ready_state(this: &MediaStreamTrack) -> MediaStreamTrackState;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStreamTrack" , js_name = onended ) ]
    ///Getter for the `onended` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onended)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`*
    pub fn onended(this: &MediaStreamTrack) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaStreamTrack" , js_name = onended ) ]
    ///Setter for the `onended` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onended)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`*
    pub fn set_onended(this: &MediaStreamTrack, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaStreamTrack" , js_name = applyConstraints ) ]
    ///The `applyConstraints()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/applyConstraints)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`*
    pub fn apply_constraints(this: &MediaStreamTrack) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "MediaTrackConstraints")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaStreamTrack" , js_name = applyConstraints ) ]
    ///The `applyConstraints()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/applyConstraints)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaTrackConstraints`*
    pub fn apply_constraints_with_constraints(
        this: &MediaStreamTrack,
        constraints: &MediaTrackConstraints,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "MediaStreamTrack" , js_name = clone ) ]
    ///The `clone()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/clone)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`*
    pub fn clone(this: &MediaStreamTrack) -> MediaStreamTrack;

    #[cfg(feature = "MediaTrackConstraints")]
    # [ wasm_bindgen ( method , structural , js_class = "MediaStreamTrack" , js_name = getConstraints ) ]
    ///The `getConstraints()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/getConstraints)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaTrackConstraints`*
    pub fn get_constraints(this: &MediaStreamTrack) -> MediaTrackConstraints;

    #[cfg(feature = "MediaTrackSettings")]
    # [ wasm_bindgen ( method , structural , js_class = "MediaStreamTrack" , js_name = getSettings ) ]
    ///The `getSettings()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/getSettings)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaTrackSettings`*
    pub fn get_settings(this: &MediaStreamTrack) -> MediaTrackSettings;

    # [ wasm_bindgen ( method , structural , js_class = "MediaStreamTrack" , js_name = stop ) ]
    ///The `stop()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/stop)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`*
    pub fn stop(this: &MediaStreamTrack);

}
