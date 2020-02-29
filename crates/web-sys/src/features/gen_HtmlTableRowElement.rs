use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTableRowElement , typescript_type = "HTMLTableRowElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlTableRowElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableRowElement`*
    pub type HtmlTableRowElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableRowElement" , js_name = rowIndex ) ]
    ///Getter for the `rowIndex` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/rowIndex)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableRowElement`*
    pub fn row_index(this: &HtmlTableRowElement) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableRowElement" , js_name = sectionRowIndex ) ]
    ///Getter for the `sectionRowIndex` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/sectionRowIndex)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableRowElement`*
    pub fn section_row_index(this: &HtmlTableRowElement) -> i32;

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableRowElement" , js_name = cells ) ]
    ///Getter for the `cells` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/cells)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlTableRowElement`*
    pub fn cells(this: &HtmlTableRowElement) -> HtmlCollection;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableRowElement" , js_name = align ) ]
    ///Getter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableRowElement`*
    pub fn align(this: &HtmlTableRowElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableRowElement" , js_name = align ) ]
    ///Setter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableRowElement`*
    pub fn set_align(this: &HtmlTableRowElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableRowElement" , js_name = ch ) ]
    ///Getter for the `ch` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/ch)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableRowElement`*
    pub fn ch(this: &HtmlTableRowElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableRowElement" , js_name = ch ) ]
    ///Setter for the `ch` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/ch)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableRowElement`*
    pub fn set_ch(this: &HtmlTableRowElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableRowElement" , js_name = chOff ) ]
    ///Getter for the `chOff` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/chOff)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableRowElement`*
    pub fn ch_off(this: &HtmlTableRowElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableRowElement" , js_name = chOff ) ]
    ///Setter for the `chOff` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/chOff)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableRowElement`*
    pub fn set_ch_off(this: &HtmlTableRowElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableRowElement" , js_name = vAlign ) ]
    ///Getter for the `vAlign` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/vAlign)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableRowElement`*
    pub fn v_align(this: &HtmlTableRowElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableRowElement" , js_name = vAlign ) ]
    ///Setter for the `vAlign` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/vAlign)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableRowElement`*
    pub fn set_v_align(this: &HtmlTableRowElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableRowElement" , js_name = bgColor ) ]
    ///Getter for the `bgColor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/bgColor)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableRowElement`*
    pub fn bg_color(this: &HtmlTableRowElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableRowElement" , js_name = bgColor ) ]
    ///Setter for the `bgColor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/bgColor)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableRowElement`*
    pub fn set_bg_color(this: &HtmlTableRowElement, value: &str);

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTableRowElement" , js_name = deleteCell ) ]
    ///The `deleteCell()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/deleteCell)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableRowElement`*
    pub fn delete_cell(this: &HtmlTableRowElement, index: i32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTableRowElement" , js_name = insertCell ) ]
    ///The `insertCell()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/insertCell)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableRowElement`*
    pub fn insert_cell(this: &HtmlTableRowElement) -> Result<HtmlElement, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTableRowElement" , js_name = insertCell ) ]
    ///The `insertCell()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/insertCell)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableRowElement`*
    pub fn insert_cell_with_index(
        this: &HtmlTableRowElement,
        index: i32,
    ) -> Result<HtmlElement, JsValue>;

}
