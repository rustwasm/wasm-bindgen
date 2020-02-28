use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = TreeView , typescript_name = TreeView ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TreeView` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    pub type TreeView;
    # [ wasm_bindgen ( structural , method , getter , js_name = rowCount ) ]
    #[doc = "Getter for the `rowCount` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/rowCount)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn row_count(this: &TreeView) -> i32;
    #[cfg(feature = "DataTransfer")]
    # [ wasm_bindgen ( catch , method , structural , js_name = canDrop ) ]
    #[doc = "The `canDrop()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/canDrop)\n\n*This API requires the following crate features to be activated: `DataTransfer`, `TreeView`*"]
    pub fn can_drop(
        this: &TreeView,
        row: i32,
        orientation: i32,
        data_transfer: Option<&DataTransfer>,
    ) -> Result<bool, JsValue>;
    #[cfg(feature = "DataTransfer")]
    # [ wasm_bindgen ( catch , method , structural , js_name = drop ) ]
    #[doc = "The `drop()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/drop)\n\n*This API requires the following crate features to be activated: `DataTransfer`, `TreeView`*"]
    pub fn drop(
        this: &TreeView,
        row: i32,
        orientation: i32,
        data_transfer: Option<&DataTransfer>,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = getLevel ) ]
    #[doc = "The `getLevel()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/getLevel)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn get_level(this: &TreeView, row: i32) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = getParentIndex ) ]
    #[doc = "The `getParentIndex()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/getParentIndex)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn get_parent_index(this: &TreeView, row: i32) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = getRowProperties ) ]
    #[doc = "The `getRowProperties()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/getRowProperties)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn get_row_properties(this: &TreeView, row: i32) -> Result<String, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = hasNextSibling ) ]
    #[doc = "The `hasNextSibling()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/hasNextSibling)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn has_next_sibling(this: &TreeView, row: i32, after_index: i32) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = isContainer ) ]
    #[doc = "The `isContainer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isContainer)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn is_container(this: &TreeView, row: i32) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = isContainerEmpty ) ]
    #[doc = "The `isContainerEmpty()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isContainerEmpty)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn is_container_empty(this: &TreeView, row: i32) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = isContainerOpen ) ]
    #[doc = "The `isContainerOpen()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isContainerOpen)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn is_container_open(this: &TreeView, row: i32) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = isSeparator ) ]
    #[doc = "The `isSeparator()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isSeparator)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn is_separator(this: &TreeView, row: i32) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = isSorted ) ]
    #[doc = "The `isSorted()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isSorted)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn is_sorted(this: &TreeView) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = performAction ) ]
    #[doc = "The `performAction()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/performAction)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn perform_action(this: &TreeView, action: &str);
    # [ wasm_bindgen ( method , structural , js_name = performActionOnRow ) ]
    #[doc = "The `performActionOnRow()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/performActionOnRow)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn perform_action_on_row(this: &TreeView, action: &str, row: i32);
    # [ wasm_bindgen ( method , structural , js_name = selectionChanged ) ]
    #[doc = "The `selectionChanged()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/selectionChanged)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn selection_changed(this: &TreeView);
    #[cfg(feature = "TreeBoxObject")]
    # [ wasm_bindgen ( catch , method , structural , js_name = setTree ) ]
    #[doc = "The `setTree()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/setTree)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`, `TreeView`*"]
    pub fn set_tree(this: &TreeView, tree: Option<&TreeBoxObject>) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = toggleOpenState ) ]
    #[doc = "The `toggleOpenState()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/toggleOpenState)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn toggle_open_state(this: &TreeView, row: i32) -> Result<(), JsValue>;
}
impl TreeView {
    pub const DROP_BEFORE: i16 = -1i64 as i16;
    pub const DROP_ON: i16 = 0i64 as i16;
    pub const DROP_AFTER: i16 = 1u64 as i16;
}
