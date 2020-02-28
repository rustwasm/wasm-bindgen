use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = TreeBoxObject , typescript_name = TreeBoxObject ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TreeBoxObject` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub type TreeBoxObject;
    # [ wasm_bindgen ( structural , method , getter , js_class = "TreeBoxObject" , js_name = focused ) ]
    #[doc = "Getter for the `focused` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/focused)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn focused(this: &TreeBoxObject) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "TreeBoxObject" , js_name = focused ) ]
    #[doc = "Setter for the `focused` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/focused)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn set_focused(this: &TreeBoxObject, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_class = "TreeBoxObject" , js_name = treeBody ) ]
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `treeBody` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/treeBody)\n\n*This API requires the following crate features to be activated: `Element`, `TreeBoxObject`*"]
    pub fn tree_body(this: &TreeBoxObject) -> Option<Element>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "TreeBoxObject" , js_name = rowHeight ) ]
    #[doc = "Getter for the `rowHeight` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/rowHeight)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn row_height(this: &TreeBoxObject) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "TreeBoxObject" , js_name = rowWidth ) ]
    #[doc = "Getter for the `rowWidth` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/rowWidth)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn row_width(this: &TreeBoxObject) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "TreeBoxObject" , js_name = horizontalPosition ) ]
    #[doc = "Getter for the `horizontalPosition` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/horizontalPosition)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn horizontal_position(this: &TreeBoxObject) -> i32;
    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = beginUpdateBatch ) ]
    #[doc = "The `beginUpdateBatch()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/beginUpdateBatch)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn begin_update_batch(this: &TreeBoxObject);
    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = clearStyleAndImageCaches ) ]
    #[doc = "The `clearStyleAndImageCaches()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/clearStyleAndImageCaches)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn clear_style_and_image_caches(this: &TreeBoxObject);
    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = endUpdateBatch ) ]
    #[doc = "The `endUpdateBatch()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/endUpdateBatch)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn end_update_batch(this: &TreeBoxObject);
    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = ensureRowIsVisible ) ]
    #[doc = "The `ensureRowIsVisible()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/ensureRowIsVisible)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn ensure_row_is_visible(this: &TreeBoxObject, index: i32);
    #[cfg(feature = "TreeCellInfo")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeBoxObject" , js_name = getCellAt ) ]
    #[doc = "The `getCellAt()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getCellAt)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`, `TreeCellInfo`*"]
    pub fn get_cell_at(this: &TreeBoxObject, x: i32, y: i32) -> Result<TreeCellInfo, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeBoxObject" , js_name = getCellAt ) ]
    #[doc = "The `getCellAt()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getCellAt)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn get_cell_at_with_row_and_column_and_child_elt(
        this: &TreeBoxObject,
        x: i32,
        y: i32,
        row: &::js_sys::Object,
        column: &::js_sys::Object,
        child_elt: &::js_sys::Object,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = getFirstVisibleRow ) ]
    #[doc = "The `getFirstVisibleRow()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getFirstVisibleRow)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn get_first_visible_row(this: &TreeBoxObject) -> i32;
    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = getLastVisibleRow ) ]
    #[doc = "The `getLastVisibleRow()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getLastVisibleRow)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn get_last_visible_row(this: &TreeBoxObject) -> i32;
    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = getPageLength ) ]
    #[doc = "The `getPageLength()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getPageLength)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn get_page_length(this: &TreeBoxObject) -> i32;
    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = getRowAt ) ]
    #[doc = "The `getRowAt()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getRowAt)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn get_row_at(this: &TreeBoxObject, x: i32, y: i32) -> i32;
    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = invalidate ) ]
    #[doc = "The `invalidate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/invalidate)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn invalidate(this: &TreeBoxObject);
    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = invalidateRange ) ]
    #[doc = "The `invalidateRange()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/invalidateRange)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn invalidate_range(this: &TreeBoxObject, start_index: i32, end_index: i32);
    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = invalidateRow ) ]
    #[doc = "The `invalidateRow()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/invalidateRow)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn invalidate_row(this: &TreeBoxObject, index: i32);
    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = rowCountChanged ) ]
    #[doc = "The `rowCountChanged()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/rowCountChanged)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn row_count_changed(this: &TreeBoxObject, index: i32, count: i32);
    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = scrollByLines ) ]
    #[doc = "The `scrollByLines()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/scrollByLines)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn scroll_by_lines(this: &TreeBoxObject, num_lines: i32);
    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = scrollByPages ) ]
    #[doc = "The `scrollByPages()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/scrollByPages)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn scroll_by_pages(this: &TreeBoxObject, num_pages: i32);
    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = scrollToRow ) ]
    #[doc = "The `scrollToRow()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/scrollToRow)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn scroll_to_row(this: &TreeBoxObject, index: i32);
}
