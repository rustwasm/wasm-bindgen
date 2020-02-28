use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTableElement , typescript_name = HTMLTableElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTableElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub type HtmlTableElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = caption ) ]
    #[cfg(feature = "HtmlTableCaptionElement")]
    #[doc = "Getter for the `caption` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/caption)\n\n*This API requires the following crate features to be activated: `HtmlTableCaptionElement`, `HtmlTableElement`*"]
    pub fn caption(this: &HtmlTableElement) -> Option<HtmlTableCaptionElement>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = caption ) ]
    #[cfg(feature = "HtmlTableCaptionElement")]
    #[doc = "Setter for the `caption` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/caption)\n\n*This API requires the following crate features to be activated: `HtmlTableCaptionElement`, `HtmlTableElement`*"]
    pub fn set_caption(this: &HtmlTableElement, value: Option<&HtmlTableCaptionElement>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = tHead ) ]
    #[cfg(feature = "HtmlTableSectionElement")]
    #[doc = "Getter for the `tHead` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tHead)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`, `HtmlTableSectionElement`*"]
    pub fn t_head(this: &HtmlTableElement) -> Option<HtmlTableSectionElement>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = tHead ) ]
    #[cfg(feature = "HtmlTableSectionElement")]
    #[doc = "Setter for the `tHead` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tHead)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`, `HtmlTableSectionElement`*"]
    pub fn set_t_head(this: &HtmlTableElement, value: Option<&HtmlTableSectionElement>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = tFoot ) ]
    #[cfg(feature = "HtmlTableSectionElement")]
    #[doc = "Getter for the `tFoot` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tFoot)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`, `HtmlTableSectionElement`*"]
    pub fn t_foot(this: &HtmlTableElement) -> Option<HtmlTableSectionElement>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = tFoot ) ]
    #[cfg(feature = "HtmlTableSectionElement")]
    #[doc = "Setter for the `tFoot` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tFoot)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`, `HtmlTableSectionElement`*"]
    pub fn set_t_foot(this: &HtmlTableElement, value: Option<&HtmlTableSectionElement>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = tBodies ) ]
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `tBodies` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tBodies)\n\n*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlTableElement`*"]
    pub fn t_bodies(this: &HtmlTableElement) -> HtmlCollection;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = rows ) ]
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `rows` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/rows)\n\n*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlTableElement`*"]
    pub fn rows(this: &HtmlTableElement) -> HtmlCollection;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = align ) ]
    #[doc = "Getter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn align(this: &HtmlTableElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = align ) ]
    #[doc = "Setter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn set_align(this: &HtmlTableElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = border ) ]
    #[doc = "Getter for the `border` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/border)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn border(this: &HtmlTableElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = border ) ]
    #[doc = "Setter for the `border` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/border)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn set_border(this: &HtmlTableElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = frame ) ]
    #[doc = "Getter for the `frame` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/frame)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn frame(this: &HtmlTableElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = frame ) ]
    #[doc = "Setter for the `frame` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/frame)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn set_frame(this: &HtmlTableElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = rules ) ]
    #[doc = "Getter for the `rules` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/rules)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn rules(this: &HtmlTableElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = rules ) ]
    #[doc = "Setter for the `rules` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/rules)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn set_rules(this: &HtmlTableElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = summary ) ]
    #[doc = "Getter for the `summary` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/summary)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn summary(this: &HtmlTableElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = summary ) ]
    #[doc = "Setter for the `summary` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/summary)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn set_summary(this: &HtmlTableElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = width ) ]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/width)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn width(this: &HtmlTableElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = width ) ]
    #[doc = "Setter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/width)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn set_width(this: &HtmlTableElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = bgColor ) ]
    #[doc = "Getter for the `bgColor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/bgColor)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn bg_color(this: &HtmlTableElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = bgColor ) ]
    #[doc = "Setter for the `bgColor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/bgColor)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn set_bg_color(this: &HtmlTableElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = cellPadding ) ]
    #[doc = "Getter for the `cellPadding` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellPadding)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn cell_padding(this: &HtmlTableElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = cellPadding ) ]
    #[doc = "Setter for the `cellPadding` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellPadding)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn set_cell_padding(this: &HtmlTableElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = cellSpacing ) ]
    #[doc = "Getter for the `cellSpacing` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellSpacing)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn cell_spacing(this: &HtmlTableElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = cellSpacing ) ]
    #[doc = "Setter for the `cellSpacing` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellSpacing)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn set_cell_spacing(this: &HtmlTableElement, value: &str);
    #[cfg(feature = "HtmlElement")]
    # [ wasm_bindgen ( method , structural , js_class = "HTMLTableElement" , js_name = createCaption ) ]
    #[doc = "The `createCaption()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createCaption)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlTableElement`*"]
    pub fn create_caption(this: &HtmlTableElement) -> HtmlElement;
    #[cfg(feature = "HtmlElement")]
    # [ wasm_bindgen ( method , structural , js_class = "HTMLTableElement" , js_name = createTBody ) ]
    #[doc = "The `createTBody()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createTBody)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlTableElement`*"]
    pub fn create_t_body(this: &HtmlTableElement) -> HtmlElement;
    #[cfg(feature = "HtmlElement")]
    # [ wasm_bindgen ( method , structural , js_class = "HTMLTableElement" , js_name = createTFoot ) ]
    #[doc = "The `createTFoot()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createTFoot)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlTableElement`*"]
    pub fn create_t_foot(this: &HtmlTableElement) -> HtmlElement;
    #[cfg(feature = "HtmlElement")]
    # [ wasm_bindgen ( method , structural , js_class = "HTMLTableElement" , js_name = createTHead ) ]
    #[doc = "The `createTHead()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createTHead)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlTableElement`*"]
    pub fn create_t_head(this: &HtmlTableElement) -> HtmlElement;
    # [ wasm_bindgen ( method , structural , js_class = "HTMLTableElement" , js_name = deleteCaption ) ]
    #[doc = "The `deleteCaption()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteCaption)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn delete_caption(this: &HtmlTableElement);
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTableElement" , js_name = deleteRow ) ]
    #[doc = "The `deleteRow()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteRow)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn delete_row(this: &HtmlTableElement, index: i32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "HTMLTableElement" , js_name = deleteTFoot ) ]
    #[doc = "The `deleteTFoot()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteTFoot)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn delete_t_foot(this: &HtmlTableElement);
    # [ wasm_bindgen ( method , structural , js_class = "HTMLTableElement" , js_name = deleteTHead ) ]
    #[doc = "The `deleteTHead()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteTHead)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn delete_t_head(this: &HtmlTableElement);
    #[cfg(feature = "HtmlElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTableElement" , js_name = insertRow ) ]
    #[doc = "The `insertRow()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/insertRow)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlTableElement`*"]
    pub fn insert_row(this: &HtmlTableElement) -> Result<HtmlElement, JsValue>;
    #[cfg(feature = "HtmlElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTableElement" , js_name = insertRow ) ]
    #[doc = "The `insertRow()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/insertRow)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlTableElement`*"]
    pub fn insert_row_with_index(
        this: &HtmlTableElement,
        index: i32,
    ) -> Result<HtmlElement, JsValue>;
}
