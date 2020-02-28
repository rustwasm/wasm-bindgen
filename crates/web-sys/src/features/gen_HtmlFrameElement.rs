use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLFrameElement , typescript_name = HTMLFrameElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlFrameElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub type HtmlFrameElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/name)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn name(this: &HtmlFrameElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = name ) ]
    #[doc = "Setter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/name)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn set_name(this: &HtmlFrameElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = scrolling ) ]
    #[doc = "Getter for the `scrolling` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/scrolling)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn scrolling(this: &HtmlFrameElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = scrolling ) ]
    #[doc = "Setter for the `scrolling` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/scrolling)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn set_scrolling(this: &HtmlFrameElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = src ) ]
    #[doc = "Getter for the `src` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/src)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn src(this: &HtmlFrameElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = src ) ]
    #[doc = "Setter for the `src` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/src)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn set_src(this: &HtmlFrameElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = frameBorder ) ]
    #[doc = "Getter for the `frameBorder` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/frameBorder)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn frame_border(this: &HtmlFrameElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = frameBorder ) ]
    #[doc = "Setter for the `frameBorder` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/frameBorder)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn set_frame_border(this: &HtmlFrameElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = longDesc ) ]
    #[doc = "Getter for the `longDesc` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/longDesc)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn long_desc(this: &HtmlFrameElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = longDesc ) ]
    #[doc = "Setter for the `longDesc` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/longDesc)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn set_long_desc(this: &HtmlFrameElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = noResize ) ]
    #[doc = "Getter for the `noResize` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/noResize)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn no_resize(this: &HtmlFrameElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = noResize ) ]
    #[doc = "Setter for the `noResize` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/noResize)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn set_no_resize(this: &HtmlFrameElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = contentDocument ) ]
    #[cfg(feature = "Document")]
    #[doc = "Getter for the `contentDocument` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/contentDocument)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlFrameElement`*"]
    pub fn content_document(this: &HtmlFrameElement) -> Option<Document>;
    # [ wasm_bindgen ( structural , method , getter , js_name = contentWindow ) ]
    #[cfg(feature = "Window")]
    #[doc = "Getter for the `contentWindow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/contentWindow)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`, `Window`*"]
    pub fn content_window(this: &HtmlFrameElement) -> Option<Window>;
    # [ wasm_bindgen ( structural , method , getter , js_name = marginHeight ) ]
    #[doc = "Getter for the `marginHeight` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/marginHeight)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn margin_height(this: &HtmlFrameElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = marginHeight ) ]
    #[doc = "Setter for the `marginHeight` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/marginHeight)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn set_margin_height(this: &HtmlFrameElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = marginWidth ) ]
    #[doc = "Getter for the `marginWidth` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/marginWidth)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn margin_width(this: &HtmlFrameElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = marginWidth ) ]
    #[doc = "Setter for the `marginWidth` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/marginWidth)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn set_margin_width(this: &HtmlFrameElement, value: &str);
}
