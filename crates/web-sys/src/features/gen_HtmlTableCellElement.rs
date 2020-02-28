use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTableCellElement , typescript_name = HTMLTableCellElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTableCellElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub type HtmlTableCellElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = colSpan ) ]
    #[doc = "Getter for the `colSpan` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/colSpan)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn col_span(this: &HtmlTableCellElement) -> u32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = colSpan ) ]
    #[doc = "Setter for the `colSpan` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/colSpan)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_col_span(this: &HtmlTableCellElement, value: u32);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = rowSpan ) ]
    #[doc = "Getter for the `rowSpan` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/rowSpan)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn row_span(this: &HtmlTableCellElement) -> u32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = rowSpan ) ]
    #[doc = "Setter for the `rowSpan` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/rowSpan)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_row_span(this: &HtmlTableCellElement, value: u32);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = headers ) ]
    #[doc = "Getter for the `headers` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/headers)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn headers(this: &HtmlTableCellElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = headers ) ]
    #[doc = "Setter for the `headers` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/headers)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_headers(this: &HtmlTableCellElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = cellIndex ) ]
    #[doc = "Getter for the `cellIndex` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/cellIndex)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn cell_index(this: &HtmlTableCellElement) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = align ) ]
    #[doc = "Getter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn align(this: &HtmlTableCellElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = align ) ]
    #[doc = "Setter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_align(this: &HtmlTableCellElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = axis ) ]
    #[doc = "Getter for the `axis` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/axis)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn axis(this: &HtmlTableCellElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = axis ) ]
    #[doc = "Setter for the `axis` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/axis)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_axis(this: &HtmlTableCellElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = height ) ]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/height)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn height(this: &HtmlTableCellElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = height ) ]
    #[doc = "Setter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/height)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_height(this: &HtmlTableCellElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = width ) ]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/width)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn width(this: &HtmlTableCellElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = width ) ]
    #[doc = "Setter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/width)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_width(this: &HtmlTableCellElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = ch ) ]
    #[doc = "Getter for the `ch` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/ch)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn ch(this: &HtmlTableCellElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = ch ) ]
    #[doc = "Setter for the `ch` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/ch)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_ch(this: &HtmlTableCellElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = chOff ) ]
    #[doc = "Getter for the `chOff` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/chOff)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn ch_off(this: &HtmlTableCellElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = chOff ) ]
    #[doc = "Setter for the `chOff` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/chOff)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_ch_off(this: &HtmlTableCellElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = noWrap ) ]
    #[doc = "Getter for the `noWrap` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/noWrap)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn no_wrap(this: &HtmlTableCellElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = noWrap ) ]
    #[doc = "Setter for the `noWrap` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/noWrap)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_no_wrap(this: &HtmlTableCellElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = vAlign ) ]
    #[doc = "Getter for the `vAlign` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/vAlign)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn v_align(this: &HtmlTableCellElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = vAlign ) ]
    #[doc = "Setter for the `vAlign` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/vAlign)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_v_align(this: &HtmlTableCellElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCellElement" , js_name = bgColor ) ]
    #[doc = "Getter for the `bgColor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/bgColor)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn bg_color(this: &HtmlTableCellElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCellElement" , js_name = bgColor ) ]
    #[doc = "Setter for the `bgColor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/bgColor)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_bg_color(this: &HtmlTableCellElement, value: &str);
}
