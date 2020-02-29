use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = OfflineAudioCompletionEvent , typescript_type = "OfflineAudioCompletionEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `OfflineAudioCompletionEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioCompletionEvent)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioCompletionEvent`*
    pub type OfflineAudioCompletionEvent;

    #[cfg(feature = "AudioBuffer")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "OfflineAudioCompletionEvent" , js_name = renderedBuffer ) ]
    ///Getter for the `renderedBuffer` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioCompletionEvent/renderedBuffer)
    ///
    ///*This API requires the following crate features to be activated: `AudioBuffer`, `OfflineAudioCompletionEvent`*
    pub fn rendered_buffer(this: &OfflineAudioCompletionEvent) -> AudioBuffer;

    #[cfg(feature = "OfflineAudioCompletionEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "OfflineAudioCompletionEvent")]
    ///The `new OfflineAudioCompletionEvent(..)` constructor, creating a new instance of `OfflineAudioCompletionEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioCompletionEvent/OfflineAudioCompletionEvent)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioCompletionEvent`, `OfflineAudioCompletionEventInit`*
    pub fn new(
        type_: &str,
        event_init_dict: &OfflineAudioCompletionEventInit,
    ) -> Result<OfflineAudioCompletionEvent, JsValue>;

}
