use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `AuthenticatorAssertionResponse` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse)\n\n*This API requires the following crate features to be activated: `AuthenticatorAssertionResponse`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct AuthenticatorAssertionResponse {
    obj: AuthenticatorResponse,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_AuthenticatorAssertionResponse: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for AuthenticatorAssertionResponse {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(30u32);
            inform(65u32);
            inform(117u32);
            inform(116u32);
            inform(104u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(105u32);
            inform(99u32);
            inform(97u32);
            inform(116u32);
            inform(111u32);
            inform(114u32);
            inform(65u32);
            inform(115u32);
            inform(115u32);
            inform(101u32);
            inform(114u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(82u32);
            inform(101u32);
            inform(115u32);
            inform(112u32);
            inform(111u32);
            inform(110u32);
            inform(115u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for AuthenticatorAssertionResponse {
        type Target = AuthenticatorResponse;
        #[inline]
        fn deref(&self) -> &AuthenticatorResponse {
            &self.obj
        }
    }
    impl IntoWasmAbi for AuthenticatorAssertionResponse {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for AuthenticatorAssertionResponse {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AuthenticatorAssertionResponse {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for AuthenticatorAssertionResponse {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            AuthenticatorAssertionResponse {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for AuthenticatorAssertionResponse {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a AuthenticatorAssertionResponse {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for AuthenticatorAssertionResponse {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<AuthenticatorAssertionResponse>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(AuthenticatorAssertionResponse {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for AuthenticatorAssertionResponse {
        #[inline]
        fn from(obj: JsValue) -> AuthenticatorAssertionResponse {
            AuthenticatorAssertionResponse { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for AuthenticatorAssertionResponse {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<AuthenticatorAssertionResponse> for AuthenticatorAssertionResponse {
        #[inline]
        fn as_ref(&self) -> &AuthenticatorAssertionResponse {
            self
        }
    }
    impl From<AuthenticatorAssertionResponse> for JsValue {
        #[inline]
        fn from(obj: AuthenticatorAssertionResponse) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for AuthenticatorAssertionResponse {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_AuthenticatorAssertionResponse(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_AuthenticatorAssertionResponse(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_AuthenticatorAssertionResponse(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AuthenticatorAssertionResponse { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AuthenticatorAssertionResponse) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<AuthenticatorAssertionResponse> for AuthenticatorResponse {
    #[inline]
    fn from(obj: AuthenticatorAssertionResponse) -> AuthenticatorResponse {
        use wasm_bindgen::JsCast;
        AuthenticatorResponse::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AuthenticatorResponse> for AuthenticatorAssertionResponse {
    #[inline]
    fn as_ref(&self) -> &AuthenticatorResponse {
        use wasm_bindgen::JsCast;
        AuthenticatorResponse::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<AuthenticatorAssertionResponse> for ::js_sys::Object {
    #[inline]
    fn from(obj: AuthenticatorAssertionResponse) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for AuthenticatorAssertionResponse {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "AuthenticatorAssertionResponse",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_authenticator_data_AuthenticatorAssertionResponse() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AuthenticatorAssertionResponse as WasmDescribe>::describe();
    <::js_sys::ArrayBuffer as WasmDescribe>::describe();
}
impl AuthenticatorAssertionResponse {
    #[cfg(all(feature = "AuthenticatorAssertionResponse",))]
    #[allow(bad_style)]
    #[doc = "The `authenticatorData` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse/authenticatorData)\n\n*This API requires the following crate features to be activated: `AuthenticatorAssertionResponse`*"]
    #[allow(clippy::all)]
    pub fn authenticator_data(&self) -> ::js_sys::ArrayBuffer {
        #[cfg(all(feature = "AuthenticatorAssertionResponse",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_authenticator_data_AuthenticatorAssertionResponse(
                self_: <&AuthenticatorAssertionResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::ArrayBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_authenticator_data_AuthenticatorAssertionResponse(
            self_: <&AuthenticatorAssertionResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::ArrayBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = < & AuthenticatorAssertionResponse as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( self ) ;
                __widl_f_authenticator_data_AuthenticatorAssertionResponse(self_)
            };
            <::js_sys::ArrayBuffer as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AuthenticatorAssertionResponse",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_signature_AuthenticatorAssertionResponse() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AuthenticatorAssertionResponse as WasmDescribe>::describe();
    <::js_sys::ArrayBuffer as WasmDescribe>::describe();
}
impl AuthenticatorAssertionResponse {
    #[cfg(all(feature = "AuthenticatorAssertionResponse",))]
    #[allow(bad_style)]
    #[doc = "The `signature` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse/signature)\n\n*This API requires the following crate features to be activated: `AuthenticatorAssertionResponse`*"]
    #[allow(clippy::all)]
    pub fn signature(&self) -> ::js_sys::ArrayBuffer {
        #[cfg(all(feature = "AuthenticatorAssertionResponse",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_signature_AuthenticatorAssertionResponse(
                self_: <&AuthenticatorAssertionResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::ArrayBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_signature_AuthenticatorAssertionResponse(
            self_: <&AuthenticatorAssertionResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::ArrayBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = < & AuthenticatorAssertionResponse as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( self ) ;
                __widl_f_signature_AuthenticatorAssertionResponse(self_)
            };
            <::js_sys::ArrayBuffer as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AuthenticatorAssertionResponse",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_user_handle_AuthenticatorAssertionResponse() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AuthenticatorAssertionResponse as WasmDescribe>::describe();
    <Option<::js_sys::ArrayBuffer> as WasmDescribe>::describe();
}
impl AuthenticatorAssertionResponse {
    #[cfg(all(feature = "AuthenticatorAssertionResponse",))]
    #[allow(bad_style)]
    #[doc = "The `userHandle` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse/userHandle)\n\n*This API requires the following crate features to be activated: `AuthenticatorAssertionResponse`*"]
    #[allow(clippy::all)]
    pub fn user_handle(&self) -> Option<::js_sys::ArrayBuffer> {
        #[cfg(all(feature = "AuthenticatorAssertionResponse",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_user_handle_AuthenticatorAssertionResponse(
                self_: <&AuthenticatorAssertionResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::ArrayBuffer> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_user_handle_AuthenticatorAssertionResponse(
            self_: <&AuthenticatorAssertionResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::ArrayBuffer> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = < & AuthenticatorAssertionResponse as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( self ) ;
                __widl_f_user_handle_AuthenticatorAssertionResponse(self_)
            };
            <Option<::js_sys::ArrayBuffer> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_b24c5d9d254bbcda: [u8; 575usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xFD\x01\0\0\0\0\x04\0\0\x02\x1EAuthenticatorAssertionResponse0__widl_instanceof_AuthenticatorAssertionResponse\0\0\0\0:__widl_f_authenticator_data_AuthenticatorAssertionResponse\0\0\0\x01\x1EAuthenticatorAssertionResponse\x01\0\x01\x11authenticatorData\x01\x01\x05self_\x11authenticatorData\0\0\01__widl_f_signature_AuthenticatorAssertionResponse\0\0\0\x01\x1EAuthenticatorAssertionResponse\x01\0\x01\tsignature\x01\x01\x05self_\tsignature\0\0\03__widl_f_user_handle_AuthenticatorAssertionResponse\0\0\0\x01\x1EAuthenticatorAssertionResponse\x01\0\x01\nuserHandle\x01\x01\x05self_\nuserHandle\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
