use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTableRowElement , typescript_name = HTMLTableRowElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTableRowElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub type HtmlTableRowElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableRowElement" , js_name = rowIndex ) ]
    #[doc = "Getter for the `rowIndex` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/rowIndex)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn row_index(this: &HtmlTableRowElement) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableRowElement" , js_name = sectionRowIndex ) ]
    #[doc = "Getter for the `sectionRowIndex` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/sectionRowIndex)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn section_row_index(this: &HtmlTableRowElement) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableRowElement" , js_name = cells ) ]
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `cells` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/cells)\n\n*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlTableRowElement`*"]
    pub fn cells(this: &HtmlTableRowElement) -> HtmlCollection;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableRowElement" , js_name = align ) ]
    #[doc = "Getter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn align(this: &HtmlTableRowElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableRowElement" , js_name = align ) ]
    #[doc = "Setter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn set_align(this: &HtmlTableRowElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableRowElement" , js_name = ch ) ]
    #[doc = "Getter for the `ch` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/ch)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn ch(this: &HtmlTableRowElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableRowElement" , js_name = ch ) ]
    #[doc = "Setter for the `ch` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/ch)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn set_ch(this: &HtmlTableRowElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableRowElement" , js_name = chOff ) ]
    #[doc = "Getter for the `chOff` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/chOff)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn ch_off(this: &HtmlTableRowElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableRowElement" , js_name = chOff ) ]
    #[doc = "Setter for the `chOff` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/chOff)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn set_ch_off(this: &HtmlTableRowElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableRowElement" , js_name = vAlign ) ]
    #[doc = "Getter for the `vAlign` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/vAlign)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn v_align(this: &HtmlTableRowElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableRowElement" , js_name = vAlign ) ]
    #[doc = "Setter for the `vAlign` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/vAlign)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn set_v_align(this: &HtmlTableRowElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableRowElement" , js_name = bgColor ) ]
    #[doc = "Getter for the `bgColor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/bgColor)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn bg_color(this: &HtmlTableRowElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableRowElement" , js_name = bgColor ) ]
    #[doc = "Setter for the `bgColor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/bgColor)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn set_bg_color(this: &HtmlTableRowElement, value: &str);
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTableRowElement" , js_name = deleteCell ) ]
    #[doc = "The `deleteCell()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/deleteCell)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn delete_cell(this: &HtmlTableRowElement, index: i32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTableRowElement" , js_name = insertCell ) ]
    #[doc = "The `insertCell()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/insertCell)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn insert_cell(this: &HtmlTableRowElement) -> Result<HtmlElement, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTableRowElement" , js_name = insertCell ) ]
    #[doc = "The `insertCell()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/insertCell)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn insert_cell_with_index(
        this: &HtmlTableRowElement,
        index: i32,
    ) -> Result<HtmlElement, JsValue>;
}
