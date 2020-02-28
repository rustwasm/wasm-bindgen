use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PaintRequestList , typescript_name = PaintRequestList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaintRequestList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequestList)\n\n*This API requires the following crate features to be activated: `PaintRequestList`*"]
    pub type PaintRequestList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PaintRequestList" , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequestList/length)\n\n*This API requires the following crate features to be activated: `PaintRequestList`*"]
    pub fn length(this: &PaintRequestList) -> u32;
    #[cfg(feature = "PaintRequest")]
    # [ wasm_bindgen ( method , structural , js_class = "PaintRequestList" , js_name = item ) ]
    #[doc = "The `item()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequestList/item)\n\n*This API requires the following crate features to be activated: `PaintRequest`, `PaintRequestList`*"]
    pub fn item(this: &PaintRequestList, index: u32) -> Option<PaintRequest>;
    #[cfg(feature = "PaintRequest")]
    #[wasm_bindgen(method, structural, js_class = "PaintRequestList", indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `PaintRequest`, `PaintRequestList`*"]
    pub fn get(this: &PaintRequestList, index: u32) -> Option<PaintRequest>;
}
