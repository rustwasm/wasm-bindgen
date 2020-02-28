use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGStringList , typescript_name = SVGStringList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgStringList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList)\n\n*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub type SvgStringList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGStringList" , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/length)\n\n*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub fn length(this: &SvgStringList) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGStringList" , js_name = numberOfItems ) ]
    #[doc = "Getter for the `numberOfItems` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/numberOfItems)\n\n*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub fn number_of_items(this: &SvgStringList) -> u32;
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGStringList" , js_name = appendItem ) ]
    #[doc = "The `appendItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/appendItem)\n\n*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub fn append_item(this: &SvgStringList, new_item: &str) -> Result<String, JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "SVGStringList" , js_name = clear ) ]
    #[doc = "The `clear()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/clear)\n\n*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub fn clear(this: &SvgStringList);
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGStringList" , js_name = getItem ) ]
    #[doc = "The `getItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/getItem)\n\n*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub fn get_item(this: &SvgStringList, index: u32) -> Result<String, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGStringList" , js_name = initialize ) ]
    #[doc = "The `initialize()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/initialize)\n\n*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub fn initialize(this: &SvgStringList, new_item: &str) -> Result<String, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGStringList" , js_name = insertItemBefore ) ]
    #[doc = "The `insertItemBefore()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/insertItemBefore)\n\n*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub fn insert_item_before(
        this: &SvgStringList,
        new_item: &str,
        index: u32,
    ) -> Result<String, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGStringList" , js_name = removeItem ) ]
    #[doc = "The `removeItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/removeItem)\n\n*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub fn remove_item(this: &SvgStringList, index: u32) -> Result<String, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGStringList" , js_name = replaceItem ) ]
    #[doc = "The `replaceItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/replaceItem)\n\n*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub fn replace_item(
        this: &SvgStringList,
        new_item: &str,
        index: u32,
    ) -> Result<String, JsValue>;
    #[wasm_bindgen(method, structural, js_class = "SVGStringList", indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub fn get(this: &SvgStringList, index: u32) -> Option<String>;
}
