use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgGeometryElement , extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGPathElement , typescript_name = SVGPathElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgPathElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathElement`*
    pub type SvgPathElement;

    #[cfg(feature = "SvgPathSegList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathElement" , js_name = pathSegList ) ]
    ///Getter for the `pathSegList` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement/pathSegList)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathElement`, `SvgPathSegList`*
    pub fn path_seg_list(this: &SvgPathElement) -> SvgPathSegList;

    #[cfg(feature = "SvgPathSegList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathElement" , js_name = animatedPathSegList ) ]
    ///Getter for the `animatedPathSegList` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement/animatedPathSegList)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathElement`, `SvgPathSegList`*
    pub fn animated_path_seg_list(this: &SvgPathElement) -> SvgPathSegList;

    # [ wasm_bindgen ( method , structural , js_class = "SVGPathElement" , js_name = getPathSegAtLength ) ]
    ///The `getPathSegAtLength()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement/getPathSegAtLength)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathElement`*
    pub fn get_path_seg_at_length(this: &SvgPathElement, distance: f32) -> u32;

}
