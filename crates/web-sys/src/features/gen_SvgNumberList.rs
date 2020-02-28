use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGNumberList , typescript_name = SVGNumberList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgNumberList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList)\n\n*This API requires the following crate features to be activated: `SvgNumberList`*"]
    pub type SvgNumberList;
    # [ wasm_bindgen ( structural , method , getter , js_name = numberOfItems ) ]
    #[doc = "Getter for the `numberOfItems` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/numberOfItems)\n\n*This API requires the following crate features to be activated: `SvgNumberList`*"]
    pub fn number_of_items(this: &SvgNumberList) -> u32;
    #[cfg(feature = "SvgNumber")]
    # [ wasm_bindgen ( catch , method , structural , js_name = appendItem ) ]
    #[doc = "The `appendItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/appendItem)\n\n*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
    pub fn append_item(this: &SvgNumberList, new_item: &SvgNumber) -> Result<SvgNumber, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = clear ) ]
    #[doc = "The `clear()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/clear)\n\n*This API requires the following crate features to be activated: `SvgNumberList`*"]
    pub fn clear(this: &SvgNumberList) -> Result<(), JsValue>;
    #[cfg(feature = "SvgNumber")]
    # [ wasm_bindgen ( catch , method , structural , js_name = getItem ) ]
    #[doc = "The `getItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/getItem)\n\n*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
    pub fn get_item(this: &SvgNumberList, index: u32) -> Result<SvgNumber, JsValue>;
    #[cfg(feature = "SvgNumber")]
    # [ wasm_bindgen ( catch , method , structural , js_name = initialize ) ]
    #[doc = "The `initialize()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/initialize)\n\n*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
    pub fn initialize(this: &SvgNumberList, new_item: &SvgNumber) -> Result<SvgNumber, JsValue>;
    #[cfg(feature = "SvgNumber")]
    # [ wasm_bindgen ( catch , method , structural , js_name = insertItemBefore ) ]
    #[doc = "The `insertItemBefore()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/insertItemBefore)\n\n*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
    pub fn insert_item_before(
        this: &SvgNumberList,
        new_item: &SvgNumber,
        index: u32,
    ) -> Result<SvgNumber, JsValue>;
    #[cfg(feature = "SvgNumber")]
    # [ wasm_bindgen ( catch , method , structural , js_name = removeItem ) ]
    #[doc = "The `removeItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/removeItem)\n\n*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
    pub fn remove_item(this: &SvgNumberList, index: u32) -> Result<SvgNumber, JsValue>;
    #[cfg(feature = "SvgNumber")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceItem ) ]
    #[doc = "The `replaceItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/replaceItem)\n\n*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
    pub fn replace_item(
        this: &SvgNumberList,
        new_item: &SvgNumber,
        index: u32,
    ) -> Result<SvgNumber, JsValue>;
    #[cfg(feature = "SvgNumber")]
    #[wasm_bindgen(catch, method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
    pub fn get(this: &SvgNumberList, index: u32) -> Result<SvgNumber, JsValue>;
}
