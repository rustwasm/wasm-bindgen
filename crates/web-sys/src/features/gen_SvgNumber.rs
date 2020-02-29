use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGNumber , typescript_name = SVGNumber ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgNumber` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumber)
    ///
    ///*This API requires the following crate features to be activated: `SvgNumber`*
    pub type SvgNumber;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGNumber" , js_name = value ) ]
    ///Getter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumber/value)
    ///
    ///*This API requires the following crate features to be activated: `SvgNumber`*
    pub fn value(this: &SvgNumber) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGNumber" , js_name = value ) ]
    ///Setter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumber/value)
    ///
    ///*This API requires the following crate features to be activated: `SvgNumber`*
    pub fn set_value(this: &SvgNumber, value: f32);

}
