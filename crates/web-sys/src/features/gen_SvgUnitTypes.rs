use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGUnitTypes , typescript_type = "SVGUnitTypes" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgUnitTypes` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGUnitTypes)
    ///
    ///*This API requires the following crate features to be activated: `SvgUnitTypes`*
    pub type SvgUnitTypes;

}

impl SvgUnitTypes {
    ///The `SVGUnitTypes.SVG_UNIT_TYPE_UNKNOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgUnitTypes`*

    pub const SVG_UNIT_TYPE_UNKNOWN: u16 = 0i64 as u16;

    ///The `SVGUnitTypes.SVG_UNIT_TYPE_USERSPACEONUSE` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgUnitTypes`*

    pub const SVG_UNIT_TYPE_USERSPACEONUSE: u16 = 1u64 as u16;

    ///The `SVGUnitTypes.SVG_UNIT_TYPE_OBJECTBOUNDINGBOX` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgUnitTypes`*

    pub const SVG_UNIT_TYPE_OBJECTBOUNDINGBOX: u16 = 2u64 as u16;
}
