use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTableElement , typescript_type = "HTMLTableElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlTableElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub type HtmlTableElement;

    #[cfg(feature = "HtmlTableCaptionElement")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = caption ) ]
    ///Getter for the `caption` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/caption)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCaptionElement`, `HtmlTableElement`*
    pub fn caption(this: &HtmlTableElement) -> Option<HtmlTableCaptionElement>;

    #[cfg(feature = "HtmlTableCaptionElement")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = caption ) ]
    ///Setter for the `caption` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/caption)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCaptionElement`, `HtmlTableElement`*
    pub fn set_caption(this: &HtmlTableElement, value: Option<&HtmlTableCaptionElement>);

    #[cfg(feature = "HtmlTableSectionElement")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = tHead ) ]
    ///Getter for the `tHead` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tHead)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`, `HtmlTableSectionElement`*
    pub fn t_head(this: &HtmlTableElement) -> Option<HtmlTableSectionElement>;

    #[cfg(feature = "HtmlTableSectionElement")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = tHead ) ]
    ///Setter for the `tHead` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tHead)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`, `HtmlTableSectionElement`*
    pub fn set_t_head(this: &HtmlTableElement, value: Option<&HtmlTableSectionElement>);

    #[cfg(feature = "HtmlTableSectionElement")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = tFoot ) ]
    ///Getter for the `tFoot` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tFoot)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`, `HtmlTableSectionElement`*
    pub fn t_foot(this: &HtmlTableElement) -> Option<HtmlTableSectionElement>;

    #[cfg(feature = "HtmlTableSectionElement")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = tFoot ) ]
    ///Setter for the `tFoot` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tFoot)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`, `HtmlTableSectionElement`*
    pub fn set_t_foot(this: &HtmlTableElement, value: Option<&HtmlTableSectionElement>);

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = tBodies ) ]
    ///Getter for the `tBodies` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tBodies)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlTableElement`*
    pub fn t_bodies(this: &HtmlTableElement) -> HtmlCollection;

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = rows ) ]
    ///Getter for the `rows` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/rows)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlTableElement`*
    pub fn rows(this: &HtmlTableElement) -> HtmlCollection;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = align ) ]
    ///Getter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn align(this: &HtmlTableElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = align ) ]
    ///Setter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn set_align(this: &HtmlTableElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = border ) ]
    ///Getter for the `border` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/border)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn border(this: &HtmlTableElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = border ) ]
    ///Setter for the `border` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/border)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn set_border(this: &HtmlTableElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = frame ) ]
    ///Getter for the `frame` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/frame)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn frame(this: &HtmlTableElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = frame ) ]
    ///Setter for the `frame` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/frame)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn set_frame(this: &HtmlTableElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = rules ) ]
    ///Getter for the `rules` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/rules)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn rules(this: &HtmlTableElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = rules ) ]
    ///Setter for the `rules` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/rules)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn set_rules(this: &HtmlTableElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = summary ) ]
    ///Getter for the `summary` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/summary)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn summary(this: &HtmlTableElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = summary ) ]
    ///Setter for the `summary` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/summary)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn set_summary(this: &HtmlTableElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn width(this: &HtmlTableElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = width ) ]
    ///Setter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn set_width(this: &HtmlTableElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = bgColor ) ]
    ///Getter for the `bgColor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/bgColor)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn bg_color(this: &HtmlTableElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = bgColor ) ]
    ///Setter for the `bgColor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/bgColor)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn set_bg_color(this: &HtmlTableElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = cellPadding ) ]
    ///Getter for the `cellPadding` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellPadding)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn cell_padding(this: &HtmlTableElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = cellPadding ) ]
    ///Setter for the `cellPadding` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellPadding)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn set_cell_padding(this: &HtmlTableElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableElement" , js_name = cellSpacing ) ]
    ///Getter for the `cellSpacing` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellSpacing)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn cell_spacing(this: &HtmlTableElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableElement" , js_name = cellSpacing ) ]
    ///Setter for the `cellSpacing` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellSpacing)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn set_cell_spacing(this: &HtmlTableElement, value: &str);

    # [ wasm_bindgen ( method , structural , js_class = "HTMLTableElement" , js_name = createCaption ) ]
    ///The `createCaption()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createCaption)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn create_caption(this: &HtmlTableElement) -> HtmlElement;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLTableElement" , js_name = createTBody ) ]
    ///The `createTBody()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createTBody)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn create_t_body(this: &HtmlTableElement) -> HtmlElement;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLTableElement" , js_name = createTFoot ) ]
    ///The `createTFoot()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createTFoot)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn create_t_foot(this: &HtmlTableElement) -> HtmlElement;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLTableElement" , js_name = createTHead ) ]
    ///The `createTHead()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createTHead)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn create_t_head(this: &HtmlTableElement) -> HtmlElement;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLTableElement" , js_name = deleteCaption ) ]
    ///The `deleteCaption()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteCaption)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn delete_caption(this: &HtmlTableElement);

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTableElement" , js_name = deleteRow ) ]
    ///The `deleteRow()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteRow)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn delete_row(this: &HtmlTableElement, index: i32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLTableElement" , js_name = deleteTFoot ) ]
    ///The `deleteTFoot()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteTFoot)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn delete_t_foot(this: &HtmlTableElement);

    # [ wasm_bindgen ( method , structural , js_class = "HTMLTableElement" , js_name = deleteTHead ) ]
    ///The `deleteTHead()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteTHead)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn delete_t_head(this: &HtmlTableElement);

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTableElement" , js_name = insertRow ) ]
    ///The `insertRow()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/insertRow)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn insert_row(this: &HtmlTableElement) -> Result<HtmlElement, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTableElement" , js_name = insertRow ) ]
    ///The `insertRow()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/insertRow)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableElement`*
    pub fn insert_row_with_index(
        this: &HtmlTableElement,
        index: i32,
    ) -> Result<HtmlElement, JsValue>;

}
