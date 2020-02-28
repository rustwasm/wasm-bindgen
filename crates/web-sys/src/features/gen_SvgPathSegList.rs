use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGPathSegList , typescript_name = SVGPathSegList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegList)\n\n*This API requires the following crate features to be activated: `SvgPathSegList`*"]
    pub type SvgPathSegList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegList" , js_name = numberOfItems ) ]
    #[doc = "Getter for the `numberOfItems` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegList/numberOfItems)\n\n*This API requires the following crate features to be activated: `SvgPathSegList`*"]
    pub fn number_of_items(this: &SvgPathSegList) -> u32;
    #[cfg(feature = "SvgPathSeg")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGPathSegList" , js_name = getItem ) ]
    #[doc = "The `getItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegList/getItem)\n\n*This API requires the following crate features to be activated: `SvgPathSeg`, `SvgPathSegList`*"]
    pub fn get_item(this: &SvgPathSegList, index: u32) -> Result<SvgPathSeg, JsValue>;
    #[cfg(feature = "SvgPathSeg")]
    #[wasm_bindgen(
        catch,
        method,
        structural,
        js_class = "SVGPathSegList",
        indexing_getter
    )]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `SvgPathSeg`, `SvgPathSegList`*"]
    pub fn get(this: &SvgPathSegList, index: u32) -> Result<SvgPathSeg, JsValue>;
}
