use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = ListBoxObject , typescript_name = ListBoxObject ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ListBoxObject` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject)
    ///
    ///*This API requires the following crate features to be activated: `ListBoxObject`*
    pub type ListBoxObject;

    # [ wasm_bindgen ( method , structural , js_class = "ListBoxObject" , js_name = ensureIndexIsVisible ) ]
    ///The `ensureIndexIsVisible()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/ensureIndexIsVisible)
    ///
    ///*This API requires the following crate features to be activated: `ListBoxObject`*
    pub fn ensure_index_is_visible(this: &ListBoxObject, row_index: i32);

    # [ wasm_bindgen ( method , structural , js_class = "ListBoxObject" , js_name = getIndexOfFirstVisibleRow ) ]
    ///The `getIndexOfFirstVisibleRow()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getIndexOfFirstVisibleRow)
    ///
    ///*This API requires the following crate features to be activated: `ListBoxObject`*
    pub fn get_index_of_first_visible_row(this: &ListBoxObject) -> i32;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_class = "ListBoxObject" , js_name = getIndexOfItem ) ]
    ///The `getIndexOfItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getIndexOfItem)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `ListBoxObject`*
    pub fn get_index_of_item(this: &ListBoxObject, item: &Element) -> i32;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_class = "ListBoxObject" , js_name = getItemAtIndex ) ]
    ///The `getItemAtIndex()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getItemAtIndex)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `ListBoxObject`*
    pub fn get_item_at_index(this: &ListBoxObject, index: i32) -> Option<Element>;

    # [ wasm_bindgen ( method , structural , js_class = "ListBoxObject" , js_name = getNumberOfVisibleRows ) ]
    ///The `getNumberOfVisibleRows()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getNumberOfVisibleRows)
    ///
    ///*This API requires the following crate features to be activated: `ListBoxObject`*
    pub fn get_number_of_visible_rows(this: &ListBoxObject) -> i32;

    # [ wasm_bindgen ( method , structural , js_class = "ListBoxObject" , js_name = getRowCount ) ]
    ///The `getRowCount()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getRowCount)
    ///
    ///*This API requires the following crate features to be activated: `ListBoxObject`*
    pub fn get_row_count(this: &ListBoxObject) -> i32;

    # [ wasm_bindgen ( method , structural , js_class = "ListBoxObject" , js_name = getRowHeight ) ]
    ///The `getRowHeight()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getRowHeight)
    ///
    ///*This API requires the following crate features to be activated: `ListBoxObject`*
    pub fn get_row_height(this: &ListBoxObject) -> i32;

    # [ wasm_bindgen ( method , structural , js_class = "ListBoxObject" , js_name = scrollByLines ) ]
    ///The `scrollByLines()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/scrollByLines)
    ///
    ///*This API requires the following crate features to be activated: `ListBoxObject`*
    pub fn scroll_by_lines(this: &ListBoxObject, num_lines: i32);

    # [ wasm_bindgen ( method , structural , js_class = "ListBoxObject" , js_name = scrollToIndex ) ]
    ///The `scrollToIndex()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/scrollToIndex)
    ///
    ///*This API requires the following crate features to be activated: `ListBoxObject`*
    pub fn scroll_to_index(this: &ListBoxObject, row_index: i32);

}
