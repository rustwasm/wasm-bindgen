use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Selection , typescript_name = Selection ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Selection` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    pub type Selection;
    # [ wasm_bindgen ( structural , method , getter , js_name = anchorNode ) ]
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `anchorNode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/anchorNode)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    pub fn anchor_node(this: &Selection) -> Option<Node>;
    # [ wasm_bindgen ( structural , method , getter , js_name = anchorOffset ) ]
    #[doc = "Getter for the `anchorOffset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/anchorOffset)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    pub fn anchor_offset(this: &Selection) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = focusNode ) ]
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `focusNode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/focusNode)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    pub fn focus_node(this: &Selection) -> Option<Node>;
    # [ wasm_bindgen ( structural , method , getter , js_name = focusOffset ) ]
    #[doc = "Getter for the `focusOffset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/focusOffset)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    pub fn focus_offset(this: &Selection) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = isCollapsed ) ]
    #[doc = "Getter for the `isCollapsed` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/isCollapsed)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    pub fn is_collapsed(this: &Selection) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = rangeCount ) ]
    #[doc = "Getter for the `rangeCount` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/rangeCount)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    pub fn range_count(this: &Selection) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/type)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    pub fn type_(this: &Selection) -> String;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = caretBidiLevel ) ]
    #[doc = "Getter for the `caretBidiLevel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/caretBidiLevel)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    pub fn caret_bidi_level(this: &Selection) -> Result<Option<i16>, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = caretBidiLevel ) ]
    #[doc = "Setter for the `caretBidiLevel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/caretBidiLevel)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    pub fn set_caret_bidi_level(this: &Selection, value: Result<Option<i16>, JsValue>);
    #[cfg(feature = "Range")]
    # [ wasm_bindgen ( catch , method , structural , js_name = addRange ) ]
    #[doc = "The `addRange()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/addRange)\n\n*This API requires the following crate features to be activated: `Range`, `Selection`*"]
    pub fn add_range(this: &Selection, range: &Range) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = collapse ) ]
    #[doc = "The `collapse()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapse)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    pub fn collapse(this: &Selection, node: Option<&Node>) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = collapse ) ]
    #[doc = "The `collapse()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapse)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    pub fn collapse_with_offset(
        this: &Selection,
        node: Option<&Node>,
        offset: u32,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = collapseToEnd ) ]
    #[doc = "The `collapseToEnd()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapseToEnd)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    pub fn collapse_to_end(this: &Selection) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = collapseToStart ) ]
    #[doc = "The `collapseToStart()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapseToStart)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    pub fn collapse_to_start(this: &Selection) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = containsNode ) ]
    #[doc = "The `containsNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/containsNode)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    pub fn contains_node(this: &Selection, node: &Node) -> Result<bool, JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = containsNode ) ]
    #[doc = "The `containsNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/containsNode)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    pub fn contains_node_with_allow_partial_containment(
        this: &Selection,
        node: &Node,
        allow_partial_containment: bool,
    ) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = deleteFromDocument ) ]
    #[doc = "The `deleteFromDocument()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/deleteFromDocument)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    pub fn delete_from_document(this: &Selection) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = empty ) ]
    #[doc = "The `empty()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/empty)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    pub fn empty(this: &Selection) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = extend ) ]
    #[doc = "The `extend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/extend)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    pub fn extend(this: &Selection, node: &Node) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = extend ) ]
    #[doc = "The `extend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/extend)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    pub fn extend_with_offset(this: &Selection, node: &Node, offset: u32) -> Result<(), JsValue>;
    #[cfg(feature = "Range")]
    # [ wasm_bindgen ( catch , method , structural , js_name = getRangeAt ) ]
    #[doc = "The `getRangeAt()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/getRangeAt)\n\n*This API requires the following crate features to be activated: `Range`, `Selection`*"]
    pub fn get_range_at(this: &Selection, index: u32) -> Result<Range, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = modify ) ]
    #[doc = "The `modify()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/modify)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    pub fn modify(
        this: &Selection,
        alter: &str,
        direction: &str,
        granularity: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = removeAllRanges ) ]
    #[doc = "The `removeAllRanges()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/removeAllRanges)\n\n*This API requires the following crate features to be activated: `Selection`*"]
    pub fn remove_all_ranges(this: &Selection) -> Result<(), JsValue>;
    #[cfg(feature = "Range")]
    # [ wasm_bindgen ( catch , method , structural , js_name = removeRange ) ]
    #[doc = "The `removeRange()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/removeRange)\n\n*This API requires the following crate features to be activated: `Range`, `Selection`*"]
    pub fn remove_range(this: &Selection, range: &Range) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = selectAllChildren ) ]
    #[doc = "The `selectAllChildren()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/selectAllChildren)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    pub fn select_all_children(this: &Selection, node: &Node) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = setBaseAndExtent ) ]
    #[doc = "The `setBaseAndExtent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/setBaseAndExtent)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    pub fn set_base_and_extent(
        this: &Selection,
        anchor_node: &Node,
        anchor_offset: u32,
        focus_node: &Node,
        focus_offset: u32,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = setPosition ) ]
    #[doc = "The `setPosition()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/setPosition)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    pub fn set_position(this: &Selection, node: Option<&Node>) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = setPosition ) ]
    #[doc = "The `setPosition()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/setPosition)\n\n*This API requires the following crate features to be activated: `Node`, `Selection`*"]
    pub fn set_position_with_offset(
        this: &Selection,
        node: Option<&Node>,
        offset: u32,
    ) -> Result<(), JsValue>;
}
