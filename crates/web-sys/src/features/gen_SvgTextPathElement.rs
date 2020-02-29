use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgTextContentElement , extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGTextPathElement , typescript_name = SVGTextPathElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgTextPathElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgTextPathElement`*
    pub type SvgTextPathElement;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTextPathElement" , js_name = startOffset ) ]
    ///Getter for the `startOffset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement/startOffset)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgTextPathElement`*
    pub fn start_offset(this: &SvgTextPathElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedEnumeration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTextPathElement" , js_name = method ) ]
    ///Getter for the `method` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement/method)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgTextPathElement`*
    pub fn method(this: &SvgTextPathElement) -> SvgAnimatedEnumeration;

    #[cfg(feature = "SvgAnimatedEnumeration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTextPathElement" , js_name = spacing ) ]
    ///Getter for the `spacing` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement/spacing)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgTextPathElement`*
    pub fn spacing(this: &SvgTextPathElement) -> SvgAnimatedEnumeration;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTextPathElement" , js_name = href ) ]
    ///Getter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement/href)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgTextPathElement`*
    pub fn href(this: &SvgTextPathElement) -> SvgAnimatedString;

}

impl SvgTextPathElement {
    ///The `SVGTextPathElement.TEXTPATH_METHODTYPE_UNKNOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgTextPathElement`*

    pub const TEXTPATH_METHODTYPE_UNKNOWN: u16 = 0i64 as u16;

    ///The `SVGTextPathElement.TEXTPATH_METHODTYPE_ALIGN` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgTextPathElement`*

    pub const TEXTPATH_METHODTYPE_ALIGN: u16 = 1u64 as u16;

    ///The `SVGTextPathElement.TEXTPATH_METHODTYPE_STRETCH` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgTextPathElement`*

    pub const TEXTPATH_METHODTYPE_STRETCH: u16 = 2u64 as u16;

    ///The `SVGTextPathElement.TEXTPATH_SPACINGTYPE_UNKNOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgTextPathElement`*

    pub const TEXTPATH_SPACINGTYPE_UNKNOWN: u16 = 0i64 as u16;

    ///The `SVGTextPathElement.TEXTPATH_SPACINGTYPE_AUTO` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgTextPathElement`*

    pub const TEXTPATH_SPACINGTYPE_AUTO: u16 = 1u64 as u16;

    ///The `SVGTextPathElement.TEXTPATH_SPACINGTYPE_EXACT` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgTextPathElement`*

    pub const TEXTPATH_SPACINGTYPE_EXACT: u16 = 2u64 as u16;
}
