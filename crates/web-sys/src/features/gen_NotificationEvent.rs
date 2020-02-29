use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = ExtendableEvent , extends = Event , extends = :: js_sys :: Object , js_name = NotificationEvent , typescript_name = NotificationEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `NotificationEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NotificationEvent)
    ///
    ///*This API requires the following crate features to be activated: `NotificationEvent`*
    pub type NotificationEvent;

    #[cfg(feature = "Notification")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "NotificationEvent" , js_name = notification ) ]
    ///Getter for the `notification` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NotificationEvent/notification)
    ///
    ///*This API requires the following crate features to be activated: `Notification`, `NotificationEvent`*
    pub fn notification(this: &NotificationEvent) -> Notification;

    #[cfg(feature = "NotificationEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "NotificationEvent")]
    ///The `new NotificationEvent(..)` constructor, creating a new instance of `NotificationEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NotificationEvent/NotificationEvent)
    ///
    ///*This API requires the following crate features to be activated: `NotificationEvent`, `NotificationEventInit`*
    pub fn new(
        type_: &str,
        event_init_dict: &NotificationEventInit,
    ) -> Result<NotificationEvent, JsValue>;

}
