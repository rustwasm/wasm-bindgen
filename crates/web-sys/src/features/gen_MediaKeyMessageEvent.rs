use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = MediaKeyMessageEvent , typescript_name = MediaKeyMessageEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeyMessageEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent)\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEvent`*"]
    pub type MediaKeyMessageEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = messageType ) ]
    #[cfg(feature = "MediaKeyMessageType")]
    #[doc = "Getter for the `messageType` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent/messageType)\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEvent`, `MediaKeyMessageType`*"]
    pub fn message_type(this: &MediaKeyMessageEvent) -> MediaKeyMessageType;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = message ) ]
    #[doc = "Getter for the `message` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent/message)\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEvent`*"]
    pub fn message(this: &MediaKeyMessageEvent) -> Result<::js_sys::ArrayBuffer, JsValue>;
    #[cfg(feature = "MediaKeyMessageEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new MediaKeyMessageEvent(..)` constructor, creating a new instance of `MediaKeyMessageEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent/MediaKeyMessageEvent)\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEvent`, `MediaKeyMessageEventInit`*"]
    pub fn new(
        this: &MediaKeyMessageEvent,
        type_: &str,
        event_init_dict: &MediaKeyMessageEventInit,
    ) -> Result<MediaKeyMessageEvent, JsValue>;
}
