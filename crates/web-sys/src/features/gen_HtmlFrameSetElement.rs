use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLFrameSetElement , typescript_name = HTMLFrameSetElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlFrameSetElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub type HtmlFrameSetElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = cols ) ]
    #[doc = "Getter for the `cols` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/cols)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn cols(this: &HtmlFrameSetElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = cols ) ]
    #[doc = "Setter for the `cols` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/cols)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_cols(this: &HtmlFrameSetElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = rows ) ]
    #[doc = "Getter for the `rows` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/rows)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn rows(this: &HtmlFrameSetElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = rows ) ]
    #[doc = "Setter for the `rows` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/rows)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_rows(this: &HtmlFrameSetElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = onafterprint ) ]
    #[doc = "Getter for the `onafterprint` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onafterprint)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onafterprint(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onafterprint ) ]
    #[doc = "Setter for the `onafterprint` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onafterprint)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onafterprint(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onbeforeprint ) ]
    #[doc = "Getter for the `onbeforeprint` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onbeforeprint)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onbeforeprint(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onbeforeprint ) ]
    #[doc = "Setter for the `onbeforeprint` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onbeforeprint)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onbeforeprint(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onbeforeunload ) ]
    #[doc = "Getter for the `onbeforeunload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onbeforeunload)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onbeforeunload(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onbeforeunload ) ]
    #[doc = "Setter for the `onbeforeunload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onbeforeunload)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onbeforeunload(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onhashchange ) ]
    #[doc = "Getter for the `onhashchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onhashchange)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onhashchange(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onhashchange ) ]
    #[doc = "Setter for the `onhashchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onhashchange)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onhashchange(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onlanguagechange ) ]
    #[doc = "Getter for the `onlanguagechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onlanguagechange)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onlanguagechange(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onlanguagechange ) ]
    #[doc = "Setter for the `onlanguagechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onlanguagechange)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onlanguagechange(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmessage ) ]
    #[doc = "Getter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onmessage)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onmessage(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmessage ) ]
    #[doc = "Setter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onmessage)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onmessage(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmessageerror ) ]
    #[doc = "Getter for the `onmessageerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onmessageerror)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onmessageerror(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmessageerror ) ]
    #[doc = "Setter for the `onmessageerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onmessageerror)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onmessageerror(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onoffline ) ]
    #[doc = "Getter for the `onoffline` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onoffline)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onoffline(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onoffline ) ]
    #[doc = "Setter for the `onoffline` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onoffline)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onoffline(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ononline ) ]
    #[doc = "Getter for the `ononline` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/ononline)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn ononline(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ononline ) ]
    #[doc = "Setter for the `ononline` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/ononline)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_ononline(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpagehide ) ]
    #[doc = "Getter for the `onpagehide` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpagehide)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onpagehide(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpagehide ) ]
    #[doc = "Setter for the `onpagehide` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpagehide)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onpagehide(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpageshow ) ]
    #[doc = "Getter for the `onpageshow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpageshow)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onpageshow(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpageshow ) ]
    #[doc = "Setter for the `onpageshow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpageshow)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onpageshow(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpopstate ) ]
    #[doc = "Getter for the `onpopstate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpopstate)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onpopstate(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpopstate ) ]
    #[doc = "Setter for the `onpopstate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpopstate)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onpopstate(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onstorage ) ]
    #[doc = "Getter for the `onstorage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onstorage)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onstorage(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onstorage ) ]
    #[doc = "Setter for the `onstorage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onstorage)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onstorage(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onunload ) ]
    #[doc = "Getter for the `onunload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onunload)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onunload(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onunload ) ]
    #[doc = "Setter for the `onunload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onunload)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onunload(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
}
