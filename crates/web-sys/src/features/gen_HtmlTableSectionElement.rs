use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTableSectionElement , typescript_name = HTMLTableSectionElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlTableSectionElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableSectionElement`*
    pub type HtmlTableSectionElement;

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableSectionElement" , js_name = rows ) ]
    ///Getter for the `rows` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/rows)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlTableSectionElement`*
    pub fn rows(this: &HtmlTableSectionElement) -> HtmlCollection;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableSectionElement" , js_name = align ) ]
    ///Getter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableSectionElement`*
    pub fn align(this: &HtmlTableSectionElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableSectionElement" , js_name = align ) ]
    ///Setter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableSectionElement`*
    pub fn set_align(this: &HtmlTableSectionElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableSectionElement" , js_name = ch ) ]
    ///Getter for the `ch` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/ch)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableSectionElement`*
    pub fn ch(this: &HtmlTableSectionElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableSectionElement" , js_name = ch ) ]
    ///Setter for the `ch` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/ch)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableSectionElement`*
    pub fn set_ch(this: &HtmlTableSectionElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableSectionElement" , js_name = chOff ) ]
    ///Getter for the `chOff` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/chOff)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableSectionElement`*
    pub fn ch_off(this: &HtmlTableSectionElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableSectionElement" , js_name = chOff ) ]
    ///Setter for the `chOff` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/chOff)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableSectionElement`*
    pub fn set_ch_off(this: &HtmlTableSectionElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableSectionElement" , js_name = vAlign ) ]
    ///Getter for the `vAlign` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/vAlign)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableSectionElement`*
    pub fn v_align(this: &HtmlTableSectionElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableSectionElement" , js_name = vAlign ) ]
    ///Setter for the `vAlign` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/vAlign)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableSectionElement`*
    pub fn set_v_align(this: &HtmlTableSectionElement, value: &str);

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTableSectionElement" , js_name = deleteRow ) ]
    ///The `deleteRow()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/deleteRow)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableSectionElement`*
    pub fn delete_row(this: &HtmlTableSectionElement, index: i32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTableSectionElement" , js_name = insertRow ) ]
    ///The `insertRow()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/insertRow)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableSectionElement`*
    pub fn insert_row(this: &HtmlTableSectionElement) -> Result<HtmlElement, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTableSectionElement" , js_name = insertRow ) ]
    ///The `insertRow()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/insertRow)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableSectionElement`*
    pub fn insert_row_with_index(
        this: &HtmlTableSectionElement,
        index: i32,
    ) -> Result<HtmlElement, JsValue>;

}
