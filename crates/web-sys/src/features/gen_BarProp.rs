use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = BarProp , typescript_name = BarProp ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `BarProp` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BarProp)
    ///
    ///*This API requires the following crate features to be activated: `BarProp`*
    pub type BarProp;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "BarProp" , js_name = visible ) ]
    ///Getter for the `visible` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BarProp/visible)
    ///
    ///*This API requires the following crate features to be activated: `BarProp`*
    pub fn visible(this: &BarProp) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "BarProp" , js_name = visible ) ]
    ///Setter for the `visible` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BarProp/visible)
    ///
    ///*This API requires the following crate features to be activated: `BarProp`*
    pub fn set_visible(this: &BarProp, value: bool) -> Result<(), JsValue>;

}
