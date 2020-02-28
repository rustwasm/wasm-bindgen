use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTableSectionElement , typescript_name = HTMLTableSectionElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTableSectionElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement)\n\n*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub type HtmlTableSectionElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = rows ) ]
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `rows` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/rows)\n\n*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlTableSectionElement`*"]
    pub fn rows(this: &HtmlTableSectionElement) -> HtmlCollection;
    # [ wasm_bindgen ( structural , method , getter , js_name = align ) ]
    #[doc = "Getter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub fn align(this: &HtmlTableSectionElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = align ) ]
    #[doc = "Setter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub fn set_align(this: &HtmlTableSectionElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = ch ) ]
    #[doc = "Getter for the `ch` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/ch)\n\n*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub fn ch(this: &HtmlTableSectionElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = ch ) ]
    #[doc = "Setter for the `ch` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/ch)\n\n*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub fn set_ch(this: &HtmlTableSectionElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = chOff ) ]
    #[doc = "Getter for the `chOff` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/chOff)\n\n*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub fn ch_off(this: &HtmlTableSectionElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = chOff ) ]
    #[doc = "Setter for the `chOff` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/chOff)\n\n*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub fn set_ch_off(this: &HtmlTableSectionElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = vAlign ) ]
    #[doc = "Getter for the `vAlign` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/vAlign)\n\n*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub fn v_align(this: &HtmlTableSectionElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = vAlign ) ]
    #[doc = "Setter for the `vAlign` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/vAlign)\n\n*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub fn set_v_align(this: &HtmlTableSectionElement, value: &str);
    # [ wasm_bindgen ( catch , method , structural , js_name = deleteRow ) ]
    #[doc = "The `deleteRow()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/deleteRow)\n\n*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub fn delete_row(this: &HtmlTableSectionElement, index: i32) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlElement")]
    # [ wasm_bindgen ( catch , method , structural , js_name = insertRow ) ]
    #[doc = "The `insertRow()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/insertRow)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlTableSectionElement`*"]
    pub fn insert_row(this: &HtmlTableSectionElement) -> Result<HtmlElement, JsValue>;
    #[cfg(feature = "HtmlElement")]
    # [ wasm_bindgen ( catch , method , structural , js_name = insertRow ) ]
    #[doc = "The `insertRow()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/insertRow)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlTableSectionElement`*"]
    pub fn insert_row_with_index(
        this: &HtmlTableSectionElement,
        index: i32,
    ) -> Result<HtmlElement, JsValue>;
}
