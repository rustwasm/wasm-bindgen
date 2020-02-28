use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLIFrameElement , typescript_name = HTMLIFrameElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlIFrameElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub type HtmlIFrameElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = src ) ]
    #[doc = "Getter for the `src` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/src)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn src(this: &HtmlIFrameElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = src ) ]
    #[doc = "Setter for the `src` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/src)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_src(this: &HtmlIFrameElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = srcdoc ) ]
    #[doc = "Getter for the `srcdoc` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/srcdoc)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn srcdoc(this: &HtmlIFrameElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = srcdoc ) ]
    #[doc = "Setter for the `srcdoc` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/srcdoc)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_srcdoc(this: &HtmlIFrameElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/name)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn name(this: &HtmlIFrameElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = name ) ]
    #[doc = "Setter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/name)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_name(this: &HtmlIFrameElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = sandbox ) ]
    #[cfg(feature = "DomTokenList")]
    #[doc = "Getter for the `sandbox` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/sandbox)\n\n*This API requires the following crate features to be activated: `DomTokenList`, `HtmlIFrameElement`*"]
    pub fn sandbox(this: &HtmlIFrameElement) -> DomTokenList;
    # [ wasm_bindgen ( structural , method , getter , js_name = allowFullscreen ) ]
    #[doc = "Getter for the `allowFullscreen` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowFullscreen)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn allow_fullscreen(this: &HtmlIFrameElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = allowFullscreen ) ]
    #[doc = "Setter for the `allowFullscreen` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowFullscreen)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_allow_fullscreen(this: &HtmlIFrameElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = allowPaymentRequest ) ]
    #[doc = "Getter for the `allowPaymentRequest` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowPaymentRequest)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn allow_payment_request(this: &HtmlIFrameElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = allowPaymentRequest ) ]
    #[doc = "Setter for the `allowPaymentRequest` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowPaymentRequest)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_allow_payment_request(this: &HtmlIFrameElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = width ) ]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/width)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn width(this: &HtmlIFrameElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = width ) ]
    #[doc = "Setter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/width)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_width(this: &HtmlIFrameElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = height ) ]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/height)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn height(this: &HtmlIFrameElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = height ) ]
    #[doc = "Setter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/height)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_height(this: &HtmlIFrameElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = referrerPolicy ) ]
    #[doc = "Getter for the `referrerPolicy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn referrer_policy(this: &HtmlIFrameElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = referrerPolicy ) ]
    #[doc = "Setter for the `referrerPolicy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_referrer_policy(this: &HtmlIFrameElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = contentDocument ) ]
    #[cfg(feature = "Document")]
    #[doc = "Getter for the `contentDocument` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/contentDocument)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlIFrameElement`*"]
    pub fn content_document(this: &HtmlIFrameElement) -> Option<Document>;
    # [ wasm_bindgen ( structural , method , getter , js_name = contentWindow ) ]
    #[cfg(feature = "Window")]
    #[doc = "Getter for the `contentWindow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/contentWindow)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`, `Window`*"]
    pub fn content_window(this: &HtmlIFrameElement) -> Option<Window>;
    # [ wasm_bindgen ( structural , method , getter , js_name = align ) ]
    #[doc = "Getter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/align)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn align(this: &HtmlIFrameElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = align ) ]
    #[doc = "Setter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/align)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_align(this: &HtmlIFrameElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = scrolling ) ]
    #[doc = "Getter for the `scrolling` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/scrolling)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn scrolling(this: &HtmlIFrameElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = scrolling ) ]
    #[doc = "Setter for the `scrolling` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/scrolling)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_scrolling(this: &HtmlIFrameElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = frameBorder ) ]
    #[doc = "Getter for the `frameBorder` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/frameBorder)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn frame_border(this: &HtmlIFrameElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = frameBorder ) ]
    #[doc = "Setter for the `frameBorder` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/frameBorder)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_frame_border(this: &HtmlIFrameElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = longDesc ) ]
    #[doc = "Getter for the `longDesc` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/longDesc)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn long_desc(this: &HtmlIFrameElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = longDesc ) ]
    #[doc = "Setter for the `longDesc` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/longDesc)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_long_desc(this: &HtmlIFrameElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = marginHeight ) ]
    #[doc = "Getter for the `marginHeight` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginHeight)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn margin_height(this: &HtmlIFrameElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = marginHeight ) ]
    #[doc = "Setter for the `marginHeight` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginHeight)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_margin_height(this: &HtmlIFrameElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = marginWidth ) ]
    #[doc = "Getter for the `marginWidth` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginWidth)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn margin_width(this: &HtmlIFrameElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = marginWidth ) ]
    #[doc = "Setter for the `marginWidth` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginWidth)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_margin_width(this: &HtmlIFrameElement, value: String);
    #[cfg(feature = "Document")]
    # [ wasm_bindgen ( method , structural , js_name = getSVGDocument ) ]
    #[doc = "The `getSVGDocument()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/getSVGDocument)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlIFrameElement`*"]
    pub fn get_svg_document(this: &HtmlIFrameElement) -> Option<Document>;
}
