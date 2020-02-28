use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFilterElement , typescript_name = SVGFilterElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgFilterElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement)\n\n*This API requires the following crate features to be activated: `SvgFilterElement`*"]
    pub type SvgFilterElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = filterUnits ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `filterUnits` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/filterUnits)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgFilterElement`*"]
    pub fn filter_units(this: &SvgFilterElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_name = primitiveUnits ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `primitiveUnits` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/primitiveUnits)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgFilterElement`*"]
    pub fn primitive_units(this: &SvgFilterElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgFilterElement`*"]
    pub fn x(this: &SvgFilterElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgFilterElement`*"]
    pub fn y(this: &SvgFilterElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgFilterElement`*"]
    pub fn width(this: &SvgFilterElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgFilterElement`*"]
    pub fn height(this: &SvgFilterElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = href ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/href)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgFilterElement`*"]
    pub fn href(this: &SvgFilterElement) -> SvgAnimatedString;
}
