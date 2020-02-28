use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGViewElement , typescript_name = SVGViewElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgViewElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement)\n\n*This API requires the following crate features to be activated: `SvgViewElement`*"]
    pub type SvgViewElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = viewBox ) ]
    #[cfg(feature = "SvgAnimatedRect")]
    #[doc = "Getter for the `viewBox` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/viewBox)\n\n*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgViewElement`*"]
    pub fn view_box(this: &SvgViewElement) -> SvgAnimatedRect;
    # [ wasm_bindgen ( structural , method , getter , js_name = preserveAspectRatio ) ]
    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    #[doc = "Getter for the `preserveAspectRatio` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/preserveAspectRatio)\n\n*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgViewElement`*"]
    pub fn preserve_aspect_ratio(this: &SvgViewElement) -> SvgAnimatedPreserveAspectRatio;
    # [ wasm_bindgen ( structural , method , getter , js_name = zoomAndPan ) ]
    #[doc = "Getter for the `zoomAndPan` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/zoomAndPan)\n\n*This API requires the following crate features to be activated: `SvgViewElement`*"]
    pub fn zoom_and_pan(this: &SvgViewElement) -> u16;
    # [ wasm_bindgen ( structural , method , setter , js_name = zoomAndPan ) ]
    #[doc = "Setter for the `zoomAndPan` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/zoomAndPan)\n\n*This API requires the following crate features to be activated: `SvgViewElement`*"]
    pub fn set_zoom_and_pan(this: &SvgViewElement, value: u16);
}
impl SvgViewElement {
    pub const SVG_ZOOMANDPAN_UNKNOWN: u16 = 0i64 as u16;
    pub const SVG_ZOOMANDPAN_DISABLE: u16 = 1u64 as u16;
    pub const SVG_ZOOMANDPAN_MAGNIFY: u16 = 2u64 as u16;
}
