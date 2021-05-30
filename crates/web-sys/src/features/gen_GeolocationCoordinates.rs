#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GeolocationCoordinates , typescript_type = "GeolocationCoordinates")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GeolocationCoordinates` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationCoordinates`*"]
    pub type GeolocationCoordinates;
    # [wasm_bindgen (structural , method , getter , js_class = "GeolocationCoordinates" , js_name = accuracy)]
    #[doc = "Getter for the `accuracy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/accuracy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationCoordinates`*"]
    pub fn accuracy(this: &GeolocationCoordinates) -> f64;
    # [wasm_bindgen (structural , method , getter , js_class = "GeolocationCoordinates" , js_name = latitude)]
    #[doc = "Getter for the `latitude` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/latitude)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationCoordinates`*"]
    pub fn latitude(this: &GeolocationCoordinates) -> f64;
    # [wasm_bindgen (structural , method , getter , js_class = "GeolocationCoordinates" , js_name = longitude)]
    #[doc = "Getter for the `longitude` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/longitude)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationCoordinates`*"]
    pub fn longitude(this: &GeolocationCoordinates) -> f64;
    # [wasm_bindgen (structural , method , getter , js_class = "GeolocationCoordinates" , js_name = altitude)]
    #[doc = "Getter for the `altitude` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/altitude)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationCoordinates`*"]
    pub fn altitude(this: &GeolocationCoordinates) -> Option<f64>;
    # [wasm_bindgen (structural , method , getter , js_class = "GeolocationCoordinates" , js_name = altitudeAccuracy)]
    #[doc = "Getter for the `altitudeAccuracy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/altitudeAccuracy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationCoordinates`*"]
    pub fn altitude_accuracy(this: &GeolocationCoordinates) -> Option<f64>;
    # [wasm_bindgen (structural , method , getter , js_class = "GeolocationCoordinates" , js_name = heading)]
    #[doc = "Getter for the `heading` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/heading)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationCoordinates`*"]
    pub fn heading(this: &GeolocationCoordinates) -> Option<f64>;
    # [wasm_bindgen (structural , method , getter , js_class = "GeolocationCoordinates" , js_name = speed)]
    #[doc = "Getter for the `speed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/speed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationCoordinates`*"]
    pub fn speed(this: &GeolocationCoordinates) -> Option<f64>;
}
