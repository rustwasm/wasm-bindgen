use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGPathSegList , typescript_name = SVGPathSegList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgPathSegList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegList)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegList`*
    pub type SvgPathSegList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegList" , js_name = numberOfItems ) ]
    ///Getter for the `numberOfItems` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegList/numberOfItems)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegList`*
    pub fn number_of_items(this: &SvgPathSegList) -> u32;

    #[cfg(feature = "SvgPathSeg")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGPathSegList" , js_name = getItem ) ]
    ///The `getItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegList/getItem)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`, `SvgPathSegList`*
    pub fn get_item(this: &SvgPathSegList, index: u32) -> Result<SvgPathSeg, JsValue>;

    #[cfg(feature = "SvgPathSeg")]
    #[wasm_bindgen(
        catch,
        method,
        structural,
        js_class = "SVGPathSegList",
        indexing_getter
    )]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSeg`, `SvgPathSegList`*
    pub fn get(this: &SvgPathSegList, index: u32) -> Result<SvgPathSeg, JsValue>;

}
