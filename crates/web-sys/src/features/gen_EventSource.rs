use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = EventSource , typescript_name = EventSource ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `EventSource` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource)
    ///
    ///*This API requires the following crate features to be activated: `EventSource`*
    pub type EventSource;

    # [ wasm_bindgen ( structural , method , getter , js_class = "EventSource" , js_name = url ) ]
    ///Getter for the `url` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/url)
    ///
    ///*This API requires the following crate features to be activated: `EventSource`*
    pub fn url(this: &EventSource) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "EventSource" , js_name = withCredentials ) ]
    ///Getter for the `withCredentials` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/withCredentials)
    ///
    ///*This API requires the following crate features to be activated: `EventSource`*
    pub fn with_credentials(this: &EventSource) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "EventSource" , js_name = readyState ) ]
    ///Getter for the `readyState` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/readyState)
    ///
    ///*This API requires the following crate features to be activated: `EventSource`*
    pub fn ready_state(this: &EventSource) -> u16;

    # [ wasm_bindgen ( structural , method , getter , js_class = "EventSource" , js_name = onopen ) ]
    ///Getter for the `onopen` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onopen)
    ///
    ///*This API requires the following crate features to be activated: `EventSource`*
    pub fn onopen(this: &EventSource) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "EventSource" , js_name = onopen ) ]
    ///Setter for the `onopen` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onopen)
    ///
    ///*This API requires the following crate features to be activated: `EventSource`*
    pub fn set_onopen(this: &EventSource, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "EventSource" , js_name = onmessage ) ]
    ///Getter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `EventSource`*
    pub fn onmessage(this: &EventSource) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "EventSource" , js_name = onmessage ) ]
    ///Setter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `EventSource`*
    pub fn set_onmessage(this: &EventSource, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "EventSource" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onerror)
    ///
    ///*This API requires the following crate features to be activated: `EventSource`*
    pub fn onerror(this: &EventSource) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "EventSource" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onerror)
    ///
    ///*This API requires the following crate features to be activated: `EventSource`*
    pub fn set_onerror(this: &EventSource, value: Option<&::js_sys::Function>);

    #[wasm_bindgen(catch, constructor, js_class = "EventSource")]
    ///The `new EventSource(..)` constructor, creating a new instance of `EventSource`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/EventSource)
    ///
    ///*This API requires the following crate features to be activated: `EventSource`*
    pub fn new(url: &str) -> Result<EventSource, JsValue>;

    #[cfg(feature = "EventSourceInit")]
    #[wasm_bindgen(catch, constructor, js_class = "EventSource")]
    ///The `new EventSource(..)` constructor, creating a new instance of `EventSource`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/EventSource)
    ///
    ///*This API requires the following crate features to be activated: `EventSource`, `EventSourceInit`*
    pub fn new_with_event_source_init_dict(
        url: &str,
        event_source_init_dict: &EventSourceInit,
    ) -> Result<EventSource, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "EventSource" , js_name = close ) ]
    ///The `close()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/close)
    ///
    ///*This API requires the following crate features to be activated: `EventSource`*
    pub fn close(this: &EventSource);

}

impl EventSource {
    ///The `EventSource.CONNECTING` const.
    ///
    ///*This API requires the following crate features to be activated: `EventSource`*

    pub const CONNECTING: u16 = 0i64 as u16;

    ///The `EventSource.OPEN` const.
    ///
    ///*This API requires the following crate features to be activated: `EventSource`*

    pub const OPEN: u16 = 1u64 as u16;

    ///The `EventSource.CLOSED` const.
    ///
    ///*This API requires the following crate features to be activated: `EventSource`*

    pub const CLOSED: u16 = 2u64 as u16;
}
