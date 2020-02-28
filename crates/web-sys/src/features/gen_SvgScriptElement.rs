use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGScriptElement , typescript_name = SVGScriptElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgScriptElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement)\n\n*This API requires the following crate features to be activated: `SvgScriptElement`*"]
    pub type SvgScriptElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/type)\n\n*This API requires the following crate features to be activated: `SvgScriptElement`*"]
    pub fn type_(this: &SvgScriptElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = type ) ]
    #[doc = "Setter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/type)\n\n*This API requires the following crate features to be activated: `SvgScriptElement`*"]
    pub fn set_type(this: &SvgScriptElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = crossOrigin ) ]
    #[doc = "Getter for the `crossOrigin` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/crossOrigin)\n\n*This API requires the following crate features to be activated: `SvgScriptElement`*"]
    pub fn cross_origin(this: &SvgScriptElement) -> Option<String>;
    # [ wasm_bindgen ( structural , method , setter , js_name = crossOrigin ) ]
    #[doc = "Setter for the `crossOrigin` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/crossOrigin)\n\n*This API requires the following crate features to be activated: `SvgScriptElement`*"]
    pub fn set_cross_origin(this: &SvgScriptElement, value: Option<&str>);
    # [ wasm_bindgen ( structural , method , getter , js_name = href ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/href)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgScriptElement`*"]
    pub fn href(this: &SvgScriptElement) -> SvgAnimatedString;
}
