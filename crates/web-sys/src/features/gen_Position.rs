use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = Position , typescript_type = "Position" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Position` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Position)
    ///
    ///*This API requires the following crate features to be activated: `Position`*
    pub type Position;

    #[cfg(feature = "Coordinates")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Position" , js_name = coords ) ]
    ///Getter for the `coords` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Position/coords)
    ///
    ///*This API requires the following crate features to be activated: `Coordinates`, `Position`*
    pub fn coords(this: &Position) -> Coordinates;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Position" , js_name = timestamp ) ]
    ///Getter for the `timestamp` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Position/timestamp)
    ///
    ///*This API requires the following crate features to be activated: `Position`*
    pub fn timestamp(this: &Position) -> f64;

}
