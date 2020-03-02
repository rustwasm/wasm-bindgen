use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = SVGPathSeg , typescript_type = "SVGPathSeg" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgPathSeg` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSeg)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*
    pub type SvgPathSeg;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSeg" , js_name = pathSegType ) ]
    ///Getter for the `pathSegType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSeg/pathSegType)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*
    pub fn path_seg_type(this: &SvgPathSeg) -> u16;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSeg" , js_name = pathSegTypeAsLetter ) ]
    ///Getter for the `pathSegTypeAsLetter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSeg/pathSegTypeAsLetter)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*
    pub fn path_seg_type_as_letter(this: &SvgPathSeg) -> String;

}

impl SvgPathSeg {
    ///The `SVGPathSeg.PATHSEG_UNKNOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*

    pub const PATHSEG_UNKNOWN: u16 = 0i64 as u16;

    ///The `SVGPathSeg.PATHSEG_CLOSEPATH` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*

    pub const PATHSEG_CLOSEPATH: u16 = 1u64 as u16;

    ///The `SVGPathSeg.PATHSEG_MOVETO_ABS` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*

    pub const PATHSEG_MOVETO_ABS: u16 = 2u64 as u16;

    ///The `SVGPathSeg.PATHSEG_MOVETO_REL` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*

    pub const PATHSEG_MOVETO_REL: u16 = 3u64 as u16;

    ///The `SVGPathSeg.PATHSEG_LINETO_ABS` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*

    pub const PATHSEG_LINETO_ABS: u16 = 4u64 as u16;

    ///The `SVGPathSeg.PATHSEG_LINETO_REL` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*

    pub const PATHSEG_LINETO_REL: u16 = 5u64 as u16;

    ///The `SVGPathSeg.PATHSEG_CURVETO_CUBIC_ABS` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*

    pub const PATHSEG_CURVETO_CUBIC_ABS: u16 = 6u64 as u16;

    ///The `SVGPathSeg.PATHSEG_CURVETO_CUBIC_REL` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*

    pub const PATHSEG_CURVETO_CUBIC_REL: u16 = 7u64 as u16;

    ///The `SVGPathSeg.PATHSEG_CURVETO_QUADRATIC_ABS` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*

    pub const PATHSEG_CURVETO_QUADRATIC_ABS: u16 = 8u64 as u16;

    ///The `SVGPathSeg.PATHSEG_CURVETO_QUADRATIC_REL` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*

    pub const PATHSEG_CURVETO_QUADRATIC_REL: u16 = 9u64 as u16;

    ///The `SVGPathSeg.PATHSEG_ARC_ABS` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*

    pub const PATHSEG_ARC_ABS: u16 = 10u64 as u16;

    ///The `SVGPathSeg.PATHSEG_ARC_REL` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*

    pub const PATHSEG_ARC_REL: u16 = 11u64 as u16;

    ///The `SVGPathSeg.PATHSEG_LINETO_HORIZONTAL_ABS` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*

    pub const PATHSEG_LINETO_HORIZONTAL_ABS: u16 = 12u64 as u16;

    ///The `SVGPathSeg.PATHSEG_LINETO_HORIZONTAL_REL` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*

    pub const PATHSEG_LINETO_HORIZONTAL_REL: u16 = 13u64 as u16;

    ///The `SVGPathSeg.PATHSEG_LINETO_VERTICAL_ABS` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*

    pub const PATHSEG_LINETO_VERTICAL_ABS: u16 = 14u64 as u16;

    ///The `SVGPathSeg.PATHSEG_LINETO_VERTICAL_REL` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*

    pub const PATHSEG_LINETO_VERTICAL_REL: u16 = 15u64 as u16;

    ///The `SVGPathSeg.PATHSEG_CURVETO_CUBIC_SMOOTH_ABS` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*

    pub const PATHSEG_CURVETO_CUBIC_SMOOTH_ABS: u16 = 16u64 as u16;

    ///The `SVGPathSeg.PATHSEG_CURVETO_CUBIC_SMOOTH_REL` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*

    pub const PATHSEG_CURVETO_CUBIC_SMOOTH_REL: u16 = 17u64 as u16;

    ///The `SVGPathSeg.PATHSEG_CURVETO_QUADRATIC_SMOOTH_ABS` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*

    pub const PATHSEG_CURVETO_QUADRATIC_SMOOTH_ABS: u16 = 18u64 as u16;

    ///The `SVGPathSeg.PATHSEG_CURVETO_QUADRATIC_SMOOTH_REL` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`*

    pub const PATHSEG_CURVETO_QUADRATIC_SMOOTH_REL: u16 = 19u64 as u16;
}
