use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = TreeView , typescript_name = TreeView ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `TreeView` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView)
    ///
    ///*This API requires the following crate features to be activated: `TreeView`*
    pub type TreeView;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TreeView" , js_name = rowCount ) ]
    ///Getter for the `rowCount` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/rowCount)
    ///
    ///*This API requires the following crate features to be activated: `TreeView`*
    pub fn row_count(this: &TreeView) -> i32;

    #[cfg(feature = "DataTransfer")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeView" , js_name = canDrop ) ]
    ///The `canDrop()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/canDrop)
    ///
    ///*This API requires the following crate features to be activated: `DataTransfer`, `TreeView`*
    pub fn can_drop(
        this: &TreeView,
        row: i32,
        orientation: i32,
        data_transfer: Option<&DataTransfer>,
    ) -> Result<bool, JsValue>;

    #[cfg(feature = "DataTransfer")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeView" , js_name = drop ) ]
    ///The `drop()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/drop)
    ///
    ///*This API requires the following crate features to be activated: `DataTransfer`, `TreeView`*
    pub fn drop(
        this: &TreeView,
        row: i32,
        orientation: i32,
        data_transfer: Option<&DataTransfer>,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeView" , js_name = getLevel ) ]
    ///The `getLevel()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/getLevel)
    ///
    ///*This API requires the following crate features to be activated: `TreeView`*
    pub fn get_level(this: &TreeView, row: i32) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeView" , js_name = getParentIndex ) ]
    ///The `getParentIndex()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/getParentIndex)
    ///
    ///*This API requires the following crate features to be activated: `TreeView`*
    pub fn get_parent_index(this: &TreeView, row: i32) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeView" , js_name = getRowProperties ) ]
    ///The `getRowProperties()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/getRowProperties)
    ///
    ///*This API requires the following crate features to be activated: `TreeView`*
    pub fn get_row_properties(this: &TreeView, row: i32) -> Result<String, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeView" , js_name = hasNextSibling ) ]
    ///The `hasNextSibling()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/hasNextSibling)
    ///
    ///*This API requires the following crate features to be activated: `TreeView`*
    pub fn has_next_sibling(this: &TreeView, row: i32, after_index: i32) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeView" , js_name = isContainer ) ]
    ///The `isContainer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isContainer)
    ///
    ///*This API requires the following crate features to be activated: `TreeView`*
    pub fn is_container(this: &TreeView, row: i32) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeView" , js_name = isContainerEmpty ) ]
    ///The `isContainerEmpty()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isContainerEmpty)
    ///
    ///*This API requires the following crate features to be activated: `TreeView`*
    pub fn is_container_empty(this: &TreeView, row: i32) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeView" , js_name = isContainerOpen ) ]
    ///The `isContainerOpen()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isContainerOpen)
    ///
    ///*This API requires the following crate features to be activated: `TreeView`*
    pub fn is_container_open(this: &TreeView, row: i32) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeView" , js_name = isSeparator ) ]
    ///The `isSeparator()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isSeparator)
    ///
    ///*This API requires the following crate features to be activated: `TreeView`*
    pub fn is_separator(this: &TreeView, row: i32) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "TreeView" , js_name = isSorted ) ]
    ///The `isSorted()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isSorted)
    ///
    ///*This API requires the following crate features to be activated: `TreeView`*
    pub fn is_sorted(this: &TreeView) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "TreeView" , js_name = performAction ) ]
    ///The `performAction()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/performAction)
    ///
    ///*This API requires the following crate features to be activated: `TreeView`*
    pub fn perform_action(this: &TreeView, action: &str);

    # [ wasm_bindgen ( method , structural , js_class = "TreeView" , js_name = performActionOnRow ) ]
    ///The `performActionOnRow()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/performActionOnRow)
    ///
    ///*This API requires the following crate features to be activated: `TreeView`*
    pub fn perform_action_on_row(this: &TreeView, action: &str, row: i32);

    # [ wasm_bindgen ( method , structural , js_class = "TreeView" , js_name = selectionChanged ) ]
    ///The `selectionChanged()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/selectionChanged)
    ///
    ///*This API requires the following crate features to be activated: `TreeView`*
    pub fn selection_changed(this: &TreeView);

    #[cfg(feature = "TreeBoxObject")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeView" , js_name = setTree ) ]
    ///The `setTree()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/setTree)
    ///
    ///*This API requires the following crate features to be activated: `TreeBoxObject`, `TreeView`*
    pub fn set_tree(this: &TreeView, tree: Option<&TreeBoxObject>) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeView" , js_name = toggleOpenState ) ]
    ///The `toggleOpenState()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/toggleOpenState)
    ///
    ///*This API requires the following crate features to be activated: `TreeView`*
    pub fn toggle_open_state(this: &TreeView, row: i32) -> Result<(), JsValue>;

}

impl TreeView {
    ///The `TreeView.DROP_BEFORE` const.
    ///
    ///*This API requires the following crate features to be activated: `TreeView`*

    pub const DROP_BEFORE: i16 = -1i64 as i16;

    ///The `TreeView.DROP_ON` const.
    ///
    ///*This API requires the following crate features to be activated: `TreeView`*

    pub const DROP_ON: i16 = 0i64 as i16;

    ///The `TreeView.DROP_AFTER` const.
    ///
    ///*This API requires the following crate features to be activated: `TreeView`*

    pub const DROP_AFTER: i16 = 1u64 as i16;
}
