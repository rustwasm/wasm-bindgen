use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGClipPathElement , typescript_name = SVGClipPathElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgClipPathElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGClipPathElement)\n\n*This API requires the following crate features to be activated: `SvgClipPathElement`*"]
    pub type SvgClipPathElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGClipPathElement" , js_name = clipPathUnits ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `clipPathUnits` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGClipPathElement/clipPathUnits)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgClipPathElement`*"]
    pub fn clip_path_units(this: &SvgClipPathElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGClipPathElement" , js_name = transform ) ]
    #[cfg(feature = "SvgAnimatedTransformList")]
    #[doc = "Getter for the `transform` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGClipPathElement/transform)\n\n*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgClipPathElement`*"]
    pub fn transform(this: &SvgClipPathElement) -> SvgAnimatedTransformList;
}
