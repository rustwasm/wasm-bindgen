use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEMergeNodeElement , typescript_name = SVGFEMergeNodeElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeMergeNodeElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMergeNodeElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeMergeNodeElement`*"]
    pub type SvgfeMergeNodeElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEMergeNodeElement" , js_name = in1 ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMergeNodeElement/in1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeMergeNodeElement`*"]
    pub fn in1(this: &SvgfeMergeNodeElement) -> SvgAnimatedString;
}
