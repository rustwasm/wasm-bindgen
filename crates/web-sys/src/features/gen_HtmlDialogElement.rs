use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLDialogElement , typescript_name = HTMLDialogElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlDialogElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement)\n\n*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    pub type HtmlDialogElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = open ) ]
    #[doc = "Getter for the `open` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/open)\n\n*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    pub fn open(this: &HtmlDialogElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = open ) ]
    #[doc = "Setter for the `open` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/open)\n\n*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    pub fn set_open(this: &HtmlDialogElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = returnValue ) ]
    #[doc = "Getter for the `returnValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/returnValue)\n\n*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    pub fn return_value(this: &HtmlDialogElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = returnValue ) ]
    #[doc = "Setter for the `returnValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/returnValue)\n\n*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    pub fn set_return_value(this: &HtmlDialogElement, value: &str);
    # [ wasm_bindgen ( method , structural , js_name = close ) ]
    #[doc = "The `close()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/close)\n\n*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    pub fn close(this: &HtmlDialogElement);
    # [ wasm_bindgen ( method , structural , js_name = close ) ]
    #[doc = "The `close()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/close)\n\n*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    pub fn close_with_return_value(this: &HtmlDialogElement, return_value: &str);
    # [ wasm_bindgen ( method , structural , js_name = show ) ]
    #[doc = "The `show()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/show)\n\n*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    pub fn show(this: &HtmlDialogElement);
    # [ wasm_bindgen ( catch , method , structural , js_name = showModal ) ]
    #[doc = "The `showModal()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/showModal)\n\n*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    pub fn show_modal(this: &HtmlDialogElement) -> Result<(), JsValue>;
}
