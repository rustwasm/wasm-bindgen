#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Permissions , typescript_type = "Permissions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Permissions` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Permissions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Permissions`*"]
    pub type Permissions;
    # [wasm_bindgen (catch , method , structural , js_class = "Permissions" , js_name = query)]
    #[doc = "The `query()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Permissions/query)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Permissions`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise of the successful result can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[PermissionStatus]</code>. It can be converted like `<code>let result: [PermissionStatus] = result?.await.into();</code>."]
    pub fn query(
        this: &Permissions,
        permission: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "Permissions" , js_name = revoke)]
    #[doc = "The `revoke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Permissions/revoke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Permissions`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise of the successful result can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[PermissionStatus]</code>. It can be converted like `<code>let result: [PermissionStatus] = result?.await.into();</code>."]
    pub fn revoke(
        this: &Permissions,
        permission: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
}
