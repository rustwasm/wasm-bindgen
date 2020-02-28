use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = GetUserMediaRequest , typescript_name = GetUserMediaRequest ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GetUserMediaRequest` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest)\n\n*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
    pub type GetUserMediaRequest;
    # [ wasm_bindgen ( structural , method , getter , js_class = "GetUserMediaRequest" , js_name = windowID ) ]
    #[doc = "Getter for the `windowID` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/windowID)\n\n*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
    pub fn window_id(this: &GetUserMediaRequest) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "GetUserMediaRequest" , js_name = innerWindowID ) ]
    #[doc = "Getter for the `innerWindowID` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/innerWindowID)\n\n*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
    pub fn inner_window_id(this: &GetUserMediaRequest) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "GetUserMediaRequest" , js_name = callID ) ]
    #[doc = "Getter for the `callID` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/callID)\n\n*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
    pub fn call_id(this: &GetUserMediaRequest) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "GetUserMediaRequest" , js_name = rawID ) ]
    #[doc = "Getter for the `rawID` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/rawID)\n\n*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
    pub fn raw_id(this: &GetUserMediaRequest) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "GetUserMediaRequest" , js_name = mediaSource ) ]
    #[doc = "Getter for the `mediaSource` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/mediaSource)\n\n*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
    pub fn media_source(this: &GetUserMediaRequest) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "GetUserMediaRequest" , js_name = isSecure ) ]
    #[doc = "Getter for the `isSecure` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/isSecure)\n\n*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
    pub fn is_secure(this: &GetUserMediaRequest) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_class = "GetUserMediaRequest" , js_name = isHandlingUserInput ) ]
    #[doc = "Getter for the `isHandlingUserInput` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/isHandlingUserInput)\n\n*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
    pub fn is_handling_user_input(this: &GetUserMediaRequest) -> bool;
    #[cfg(feature = "MediaStreamConstraints")]
    # [ wasm_bindgen ( method , structural , js_class = "GetUserMediaRequest" , js_name = getConstraints ) ]
    #[doc = "The `getConstraints()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/getConstraints)\n\n*This API requires the following crate features to be activated: `GetUserMediaRequest`, `MediaStreamConstraints`*"]
    pub fn get_constraints(this: &GetUserMediaRequest) -> MediaStreamConstraints;
}
