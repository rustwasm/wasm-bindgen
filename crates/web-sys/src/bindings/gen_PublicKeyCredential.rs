use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PublicKeyCredential` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential)\n\n*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PublicKeyCredential {
    obj: Credential,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PublicKeyCredential: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PublicKeyCredential {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(80u32);
            inform(117u32);
            inform(98u32);
            inform(108u32);
            inform(105u32);
            inform(99u32);
            inform(75u32);
            inform(101u32);
            inform(121u32);
            inform(67u32);
            inform(114u32);
            inform(101u32);
            inform(100u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(105u32);
            inform(97u32);
            inform(108u32);
        }
    }
    impl core::ops::Deref for PublicKeyCredential {
        type Target = Credential;
        #[inline]
        fn deref(&self) -> &Credential {
            &self.obj
        }
    }
    impl IntoWasmAbi for PublicKeyCredential {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PublicKeyCredential {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PublicKeyCredential {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PublicKeyCredential {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PublicKeyCredential {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PublicKeyCredential {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PublicKeyCredential {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PublicKeyCredential {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PublicKeyCredential>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PublicKeyCredential {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PublicKeyCredential {
        #[inline]
        fn from(obj: JsValue) -> PublicKeyCredential {
            PublicKeyCredential { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PublicKeyCredential {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PublicKeyCredential> for PublicKeyCredential {
        #[inline]
        fn as_ref(&self) -> &PublicKeyCredential {
            self
        }
    }
    impl From<PublicKeyCredential> for JsValue {
        #[inline]
        fn from(obj: PublicKeyCredential) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PublicKeyCredential {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PublicKeyCredential(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PublicKeyCredential(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PublicKeyCredential(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PublicKeyCredential { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PublicKeyCredential) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PublicKeyCredential> for Credential {
    #[inline]
    fn from(obj: PublicKeyCredential) -> Credential {
        use wasm_bindgen::JsCast;
        Credential::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Credential> for PublicKeyCredential {
    #[inline]
    fn as_ref(&self) -> &Credential {
        use wasm_bindgen::JsCast;
        Credential::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PublicKeyCredential> for ::js_sys::Object {
    #[inline]
    fn from(obj: PublicKeyCredential) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PublicKeyCredential {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(
    feature = "AuthenticationExtensionsClientOutputs",
    feature = "PublicKeyCredential",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_client_extension_results_PublicKeyCredential() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PublicKeyCredential as WasmDescribe>::describe();
    <AuthenticationExtensionsClientOutputs as WasmDescribe>::describe();
}
impl PublicKeyCredential {
    #[cfg(all(
        feature = "AuthenticationExtensionsClientOutputs",
        feature = "PublicKeyCredential",
    ))]
    #[allow(bad_style)]
    #[doc = "The `getClientExtensionResults()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/getClientExtensionResults)\n\n*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `PublicKeyCredential`*"]
    #[allow(clippy::all)]
    pub fn get_client_extension_results(&self) -> AuthenticationExtensionsClientOutputs {
        #[cfg(all(
            feature = "AuthenticationExtensionsClientOutputs",
            feature = "PublicKeyCredential",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_client_extension_results_PublicKeyCredential(
                self_: <&PublicKeyCredential as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AuthenticationExtensionsClientOutputs as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_client_extension_results_PublicKeyCredential(
            self_: <&PublicKeyCredential as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AuthenticationExtensionsClientOutputs as wasm_bindgen::convert::FromWasmAbi>::Abi
        {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PublicKeyCredential as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_client_extension_results_PublicKeyCredential(self_)
            };
            <AuthenticationExtensionsClientOutputs as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            )
        }
    }
}
#[cfg(all(feature = "PublicKeyCredential",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_user_verifying_platform_authenticator_available_PublicKeyCredential(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl PublicKeyCredential {
    #[cfg(all(feature = "PublicKeyCredential",))]
    #[allow(bad_style)]
    #[doc = "The `isUserVerifyingPlatformAuthenticatorAvailable()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/isUserVerifyingPlatformAuthenticatorAvailable)\n\n*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
    #[allow(clippy::all)]
    pub fn is_user_verifying_platform_authenticator_available() -> ::js_sys::Promise {
        #[cfg(all(feature = "PublicKeyCredential",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_user_verifying_platform_authenticator_available_PublicKeyCredential(
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_user_verifying_platform_authenticator_available_PublicKeyCredential(
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                __widl_f_is_user_verifying_platform_authenticator_available_PublicKeyCredential()
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PublicKeyCredential",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_raw_id_PublicKeyCredential() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PublicKeyCredential as WasmDescribe>::describe();
    <::js_sys::ArrayBuffer as WasmDescribe>::describe();
}
impl PublicKeyCredential {
    #[cfg(all(feature = "PublicKeyCredential",))]
    #[allow(bad_style)]
    #[doc = "The `rawId` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/rawId)\n\n*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
    #[allow(clippy::all)]
    pub fn raw_id(&self) -> ::js_sys::ArrayBuffer {
        #[cfg(all(feature = "PublicKeyCredential",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_raw_id_PublicKeyCredential(
                self_: <&PublicKeyCredential as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::ArrayBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_raw_id_PublicKeyCredential(
            self_: <&PublicKeyCredential as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::ArrayBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PublicKeyCredential as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_raw_id_PublicKeyCredential(self_)
            };
            <::js_sys::ArrayBuffer as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AuthenticatorResponse", feature = "PublicKeyCredential",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_response_PublicKeyCredential() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PublicKeyCredential as WasmDescribe>::describe();
    <AuthenticatorResponse as WasmDescribe>::describe();
}
impl PublicKeyCredential {
    #[cfg(all(feature = "AuthenticatorResponse", feature = "PublicKeyCredential",))]
    #[allow(bad_style)]
    #[doc = "The `response` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/response)\n\n*This API requires the following crate features to be activated: `AuthenticatorResponse`, `PublicKeyCredential`*"]
    #[allow(clippy::all)]
    pub fn response(&self) -> AuthenticatorResponse {
        #[cfg(all(feature = "AuthenticatorResponse", feature = "PublicKeyCredential",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_response_PublicKeyCredential(
                self_: <&PublicKeyCredential as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AuthenticatorResponse as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_response_PublicKeyCredential(
            self_: <&PublicKeyCredential as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AuthenticatorResponse as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PublicKeyCredential as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_response_PublicKeyCredential(self_)
            };
            <AuthenticatorResponse as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_2341d04f15a2a752: [u8; 627usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}1\x02\0\0\0\0\x05\0\0\x02\x13PublicKeyCredential%__widl_instanceof_PublicKeyCredential\0\0\0\09__widl_f_get_client_extension_results_PublicKeyCredential\0\0\0\x01\x13PublicKeyCredential\x01\0\0\x01\x01\x05self_\x19getClientExtensionResults\0\0\0O__widl_f_is_user_verifying_platform_authenticator_available_PublicKeyCredential\0\0\0\x01\x13PublicKeyCredential\x01\x01\0\x01\0-isUserVerifyingPlatformAuthenticatorAvailable\0\0\0#__widl_f_raw_id_PublicKeyCredential\0\0\0\x01\x13PublicKeyCredential\x01\0\x01\x05rawId\x01\x01\x05self_\x05rawId\0\0\0%__widl_f_response_PublicKeyCredential\0\0\0\x01\x13PublicKeyCredential\x01\0\x01\x08response\x01\x01\x05self_\x08response\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
