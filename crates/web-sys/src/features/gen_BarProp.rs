use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = BarProp , typescript_name = BarProp ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BarProp` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BarProp)\n\n*This API requires the following crate features to be activated: `BarProp`*"]
    pub type BarProp;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = visible ) ]
    #[doc = "Getter for the `visible` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BarProp/visible)\n\n*This API requires the following crate features to be activated: `BarProp`*"]
    pub fn visible(this: &BarProp) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = visible ) ]
    #[doc = "Setter for the `visible` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BarProp/visible)\n\n*This API requires the following crate features to be activated: `BarProp`*"]
    pub fn set_visible(this: &BarProp, value: bool) -> Result<(), JsValue>;
}
