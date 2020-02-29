use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLBodyElement , typescript_name = HTMLBodyElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlBodyElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub type HtmlBodyElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = text ) ]
    ///Getter for the `text` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/text)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn text(this: &HtmlBodyElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = text ) ]
    ///Setter for the `text` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/text)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn set_text(this: &HtmlBodyElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = link ) ]
    ///Getter for the `link` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/link)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn link(this: &HtmlBodyElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = link ) ]
    ///Setter for the `link` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/link)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn set_link(this: &HtmlBodyElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = vLink ) ]
    ///Getter for the `vLink` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/vLink)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn v_link(this: &HtmlBodyElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = vLink ) ]
    ///Setter for the `vLink` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/vLink)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn set_v_link(this: &HtmlBodyElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = aLink ) ]
    ///Getter for the `aLink` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/aLink)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn a_link(this: &HtmlBodyElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = aLink ) ]
    ///Setter for the `aLink` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/aLink)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn set_a_link(this: &HtmlBodyElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = bgColor ) ]
    ///Getter for the `bgColor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/bgColor)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn bg_color(this: &HtmlBodyElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = bgColor ) ]
    ///Setter for the `bgColor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/bgColor)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn set_bg_color(this: &HtmlBodyElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = background ) ]
    ///Getter for the `background` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/background)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn background(this: &HtmlBodyElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = background ) ]
    ///Setter for the `background` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/background)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn set_background(this: &HtmlBodyElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onafterprint ) ]
    ///Getter for the `onafterprint` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onafterprint)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn onafterprint(this: &HtmlBodyElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onafterprint ) ]
    ///Setter for the `onafterprint` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onafterprint)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn set_onafterprint(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onbeforeprint ) ]
    ///Getter for the `onbeforeprint` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onbeforeprint)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn onbeforeprint(this: &HtmlBodyElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onbeforeprint ) ]
    ///Setter for the `onbeforeprint` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onbeforeprint)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn set_onbeforeprint(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onbeforeunload ) ]
    ///Getter for the `onbeforeunload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onbeforeunload)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn onbeforeunload(this: &HtmlBodyElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onbeforeunload ) ]
    ///Setter for the `onbeforeunload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onbeforeunload)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn set_onbeforeunload(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onhashchange ) ]
    ///Getter for the `onhashchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onhashchange)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn onhashchange(this: &HtmlBodyElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onhashchange ) ]
    ///Setter for the `onhashchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onhashchange)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn set_onhashchange(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onlanguagechange ) ]
    ///Getter for the `onlanguagechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onlanguagechange)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn onlanguagechange(this: &HtmlBodyElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onlanguagechange ) ]
    ///Setter for the `onlanguagechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onlanguagechange)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn set_onlanguagechange(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onmessage ) ]
    ///Getter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn onmessage(this: &HtmlBodyElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onmessage ) ]
    ///Setter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn set_onmessage(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onmessageerror ) ]
    ///Getter for the `onmessageerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onmessageerror)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn onmessageerror(this: &HtmlBodyElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onmessageerror ) ]
    ///Setter for the `onmessageerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onmessageerror)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn set_onmessageerror(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onoffline ) ]
    ///Getter for the `onoffline` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onoffline)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn onoffline(this: &HtmlBodyElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onoffline ) ]
    ///Setter for the `onoffline` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onoffline)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn set_onoffline(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = ononline ) ]
    ///Getter for the `ononline` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/ononline)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn ononline(this: &HtmlBodyElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = ononline ) ]
    ///Setter for the `ononline` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/ononline)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn set_ononline(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onpagehide ) ]
    ///Getter for the `onpagehide` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpagehide)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn onpagehide(this: &HtmlBodyElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onpagehide ) ]
    ///Setter for the `onpagehide` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpagehide)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn set_onpagehide(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onpageshow ) ]
    ///Getter for the `onpageshow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpageshow)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn onpageshow(this: &HtmlBodyElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onpageshow ) ]
    ///Setter for the `onpageshow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpageshow)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn set_onpageshow(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onpopstate ) ]
    ///Getter for the `onpopstate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpopstate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn onpopstate(this: &HtmlBodyElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onpopstate ) ]
    ///Setter for the `onpopstate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpopstate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn set_onpopstate(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onstorage ) ]
    ///Getter for the `onstorage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onstorage)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn onstorage(this: &HtmlBodyElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onstorage ) ]
    ///Setter for the `onstorage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onstorage)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn set_onstorage(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBodyElement" , js_name = onunload ) ]
    ///Getter for the `onunload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onunload)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn onunload(this: &HtmlBodyElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBodyElement" , js_name = onunload ) ]
    ///Setter for the `onunload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onunload)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBodyElement`*
    pub fn set_onunload(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);

}
