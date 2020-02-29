use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGStringList , typescript_type = "SVGStringList" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgStringList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList)
    ///
    ///*This API requires the following crate features to be activated: `SvgStringList`*
    pub type SvgStringList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGStringList" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/length)
    ///
    ///*This API requires the following crate features to be activated: `SvgStringList`*
    pub fn length(this: &SvgStringList) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGStringList" , js_name = numberOfItems ) ]
    ///Getter for the `numberOfItems` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/numberOfItems)
    ///
    ///*This API requires the following crate features to be activated: `SvgStringList`*
    pub fn number_of_items(this: &SvgStringList) -> u32;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGStringList" , js_name = appendItem ) ]
    ///The `appendItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/appendItem)
    ///
    ///*This API requires the following crate features to be activated: `SvgStringList`*
    pub fn append_item(this: &SvgStringList, new_item: &str) -> Result<String, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "SVGStringList" , js_name = clear ) ]
    ///The `clear()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/clear)
    ///
    ///*This API requires the following crate features to be activated: `SvgStringList`*
    pub fn clear(this: &SvgStringList);

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGStringList" , js_name = getItem ) ]
    ///The `getItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/getItem)
    ///
    ///*This API requires the following crate features to be activated: `SvgStringList`*
    pub fn get_item(this: &SvgStringList, index: u32) -> Result<String, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGStringList" , js_name = initialize ) ]
    ///The `initialize()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/initialize)
    ///
    ///*This API requires the following crate features to be activated: `SvgStringList`*
    pub fn initialize(this: &SvgStringList, new_item: &str) -> Result<String, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGStringList" , js_name = insertItemBefore ) ]
    ///The `insertItemBefore()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/insertItemBefore)
    ///
    ///*This API requires the following crate features to be activated: `SvgStringList`*
    pub fn insert_item_before(
        this: &SvgStringList,
        new_item: &str,
        index: u32,
    ) -> Result<String, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGStringList" , js_name = removeItem ) ]
    ///The `removeItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/removeItem)
    ///
    ///*This API requires the following crate features to be activated: `SvgStringList`*
    pub fn remove_item(this: &SvgStringList, index: u32) -> Result<String, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGStringList" , js_name = replaceItem ) ]
    ///The `replaceItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/replaceItem)
    ///
    ///*This API requires the following crate features to be activated: `SvgStringList`*
    pub fn replace_item(
        this: &SvgStringList,
        new_item: &str,
        index: u32,
    ) -> Result<String, JsValue>;

    #[wasm_bindgen(method, structural, js_class = "SVGStringList", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `SvgStringList`*
    pub fn get(this: &SvgStringList, index: u32) -> Option<String>;

}
