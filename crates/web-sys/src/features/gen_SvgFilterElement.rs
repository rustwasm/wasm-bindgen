use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFilterElement , typescript_name = SVGFilterElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgFilterElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgFilterElement`*
    pub type SvgFilterElement;

    #[cfg(feature = "SvgAnimatedEnumeration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFilterElement" , js_name = filterUnits ) ]
    ///Getter for the `filterUnits` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/filterUnits)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgFilterElement`*
    pub fn filter_units(this: &SvgFilterElement) -> SvgAnimatedEnumeration;

    #[cfg(feature = "SvgAnimatedEnumeration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFilterElement" , js_name = primitiveUnits ) ]
    ///Getter for the `primitiveUnits` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/primitiveUnits)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgFilterElement`*
    pub fn primitive_units(this: &SvgFilterElement) -> SvgAnimatedEnumeration;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFilterElement" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgFilterElement`*
    pub fn x(this: &SvgFilterElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFilterElement" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgFilterElement`*
    pub fn y(this: &SvgFilterElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFilterElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/width)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgFilterElement`*
    pub fn width(this: &SvgFilterElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFilterElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/height)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgFilterElement`*
    pub fn height(this: &SvgFilterElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFilterElement" , js_name = href ) ]
    ///Getter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/href)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgFilterElement`*
    pub fn href(this: &SvgFilterElement) -> SvgAnimatedString;

}
