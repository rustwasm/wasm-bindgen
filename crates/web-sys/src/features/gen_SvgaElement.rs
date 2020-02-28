use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGAElement , typescript_name = SVGAElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgaElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub type SvgaElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = target ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `target` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/target)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgaElement`*"]
    pub fn target(this: &SvgaElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( structural , method , getter , js_name = download ) ]
    #[doc = "Getter for the `download` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/download)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn download(this: &SvgaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = download ) ]
    #[doc = "Setter for the `download` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/download)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn set_download(this: &SvgaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = ping ) ]
    #[doc = "Getter for the `ping` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/ping)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn ping(this: &SvgaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = ping ) ]
    #[doc = "Setter for the `ping` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/ping)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn set_ping(this: &SvgaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = rel ) ]
    #[doc = "Getter for the `rel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/rel)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn rel(this: &SvgaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = rel ) ]
    #[doc = "Setter for the `rel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/rel)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn set_rel(this: &SvgaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = referrerPolicy ) ]
    #[doc = "Getter for the `referrerPolicy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn referrer_policy(this: &SvgaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = referrerPolicy ) ]
    #[doc = "Setter for the `referrerPolicy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn set_referrer_policy(this: &SvgaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = relList ) ]
    #[cfg(feature = "DomTokenList")]
    #[doc = "Getter for the `relList` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/relList)\n\n*This API requires the following crate features to be activated: `DomTokenList`, `SvgaElement`*"]
    pub fn rel_list(this: &SvgaElement) -> DomTokenList;
    # [ wasm_bindgen ( structural , method , getter , js_name = hreflang ) ]
    #[doc = "Getter for the `hreflang` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/hreflang)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn hreflang(this: &SvgaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = hreflang ) ]
    #[doc = "Setter for the `hreflang` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/hreflang)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn set_hreflang(this: &SvgaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/type)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn type_(this: &SvgaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = type ) ]
    #[doc = "Setter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/type)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn set_type(this: &SvgaElement, value: &str);
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = text ) ]
    #[doc = "Getter for the `text` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/text)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn text(this: &SvgaElement) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = text ) ]
    #[doc = "Setter for the `text` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/text)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn set_text(this: &SvgaElement, value: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = href ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/href)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgaElement`*"]
    pub fn href(this: &SvgaElement) -> SvgAnimatedString;
}
