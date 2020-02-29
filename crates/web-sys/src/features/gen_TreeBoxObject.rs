use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = TreeBoxObject , typescript_type = "TreeBoxObject" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `TreeBoxObject` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub type TreeBoxObject;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TreeBoxObject" , js_name = focused ) ]
    ///Getter for the `focused` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/focused)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub fn focused(this: &TreeBoxObject) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "TreeBoxObject" , js_name = focused ) ]
    ///Setter for the `focused` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/focused)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub fn set_focused(this: &TreeBoxObject, value: bool);

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "TreeBoxObject" , js_name = treeBody ) ]
    ///Getter for the `treeBody` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/treeBody)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `TreeBoxObject`*
    pub fn tree_body(this: &TreeBoxObject) -> Option<Element>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TreeBoxObject" , js_name = rowHeight ) ]
    ///Getter for the `rowHeight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/rowHeight)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub fn row_height(this: &TreeBoxObject) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TreeBoxObject" , js_name = rowWidth ) ]
    ///Getter for the `rowWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/rowWidth)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub fn row_width(this: &TreeBoxObject) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TreeBoxObject" , js_name = horizontalPosition ) ]
    ///Getter for the `horizontalPosition` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/horizontalPosition)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub fn horizontal_position(this: &TreeBoxObject) -> i32;

    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = beginUpdateBatch ) ]
    ///The `beginUpdateBatch()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/beginUpdateBatch)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub fn begin_update_batch(this: &TreeBoxObject);

    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = clearStyleAndImageCaches ) ]
    ///The `clearStyleAndImageCaches()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/clearStyleAndImageCaches)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub fn clear_style_and_image_caches(this: &TreeBoxObject);

    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = endUpdateBatch ) ]
    ///The `endUpdateBatch()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/endUpdateBatch)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub fn end_update_batch(this: &TreeBoxObject);

    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = ensureRowIsVisible ) ]
    ///The `ensureRowIsVisible()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/ensureRowIsVisible)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub fn ensure_row_is_visible(this: &TreeBoxObject, index: i32);

    #[cfg(feature = "TreeCellInfo")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeBoxObject" , js_name = getCellAt ) ]
    ///The `getCellAt()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getCellAt)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`, `TreeCellInfo`*
    pub fn get_cell_at(this: &TreeBoxObject, x: i32, y: i32) -> Result<TreeCellInfo, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeBoxObject" , js_name = getCellAt ) ]
    ///The `getCellAt()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getCellAt)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub fn get_cell_at_with_row_and_column_and_child_elt(
        this: &TreeBoxObject,
        x: i32,
        y: i32,
        row: &::js_sys::Object,
        column: &::js_sys::Object,
        child_elt: &::js_sys::Object,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = getFirstVisibleRow ) ]
    ///The `getFirstVisibleRow()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getFirstVisibleRow)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub fn get_first_visible_row(this: &TreeBoxObject) -> i32;

    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = getLastVisibleRow ) ]
    ///The `getLastVisibleRow()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getLastVisibleRow)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub fn get_last_visible_row(this: &TreeBoxObject) -> i32;

    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = getPageLength ) ]
    ///The `getPageLength()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getPageLength)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub fn get_page_length(this: &TreeBoxObject) -> i32;

    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = getRowAt ) ]
    ///The `getRowAt()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getRowAt)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub fn get_row_at(this: &TreeBoxObject, x: i32, y: i32) -> i32;

    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = invalidate ) ]
    ///The `invalidate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/invalidate)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub fn invalidate(this: &TreeBoxObject);

    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = invalidateRange ) ]
    ///The `invalidateRange()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/invalidateRange)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub fn invalidate_range(this: &TreeBoxObject, start_index: i32, end_index: i32);

    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = invalidateRow ) ]
    ///The `invalidateRow()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/invalidateRow)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub fn invalidate_row(this: &TreeBoxObject, index: i32);

    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = rowCountChanged ) ]
    ///The `rowCountChanged()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/rowCountChanged)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub fn row_count_changed(this: &TreeBoxObject, index: i32, count: i32);

    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = scrollByLines ) ]
    ///The `scrollByLines()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/scrollByLines)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub fn scroll_by_lines(this: &TreeBoxObject, num_lines: i32);

    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = scrollByPages ) ]
    ///The `scrollByPages()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/scrollByPages)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub fn scroll_by_pages(this: &TreeBoxObject, num_pages: i32);

    # [ wasm_bindgen ( method , structural , js_class = "TreeBoxObject" , js_name = scrollToRow ) ]
    ///The `scrollToRow()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/scrollToRow)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`*
    pub fn scroll_to_row(this: &TreeBoxObject, index: i32);

}
