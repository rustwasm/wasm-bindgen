use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Permissions , typescript_name = Permissions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Permissions` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Permissions)\n\n*This API requires the following crate features to be activated: `Permissions`*"]
    pub type Permissions;
    # [ wasm_bindgen ( catch , method , structural , js_name = query ) ]
    #[doc = "The `query()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Permissions/query)\n\n*This API requires the following crate features to be activated: `Permissions`*"]
    pub fn query(
        this: &Permissions,
        permission: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = revoke ) ]
    #[doc = "The `revoke()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Permissions/revoke)\n\n*This API requires the following crate features to be activated: `Permissions`*"]
    pub fn revoke(
        this: &Permissions,
        permission: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
}
