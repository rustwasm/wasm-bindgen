#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GeolocationPosition , typescript_type = "GeolocationPosition")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GeolocationPosition` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationPosition`*"]
    pub type GeolocationPosition;
    #[cfg(feature = "GeolocationCoordinates")]
    # [wasm_bindgen (structural , method , getter , js_class = "GeolocationPosition" , js_name = coords)]
    #[doc = "Getter for the `coords` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationPosition/coords)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationCoordinates`, `GeolocationPosition`*"]
    pub fn coords(this: &GeolocationPosition) -> GeolocationCoordinates;
    # [wasm_bindgen (structural , method , getter , js_class = "GeolocationPosition" , js_name = timestamp)]
    #[doc = "Getter for the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationPosition/timestamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationPosition`*"]
    pub fn timestamp(this: &GeolocationPosition) -> f64;
}
