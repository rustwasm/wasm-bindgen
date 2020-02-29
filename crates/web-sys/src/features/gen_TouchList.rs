use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = TouchList , typescript_type = "TouchList" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `TouchList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchList)
    ///
    ///*This API requires the following crate features to be activated: `TouchList`*
    pub type TouchList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TouchList" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchList/length)
    ///
    ///*This API requires the following crate features to be activated: `TouchList`*
    pub fn length(this: &TouchList) -> u32;

    #[cfg(feature = "Touch")]
    # [ wasm_bindgen ( method , structural , js_class = "TouchList" , js_name = item ) ]
    ///The `item()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchList/item)
    ///
    ///*This API requires the following crate features to be activated: `Touch`, `TouchList`*
    pub fn item(this: &TouchList, index: u32) -> Option<Touch>;

    #[cfg(feature = "Touch")]
    #[wasm_bindgen(method, structural, js_class = "TouchList", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `Touch`, `TouchList`*
    pub fn get(this: &TouchList, index: u32) -> Option<Touch>;

}
