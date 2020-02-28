use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = PositionError , typescript_name = PositionError ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PositionError` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PositionError)\n\n*This API requires the following crate features to be activated: `PositionError`*"]
    pub type PositionError;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PositionError" , js_name = code ) ]
    #[doc = "Getter for the `code` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PositionError/code)\n\n*This API requires the following crate features to be activated: `PositionError`*"]
    pub fn code(this: &PositionError) -> u16;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PositionError" , js_name = message ) ]
    #[doc = "Getter for the `message` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PositionError/message)\n\n*This API requires the following crate features to be activated: `PositionError`*"]
    pub fn message(this: &PositionError) -> String;
}
impl PositionError {
    pub const PERMISSION_DENIED: u16 = 1u64 as u16;
    pub const POSITION_UNAVAILABLE: u16 = 2u64 as u16;
    pub const TIMEOUT: u16 = 3u64 as u16;
}
