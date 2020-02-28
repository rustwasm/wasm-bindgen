use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGTransformList , typescript_name = SVGTransformList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgTransformList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList)\n\n*This API requires the following crate features to be activated: `SvgTransformList`*"]
    pub type SvgTransformList;
    # [ wasm_bindgen ( structural , method , getter , js_name = numberOfItems ) ]
    #[doc = "Getter for the `numberOfItems` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/numberOfItems)\n\n*This API requires the following crate features to be activated: `SvgTransformList`*"]
    pub fn number_of_items(this: &SvgTransformList) -> u32;
    #[cfg(feature = "SvgTransform")]
    # [ wasm_bindgen ( catch , method , structural , js_name = appendItem ) ]
    #[doc = "The `appendItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/appendItem)\n\n*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    pub fn append_item(
        this: &SvgTransformList,
        new_item: &SvgTransform,
    ) -> Result<SvgTransform, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = clear ) ]
    #[doc = "The `clear()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/clear)\n\n*This API requires the following crate features to be activated: `SvgTransformList`*"]
    pub fn clear(this: &SvgTransformList) -> Result<(), JsValue>;
    #[cfg(feature = "SvgTransform")]
    # [ wasm_bindgen ( catch , method , structural , js_name = consolidate ) ]
    #[doc = "The `consolidate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/consolidate)\n\n*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    pub fn consolidate(this: &SvgTransformList) -> Result<Option<SvgTransform>, JsValue>;
    #[cfg(all(feature = "SvgMatrix", feature = "SvgTransform",))]
    # [ wasm_bindgen ( method , structural , js_name = createSVGTransformFromMatrix ) ]
    #[doc = "The `createSVGTransformFromMatrix()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/createSVGTransformFromMatrix)\n\n*This API requires the following crate features to be activated: `SvgMatrix`, `SvgTransform`, `SvgTransformList`*"]
    pub fn create_svg_transform_from_matrix(
        this: &SvgTransformList,
        matrix: &SvgMatrix,
    ) -> SvgTransform;
    #[cfg(feature = "SvgTransform")]
    # [ wasm_bindgen ( catch , method , structural , js_name = getItem ) ]
    #[doc = "The `getItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/getItem)\n\n*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    pub fn get_item(this: &SvgTransformList, index: u32) -> Result<SvgTransform, JsValue>;
    #[cfg(feature = "SvgTransform")]
    # [ wasm_bindgen ( catch , method , structural , js_name = initialize ) ]
    #[doc = "The `initialize()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/initialize)\n\n*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    pub fn initialize(
        this: &SvgTransformList,
        new_item: &SvgTransform,
    ) -> Result<SvgTransform, JsValue>;
    #[cfg(feature = "SvgTransform")]
    # [ wasm_bindgen ( catch , method , structural , js_name = insertItemBefore ) ]
    #[doc = "The `insertItemBefore()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/insertItemBefore)\n\n*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    pub fn insert_item_before(
        this: &SvgTransformList,
        new_item: &SvgTransform,
        index: u32,
    ) -> Result<SvgTransform, JsValue>;
    #[cfg(feature = "SvgTransform")]
    # [ wasm_bindgen ( catch , method , structural , js_name = removeItem ) ]
    #[doc = "The `removeItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/removeItem)\n\n*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    pub fn remove_item(this: &SvgTransformList, index: u32) -> Result<SvgTransform, JsValue>;
    #[cfg(feature = "SvgTransform")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceItem ) ]
    #[doc = "The `replaceItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/replaceItem)\n\n*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    pub fn replace_item(
        this: &SvgTransformList,
        new_item: &SvgTransform,
        index: u32,
    ) -> Result<SvgTransform, JsValue>;
    #[cfg(feature = "SvgTransform")]
    #[wasm_bindgen(catch, method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    pub fn get(this: &SvgTransformList, index: u32) -> Result<SvgTransform, JsValue>;
}
