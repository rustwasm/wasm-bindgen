use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PaintRequest , typescript_type = "PaintRequest" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PaintRequest` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequest)
    ///
    ///*This API requires the following crate features to be activated: `PaintRequest`*
    pub type PaintRequest;

    #[cfg(feature = "DomRect")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "PaintRequest" , js_name = clientRect ) ]
    ///Getter for the `clientRect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequest/clientRect)
    ///
    ///*This API requires the following crate features to be activated: `DomRect`, `PaintRequest`*
    pub fn client_rect(this: &PaintRequest) -> DomRect;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaintRequest" , js_name = reason ) ]
    ///Getter for the `reason` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequest/reason)
    ///
    ///*This API requires the following crate features to be activated: `PaintRequest`*
    pub fn reason(this: &PaintRequest) -> String;

}
