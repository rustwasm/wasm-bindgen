use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = MediaKeyMessageEvent , typescript_name = MediaKeyMessageEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaKeyMessageEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeyMessageEvent`*
    pub type MediaKeyMessageEvent;

    #[cfg(feature = "MediaKeyMessageType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaKeyMessageEvent" , js_name = messageType ) ]
    ///Getter for the `messageType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent/messageType)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeyMessageEvent`, `MediaKeyMessageType`*
    pub fn message_type(this: &MediaKeyMessageEvent) -> MediaKeyMessageType;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "MediaKeyMessageEvent" , js_name = message ) ]
    ///Getter for the `message` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent/message)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeyMessageEvent`*
    pub fn message(this: &MediaKeyMessageEvent) -> Result<::js_sys::ArrayBuffer, JsValue>;

    #[cfg(feature = "MediaKeyMessageEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "MediaKeyMessageEvent")]
    ///The `new MediaKeyMessageEvent(..)` constructor, creating a new instance of `MediaKeyMessageEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent/MediaKeyMessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeyMessageEvent`, `MediaKeyMessageEventInit`*
    pub fn new(
        type_: &str,
        event_init_dict: &MediaKeyMessageEventInit,
    ) -> Result<MediaKeyMessageEvent, JsValue>;

}
