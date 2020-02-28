use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PaintRequest , typescript_name = PaintRequest ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaintRequest` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequest)\n\n*This API requires the following crate features to be activated: `PaintRequest`*"]
    pub type PaintRequest;
    # [ wasm_bindgen ( structural , method , getter , js_name = clientRect ) ]
    #[cfg(feature = "DomRect")]
    #[doc = "Getter for the `clientRect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequest/clientRect)\n\n*This API requires the following crate features to be activated: `DomRect`, `PaintRequest`*"]
    pub fn client_rect(this: &PaintRequest) -> DomRect;
    # [ wasm_bindgen ( structural , method , getter , js_name = reason ) ]
    #[doc = "Getter for the `reason` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequest/reason)\n\n*This API requires the following crate features to be activated: `PaintRequest`*"]
    pub fn reason(this: &PaintRequest) -> String;
}
