use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = OfflineAudioCompletionEvent , typescript_name = OfflineAudioCompletionEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OfflineAudioCompletionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioCompletionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEvent`*"]
    pub type OfflineAudioCompletionEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "OfflineAudioCompletionEvent" , js_name = renderedBuffer ) ]
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Getter for the `renderedBuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioCompletionEvent/renderedBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `OfflineAudioCompletionEvent`*"]
    pub fn rendered_buffer(this: &OfflineAudioCompletionEvent) -> AudioBuffer;
    #[cfg(feature = "OfflineAudioCompletionEventInit")]
    #[wasm_bindgen(catch, js_class = "OfflineAudioCompletionEvent", constructor)]
    #[doc = "The `new OfflineAudioCompletionEvent(..)` constructor, creating a new instance of `OfflineAudioCompletionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioCompletionEvent/OfflineAudioCompletionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEvent`, `OfflineAudioCompletionEventInit`*"]
    pub fn new(
        this: &OfflineAudioCompletionEvent,
        type_: &str,
        event_init_dict: &OfflineAudioCompletionEventInit,
    ) -> Result<OfflineAudioCompletionEvent, JsValue>;
}
