use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = U2F , typescript_name = U2F ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `U2f` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/U2F)
    ///
    ///*This API requires the following crate features to be activated: `U2f`*
    pub type U2f;

    # [ wasm_bindgen ( catch , method , structural , js_class = "U2F" , js_name = register ) ]
    ///The `register()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/U2F/register)
    ///
    ///*This API requires the following crate features to be activated: `U2f`*
    pub fn register(
        this: &U2f,
        app_id: &str,
        register_requests: &::wasm_bindgen::JsValue,
        registered_keys: &::wasm_bindgen::JsValue,
        callback: &::js_sys::Function,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "U2F" , js_name = register ) ]
    ///The `register()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/U2F/register)
    ///
    ///*This API requires the following crate features to be activated: `U2f`*
    pub fn register_with_opt_timeout_seconds(
        this: &U2f,
        app_id: &str,
        register_requests: &::wasm_bindgen::JsValue,
        registered_keys: &::wasm_bindgen::JsValue,
        callback: &::js_sys::Function,
        opt_timeout_seconds: Option<i32>,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "U2F" , js_name = sign ) ]
    ///The `sign()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/U2F/sign)
    ///
    ///*This API requires the following crate features to be activated: `U2f`*
    pub fn sign(
        this: &U2f,
        app_id: &str,
        challenge: &str,
        registered_keys: &::wasm_bindgen::JsValue,
        callback: &::js_sys::Function,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "U2F" , js_name = sign ) ]
    ///The `sign()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/U2F/sign)
    ///
    ///*This API requires the following crate features to be activated: `U2f`*
    pub fn sign_with_opt_timeout_seconds(
        this: &U2f,
        app_id: &str,
        challenge: &str,
        registered_keys: &::wasm_bindgen::JsValue,
        callback: &::js_sys::Function,
        opt_timeout_seconds: Option<i32>,
    ) -> Result<(), JsValue>;

}

impl U2f {
    ///The `U2F.OK` const.
    ///
    ///*This API requires the following crate features to be activated: `U2f`*

    pub const OK: u16 = 0i64 as u16;

    ///The `U2F.OTHER_ERROR` const.
    ///
    ///*This API requires the following crate features to be activated: `U2f`*

    pub const OTHER_ERROR: u16 = 1u64 as u16;

    ///The `U2F.BAD_REQUEST` const.
    ///
    ///*This API requires the following crate features to be activated: `U2f`*

    pub const BAD_REQUEST: u16 = 2u64 as u16;

    ///The `U2F.CONFIGURATION_UNSUPPORTED` const.
    ///
    ///*This API requires the following crate features to be activated: `U2f`*

    pub const CONFIGURATION_UNSUPPORTED: u16 = 3u64 as u16;

    ///The `U2F.DEVICE_INELIGIBLE` const.
    ///
    ///*This API requires the following crate features to be activated: `U2f`*

    pub const DEVICE_INELIGIBLE: u16 = 4u64 as u16;

    ///The `U2F.TIMEOUT` const.
    ///
    ///*This API requires the following crate features to be activated: `U2f`*

    pub const TIMEOUT: u16 = 5u64 as u16;
}
