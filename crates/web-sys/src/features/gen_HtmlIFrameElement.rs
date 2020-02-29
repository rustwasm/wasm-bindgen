use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLIFrameElement , typescript_type = "HTMLIFrameElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlIFrameElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub type HtmlIFrameElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLIFrameElement" , js_name = src ) ]
    ///Getter for the `src` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/src)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn src(this: &HtmlIFrameElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLIFrameElement" , js_name = src ) ]
    ///Setter for the `src` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/src)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn set_src(this: &HtmlIFrameElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLIFrameElement" , js_name = srcdoc ) ]
    ///Getter for the `srcdoc` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/srcdoc)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn srcdoc(this: &HtmlIFrameElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLIFrameElement" , js_name = srcdoc ) ]
    ///Setter for the `srcdoc` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/srcdoc)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn set_srcdoc(this: &HtmlIFrameElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLIFrameElement" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn name(this: &HtmlIFrameElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLIFrameElement" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn set_name(this: &HtmlIFrameElement, value: &str);

    #[cfg(feature = "DomTokenList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLIFrameElement" , js_name = sandbox ) ]
    ///Getter for the `sandbox` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/sandbox)
    ///
    ///*This API requires the following crate features to be activated: `DomTokenList`, `HtmlIFrameElement`*
    pub fn sandbox(this: &HtmlIFrameElement) -> DomTokenList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLIFrameElement" , js_name = allowFullscreen ) ]
    ///Getter for the `allowFullscreen` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowFullscreen)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn allow_fullscreen(this: &HtmlIFrameElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLIFrameElement" , js_name = allowFullscreen ) ]
    ///Setter for the `allowFullscreen` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowFullscreen)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn set_allow_fullscreen(this: &HtmlIFrameElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLIFrameElement" , js_name = allowPaymentRequest ) ]
    ///Getter for the `allowPaymentRequest` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowPaymentRequest)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn allow_payment_request(this: &HtmlIFrameElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLIFrameElement" , js_name = allowPaymentRequest ) ]
    ///Setter for the `allowPaymentRequest` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowPaymentRequest)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn set_allow_payment_request(this: &HtmlIFrameElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLIFrameElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn width(this: &HtmlIFrameElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLIFrameElement" , js_name = width ) ]
    ///Setter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn set_width(this: &HtmlIFrameElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLIFrameElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/height)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn height(this: &HtmlIFrameElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLIFrameElement" , js_name = height ) ]
    ///Setter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/height)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn set_height(this: &HtmlIFrameElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLIFrameElement" , js_name = referrerPolicy ) ]
    ///Getter for the `referrerPolicy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/referrerPolicy)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn referrer_policy(this: &HtmlIFrameElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLIFrameElement" , js_name = referrerPolicy ) ]
    ///Setter for the `referrerPolicy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/referrerPolicy)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn set_referrer_policy(this: &HtmlIFrameElement, value: &str);

    #[cfg(feature = "Document")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLIFrameElement" , js_name = contentDocument ) ]
    ///Getter for the `contentDocument` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/contentDocument)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `HtmlIFrameElement`*
    pub fn content_document(this: &HtmlIFrameElement) -> Option<Document>;

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLIFrameElement" , js_name = contentWindow ) ]
    ///Getter for the `contentWindow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/contentWindow)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`, `Window`*
    pub fn content_window(this: &HtmlIFrameElement) -> Option<Window>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLIFrameElement" , js_name = align ) ]
    ///Getter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn align(this: &HtmlIFrameElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLIFrameElement" , js_name = align ) ]
    ///Setter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn set_align(this: &HtmlIFrameElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLIFrameElement" , js_name = scrolling ) ]
    ///Getter for the `scrolling` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/scrolling)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn scrolling(this: &HtmlIFrameElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLIFrameElement" , js_name = scrolling ) ]
    ///Setter for the `scrolling` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/scrolling)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn set_scrolling(this: &HtmlIFrameElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLIFrameElement" , js_name = frameBorder ) ]
    ///Getter for the `frameBorder` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/frameBorder)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn frame_border(this: &HtmlIFrameElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLIFrameElement" , js_name = frameBorder ) ]
    ///Setter for the `frameBorder` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/frameBorder)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn set_frame_border(this: &HtmlIFrameElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLIFrameElement" , js_name = longDesc ) ]
    ///Getter for the `longDesc` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/longDesc)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn long_desc(this: &HtmlIFrameElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLIFrameElement" , js_name = longDesc ) ]
    ///Setter for the `longDesc` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/longDesc)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn set_long_desc(this: &HtmlIFrameElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLIFrameElement" , js_name = marginHeight ) ]
    ///Getter for the `marginHeight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginHeight)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn margin_height(this: &HtmlIFrameElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLIFrameElement" , js_name = marginHeight ) ]
    ///Setter for the `marginHeight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginHeight)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn set_margin_height(this: &HtmlIFrameElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLIFrameElement" , js_name = marginWidth ) ]
    ///Getter for the `marginWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginWidth)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn margin_width(this: &HtmlIFrameElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLIFrameElement" , js_name = marginWidth ) ]
    ///Setter for the `marginWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginWidth)
    ///
    ///*This API requires the following crate features to be activated: `HtmlIFrameElement`*
    pub fn set_margin_width(this: &HtmlIFrameElement, value: &str);

    #[cfg(feature = "Document")]
    # [ wasm_bindgen ( method , structural , js_class = "HTMLIFrameElement" , js_name = getSVGDocument ) ]
    ///The `getSVGDocument()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/getSVGDocument)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `HtmlIFrameElement`*
    pub fn get_svg_document(this: &HtmlIFrameElement) -> Option<Document>;

}
