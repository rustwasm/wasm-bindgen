use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLDialogElement , typescript_type = "HTMLDialogElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlDialogElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDialogElement`*
    pub type HtmlDialogElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLDialogElement" , js_name = open ) ]
    ///Getter for the `open` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/open)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDialogElement`*
    pub fn open(this: &HtmlDialogElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLDialogElement" , js_name = open ) ]
    ///Setter for the `open` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/open)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDialogElement`*
    pub fn set_open(this: &HtmlDialogElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLDialogElement" , js_name = returnValue ) ]
    ///Getter for the `returnValue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/returnValue)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDialogElement`*
    pub fn return_value(this: &HtmlDialogElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLDialogElement" , js_name = returnValue ) ]
    ///Setter for the `returnValue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/returnValue)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDialogElement`*
    pub fn set_return_value(this: &HtmlDialogElement, value: &str);

    # [ wasm_bindgen ( method , structural , js_class = "HTMLDialogElement" , js_name = close ) ]
    ///The `close()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/close)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDialogElement`*
    pub fn close(this: &HtmlDialogElement);

    # [ wasm_bindgen ( method , structural , js_class = "HTMLDialogElement" , js_name = close ) ]
    ///The `close()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/close)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDialogElement`*
    pub fn close_with_return_value(this: &HtmlDialogElement, return_value: &str);

    # [ wasm_bindgen ( method , structural , js_class = "HTMLDialogElement" , js_name = show ) ]
    ///The `show()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/show)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDialogElement`*
    pub fn show(this: &HtmlDialogElement);

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDialogElement" , js_name = showModal ) ]
    ///The `showModal()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/showModal)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDialogElement`*
    pub fn show_modal(this: &HtmlDialogElement) -> Result<(), JsValue>;

}
