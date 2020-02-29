use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PaintRequestList , typescript_type = "PaintRequestList" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PaintRequestList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequestList)
    ///
    ///*This API requires the following crate features to be activated: `PaintRequestList`*
    pub type PaintRequestList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaintRequestList" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequestList/length)
    ///
    ///*This API requires the following crate features to be activated: `PaintRequestList`*
    pub fn length(this: &PaintRequestList) -> u32;

    #[cfg(feature = "PaintRequest")]
    # [ wasm_bindgen ( method , structural , js_class = "PaintRequestList" , js_name = item ) ]
    ///The `item()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequestList/item)
    ///
    ///*This API requires the following crate features to be activated: `PaintRequest`, `PaintRequestList`*
    pub fn item(this: &PaintRequestList, index: u32) -> Option<PaintRequest>;

    #[cfg(feature = "PaintRequest")]
    #[wasm_bindgen(method, structural, js_class = "PaintRequestList", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `PaintRequest`, `PaintRequestList`*
    pub fn get(this: &PaintRequestList, index: u32) -> Option<PaintRequest>;

}
