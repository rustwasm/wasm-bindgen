use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLFormElement , typescript_name = HTMLFormElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlFormElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub type HtmlFormElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = acceptCharset ) ]
    #[doc = "Getter for the `acceptCharset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/acceptCharset)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn accept_charset(this: &HtmlFormElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFormElement" , js_name = acceptCharset ) ]
    #[doc = "Setter for the `acceptCharset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/acceptCharset)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn set_accept_charset(this: &HtmlFormElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = action ) ]
    #[doc = "Getter for the `action` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/action)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn action(this: &HtmlFormElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFormElement" , js_name = action ) ]
    #[doc = "Setter for the `action` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/action)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn set_action(this: &HtmlFormElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = autocomplete ) ]
    #[doc = "Getter for the `autocomplete` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/autocomplete)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn autocomplete(this: &HtmlFormElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFormElement" , js_name = autocomplete ) ]
    #[doc = "Setter for the `autocomplete` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/autocomplete)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn set_autocomplete(this: &HtmlFormElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = enctype ) ]
    #[doc = "Getter for the `enctype` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/enctype)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn enctype(this: &HtmlFormElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFormElement" , js_name = enctype ) ]
    #[doc = "Setter for the `enctype` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/enctype)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn set_enctype(this: &HtmlFormElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = encoding ) ]
    #[doc = "Getter for the `encoding` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/encoding)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn encoding(this: &HtmlFormElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFormElement" , js_name = encoding ) ]
    #[doc = "Setter for the `encoding` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/encoding)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn set_encoding(this: &HtmlFormElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = method ) ]
    #[doc = "Getter for the `method` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/method)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn method(this: &HtmlFormElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFormElement" , js_name = method ) ]
    #[doc = "Setter for the `method` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/method)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn set_method(this: &HtmlFormElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/name)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn name(this: &HtmlFormElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFormElement" , js_name = name ) ]
    #[doc = "Setter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/name)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn set_name(this: &HtmlFormElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = noValidate ) ]
    #[doc = "Getter for the `noValidate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/noValidate)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn no_validate(this: &HtmlFormElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFormElement" , js_name = noValidate ) ]
    #[doc = "Setter for the `noValidate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/noValidate)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn set_no_validate(this: &HtmlFormElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = target ) ]
    #[doc = "Getter for the `target` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/target)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn target(this: &HtmlFormElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFormElement" , js_name = target ) ]
    #[doc = "Setter for the `target` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/target)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn set_target(this: &HtmlFormElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = elements ) ]
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `elements` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/elements)\n\n*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlFormElement`*"]
    pub fn elements(this: &HtmlFormElement) -> HtmlCollection;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/length)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn length(this: &HtmlFormElement) -> i32;
    # [ wasm_bindgen ( method , structural , js_class = "HTMLFormElement" , js_name = checkValidity ) ]
    #[doc = "The `checkValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/checkValidity)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn check_validity(this: &HtmlFormElement) -> bool;
    # [ wasm_bindgen ( method , structural , js_class = "HTMLFormElement" , js_name = reportValidity ) ]
    #[doc = "The `reportValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reportValidity)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn report_validity(this: &HtmlFormElement) -> bool;
    # [ wasm_bindgen ( method , structural , js_class = "HTMLFormElement" , js_name = reset ) ]
    #[doc = "The `reset()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reset)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn reset(this: &HtmlFormElement);
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLFormElement" , js_name = submit ) ]
    #[doc = "The `submit()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/submit)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn submit(this: &HtmlFormElement) -> Result<(), JsValue>;
    #[wasm_bindgen(method, structural, js_class = "HTMLFormElement", indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn get_with_index(this: &HtmlFormElement, index: u32) -> Option<Element>;
    #[wasm_bindgen(method, structural, js_class = "HTMLFormElement", indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn get_with_name(this: &HtmlFormElement, name: &str) -> Option<::js_sys::Object>;
}
