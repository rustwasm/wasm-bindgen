use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLFormElement , typescript_name = HTMLFormElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlFormElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub type HtmlFormElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = acceptCharset ) ]
    ///Getter for the `acceptCharset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/acceptCharset)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn accept_charset(this: &HtmlFormElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFormElement" , js_name = acceptCharset ) ]
    ///Setter for the `acceptCharset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/acceptCharset)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn set_accept_charset(this: &HtmlFormElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = action ) ]
    ///Getter for the `action` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/action)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn action(this: &HtmlFormElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFormElement" , js_name = action ) ]
    ///Setter for the `action` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/action)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn set_action(this: &HtmlFormElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = autocomplete ) ]
    ///Getter for the `autocomplete` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/autocomplete)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn autocomplete(this: &HtmlFormElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFormElement" , js_name = autocomplete ) ]
    ///Setter for the `autocomplete` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/autocomplete)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn set_autocomplete(this: &HtmlFormElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = enctype ) ]
    ///Getter for the `enctype` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/enctype)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn enctype(this: &HtmlFormElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFormElement" , js_name = enctype ) ]
    ///Setter for the `enctype` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/enctype)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn set_enctype(this: &HtmlFormElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = encoding ) ]
    ///Getter for the `encoding` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/encoding)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn encoding(this: &HtmlFormElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFormElement" , js_name = encoding ) ]
    ///Setter for the `encoding` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/encoding)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn set_encoding(this: &HtmlFormElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = method ) ]
    ///Getter for the `method` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/method)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn method(this: &HtmlFormElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFormElement" , js_name = method ) ]
    ///Setter for the `method` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/method)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn set_method(this: &HtmlFormElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn name(this: &HtmlFormElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFormElement" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn set_name(this: &HtmlFormElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = noValidate ) ]
    ///Getter for the `noValidate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/noValidate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn no_validate(this: &HtmlFormElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFormElement" , js_name = noValidate ) ]
    ///Setter for the `noValidate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/noValidate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn set_no_validate(this: &HtmlFormElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = target ) ]
    ///Getter for the `target` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/target)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn target(this: &HtmlFormElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFormElement" , js_name = target ) ]
    ///Setter for the `target` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/target)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn set_target(this: &HtmlFormElement, value: &str);

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = elements ) ]
    ///Getter for the `elements` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/elements)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlFormElement`*
    pub fn elements(this: &HtmlFormElement) -> HtmlCollection;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFormElement" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/length)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn length(this: &HtmlFormElement) -> i32;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLFormElement" , js_name = checkValidity ) ]
    ///The `checkValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/checkValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn check_validity(this: &HtmlFormElement) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLFormElement" , js_name = reportValidity ) ]
    ///The `reportValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reportValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn report_validity(this: &HtmlFormElement) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLFormElement" , js_name = reset ) ]
    ///The `reset()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reset)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn reset(this: &HtmlFormElement);

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLFormElement" , js_name = submit ) ]
    ///The `submit()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/submit)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn submit(this: &HtmlFormElement) -> Result<(), JsValue>;

    #[wasm_bindgen(method, structural, js_class = "HTMLFormElement", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn get_with_index(this: &HtmlFormElement, index: u32) -> Option<Element>;

    #[wasm_bindgen(method, structural, js_class = "HTMLFormElement", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`*
    pub fn get_with_name(this: &HtmlFormElement, name: &str) -> Option<::js_sys::Object>;

}
