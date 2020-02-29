use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGAElement , typescript_type = "SVGAElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgaElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgaElement`*
    pub type SvgaElement;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAElement" , js_name = target ) ]
    ///Getter for the `target` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/target)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgaElement`*
    pub fn target(this: &SvgaElement) -> SvgAnimatedString;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAElement" , js_name = download ) ]
    ///Getter for the `download` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/download)
    ///
    ///*This API requires the following crate features to be activated: `SvgaElement`*
    pub fn download(this: &SvgaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGAElement" , js_name = download ) ]
    ///Setter for the `download` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/download)
    ///
    ///*This API requires the following crate features to be activated: `SvgaElement`*
    pub fn set_download(this: &SvgaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAElement" , js_name = ping ) ]
    ///Getter for the `ping` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/ping)
    ///
    ///*This API requires the following crate features to be activated: `SvgaElement`*
    pub fn ping(this: &SvgaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGAElement" , js_name = ping ) ]
    ///Setter for the `ping` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/ping)
    ///
    ///*This API requires the following crate features to be activated: `SvgaElement`*
    pub fn set_ping(this: &SvgaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAElement" , js_name = rel ) ]
    ///Getter for the `rel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/rel)
    ///
    ///*This API requires the following crate features to be activated: `SvgaElement`*
    pub fn rel(this: &SvgaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGAElement" , js_name = rel ) ]
    ///Setter for the `rel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/rel)
    ///
    ///*This API requires the following crate features to be activated: `SvgaElement`*
    pub fn set_rel(this: &SvgaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAElement" , js_name = referrerPolicy ) ]
    ///Getter for the `referrerPolicy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/referrerPolicy)
    ///
    ///*This API requires the following crate features to be activated: `SvgaElement`*
    pub fn referrer_policy(this: &SvgaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGAElement" , js_name = referrerPolicy ) ]
    ///Setter for the `referrerPolicy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/referrerPolicy)
    ///
    ///*This API requires the following crate features to be activated: `SvgaElement`*
    pub fn set_referrer_policy(this: &SvgaElement, value: &str);

    #[cfg(feature = "DomTokenList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAElement" , js_name = relList ) ]
    ///Getter for the `relList` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/relList)
    ///
    ///*This API requires the following crate features to be activated: `DomTokenList`, `SvgaElement`*
    pub fn rel_list(this: &SvgaElement) -> DomTokenList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAElement" , js_name = hreflang ) ]
    ///Getter for the `hreflang` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/hreflang)
    ///
    ///*This API requires the following crate features to be activated: `SvgaElement`*
    pub fn hreflang(this: &SvgaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGAElement" , js_name = hreflang ) ]
    ///Setter for the `hreflang` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/hreflang)
    ///
    ///*This API requires the following crate features to be activated: `SvgaElement`*
    pub fn set_hreflang(this: &SvgaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/type)
    ///
    ///*This API requires the following crate features to be activated: `SvgaElement`*
    pub fn type_(this: &SvgaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGAElement" , js_name = type ) ]
    ///Setter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/type)
    ///
    ///*This API requires the following crate features to be activated: `SvgaElement`*
    pub fn set_type(this: &SvgaElement, value: &str);

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "SVGAElement" , js_name = text ) ]
    ///Getter for the `text` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/text)
    ///
    ///*This API requires the following crate features to be activated: `SvgaElement`*
    pub fn text(this: &SvgaElement) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "SVGAElement" , js_name = text ) ]
    ///Setter for the `text` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/text)
    ///
    ///*This API requires the following crate features to be activated: `SvgaElement`*
    pub fn set_text(this: &SvgaElement, value: &str) -> Result<(), JsValue>;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAElement" , js_name = href ) ]
    ///Getter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/href)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgaElement`*
    pub fn href(this: &SvgaElement) -> SvgAnimatedString;

}
