use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = PositionError , typescript_type = "PositionError" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PositionError` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PositionError)
    ///
    ///*This API requires the following crate features to be activated: `PositionError`*
    pub type PositionError;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PositionError" , js_name = code ) ]
    ///Getter for the `code` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PositionError/code)
    ///
    ///*This API requires the following crate features to be activated: `PositionError`*
    pub fn code(this: &PositionError) -> u16;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PositionError" , js_name = message ) ]
    ///Getter for the `message` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PositionError/message)
    ///
    ///*This API requires the following crate features to be activated: `PositionError`*
    pub fn message(this: &PositionError) -> String;

}

impl PositionError {
    ///The `PositionError.PERMISSION_DENIED` const.
    ///
    ///*This API requires the following crate features to be activated: `PositionError`*

    pub const PERMISSION_DENIED: u16 = 1u64 as u16;

    ///The `PositionError.POSITION_UNAVAILABLE` const.
    ///
    ///*This API requires the following crate features to be activated: `PositionError`*

    pub const POSITION_UNAVAILABLE: u16 = 2u64 as u16;

    ///The `PositionError.TIMEOUT` const.
    ///
    ///*This API requires the following crate features to be activated: `PositionError`*

    pub const TIMEOUT: u16 = 3u64 as u16;
}
