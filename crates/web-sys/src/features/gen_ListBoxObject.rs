use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = ListBoxObject , typescript_name = ListBoxObject ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ListBoxObject` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject)\n\n*This API requires the following crate features to be activated: `ListBoxObject`*"]
    pub type ListBoxObject;
    # [ wasm_bindgen ( method , structural , js_name = ensureIndexIsVisible ) ]
    #[doc = "The `ensureIndexIsVisible()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/ensureIndexIsVisible)\n\n*This API requires the following crate features to be activated: `ListBoxObject`*"]
    pub fn ensure_index_is_visible(this: &ListBoxObject, row_index: i32);
    # [ wasm_bindgen ( method , structural , js_name = getIndexOfFirstVisibleRow ) ]
    #[doc = "The `getIndexOfFirstVisibleRow()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getIndexOfFirstVisibleRow)\n\n*This API requires the following crate features to be activated: `ListBoxObject`*"]
    pub fn get_index_of_first_visible_row(this: &ListBoxObject) -> i32;
    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_name = getIndexOfItem ) ]
    #[doc = "The `getIndexOfItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getIndexOfItem)\n\n*This API requires the following crate features to be activated: `Element`, `ListBoxObject`*"]
    pub fn get_index_of_item(this: &ListBoxObject, item: &Element) -> i32;
    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_name = getItemAtIndex ) ]
    #[doc = "The `getItemAtIndex()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getItemAtIndex)\n\n*This API requires the following crate features to be activated: `Element`, `ListBoxObject`*"]
    pub fn get_item_at_index(this: &ListBoxObject, index: i32) -> Option<Element>;
    # [ wasm_bindgen ( method , structural , js_name = getNumberOfVisibleRows ) ]
    #[doc = "The `getNumberOfVisibleRows()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getNumberOfVisibleRows)\n\n*This API requires the following crate features to be activated: `ListBoxObject`*"]
    pub fn get_number_of_visible_rows(this: &ListBoxObject) -> i32;
    # [ wasm_bindgen ( method , structural , js_name = getRowCount ) ]
    #[doc = "The `getRowCount()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getRowCount)\n\n*This API requires the following crate features to be activated: `ListBoxObject`*"]
    pub fn get_row_count(this: &ListBoxObject) -> i32;
    # [ wasm_bindgen ( method , structural , js_name = getRowHeight ) ]
    #[doc = "The `getRowHeight()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getRowHeight)\n\n*This API requires the following crate features to be activated: `ListBoxObject`*"]
    pub fn get_row_height(this: &ListBoxObject) -> i32;
    # [ wasm_bindgen ( method , structural , js_name = scrollByLines ) ]
    #[doc = "The `scrollByLines()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/scrollByLines)\n\n*This API requires the following crate features to be activated: `ListBoxObject`*"]
    pub fn scroll_by_lines(this: &ListBoxObject, num_lines: i32);
    # [ wasm_bindgen ( method , structural , js_name = scrollToIndex ) ]
    #[doc = "The `scrollToIndex()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/scrollToIndex)\n\n*This API requires the following crate features to be activated: `ListBoxObject`*"]
    pub fn scroll_to_index(this: &ListBoxObject, row_index: i32);
}
