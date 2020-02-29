use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = GetUserMediaRequest , typescript_name = GetUserMediaRequest ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GetUserMediaRequest` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest)
    ///
    ///*This API requires the following crate features to be activated: `GetUserMediaRequest`*
    pub type GetUserMediaRequest;

    # [ wasm_bindgen ( structural , method , getter , js_class = "GetUserMediaRequest" , js_name = windowID ) ]
    ///Getter for the `windowID` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/windowID)
    ///
    ///*This API requires the following crate features to be activated: `GetUserMediaRequest`*
    pub fn window_id(this: &GetUserMediaRequest) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "GetUserMediaRequest" , js_name = innerWindowID ) ]
    ///Getter for the `innerWindowID` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/innerWindowID)
    ///
    ///*This API requires the following crate features to be activated: `GetUserMediaRequest`*
    pub fn inner_window_id(this: &GetUserMediaRequest) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "GetUserMediaRequest" , js_name = callID ) ]
    ///Getter for the `callID` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/callID)
    ///
    ///*This API requires the following crate features to be activated: `GetUserMediaRequest`*
    pub fn call_id(this: &GetUserMediaRequest) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "GetUserMediaRequest" , js_name = rawID ) ]
    ///Getter for the `rawID` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/rawID)
    ///
    ///*This API requires the following crate features to be activated: `GetUserMediaRequest`*
    pub fn raw_id(this: &GetUserMediaRequest) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "GetUserMediaRequest" , js_name = mediaSource ) ]
    ///Getter for the `mediaSource` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/mediaSource)
    ///
    ///*This API requires the following crate features to be activated: `GetUserMediaRequest`*
    pub fn media_source(this: &GetUserMediaRequest) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "GetUserMediaRequest" , js_name = isSecure ) ]
    ///Getter for the `isSecure` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/isSecure)
    ///
    ///*This API requires the following crate features to be activated: `GetUserMediaRequest`*
    pub fn is_secure(this: &GetUserMediaRequest) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "GetUserMediaRequest" , js_name = isHandlingUserInput ) ]
    ///Getter for the `isHandlingUserInput` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/isHandlingUserInput)
    ///
    ///*This API requires the following crate features to be activated: `GetUserMediaRequest`*
    pub fn is_handling_user_input(this: &GetUserMediaRequest) -> bool;

    #[cfg(feature = "MediaStreamConstraints")]
    # [ wasm_bindgen ( method , structural , js_class = "GetUserMediaRequest" , js_name = getConstraints ) ]
    ///The `getConstraints()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/getConstraints)
    ///
    ///*This API requires the following crate features to be activated: `GetUserMediaRequest`, `MediaStreamConstraints`*
    pub fn get_constraints(this: &GetUserMediaRequest) -> MediaStreamConstraints;

}
