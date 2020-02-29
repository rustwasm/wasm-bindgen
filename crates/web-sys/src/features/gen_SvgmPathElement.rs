use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGMPathElement , typescript_name = SVGMPathElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgmPathElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMPathElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgmPathElement`*
    pub type SvgmPathElement;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMPathElement" , js_name = href ) ]
    ///Getter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMPathElement/href)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgmPathElement`*
    pub fn href(this: &SvgmPathElement) -> SvgAnimatedString;

}
