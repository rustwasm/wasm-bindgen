use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Range , typescript_name = Range ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Range` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range)
    ///
    ///*This API requires the following crate features to be activated: `Range`*
    pub type Range;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Range" , js_name = startContainer ) ]
    ///Getter for the `startContainer` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/startContainer)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Range`*
    pub fn start_container(this: &Range) -> Result<Node, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Range" , js_name = startOffset ) ]
    ///Getter for the `startOffset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/startOffset)
    ///
    ///*This API requires the following crate features to be activated: `Range`*
    pub fn start_offset(this: &Range) -> Result<u32, JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Range" , js_name = endContainer ) ]
    ///Getter for the `endContainer` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/endContainer)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Range`*
    pub fn end_container(this: &Range) -> Result<Node, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Range" , js_name = endOffset ) ]
    ///Getter for the `endOffset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/endOffset)
    ///
    ///*This API requires the following crate features to be activated: `Range`*
    pub fn end_offset(this: &Range) -> Result<u32, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Range" , js_name = collapsed ) ]
    ///Getter for the `collapsed` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/collapsed)
    ///
    ///*This API requires the following crate features to be activated: `Range`*
    pub fn collapsed(this: &Range) -> bool;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Range" , js_name = commonAncestorContainer ) ]
    ///Getter for the `commonAncestorContainer` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/commonAncestorContainer)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Range`*
    pub fn common_ancestor_container(this: &Range) -> Result<Node, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "Range")]
    ///The `new Range(..)` constructor, creating a new instance of `Range`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/Range)
    ///
    ///*This API requires the following crate features to be activated: `Range`*
    pub fn new() -> Result<Range, JsValue>;

    #[cfg(feature = "DocumentFragment")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Range" , js_name = cloneContents ) ]
    ///The `cloneContents()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/cloneContents)
    ///
    ///*This API requires the following crate features to be activated: `DocumentFragment`, `Range`*
    pub fn clone_contents(this: &Range) -> Result<DocumentFragment, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Range" , js_name = cloneRange ) ]
    ///The `cloneRange()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/cloneRange)
    ///
    ///*This API requires the following crate features to be activated: `Range`*
    pub fn clone_range(this: &Range) -> Range;

    # [ wasm_bindgen ( method , structural , js_class = "Range" , js_name = collapse ) ]
    ///The `collapse()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/collapse)
    ///
    ///*This API requires the following crate features to be activated: `Range`*
    pub fn collapse(this: &Range);

    # [ wasm_bindgen ( method , structural , js_class = "Range" , js_name = collapse ) ]
    ///The `collapse()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/collapse)
    ///
    ///*This API requires the following crate features to be activated: `Range`*
    pub fn collapse_with_to_start(this: &Range, to_start: bool);

    # [ wasm_bindgen ( catch , method , structural , js_class = "Range" , js_name = compareBoundaryPoints ) ]
    ///The `compareBoundaryPoints()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/compareBoundaryPoints)
    ///
    ///*This API requires the following crate features to be activated: `Range`*
    pub fn compare_boundary_points(
        this: &Range,
        how: u16,
        source_range: &Range,
    ) -> Result<i16, JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Range" , js_name = comparePoint ) ]
    ///The `comparePoint()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/comparePoint)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Range`*
    pub fn compare_point(this: &Range, node: &Node, offset: u32) -> Result<i16, JsValue>;

    #[cfg(feature = "DocumentFragment")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Range" , js_name = createContextualFragment ) ]
    ///The `createContextualFragment()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/createContextualFragment)
    ///
    ///*This API requires the following crate features to be activated: `DocumentFragment`, `Range`*
    pub fn create_contextual_fragment(
        this: &Range,
        fragment: &str,
    ) -> Result<DocumentFragment, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Range" , js_name = deleteContents ) ]
    ///The `deleteContents()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/deleteContents)
    ///
    ///*This API requires the following crate features to be activated: `Range`*
    pub fn delete_contents(this: &Range) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Range" , js_name = detach ) ]
    ///The `detach()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/detach)
    ///
    ///*This API requires the following crate features to be activated: `Range`*
    pub fn detach(this: &Range);

    #[cfg(feature = "DocumentFragment")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Range" , js_name = extractContents ) ]
    ///The `extractContents()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/extractContents)
    ///
    ///*This API requires the following crate features to be activated: `DocumentFragment`, `Range`*
    pub fn extract_contents(this: &Range) -> Result<DocumentFragment, JsValue>;

    #[cfg(feature = "DomRect")]
    # [ wasm_bindgen ( method , structural , js_class = "Range" , js_name = getBoundingClientRect ) ]
    ///The `getBoundingClientRect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/getBoundingClientRect)
    ///
    ///*This API requires the following crate features to be activated: `DomRect`, `Range`*
    pub fn get_bounding_client_rect(this: &Range) -> DomRect;

    #[cfg(feature = "DomRectList")]
    # [ wasm_bindgen ( method , structural , js_class = "Range" , js_name = getClientRects ) ]
    ///The `getClientRects()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/getClientRects)
    ///
    ///*This API requires the following crate features to be activated: `DomRectList`, `Range`*
    pub fn get_client_rects(this: &Range) -> Option<DomRectList>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Range" , js_name = insertNode ) ]
    ///The `insertNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/insertNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Range`*
    pub fn insert_node(this: &Range, node: &Node) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Range" , js_name = intersectsNode ) ]
    ///The `intersectsNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/intersectsNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Range`*
    pub fn intersects_node(this: &Range, node: &Node) -> Result<bool, JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Range" , js_name = isPointInRange ) ]
    ///The `isPointInRange()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/isPointInRange)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Range`*
    pub fn is_point_in_range(this: &Range, node: &Node, offset: u32) -> Result<bool, JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Range" , js_name = selectNode ) ]
    ///The `selectNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/selectNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Range`*
    pub fn select_node(this: &Range, ref_node: &Node) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Range" , js_name = selectNodeContents ) ]
    ///The `selectNodeContents()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/selectNodeContents)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Range`*
    pub fn select_node_contents(this: &Range, ref_node: &Node) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Range" , js_name = setEnd ) ]
    ///The `setEnd()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setEnd)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Range`*
    pub fn set_end(this: &Range, ref_node: &Node, offset: u32) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Range" , js_name = setEndAfter ) ]
    ///The `setEndAfter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setEndAfter)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Range`*
    pub fn set_end_after(this: &Range, ref_node: &Node) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Range" , js_name = setEndBefore ) ]
    ///The `setEndBefore()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setEndBefore)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Range`*
    pub fn set_end_before(this: &Range, ref_node: &Node) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Range" , js_name = setStart ) ]
    ///The `setStart()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setStart)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Range`*
    pub fn set_start(this: &Range, ref_node: &Node, offset: u32) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Range" , js_name = setStartAfter ) ]
    ///The `setStartAfter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setStartAfter)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Range`*
    pub fn set_start_after(this: &Range, ref_node: &Node) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Range" , js_name = setStartBefore ) ]
    ///The `setStartBefore()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setStartBefore)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Range`*
    pub fn set_start_before(this: &Range, ref_node: &Node) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Range" , js_name = surroundContents ) ]
    ///The `surroundContents()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/surroundContents)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Range`*
    pub fn surround_contents(this: &Range, new_parent: &Node) -> Result<(), JsValue>;

}

impl Range {
    ///The `Range.START_TO_START` const.
    ///
    ///*This API requires the following crate features to be activated: `Range`*

    pub const START_TO_START: u16 = 0i64 as u16;

    ///The `Range.START_TO_END` const.
    ///
    ///*This API requires the following crate features to be activated: `Range`*

    pub const START_TO_END: u16 = 1u64 as u16;

    ///The `Range.END_TO_END` const.
    ///
    ///*This API requires the following crate features to be activated: `Range`*

    pub const END_TO_END: u16 = 2u64 as u16;

    ///The `Range.END_TO_START` const.
    ///
    ///*This API requires the following crate features to be activated: `Range`*

    pub const END_TO_START: u16 = 3u64 as u16;
}
