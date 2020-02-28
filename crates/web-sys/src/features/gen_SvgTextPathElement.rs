use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgTextContentElement , extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGTextPathElement , typescript_name = SVGTextPathElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgTextPathElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextPathElement`*"]
    pub type SvgTextPathElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTextPathElement" , js_name = startOffset ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `startOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement/startOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgTextPathElement`*"]
    pub fn start_offset(this: &SvgTextPathElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTextPathElement" , js_name = method ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `method` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement/method)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgTextPathElement`*"]
    pub fn method(this: &SvgTextPathElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTextPathElement" , js_name = spacing ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `spacing` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement/spacing)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgTextPathElement`*"]
    pub fn spacing(this: &SvgTextPathElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTextPathElement" , js_name = href ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgTextPathElement`*"]
    pub fn href(this: &SvgTextPathElement) -> SvgAnimatedString;
}
impl SvgTextPathElement {
    pub const TEXTPATH_METHODTYPE_UNKNOWN: u16 = 0i64 as u16;
    pub const TEXTPATH_METHODTYPE_ALIGN: u16 = 1u64 as u16;
    pub const TEXTPATH_METHODTYPE_STRETCH: u16 = 2u64 as u16;
    pub const TEXTPATH_SPACINGTYPE_UNKNOWN: u16 = 0i64 as u16;
    pub const TEXTPATH_SPACINGTYPE_AUTO: u16 = 1u64 as u16;
    pub const TEXTPATH_SPACINGTYPE_EXACT: u16 = 2u64 as u16;
}
