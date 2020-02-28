use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGZoomAndPan , typescript_name = SVGZoomAndPan ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgZoomAndPan` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGZoomAndPan)\n\n*This API requires the following crate features to be activated: `SvgZoomAndPan`*"]
    pub type SvgZoomAndPan;
    # [ wasm_bindgen ( structural , method , getter , js_name = zoomAndPan ) ]
    #[doc = "Getter for the `zoomAndPan` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGZoomAndPan/zoomAndPan)\n\n*This API requires the following crate features to be activated: `SvgZoomAndPan`*"]
    pub fn zoom_and_pan(this: &SvgZoomAndPan) -> u16;
    # [ wasm_bindgen ( structural , method , setter , js_name = zoomAndPan ) ]
    #[doc = "Setter for the `zoomAndPan` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGZoomAndPan/zoomAndPan)\n\n*This API requires the following crate features to be activated: `SvgZoomAndPan`*"]
    pub fn set_zoom_and_pan(this: &SvgZoomAndPan, value: u16);
}
impl SvgZoomAndPan {
    pub const SVG_ZOOMANDPAN_UNKNOWN: u16 = 0i64 as u16;
    pub const SVG_ZOOMANDPAN_DISABLE: u16 = 1u64 as u16;
    pub const SVG_ZOOMANDPAN_MAGNIFY: u16 = 2u64 as u16;
}
