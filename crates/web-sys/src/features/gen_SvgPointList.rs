use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGPointList , typescript_name = SVGPointList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgPointList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList)
    ///
    ///*This API requires the following crate features to be activated: `SvgPointList`*
    pub type SvgPointList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPointList" , js_name = numberOfItems ) ]
    ///Getter for the `numberOfItems` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/numberOfItems)
    ///
    ///*This API requires the following crate features to be activated: `SvgPointList`*
    pub fn number_of_items(this: &SvgPointList) -> u32;

    #[cfg(feature = "SvgPoint")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGPointList" , js_name = appendItem ) ]
    ///The `appendItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/appendItem)
    ///
    ///*This API requires the following crate features to be activated: `SvgPoint`, `SvgPointList`*
    pub fn append_item(this: &SvgPointList, new_item: &SvgPoint) -> Result<SvgPoint, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGPointList" , js_name = clear ) ]
    ///The `clear()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/clear)
    ///
    ///*This API requires the following crate features to be activated: `SvgPointList`*
    pub fn clear(this: &SvgPointList) -> Result<(), JsValue>;

    #[cfg(feature = "SvgPoint")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGPointList" , js_name = getItem ) ]
    ///The `getItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/getItem)
    ///
    ///*This API requires the following crate features to be activated: `SvgPoint`, `SvgPointList`*
    pub fn get_item(this: &SvgPointList, index: u32) -> Result<SvgPoint, JsValue>;

    #[cfg(feature = "SvgPoint")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGPointList" , js_name = initialize ) ]
    ///The `initialize()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/initialize)
    ///
    ///*This API requires the following crate features to be activated: `SvgPoint`, `SvgPointList`*
    pub fn initialize(this: &SvgPointList, new_item: &SvgPoint) -> Result<SvgPoint, JsValue>;

    #[cfg(feature = "SvgPoint")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGPointList" , js_name = insertItemBefore ) ]
    ///The `insertItemBefore()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/insertItemBefore)
    ///
    ///*This API requires the following crate features to be activated: `SvgPoint`, `SvgPointList`*
    pub fn insert_item_before(
        this: &SvgPointList,
        new_item: &SvgPoint,
        index: u32,
    ) -> Result<SvgPoint, JsValue>;

    #[cfg(feature = "SvgPoint")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGPointList" , js_name = removeItem ) ]
    ///The `removeItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/removeItem)
    ///
    ///*This API requires the following crate features to be activated: `SvgPoint`, `SvgPointList`*
    pub fn remove_item(this: &SvgPointList, index: u32) -> Result<SvgPoint, JsValue>;

    #[cfg(feature = "SvgPoint")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGPointList" , js_name = replaceItem ) ]
    ///The `replaceItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/replaceItem)
    ///
    ///*This API requires the following crate features to be activated: `SvgPoint`, `SvgPointList`*
    pub fn replace_item(
        this: &SvgPointList,
        new_item: &SvgPoint,
        index: u32,
    ) -> Result<SvgPoint, JsValue>;

    #[cfg(feature = "SvgPoint")]
    #[wasm_bindgen(catch, method, structural, js_class = "SVGPointList", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `SvgPoint`, `SvgPointList`*
    pub fn get(this: &SvgPointList, index: u32) -> Result<SvgPoint, JsValue>;

}
