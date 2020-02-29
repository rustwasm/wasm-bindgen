use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = ExtendableEvent , extends = Event , extends = :: js_sys :: Object , js_name = ExtendableMessageEvent , typescript_type = "ExtendableMessageEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ExtendableMessageEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `ExtendableMessageEvent`*
    pub type ExtendableMessageEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ExtendableMessageEvent" , js_name = data ) ]
    ///Getter for the `data` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/data)
    ///
    ///*This API requires the following crate features to be activated: `ExtendableMessageEvent`*
    pub fn data(this: &ExtendableMessageEvent) -> ::wasm_bindgen::JsValue;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ExtendableMessageEvent" , js_name = origin ) ]
    ///Getter for the `origin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/origin)
    ///
    ///*This API requires the following crate features to be activated: `ExtendableMessageEvent`*
    pub fn origin(this: &ExtendableMessageEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ExtendableMessageEvent" , js_name = lastEventId ) ]
    ///Getter for the `lastEventId` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/lastEventId)
    ///
    ///*This API requires the following crate features to be activated: `ExtendableMessageEvent`*
    pub fn last_event_id(this: &ExtendableMessageEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ExtendableMessageEvent" , js_name = source ) ]
    ///Getter for the `source` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/source)
    ///
    ///*This API requires the following crate features to be activated: `ExtendableMessageEvent`*
    pub fn source(this: &ExtendableMessageEvent) -> Option<::js_sys::Object>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ExtendableMessageEvent" , js_name = ports ) ]
    ///Getter for the `ports` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/ports)
    ///
    ///*This API requires the following crate features to be activated: `ExtendableMessageEvent`*
    pub fn ports(this: &ExtendableMessageEvent) -> ::js_sys::Array;

    #[wasm_bindgen(catch, constructor, js_class = "ExtendableMessageEvent")]
    ///The `new ExtendableMessageEvent(..)` constructor, creating a new instance of `ExtendableMessageEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/ExtendableMessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `ExtendableMessageEvent`*
    pub fn new(type_: &str) -> Result<ExtendableMessageEvent, JsValue>;

    #[cfg(feature = "ExtendableMessageEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "ExtendableMessageEvent")]
    ///The `new ExtendableMessageEvent(..)` constructor, creating a new instance of `ExtendableMessageEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/ExtendableMessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `ExtendableMessageEvent`, `ExtendableMessageEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &ExtendableMessageEventInit,
    ) -> Result<ExtendableMessageEvent, JsValue>;

}
