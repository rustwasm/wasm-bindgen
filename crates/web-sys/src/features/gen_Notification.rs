use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = Notification , typescript_name = Notification ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Notification` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification)
    ///
    ///*This API requires the following crate features to be activated: `Notification`*
    pub type Notification;

    #[cfg(feature = "NotificationPermission")]
    # [ wasm_bindgen ( structural , static_method_of = Notification , getter , js_class = "Notification" , js_name = permission ) ]
    ///Getter for the `permission` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/permission)
    ///
    ///*This API requires the following crate features to be activated: `Notification`, `NotificationPermission`*
    pub fn permission() -> NotificationPermission;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Notification" , js_name = onclick ) ]
    ///Getter for the `onclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onclick)
    ///
    ///*This API requires the following crate features to be activated: `Notification`*
    pub fn onclick(this: &Notification) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Notification" , js_name = onclick ) ]
    ///Setter for the `onclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onclick)
    ///
    ///*This API requires the following crate features to be activated: `Notification`*
    pub fn set_onclick(this: &Notification, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Notification" , js_name = onshow ) ]
    ///Getter for the `onshow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onshow)
    ///
    ///*This API requires the following crate features to be activated: `Notification`*
    pub fn onshow(this: &Notification) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Notification" , js_name = onshow ) ]
    ///Setter for the `onshow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onshow)
    ///
    ///*This API requires the following crate features to be activated: `Notification`*
    pub fn set_onshow(this: &Notification, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Notification" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onerror)
    ///
    ///*This API requires the following crate features to be activated: `Notification`*
    pub fn onerror(this: &Notification) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Notification" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onerror)
    ///
    ///*This API requires the following crate features to be activated: `Notification`*
    pub fn set_onerror(this: &Notification, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Notification" , js_name = onclose ) ]
    ///Getter for the `onclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onclose)
    ///
    ///*This API requires the following crate features to be activated: `Notification`*
    pub fn onclose(this: &Notification) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Notification" , js_name = onclose ) ]
    ///Setter for the `onclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onclose)
    ///
    ///*This API requires the following crate features to be activated: `Notification`*
    pub fn set_onclose(this: &Notification, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Notification" , js_name = title ) ]
    ///Getter for the `title` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/title)
    ///
    ///*This API requires the following crate features to be activated: `Notification`*
    pub fn title(this: &Notification) -> String;

    #[cfg(feature = "NotificationDirection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Notification" , js_name = dir ) ]
    ///Getter for the `dir` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/dir)
    ///
    ///*This API requires the following crate features to be activated: `Notification`, `NotificationDirection`*
    pub fn dir(this: &Notification) -> NotificationDirection;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Notification" , js_name = lang ) ]
    ///Getter for the `lang` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/lang)
    ///
    ///*This API requires the following crate features to be activated: `Notification`*
    pub fn lang(this: &Notification) -> Option<String>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Notification" , js_name = body ) ]
    ///Getter for the `body` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/body)
    ///
    ///*This API requires the following crate features to be activated: `Notification`*
    pub fn body(this: &Notification) -> Option<String>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Notification" , js_name = tag ) ]
    ///Getter for the `tag` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/tag)
    ///
    ///*This API requires the following crate features to be activated: `Notification`*
    pub fn tag(this: &Notification) -> Option<String>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Notification" , js_name = icon ) ]
    ///Getter for the `icon` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/icon)
    ///
    ///*This API requires the following crate features to be activated: `Notification`*
    pub fn icon(this: &Notification) -> Option<String>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Notification" , js_name = requireInteraction ) ]
    ///Getter for the `requireInteraction` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/requireInteraction)
    ///
    ///*This API requires the following crate features to be activated: `Notification`*
    pub fn require_interaction(this: &Notification) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Notification" , js_name = data ) ]
    ///Getter for the `data` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/data)
    ///
    ///*This API requires the following crate features to be activated: `Notification`*
    pub fn data(this: &Notification) -> ::wasm_bindgen::JsValue;

    #[wasm_bindgen(catch, constructor, js_class = "Notification")]
    ///The `new Notification(..)` constructor, creating a new instance of `Notification`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/Notification)
    ///
    ///*This API requires the following crate features to be activated: `Notification`*
    pub fn new(title: &str) -> Result<Notification, JsValue>;

    #[cfg(feature = "NotificationOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "Notification")]
    ///The `new Notification(..)` constructor, creating a new instance of `Notification`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/Notification)
    ///
    ///*This API requires the following crate features to be activated: `Notification`, `NotificationOptions`*
    pub fn new_with_options(
        title: &str,
        options: &NotificationOptions,
    ) -> Result<Notification, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Notification" , js_name = close ) ]
    ///The `close()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/close)
    ///
    ///*This API requires the following crate features to be activated: `Notification`*
    pub fn close(this: &Notification);

    # [ wasm_bindgen ( catch , static_method_of = Notification , js_class = "Notification" , js_name = get ) ]
    ///The `get()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/get)
    ///
    ///*This API requires the following crate features to be activated: `Notification`*
    pub fn get() -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "GetNotificationOptions")]
    # [ wasm_bindgen ( catch , static_method_of = Notification , js_class = "Notification" , js_name = get ) ]
    ///The `get()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/get)
    ///
    ///*This API requires the following crate features to be activated: `GetNotificationOptions`, `Notification`*
    pub fn get_with_filter(filter: &GetNotificationOptions) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , static_method_of = Notification , js_class = "Notification" , js_name = requestPermission ) ]
    ///The `requestPermission()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/requestPermission)
    ///
    ///*This API requires the following crate features to be activated: `Notification`*
    pub fn request_permission() -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , static_method_of = Notification , js_class = "Notification" , js_name = requestPermission ) ]
    ///The `requestPermission()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/requestPermission)
    ///
    ///*This API requires the following crate features to be activated: `Notification`*
    pub fn request_permission_with_permission_callback(
        permission_callback: &::js_sys::Function,
    ) -> Result<::js_sys::Promise, JsValue>;

}
