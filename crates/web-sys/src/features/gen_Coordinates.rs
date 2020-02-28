use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = Coordinates , typescript_name = Coordinates ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Coordinates` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates)\n\n*This API requires the following crate features to be activated: `Coordinates`*"]
    pub type Coordinates;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Coordinates" , js_name = latitude ) ]
    #[doc = "Getter for the `latitude` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/latitude)\n\n*This API requires the following crate features to be activated: `Coordinates`*"]
    pub fn latitude(this: &Coordinates) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Coordinates" , js_name = longitude ) ]
    #[doc = "Getter for the `longitude` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/longitude)\n\n*This API requires the following crate features to be activated: `Coordinates`*"]
    pub fn longitude(this: &Coordinates) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Coordinates" , js_name = altitude ) ]
    #[doc = "Getter for the `altitude` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/altitude)\n\n*This API requires the following crate features to be activated: `Coordinates`*"]
    pub fn altitude(this: &Coordinates) -> Option<f64>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Coordinates" , js_name = accuracy ) ]
    #[doc = "Getter for the `accuracy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/accuracy)\n\n*This API requires the following crate features to be activated: `Coordinates`*"]
    pub fn accuracy(this: &Coordinates) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Coordinates" , js_name = altitudeAccuracy ) ]
    #[doc = "Getter for the `altitudeAccuracy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/altitudeAccuracy)\n\n*This API requires the following crate features to be activated: `Coordinates`*"]
    pub fn altitude_accuracy(this: &Coordinates) -> Option<f64>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Coordinates" , js_name = heading ) ]
    #[doc = "Getter for the `heading` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/heading)\n\n*This API requires the following crate features to be activated: `Coordinates`*"]
    pub fn heading(this: &Coordinates) -> Option<f64>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Coordinates" , js_name = speed ) ]
    #[doc = "Getter for the `speed` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/speed)\n\n*This API requires the following crate features to be activated: `Coordinates`*"]
    pub fn speed(this: &Coordinates) -> Option<f64>;
}
