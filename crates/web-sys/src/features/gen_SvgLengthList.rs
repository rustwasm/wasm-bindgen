use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGLengthList , typescript_name = SVGLengthList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgLengthList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList)
    ///
    ///*This API requires the following crate features to be activated: `SvgLengthList`*
    pub type SvgLengthList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGLengthList" , js_name = numberOfItems ) ]
    ///Getter for the `numberOfItems` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/numberOfItems)
    ///
    ///*This API requires the following crate features to be activated: `SvgLengthList`*
    pub fn number_of_items(this: &SvgLengthList) -> u32;

    #[cfg(feature = "SvgLength")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGLengthList" , js_name = appendItem ) ]
    ///The `appendItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/appendItem)
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`, `SvgLengthList`*
    pub fn append_item(this: &SvgLengthList, new_item: &SvgLength) -> Result<SvgLength, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGLengthList" , js_name = clear ) ]
    ///The `clear()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/clear)
    ///
    ///*This API requires the following crate features to be activated: `SvgLengthList`*
    pub fn clear(this: &SvgLengthList) -> Result<(), JsValue>;

    #[cfg(feature = "SvgLength")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGLengthList" , js_name = getItem ) ]
    ///The `getItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/getItem)
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`, `SvgLengthList`*
    pub fn get_item(this: &SvgLengthList, index: u32) -> Result<SvgLength, JsValue>;

    #[cfg(feature = "SvgLength")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGLengthList" , js_name = initialize ) ]
    ///The `initialize()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/initialize)
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`, `SvgLengthList`*
    pub fn initialize(this: &SvgLengthList, new_item: &SvgLength) -> Result<SvgLength, JsValue>;

    #[cfg(feature = "SvgLength")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGLengthList" , js_name = insertItemBefore ) ]
    ///The `insertItemBefore()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/insertItemBefore)
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`, `SvgLengthList`*
    pub fn insert_item_before(
        this: &SvgLengthList,
        new_item: &SvgLength,
        index: u32,
    ) -> Result<SvgLength, JsValue>;

    #[cfg(feature = "SvgLength")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGLengthList" , js_name = removeItem ) ]
    ///The `removeItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/removeItem)
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`, `SvgLengthList`*
    pub fn remove_item(this: &SvgLengthList, index: u32) -> Result<SvgLength, JsValue>;

    #[cfg(feature = "SvgLength")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGLengthList" , js_name = replaceItem ) ]
    ///The `replaceItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/replaceItem)
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`, `SvgLengthList`*
    pub fn replace_item(
        this: &SvgLengthList,
        new_item: &SvgLength,
        index: u32,
    ) -> Result<SvgLength, JsValue>;

    #[cfg(feature = "SvgLength")]
    #[wasm_bindgen(catch, method, structural, js_class = "SVGLengthList", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`, `SvgLengthList`*
    pub fn get(this: &SvgLengthList, index: u32) -> Result<SvgLength, JsValue>;

}
