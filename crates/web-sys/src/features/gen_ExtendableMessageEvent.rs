use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = ExtendableEvent , extends = Event , extends = :: js_sys :: Object , js_name = ExtendableMessageEvent , typescript_name = ExtendableMessageEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtendableMessageEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent)\n\n*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    pub type ExtendableMessageEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ExtendableMessageEvent" , js_name = data ) ]
    #[doc = "Getter for the `data` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/data)\n\n*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    pub fn data(this: &ExtendableMessageEvent) -> ::wasm_bindgen::JsValue;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ExtendableMessageEvent" , js_name = origin ) ]
    #[doc = "Getter for the `origin` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/origin)\n\n*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    pub fn origin(this: &ExtendableMessageEvent) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ExtendableMessageEvent" , js_name = lastEventId ) ]
    #[doc = "Getter for the `lastEventId` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/lastEventId)\n\n*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    pub fn last_event_id(this: &ExtendableMessageEvent) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ExtendableMessageEvent" , js_name = source ) ]
    #[doc = "Getter for the `source` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/source)\n\n*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    pub fn source(this: &ExtendableMessageEvent) -> Option<::js_sys::Object>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ExtendableMessageEvent" , js_name = ports ) ]
    #[doc = "Getter for the `ports` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/ports)\n\n*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    pub fn ports(this: &ExtendableMessageEvent) -> ::js_sys::Array;
    #[wasm_bindgen(catch, js_class = "ExtendableMessageEvent", constructor)]
    #[doc = "The `new ExtendableMessageEvent(..)` constructor, creating a new instance of `ExtendableMessageEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/ExtendableMessageEvent)\n\n*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
    pub fn new(
        this: &ExtendableMessageEvent,
        type_: &str,
    ) -> Result<ExtendableMessageEvent, JsValue>;
    #[cfg(feature = "ExtendableMessageEventInit")]
    #[wasm_bindgen(catch, js_class = "ExtendableMessageEvent", constructor)]
    #[doc = "The `new ExtendableMessageEvent(..)` constructor, creating a new instance of `ExtendableMessageEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/ExtendableMessageEvent)\n\n*This API requires the following crate features to be activated: `ExtendableMessageEvent`, `ExtendableMessageEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &ExtendableMessageEvent,
        type_: &str,
        event_init_dict: &ExtendableMessageEventInit,
    ) -> Result<ExtendableMessageEvent, JsValue>;
}
