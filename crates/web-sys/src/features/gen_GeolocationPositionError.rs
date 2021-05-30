#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GeolocationPositionError , typescript_type = "GeolocationPositionError")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GeolocationPositionError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationPositionError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationPositionError`*"]
    pub type GeolocationPositionError;
    # [wasm_bindgen (structural , method , getter , js_class = "GeolocationPositionError" , js_name = code)]
    #[doc = "Getter for the `code` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationPositionError/code)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationPositionError`*"]
    pub fn code(this: &GeolocationPositionError) -> u16;
    # [wasm_bindgen (structural , method , getter , js_class = "GeolocationPositionError" , js_name = message)]
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationPositionError/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationPositionError`*"]
    pub fn message(this: &GeolocationPositionError) -> String;
}
impl GeolocationPositionError {
    #[doc = "The `GeolocationPositionError.PERMISSION_DENIED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationPositionError`*"]
    pub const PERMISSION_DENIED: u16 = 1u64 as u16;
    #[doc = "The `GeolocationPositionError.POSITION_UNAVAILABLE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationPositionError`*"]
    pub const POSITION_UNAVAILABLE: u16 = 2u64 as u16;
    #[doc = "The `GeolocationPositionError.TIMEOUT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationPositionError`*"]
    pub const TIMEOUT: u16 = 3u64 as u16;
}
