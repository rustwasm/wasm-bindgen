use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTableCellElement , typescript_name = HTMLTableCellElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlTableCellElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub type HtmlTableCellElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = colSpan ) ]
    ///Getter for the `colSpan` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/colSpan)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn col_span(this: &HtmlTableCellElement) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = colSpan ) ]
    ///Setter for the `colSpan` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/colSpan)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn set_col_span(this: &HtmlTableCellElement, value: u32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = rowSpan ) ]
    ///Getter for the `rowSpan` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/rowSpan)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn row_span(this: &HtmlTableCellElement) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = rowSpan ) ]
    ///Setter for the `rowSpan` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/rowSpan)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn set_row_span(this: &HtmlTableCellElement, value: u32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = headers ) ]
    ///Getter for the `headers` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/headers)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn headers(this: &HtmlTableCellElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = headers ) ]
    ///Setter for the `headers` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/headers)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn set_headers(this: &HtmlTableCellElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = cellIndex ) ]
    ///Getter for the `cellIndex` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/cellIndex)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn cell_index(this: &HtmlTableCellElement) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = align ) ]
    ///Getter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn align(this: &HtmlTableCellElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = align ) ]
    ///Setter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn set_align(this: &HtmlTableCellElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = axis ) ]
    ///Getter for the `axis` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/axis)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn axis(this: &HtmlTableCellElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = axis ) ]
    ///Setter for the `axis` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/axis)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn set_axis(this: &HtmlTableCellElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/height)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn height(this: &HtmlTableCellElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = height ) ]
    ///Setter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/height)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn set_height(this: &HtmlTableCellElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn width(this: &HtmlTableCellElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = width ) ]
    ///Setter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn set_width(this: &HtmlTableCellElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = ch ) ]
    ///Getter for the `ch` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/ch)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn ch(this: &HtmlTableCellElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = ch ) ]
    ///Setter for the `ch` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/ch)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn set_ch(this: &HtmlTableCellElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = chOff ) ]
    ///Getter for the `chOff` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/chOff)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn ch_off(this: &HtmlTableCellElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = chOff ) ]
    ///Setter for the `chOff` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/chOff)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn set_ch_off(this: &HtmlTableCellElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = noWrap ) ]
    ///Getter for the `noWrap` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/noWrap)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn no_wrap(this: &HtmlTableCellElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = noWrap ) ]
    ///Setter for the `noWrap` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/noWrap)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn set_no_wrap(this: &HtmlTableCellElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = vAlign ) ]
    ///Getter for the `vAlign` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/vAlign)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn v_align(this: &HtmlTableCellElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = vAlign ) ]
    ///Setter for the `vAlign` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/vAlign)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn set_v_align(this: &HtmlTableCellElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = bgColor ) ]
    ///Getter for the `bgColor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/bgColor)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn bg_color(this: &HtmlTableCellElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = bgColor ) ]
    ///Setter for the `bgColor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/bgColor)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCellElement`*
    pub fn set_bg_color(this: &HtmlTableCellElement, value: &str);

}
