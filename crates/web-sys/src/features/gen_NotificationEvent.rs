use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = ExtendableEvent , extends = Event , extends = :: js_sys :: Object , js_name = NotificationEvent , typescript_name = NotificationEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NotificationEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NotificationEvent)\n\n*This API requires the following crate features to be activated: `NotificationEvent`*"]
    pub type NotificationEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "NotificationEvent" , js_name = notification ) ]
    #[cfg(feature = "Notification")]
    #[doc = "Getter for the `notification` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NotificationEvent/notification)\n\n*This API requires the following crate features to be activated: `Notification`, `NotificationEvent`*"]
    pub fn notification(this: &NotificationEvent) -> Notification;
    #[cfg(feature = "NotificationEventInit")]
    #[wasm_bindgen(catch, js_class = "NotificationEvent", constructor)]
    #[doc = "The `new NotificationEvent(..)` constructor, creating a new instance of `NotificationEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NotificationEvent/NotificationEvent)\n\n*This API requires the following crate features to be activated: `NotificationEvent`, `NotificationEventInit`*"]
    pub fn new(
        this: &NotificationEvent,
        type_: &str,
        event_init_dict: &NotificationEventInit,
    ) -> Result<NotificationEvent, JsValue>;
}
