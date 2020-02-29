use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGTransformList , typescript_name = SVGTransformList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgTransformList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList)
    ///
    ///*This API requires the following crate features to be activated: `SvgTransformList`*
    pub type SvgTransformList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTransformList" , js_name = numberOfItems ) ]
    ///Getter for the `numberOfItems` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/numberOfItems)
    ///
    ///*This API requires the following crate features to be activated: `SvgTransformList`*
    pub fn number_of_items(this: &SvgTransformList) -> u32;

    #[cfg(feature = "SvgTransform")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGTransformList" , js_name = appendItem ) ]
    ///The `appendItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/appendItem)
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*
    pub fn append_item(
        this: &SvgTransformList,
        new_item: &SvgTransform,
    ) -> Result<SvgTransform, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGTransformList" , js_name = clear ) ]
    ///The `clear()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/clear)
    ///
    ///*This API requires the following crate features to be activated: `SvgTransformList`*
    pub fn clear(this: &SvgTransformList) -> Result<(), JsValue>;

    #[cfg(feature = "SvgTransform")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGTransformList" , js_name = consolidate ) ]
    ///The `consolidate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/consolidate)
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*
    pub fn consolidate(this: &SvgTransformList) -> Result<Option<SvgTransform>, JsValue>;

    #[cfg(all(feature = "SvgMatrix", feature = "SvgTransform",))]
    # [ wasm_bindgen ( method , structural , js_class = "SVGTransformList" , js_name = createSVGTransformFromMatrix ) ]
    ///The `createSVGTransformFromMatrix()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/createSVGTransformFromMatrix)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`, `SvgTransform`, `SvgTransformList`*
    pub fn create_svg_transform_from_matrix(
        this: &SvgTransformList,
        matrix: &SvgMatrix,
    ) -> SvgTransform;

    #[cfg(feature = "SvgTransform")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGTransformList" , js_name = getItem ) ]
    ///The `getItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/getItem)
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*
    pub fn get_item(this: &SvgTransformList, index: u32) -> Result<SvgTransform, JsValue>;

    #[cfg(feature = "SvgTransform")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGTransformList" , js_name = initialize ) ]
    ///The `initialize()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/initialize)
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*
    pub fn initialize(
        this: &SvgTransformList,
        new_item: &SvgTransform,
    ) -> Result<SvgTransform, JsValue>;

    #[cfg(feature = "SvgTransform")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGTransformList" , js_name = insertItemBefore ) ]
    ///The `insertItemBefore()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/insertItemBefore)
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*
    pub fn insert_item_before(
        this: &SvgTransformList,
        new_item: &SvgTransform,
        index: u32,
    ) -> Result<SvgTransform, JsValue>;

    #[cfg(feature = "SvgTransform")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGTransformList" , js_name = removeItem ) ]
    ///The `removeItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/removeItem)
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*
    pub fn remove_item(this: &SvgTransformList, index: u32) -> Result<SvgTransform, JsValue>;

    #[cfg(feature = "SvgTransform")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGTransformList" , js_name = replaceItem ) ]
    ///The `replaceItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/replaceItem)
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*
    pub fn replace_item(
        this: &SvgTransformList,
        new_item: &SvgTransform,
        index: u32,
    ) -> Result<SvgTransform, JsValue>;

    #[cfg(feature = "SvgTransform")]
    #[wasm_bindgen(
        catch,
        method,
        structural,
        js_class = "SVGTransformList",
        indexing_getter
    )]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*
    pub fn get(this: &SvgTransformList, index: u32) -> Result<SvgTransform, JsValue>;

}
