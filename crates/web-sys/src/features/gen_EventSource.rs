use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = EventSource , typescript_name = EventSource ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EventSource` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    pub type EventSource;
    # [ wasm_bindgen ( structural , method , getter , js_name = url ) ]
    #[doc = "Getter for the `url` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/url)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn url(this: &EventSource) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = withCredentials ) ]
    #[doc = "Getter for the `withCredentials` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/withCredentials)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn with_credentials(this: &EventSource) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = readyState ) ]
    #[doc = "Getter for the `readyState` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/readyState)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn ready_state(this: &EventSource) -> u16;
    # [ wasm_bindgen ( structural , method , getter , js_name = onopen ) ]
    #[doc = "Getter for the `onopen` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onopen)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn onopen(this: &EventSource) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onopen ) ]
    #[doc = "Setter for the `onopen` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onopen)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn set_onopen(this: &EventSource, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmessage ) ]
    #[doc = "Getter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onmessage)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn onmessage(this: &EventSource) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmessage ) ]
    #[doc = "Setter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onmessage)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn set_onmessage(this: &EventSource, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onerror)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn onerror(this: &EventSource) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onerror)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn set_onerror(this: &EventSource, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new EventSource(..)` constructor, creating a new instance of `EventSource`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/EventSource)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn new(this: &EventSource, url: &str) -> Result<EventSource, JsValue>;
    #[cfg(feature = "EventSourceInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new EventSource(..)` constructor, creating a new instance of `EventSource`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/EventSource)\n\n*This API requires the following crate features to be activated: `EventSource`, `EventSourceInit`*"]
    pub fn new_with_event_source_init_dict(
        this: &EventSource,
        url: &str,
        event_source_init_dict: &EventSourceInit,
    ) -> Result<EventSource, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = close ) ]
    #[doc = "The `close()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/close)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn close(this: &EventSource);
}
impl EventSource {
    pub const CONNECTING: u16 = 0i64 as u16;
    pub const OPEN: u16 = 1u64 as u16;
    pub const CLOSED: u16 = 2u64 as u16;
}
