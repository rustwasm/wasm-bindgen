use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGClipPathElement , typescript_name = SVGClipPathElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgClipPathElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGClipPathElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgClipPathElement`*
    pub type SvgClipPathElement;

    #[cfg(feature = "SvgAnimatedEnumeration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGClipPathElement" , js_name = clipPathUnits ) ]
    ///Getter for the `clipPathUnits` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGClipPathElement/clipPathUnits)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgClipPathElement`*
    pub fn clip_path_units(this: &SvgClipPathElement) -> SvgAnimatedEnumeration;

    #[cfg(feature = "SvgAnimatedTransformList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGClipPathElement" , js_name = transform ) ]
    ///Getter for the `transform` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGClipPathElement/transform)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgClipPathElement`*
    pub fn transform(this: &SvgClipPathElement) -> SvgAnimatedTransformList;

}
