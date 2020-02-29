use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGAngle , typescript_type = "SVGAngle" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgAngle` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle)
    ///
    ///*This API requires the following crate features to be activated: `SvgAngle`*
    pub type SvgAngle;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAngle" , js_name = unitType ) ]
    ///Getter for the `unitType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/unitType)
    ///
    ///*This API requires the following crate features to be activated: `SvgAngle`*
    pub fn unit_type(this: &SvgAngle) -> u16;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAngle" , js_name = value ) ]
    ///Getter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/value)
    ///
    ///*This API requires the following crate features to be activated: `SvgAngle`*
    pub fn value(this: &SvgAngle) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGAngle" , js_name = value ) ]
    ///Setter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/value)
    ///
    ///*This API requires the following crate features to be activated: `SvgAngle`*
    pub fn set_value(this: &SvgAngle, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAngle" , js_name = valueInSpecifiedUnits ) ]
    ///Getter for the `valueInSpecifiedUnits` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/valueInSpecifiedUnits)
    ///
    ///*This API requires the following crate features to be activated: `SvgAngle`*
    pub fn value_in_specified_units(this: &SvgAngle) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGAngle" , js_name = valueInSpecifiedUnits ) ]
    ///Setter for the `valueInSpecifiedUnits` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/valueInSpecifiedUnits)
    ///
    ///*This API requires the following crate features to be activated: `SvgAngle`*
    pub fn set_value_in_specified_units(this: &SvgAngle, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAngle" , js_name = valueAsString ) ]
    ///Getter for the `valueAsString` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/valueAsString)
    ///
    ///*This API requires the following crate features to be activated: `SvgAngle`*
    pub fn value_as_string(this: &SvgAngle) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGAngle" , js_name = valueAsString ) ]
    ///Setter for the `valueAsString` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/valueAsString)
    ///
    ///*This API requires the following crate features to be activated: `SvgAngle`*
    pub fn set_value_as_string(this: &SvgAngle, value: &str);

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGAngle" , js_name = convertToSpecifiedUnits ) ]
    ///The `convertToSpecifiedUnits()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/convertToSpecifiedUnits)
    ///
    ///*This API requires the following crate features to be activated: `SvgAngle`*
    pub fn convert_to_specified_units(this: &SvgAngle, unit_type: u16) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGAngle" , js_name = newValueSpecifiedUnits ) ]
    ///The `newValueSpecifiedUnits()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/newValueSpecifiedUnits)
    ///
    ///*This API requires the following crate features to be activated: `SvgAngle`*
    pub fn new_value_specified_units(
        this: &SvgAngle,
        unit_type: u16,
        value_in_specified_units: f32,
    ) -> Result<(), JsValue>;

}

impl SvgAngle {
    ///The `SVGAngle.SVG_ANGLETYPE_UNKNOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgAngle`*

    pub const SVG_ANGLETYPE_UNKNOWN: u16 = 0i64 as u16;

    ///The `SVGAngle.SVG_ANGLETYPE_UNSPECIFIED` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgAngle`*

    pub const SVG_ANGLETYPE_UNSPECIFIED: u16 = 1u64 as u16;

    ///The `SVGAngle.SVG_ANGLETYPE_DEG` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgAngle`*

    pub const SVG_ANGLETYPE_DEG: u16 = 2u64 as u16;

    ///The `SVGAngle.SVG_ANGLETYPE_RAD` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgAngle`*

    pub const SVG_ANGLETYPE_RAD: u16 = 3u64 as u16;

    ///The `SVGAngle.SVG_ANGLETYPE_GRAD` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgAngle`*

    pub const SVG_ANGLETYPE_GRAD: u16 = 4u64 as u16;
}
