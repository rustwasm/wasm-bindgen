use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLFrameSetElement , typescript_type = "HTMLFrameSetElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlFrameSetElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub type HtmlFrameSetElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameSetElement" , js_name = cols ) ]
    ///Getter for the `cols` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/cols)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn cols(this: &HtmlFrameSetElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameSetElement" , js_name = cols ) ]
    ///Setter for the `cols` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/cols)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn set_cols(this: &HtmlFrameSetElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameSetElement" , js_name = rows ) ]
    ///Getter for the `rows` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/rows)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn rows(this: &HtmlFrameSetElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameSetElement" , js_name = rows ) ]
    ///Setter for the `rows` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/rows)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn set_rows(this: &HtmlFrameSetElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameSetElement" , js_name = onafterprint ) ]
    ///Getter for the `onafterprint` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onafterprint)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn onafterprint(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameSetElement" , js_name = onafterprint ) ]
    ///Setter for the `onafterprint` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onafterprint)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn set_onafterprint(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameSetElement" , js_name = onbeforeprint ) ]
    ///Getter for the `onbeforeprint` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onbeforeprint)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn onbeforeprint(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameSetElement" , js_name = onbeforeprint ) ]
    ///Setter for the `onbeforeprint` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onbeforeprint)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn set_onbeforeprint(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameSetElement" , js_name = onbeforeunload ) ]
    ///Getter for the `onbeforeunload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onbeforeunload)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn onbeforeunload(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameSetElement" , js_name = onbeforeunload ) ]
    ///Setter for the `onbeforeunload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onbeforeunload)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn set_onbeforeunload(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameSetElement" , js_name = onhashchange ) ]
    ///Getter for the `onhashchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onhashchange)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn onhashchange(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameSetElement" , js_name = onhashchange ) ]
    ///Setter for the `onhashchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onhashchange)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn set_onhashchange(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameSetElement" , js_name = onlanguagechange ) ]
    ///Getter for the `onlanguagechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onlanguagechange)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn onlanguagechange(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameSetElement" , js_name = onlanguagechange ) ]
    ///Setter for the `onlanguagechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onlanguagechange)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn set_onlanguagechange(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameSetElement" , js_name = onmessage ) ]
    ///Getter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn onmessage(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameSetElement" , js_name = onmessage ) ]
    ///Setter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn set_onmessage(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameSetElement" , js_name = onmessageerror ) ]
    ///Getter for the `onmessageerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onmessageerror)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn onmessageerror(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameSetElement" , js_name = onmessageerror ) ]
    ///Setter for the `onmessageerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onmessageerror)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn set_onmessageerror(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameSetElement" , js_name = onoffline ) ]
    ///Getter for the `onoffline` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onoffline)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn onoffline(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameSetElement" , js_name = onoffline ) ]
    ///Setter for the `onoffline` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onoffline)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn set_onoffline(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameSetElement" , js_name = ononline ) ]
    ///Getter for the `ononline` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/ononline)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn ononline(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameSetElement" , js_name = ononline ) ]
    ///Setter for the `ononline` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/ononline)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn set_ononline(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameSetElement" , js_name = onpagehide ) ]
    ///Getter for the `onpagehide` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpagehide)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn onpagehide(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameSetElement" , js_name = onpagehide ) ]
    ///Setter for the `onpagehide` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpagehide)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn set_onpagehide(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameSetElement" , js_name = onpageshow ) ]
    ///Getter for the `onpageshow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpageshow)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn onpageshow(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameSetElement" , js_name = onpageshow ) ]
    ///Setter for the `onpageshow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpageshow)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn set_onpageshow(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameSetElement" , js_name = onpopstate ) ]
    ///Getter for the `onpopstate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpopstate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn onpopstate(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameSetElement" , js_name = onpopstate ) ]
    ///Setter for the `onpopstate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpopstate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn set_onpopstate(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameSetElement" , js_name = onstorage ) ]
    ///Getter for the `onstorage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onstorage)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn onstorage(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameSetElement" , js_name = onstorage ) ]
    ///Setter for the `onstorage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onstorage)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn set_onstorage(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameSetElement" , js_name = onunload ) ]
    ///Getter for the `onunload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onunload)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn onunload(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameSetElement" , js_name = onunload ) ]
    ///Setter for the `onunload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onunload)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameSetElement`*
    pub fn set_onunload(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);

}
