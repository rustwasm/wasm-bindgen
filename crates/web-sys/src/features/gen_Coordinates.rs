use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = Coordinates , typescript_type = "Coordinates" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Coordinates` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates)
    ///
    ///*This API requires the following crate features to be activated: `Coordinates`*
    pub type Coordinates;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Coordinates" , js_name = latitude ) ]
    ///Getter for the `latitude` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/latitude)
    ///
    ///*This API requires the following crate features to be activated: `Coordinates`*
    pub fn latitude(this: &Coordinates) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Coordinates" , js_name = longitude ) ]
    ///Getter for the `longitude` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/longitude)
    ///
    ///*This API requires the following crate features to be activated: `Coordinates`*
    pub fn longitude(this: &Coordinates) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Coordinates" , js_name = altitude ) ]
    ///Getter for the `altitude` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/altitude)
    ///
    ///*This API requires the following crate features to be activated: `Coordinates`*
    pub fn altitude(this: &Coordinates) -> Option<f64>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Coordinates" , js_name = accuracy ) ]
    ///Getter for the `accuracy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/accuracy)
    ///
    ///*This API requires the following crate features to be activated: `Coordinates`*
    pub fn accuracy(this: &Coordinates) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Coordinates" , js_name = altitudeAccuracy ) ]
    ///Getter for the `altitudeAccuracy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/altitudeAccuracy)
    ///
    ///*This API requires the following crate features to be activated: `Coordinates`*
    pub fn altitude_accuracy(this: &Coordinates) -> Option<f64>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Coordinates" , js_name = heading ) ]
    ///Getter for the `heading` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/heading)
    ///
    ///*This API requires the following crate features to be activated: `Coordinates`*
    pub fn heading(this: &Coordinates) -> Option<f64>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Coordinates" , js_name = speed ) ]
    ///Getter for the `speed` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/speed)
    ///
    ///*This API requires the following crate features to be activated: `Coordinates`*
    pub fn speed(this: &Coordinates) -> Option<f64>;

}
