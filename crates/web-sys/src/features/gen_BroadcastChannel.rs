use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = BroadcastChannel , typescript_type = "BroadcastChannel" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `BroadcastChannel` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel)
    ///
    ///*This API requires the following crate features to be activated: `BroadcastChannel`*
    pub type BroadcastChannel;

    # [ wasm_bindgen ( structural , method , getter , js_class = "BroadcastChannel" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/name)
    ///
    ///*This API requires the following crate features to be activated: `BroadcastChannel`*
    pub fn name(this: &BroadcastChannel) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "BroadcastChannel" , js_name = onmessage ) ]
    ///Getter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `BroadcastChannel`*
    pub fn onmessage(this: &BroadcastChannel) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "BroadcastChannel" , js_name = onmessage ) ]
    ///Setter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `BroadcastChannel`*
    pub fn set_onmessage(this: &BroadcastChannel, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "BroadcastChannel" , js_name = onmessageerror ) ]
    ///Getter for the `onmessageerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessageerror)
    ///
    ///*This API requires the following crate features to be activated: `BroadcastChannel`*
    pub fn onmessageerror(this: &BroadcastChannel) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "BroadcastChannel" , js_name = onmessageerror ) ]
    ///Setter for the `onmessageerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessageerror)
    ///
    ///*This API requires the following crate features to be activated: `BroadcastChannel`*
    pub fn set_onmessageerror(this: &BroadcastChannel, value: Option<&::js_sys::Function>);

    #[wasm_bindgen(catch, constructor, js_class = "BroadcastChannel")]
    ///The `new BroadcastChannel(..)` constructor, creating a new instance of `BroadcastChannel`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/BroadcastChannel)
    ///
    ///*This API requires the following crate features to be activated: `BroadcastChannel`*
    pub fn new(channel: &str) -> Result<BroadcastChannel, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "BroadcastChannel" , js_name = close ) ]
    ///The `close()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/close)
    ///
    ///*This API requires the following crate features to be activated: `BroadcastChannel`*
    pub fn close(this: &BroadcastChannel);

    # [ wasm_bindgen ( catch , method , structural , js_class = "BroadcastChannel" , js_name = postMessage ) ]
    ///The `postMessage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/postMessage)
    ///
    ///*This API requires the following crate features to be activated: `BroadcastChannel`*
    pub fn post_message(
        this: &BroadcastChannel,
        message: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;

}
