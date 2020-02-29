use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGNumberList , typescript_type = "SVGNumberList" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgNumberList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList)
    ///
    ///*This API requires the following crate features to be activated: `SvgNumberList`*
    pub type SvgNumberList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGNumberList" , js_name = numberOfItems ) ]
    ///Getter for the `numberOfItems` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/numberOfItems)
    ///
    ///*This API requires the following crate features to be activated: `SvgNumberList`*
    pub fn number_of_items(this: &SvgNumberList) -> u32;

    #[cfg(feature = "SvgNumber")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGNumberList" , js_name = appendItem ) ]
    ///The `appendItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/appendItem)
    ///
    ///*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*
    pub fn append_item(this: &SvgNumberList, new_item: &SvgNumber) -> Result<SvgNumber, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGNumberList" , js_name = clear ) ]
    ///The `clear()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/clear)
    ///
    ///*This API requires the following crate features to be activated: `SvgNumberList`*
    pub fn clear(this: &SvgNumberList) -> Result<(), JsValue>;

    #[cfg(feature = "SvgNumber")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGNumberList" , js_name = getItem ) ]
    ///The `getItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/getItem)
    ///
    ///*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*
    pub fn get_item(this: &SvgNumberList, index: u32) -> Result<SvgNumber, JsValue>;

    #[cfg(feature = "SvgNumber")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGNumberList" , js_name = initialize ) ]
    ///The `initialize()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/initialize)
    ///
    ///*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*
    pub fn initialize(this: &SvgNumberList, new_item: &SvgNumber) -> Result<SvgNumber, JsValue>;

    #[cfg(feature = "SvgNumber")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGNumberList" , js_name = insertItemBefore ) ]
    ///The `insertItemBefore()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/insertItemBefore)
    ///
    ///*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*
    pub fn insert_item_before(
        this: &SvgNumberList,
        new_item: &SvgNumber,
        index: u32,
    ) -> Result<SvgNumber, JsValue>;

    #[cfg(feature = "SvgNumber")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGNumberList" , js_name = removeItem ) ]
    ///The `removeItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/removeItem)
    ///
    ///*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*
    pub fn remove_item(this: &SvgNumberList, index: u32) -> Result<SvgNumber, JsValue>;

    #[cfg(feature = "SvgNumber")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGNumberList" , js_name = replaceItem ) ]
    ///The `replaceItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/replaceItem)
    ///
    ///*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*
    pub fn replace_item(
        this: &SvgNumberList,
        new_item: &SvgNumber,
        index: u32,
    ) -> Result<SvgNumber, JsValue>;

    #[cfg(feature = "SvgNumber")]
    #[wasm_bindgen(catch, method, structural, js_class = "SVGNumberList", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*
    pub fn get(this: &SvgNumberList, index: u32) -> Result<SvgNumber, JsValue>;

}
