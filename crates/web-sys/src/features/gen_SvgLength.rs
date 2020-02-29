use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGLength , typescript_type = "SVGLength" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgLength` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength)
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`*
    pub type SvgLength;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGLength" , js_name = unitType ) ]
    ///Getter for the `unitType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/unitType)
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`*
    pub fn unit_type(this: &SvgLength) -> u16;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "SVGLength" , js_name = value ) ]
    ///Getter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/value)
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`*
    pub fn value(this: &SvgLength) -> Result<f32, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "SVGLength" , js_name = value ) ]
    ///Setter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/value)
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`*
    pub fn set_value(this: &SvgLength, value: f32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGLength" , js_name = valueInSpecifiedUnits ) ]
    ///Getter for the `valueInSpecifiedUnits` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/valueInSpecifiedUnits)
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`*
    pub fn value_in_specified_units(this: &SvgLength) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGLength" , js_name = valueInSpecifiedUnits ) ]
    ///Setter for the `valueInSpecifiedUnits` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/valueInSpecifiedUnits)
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`*
    pub fn set_value_in_specified_units(this: &SvgLength, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGLength" , js_name = valueAsString ) ]
    ///Getter for the `valueAsString` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/valueAsString)
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`*
    pub fn value_as_string(this: &SvgLength) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGLength" , js_name = valueAsString ) ]
    ///Setter for the `valueAsString` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/valueAsString)
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`*
    pub fn set_value_as_string(this: &SvgLength, value: &str);

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGLength" , js_name = convertToSpecifiedUnits ) ]
    ///The `convertToSpecifiedUnits()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/convertToSpecifiedUnits)
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`*
    pub fn convert_to_specified_units(this: &SvgLength, unit_type: u16) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGLength" , js_name = newValueSpecifiedUnits ) ]
    ///The `newValueSpecifiedUnits()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/newValueSpecifiedUnits)
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`*
    pub fn new_value_specified_units(
        this: &SvgLength,
        unit_type: u16,
        value_in_specified_units: f32,
    ) -> Result<(), JsValue>;

}

impl SvgLength {
    ///The `SVGLength.SVG_LENGTHTYPE_UNKNOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`*

    pub const SVG_LENGTHTYPE_UNKNOWN: u16 = 0i64 as u16;

    ///The `SVGLength.SVG_LENGTHTYPE_NUMBER` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`*

    pub const SVG_LENGTHTYPE_NUMBER: u16 = 1u64 as u16;

    ///The `SVGLength.SVG_LENGTHTYPE_PERCENTAGE` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`*

    pub const SVG_LENGTHTYPE_PERCENTAGE: u16 = 2u64 as u16;

    ///The `SVGLength.SVG_LENGTHTYPE_EMS` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`*

    pub const SVG_LENGTHTYPE_EMS: u16 = 3u64 as u16;

    ///The `SVGLength.SVG_LENGTHTYPE_EXS` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`*

    pub const SVG_LENGTHTYPE_EXS: u16 = 4u64 as u16;

    ///The `SVGLength.SVG_LENGTHTYPE_PX` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`*

    pub const SVG_LENGTHTYPE_PX: u16 = 5u64 as u16;

    ///The `SVGLength.SVG_LENGTHTYPE_CM` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`*

    pub const SVG_LENGTHTYPE_CM: u16 = 6u64 as u16;

    ///The `SVGLength.SVG_LENGTHTYPE_MM` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`*

    pub const SVG_LENGTHTYPE_MM: u16 = 7u64 as u16;

    ///The `SVGLength.SVG_LENGTHTYPE_IN` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`*

    pub const SVG_LENGTHTYPE_IN: u16 = 8u64 as u16;

    ///The `SVGLength.SVG_LENGTHTYPE_PT` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`*

    pub const SVG_LENGTHTYPE_PT: u16 = 9u64 as u16;

    ///The `SVGLength.SVG_LENGTHTYPE_PC` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`*

    pub const SVG_LENGTHTYPE_PC: u16 = 10u64 as u16;
}
