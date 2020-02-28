use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGPointList , typescript_name = SVGPointList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPointList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList)\n\n*This API requires the following crate features to be activated: `SvgPointList`*"]
    pub type SvgPointList;
    # [ wasm_bindgen ( structural , method , getter , js_name = numberOfItems ) ]
    #[doc = "Getter for the `numberOfItems` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/numberOfItems)\n\n*This API requires the following crate features to be activated: `SvgPointList`*"]
    pub fn number_of_items(this: &SvgPointList) -> u32;
    #[cfg(feature = "SvgPoint")]
    # [ wasm_bindgen ( catch , method , structural , js_name = appendItem ) ]
    #[doc = "The `appendItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/appendItem)\n\n*This API requires the following crate features to be activated: `SvgPoint`, `SvgPointList`*"]
    pub fn append_item(this: &SvgPointList, new_item: &SvgPoint) -> Result<SvgPoint, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = clear ) ]
    #[doc = "The `clear()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/clear)\n\n*This API requires the following crate features to be activated: `SvgPointList`*"]
    pub fn clear(this: &SvgPointList) -> Result<(), JsValue>;
    #[cfg(feature = "SvgPoint")]
    # [ wasm_bindgen ( catch , method , structural , js_name = getItem ) ]
    #[doc = "The `getItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/getItem)\n\n*This API requires the following crate features to be activated: `SvgPoint`, `SvgPointList`*"]
    pub fn get_item(this: &SvgPointList, index: u32) -> Result<SvgPoint, JsValue>;
    #[cfg(feature = "SvgPoint")]
    # [ wasm_bindgen ( catch , method , structural , js_name = initialize ) ]
    #[doc = "The `initialize()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/initialize)\n\n*This API requires the following crate features to be activated: `SvgPoint`, `SvgPointList`*"]
    pub fn initialize(this: &SvgPointList, new_item: &SvgPoint) -> Result<SvgPoint, JsValue>;
    #[cfg(feature = "SvgPoint")]
    # [ wasm_bindgen ( catch , method , structural , js_name = insertItemBefore ) ]
    #[doc = "The `insertItemBefore()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/insertItemBefore)\n\n*This API requires the following crate features to be activated: `SvgPoint`, `SvgPointList`*"]
    pub fn insert_item_before(
        this: &SvgPointList,
        new_item: &SvgPoint,
        index: u32,
    ) -> Result<SvgPoint, JsValue>;
    #[cfg(feature = "SvgPoint")]
    # [ wasm_bindgen ( catch , method , structural , js_name = removeItem ) ]
    #[doc = "The `removeItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/removeItem)\n\n*This API requires the following crate features to be activated: `SvgPoint`, `SvgPointList`*"]
    pub fn remove_item(this: &SvgPointList, index: u32) -> Result<SvgPoint, JsValue>;
    #[cfg(feature = "SvgPoint")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceItem ) ]
    #[doc = "The `replaceItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/replaceItem)\n\n*This API requires the following crate features to be activated: `SvgPoint`, `SvgPointList`*"]
    pub fn replace_item(
        this: &SvgPointList,
        new_item: &SvgPoint,
        index: u32,
    ) -> Result<SvgPoint, JsValue>;
    #[cfg(feature = "SvgPoint")]
    #[wasm_bindgen(catch, method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `SvgPoint`, `SvgPointList`*"]
    pub fn get(this: &SvgPointList, index: u32) -> Result<SvgPoint, JsValue>;
}
