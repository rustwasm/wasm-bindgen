use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `AuthenticatorResponse` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorResponse)\n\n*This API requires the following crate features to be activated: `AuthenticatorResponse`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct AuthenticatorResponse {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_AuthenticatorResponse: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for AuthenticatorResponse {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(21u32);
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
    impl core::ops::Deref for AuthenticatorResponse {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for AuthenticatorResponse {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for AuthenticatorResponse {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AuthenticatorResponse {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for AuthenticatorResponse {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            AuthenticatorResponse {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for AuthenticatorResponse {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a AuthenticatorResponse {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for AuthenticatorResponse {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<AuthenticatorResponse>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(AuthenticatorResponse {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for AuthenticatorResponse {
        #[inline]
        fn from(obj: JsValue) -> AuthenticatorResponse {
            AuthenticatorResponse { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for AuthenticatorResponse {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<AuthenticatorResponse> for AuthenticatorResponse {
        #[inline]
        fn as_ref(&self) -> &AuthenticatorResponse {
            self
        }
    }
    impl From<AuthenticatorResponse> for JsValue {
        #[inline]
        fn from(obj: AuthenticatorResponse) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for AuthenticatorResponse {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_AuthenticatorResponse(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_AuthenticatorResponse(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_AuthenticatorResponse(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AuthenticatorResponse { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AuthenticatorResponse) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<AuthenticatorResponse> for ::js_sys::Object {
    #[inline]
    fn from(obj: AuthenticatorResponse) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for AuthenticatorResponse {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "AuthenticatorResponse",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_client_data_json_AuthenticatorResponse() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AuthenticatorResponse as WasmDescribe>::describe();
    <::js_sys::ArrayBuffer as WasmDescribe>::describe();
}
impl AuthenticatorResponse {
    #[cfg(all(feature = "AuthenticatorResponse",))]
    #[allow(bad_style)]
    #[doc = "The `clientDataJSON` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorResponse/clientDataJSON)\n\n*This API requires the following crate features to be activated: `AuthenticatorResponse`*"]
    #[allow(clippy::all)]
    pub fn client_data_json(&self) -> ::js_sys::ArrayBuffer {
        #[cfg(all(feature = "AuthenticatorResponse",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_client_data_json_AuthenticatorResponse(
                self_: <&AuthenticatorResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::ArrayBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_client_data_json_AuthenticatorResponse(
            self_: <&AuthenticatorResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&AuthenticatorResponse as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_client_data_json_AuthenticatorResponse(self_)
            };
            <::js_sys::ArrayBuffer as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_203c052573d655a6: [u8; 289usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xDF\0\0\0\0\0\x02\0\0\x02\x15AuthenticatorResponse'__widl_instanceof_AuthenticatorResponse\0\0\0\0/__widl_f_client_data_json_AuthenticatorResponse\0\0\0\x01\x15AuthenticatorResponse\x01\0\x01\x0EclientDataJSON\x01\x01\x05self_\x0EclientDataJSON\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
