use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = Position , typescript_name = Position ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Position` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Position)\n\n*This API requires the following crate features to be activated: `Position`*"]
    pub type Position;
    # [ wasm_bindgen ( structural , method , getter , js_name = coords ) ]
    #[cfg(feature = "Coordinates")]
    #[doc = "Getter for the `coords` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Position/coords)\n\n*This API requires the following crate features to be activated: `Coordinates`, `Position`*"]
    pub fn coords(this: &Position) -> Coordinates;
    # [ wasm_bindgen ( structural , method , getter , js_name = timestamp ) ]
    #[doc = "Getter for the `timestamp` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Position/timestamp)\n\n*This API requires the following crate features to be activated: `Position`*"]
    pub fn timestamp(this: &Position) -> f64;
}
