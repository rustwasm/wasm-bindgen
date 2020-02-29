use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = MessageEvent , typescript_type = "MessageEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MessageEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `MessageEvent`*
    pub type MessageEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MessageEvent" , js_name = data ) ]
    ///Getter for the `data` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/data)
    ///
    ///*This API requires the following crate features to be activated: `MessageEvent`*
    pub fn data(this: &MessageEvent) -> ::wasm_bindgen::JsValue;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MessageEvent" , js_name = origin ) ]
    ///Getter for the `origin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/origin)
    ///
    ///*This API requires the following crate features to be activated: `MessageEvent`*
    pub fn origin(this: &MessageEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MessageEvent" , js_name = lastEventId ) ]
    ///Getter for the `lastEventId` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/lastEventId)
    ///
    ///*This API requires the following crate features to be activated: `MessageEvent`*
    pub fn last_event_id(this: &MessageEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MessageEvent" , js_name = source ) ]
    ///Getter for the `source` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/source)
    ///
    ///*This API requires the following crate features to be activated: `MessageEvent`*
    pub fn source(this: &MessageEvent) -> Option<::js_sys::Object>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MessageEvent" , js_name = ports ) ]
    ///Getter for the `ports` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/ports)
    ///
    ///*This API requires the following crate features to be activated: `MessageEvent`*
    pub fn ports(this: &MessageEvent) -> ::js_sys::Array;

    #[wasm_bindgen(catch, constructor, js_class = "MessageEvent")]
    ///The `new MessageEvent(..)` constructor, creating a new instance of `MessageEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/MessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `MessageEvent`*
    pub fn new(type_: &str) -> Result<MessageEvent, JsValue>;

    #[cfg(feature = "MessageEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "MessageEvent")]
    ///The `new MessageEvent(..)` constructor, creating a new instance of `MessageEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/MessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `MessageEvent`, `MessageEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &MessageEventInit,
    ) -> Result<MessageEvent, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "MessageEvent" , js_name = initMessageEvent ) ]
    ///The `initMessageEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `MessageEvent`*
    pub fn init_message_event(this: &MessageEvent, type_: &str);

    # [ wasm_bindgen ( method , structural , js_class = "MessageEvent" , js_name = initMessageEvent ) ]
    ///The `initMessageEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `MessageEvent`*
    pub fn init_message_event_with_bubbles(this: &MessageEvent, type_: &str, bubbles: bool);

    # [ wasm_bindgen ( method , structural , js_class = "MessageEvent" , js_name = initMessageEvent ) ]
    ///The `initMessageEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `MessageEvent`*
    pub fn init_message_event_with_bubbles_and_cancelable(
        this: &MessageEvent,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
    );

    # [ wasm_bindgen ( method , structural , js_class = "MessageEvent" , js_name = initMessageEvent ) ]
    ///The `initMessageEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `MessageEvent`*
    pub fn init_message_event_with_bubbles_and_cancelable_and_data(
        this: &MessageEvent,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
    );

    # [ wasm_bindgen ( method , structural , js_class = "MessageEvent" , js_name = initMessageEvent ) ]
    ///The `initMessageEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `MessageEvent`*
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin(
        this: &MessageEvent,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
    );

    # [ wasm_bindgen ( method , structural , js_class = "MessageEvent" , js_name = initMessageEvent ) ]
    ///The `initMessageEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `MessageEvent`*
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id(
        this: &MessageEvent,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
    );

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "MessageEvent" , js_name = initMessageEvent ) ]
    ///The `initMessageEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `MessageEvent`, `Window`*
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_window(
        this: &MessageEvent,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
        source: Option<&Window>,
    );

    #[cfg(feature = "MessagePort")]
    # [ wasm_bindgen ( method , structural , js_class = "MessageEvent" , js_name = initMessageEvent ) ]
    ///The `initMessageEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `MessageEvent`, `MessagePort`*
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_message_port(
        this: &MessageEvent,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
        source: Option<&MessagePort>,
    );

    #[cfg(feature = "ServiceWorker")]
    # [ wasm_bindgen ( method , structural , js_class = "MessageEvent" , js_name = initMessageEvent ) ]
    ///The `initMessageEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `MessageEvent`, `ServiceWorker`*
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_service_worker(
        this: &MessageEvent,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
        source: Option<&ServiceWorker>,
    );

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "MessageEvent" , js_name = initMessageEvent ) ]
    ///The `initMessageEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `MessageEvent`, `Window`*
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_window_and_ports(
        this: &MessageEvent,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
        source: Option<&Window>,
        ports: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "MessagePort")]
    # [ wasm_bindgen ( method , structural , js_class = "MessageEvent" , js_name = initMessageEvent ) ]
    ///The `initMessageEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `MessageEvent`, `MessagePort`*
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_message_port_and_ports(
        this: &MessageEvent,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
        source: Option<&MessagePort>,
        ports: &::wasm_bindgen::JsValue,
    );

    #[cfg(feature = "ServiceWorker")]
    # [ wasm_bindgen ( method , structural , js_class = "MessageEvent" , js_name = initMessageEvent ) ]
    ///The `initMessageEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `MessageEvent`, `ServiceWorker`*
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_service_worker_and_ports(
        this: &MessageEvent,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
        source: Option<&ServiceWorker>,
        ports: &::wasm_bindgen::JsValue,
    );

}
