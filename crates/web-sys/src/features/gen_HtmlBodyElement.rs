use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLBodyElement , typescript_name = HTMLBodyElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlBodyElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub type HtmlBodyElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = text ) ]
    #[doc = "Getter for the `text` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/text)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn text(this: &HtmlBodyElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = text ) ]
    #[doc = "Setter for the `text` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/text)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_text(this: &HtmlBodyElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = link ) ]
    #[doc = "Getter for the `link` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/link)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn link(this: &HtmlBodyElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = link ) ]
    #[doc = "Setter for the `link` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/link)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_link(this: &HtmlBodyElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = vLink ) ]
    #[doc = "Getter for the `vLink` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/vLink)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn v_link(this: &HtmlBodyElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = vLink ) ]
    #[doc = "Setter for the `vLink` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/vLink)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_v_link(this: &HtmlBodyElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = aLink ) ]
    #[doc = "Getter for the `aLink` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/aLink)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn a_link(this: &HtmlBodyElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = aLink ) ]
    #[doc = "Setter for the `aLink` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/aLink)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_a_link(this: &HtmlBodyElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = bgColor ) ]
    #[doc = "Getter for the `bgColor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/bgColor)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn bg_color(this: &HtmlBodyElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = bgColor ) ]
    #[doc = "Setter for the `bgColor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/bgColor)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_bg_color(this: &HtmlBodyElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = background ) ]
    #[doc = "Getter for the `background` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/background)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn background(this: &HtmlBodyElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = background ) ]
    #[doc = "Setter for the `background` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/background)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_background(this: &HtmlBodyElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onafterprint ) ]
    #[doc = "Getter for the `onafterprint` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onafterprint)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onafterprint(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onafterprint ) ]
    #[doc = "Setter for the `onafterprint` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onafterprint)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onafterprint(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onbeforeprint ) ]
    #[doc = "Getter for the `onbeforeprint` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onbeforeprint)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onbeforeprint(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onbeforeprint ) ]
    #[doc = "Setter for the `onbeforeprint` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onbeforeprint)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onbeforeprint(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onbeforeunload ) ]
    #[doc = "Getter for the `onbeforeunload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onbeforeunload)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onbeforeunload(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onbeforeunload ) ]
    #[doc = "Setter for the `onbeforeunload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onbeforeunload)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onbeforeunload(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onhashchange ) ]
    #[doc = "Getter for the `onhashchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onhashchange)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onhashchange(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onhashchange ) ]
    #[doc = "Setter for the `onhashchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onhashchange)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onhashchange(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onlanguagechange ) ]
    #[doc = "Getter for the `onlanguagechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onlanguagechange)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onlanguagechange(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onlanguagechange ) ]
    #[doc = "Setter for the `onlanguagechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onlanguagechange)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onlanguagechange(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onmessage ) ]
    #[doc = "Getter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onmessage)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onmessage(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onmessage ) ]
    #[doc = "Setter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onmessage)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onmessage(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onmessageerror ) ]
    #[doc = "Getter for the `onmessageerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onmessageerror)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onmessageerror(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onmessageerror ) ]
    #[doc = "Setter for the `onmessageerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onmessageerror)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onmessageerror(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onoffline ) ]
    #[doc = "Getter for the `onoffline` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onoffline)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onoffline(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onoffline ) ]
    #[doc = "Setter for the `onoffline` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onoffline)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onoffline(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = ononline ) ]
    #[doc = "Getter for the `ononline` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/ononline)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn ononline(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = ononline ) ]
    #[doc = "Setter for the `ononline` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/ononline)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_ononline(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onpagehide ) ]
    #[doc = "Getter for the `onpagehide` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpagehide)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onpagehide(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onpagehide ) ]
    #[doc = "Setter for the `onpagehide` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpagehide)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onpagehide(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onpageshow ) ]
    #[doc = "Getter for the `onpageshow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpageshow)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onpageshow(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onpageshow ) ]
    #[doc = "Setter for the `onpageshow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpageshow)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onpageshow(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onpopstate ) ]
    #[doc = "Getter for the `onpopstate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpopstate)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onpopstate(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onpopstate ) ]
    #[doc = "Setter for the `onpopstate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpopstate)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onpopstate(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onstorage ) ]
    #[doc = "Getter for the `onstorage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onstorage)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onstorage(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onstorage ) ]
    #[doc = "Setter for the `onstorage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onstorage)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onstorage(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onunload ) ]
    #[doc = "Getter for the `onunload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onunload)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onunload(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onunload ) ]
    #[doc = "Setter for the `onunload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onunload)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onunload(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
}
