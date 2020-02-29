use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGViewElement , typescript_name = SVGViewElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgViewElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgViewElement`*
    pub type SvgViewElement;

    #[cfg(feature = "SvgAnimatedRect")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGViewElement" , js_name = viewBox ) ]
    ///Getter for the `viewBox` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/viewBox)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgViewElement`*
    pub fn view_box(this: &SvgViewElement) -> SvgAnimatedRect;

    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGViewElement" , js_name = preserveAspectRatio ) ]
    ///Getter for the `preserveAspectRatio` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/preserveAspectRatio)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgViewElement`*
    pub fn preserve_aspect_ratio(this: &SvgViewElement) -> SvgAnimatedPreserveAspectRatio;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGViewElement" , js_name = zoomAndPan ) ]
    ///Getter for the `zoomAndPan` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/zoomAndPan)
    ///
    ///*This API requires the following crate features to be activated: `SvgViewElement`*
    pub fn zoom_and_pan(this: &SvgViewElement) -> u16;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGViewElement" , js_name = zoomAndPan ) ]
    ///Setter for the `zoomAndPan` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/zoomAndPan)
    ///
    ///*This API requires the following crate features to be activated: `SvgViewElement`*
    pub fn set_zoom_and_pan(this: &SvgViewElement, value: u16);

}

impl SvgViewElement {
    ///The `SVGViewElement.SVG_ZOOMANDPAN_UNKNOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgViewElement`*

    pub const SVG_ZOOMANDPAN_UNKNOWN: u16 = 0i64 as u16;

    ///The `SVGViewElement.SVG_ZOOMANDPAN_DISABLE` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgViewElement`*

    pub const SVG_ZOOMANDPAN_DISABLE: u16 = 1u64 as u16;

    ///The `SVGViewElement.SVG_ZOOMANDPAN_MAGNIFY` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgViewElement`*

    pub const SVG_ZOOMANDPAN_MAGNIFY: u16 = 2u64 as u16;
}
