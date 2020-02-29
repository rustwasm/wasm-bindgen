use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Permissions , typescript_name = Permissions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Permissions` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Permissions)
    ///
    ///*This API requires the following crate features to be activated: `Permissions`*
    pub type Permissions;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Permissions" , js_name = query ) ]
    ///The `query()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Permissions/query)
    ///
    ///*This API requires the following crate features to be activated: `Permissions`*
    pub fn query(
        this: &Permissions,
        permission: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Permissions" , js_name = revoke ) ]
    ///The `revoke()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Permissions/revoke)
    ///
    ///*This API requires the following crate features to be activated: `Permissions`*
    pub fn revoke(
        this: &Permissions,
        permission: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;

}
