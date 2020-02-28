use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGStyleElement , typescript_name = SVGStyleElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgStyleElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement)\n\n*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    pub type SvgStyleElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = xmlspace ) ]
    #[doc = "Getter for the `xmlspace` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/xmlspace)\n\n*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    pub fn xmlspace(this: &SvgStyleElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = xmlspace ) ]
    #[doc = "Setter for the `xmlspace` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/xmlspace)\n\n*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    pub fn set_xmlspace(this: &SvgStyleElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/type)\n\n*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    pub fn type_(this: &SvgStyleElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = type ) ]
    #[doc = "Setter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/type)\n\n*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    pub fn set_type(this: &SvgStyleElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = media ) ]
    #[doc = "Getter for the `media` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/media)\n\n*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    pub fn media(this: &SvgStyleElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = media ) ]
    #[doc = "Setter for the `media` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/media)\n\n*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    pub fn set_media(this: &SvgStyleElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = title ) ]
    #[doc = "Getter for the `title` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/title)\n\n*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    pub fn title(this: &SvgStyleElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = title ) ]
    #[doc = "Setter for the `title` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/title)\n\n*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    pub fn set_title(this: &SvgStyleElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = sheet ) ]
    #[cfg(feature = "StyleSheet")]
    #[doc = "Getter for the `sheet` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/sheet)\n\n*This API requires the following crate features to be activated: `StyleSheet`, `SvgStyleElement`*"]
    pub fn sheet(this: &SvgStyleElement) -> Option<StyleSheet>;
}
