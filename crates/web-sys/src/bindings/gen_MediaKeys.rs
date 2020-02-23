use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaKeys` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys)\n\n*This API requires the following crate features to be activated: `MediaKeys`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaKeys {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaKeys: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaKeys {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(75u32);
            inform(101u32);
            inform(121u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for MediaKeys {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaKeys {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaKeys {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaKeys {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaKeys {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaKeys {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaKeys {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaKeys {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaKeys {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaKeys>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaKeys {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaKeys {
        #[inline]
        fn from(obj: JsValue) -> MediaKeys {
            MediaKeys { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaKeys {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaKeys> for MediaKeys {
        #[inline]
        fn as_ref(&self) -> &MediaKeys {
            self
        }
    }
    impl From<MediaKeys> for JsValue {
        #[inline]
        fn from(obj: MediaKeys) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaKeys {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaKeys(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaKeys(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaKeys(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaKeys { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaKeys) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaKeys> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaKeys) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaKeys {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MediaKeySession", feature = "MediaKeys",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_session_MediaKeys() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaKeys as WasmDescribe>::describe();
    <MediaKeySession as WasmDescribe>::describe();
}
impl MediaKeys {
    #[cfg(all(feature = "MediaKeySession", feature = "MediaKeys",))]
    #[allow(bad_style)]
    #[doc = "The `createSession()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/createSession)\n\n*This API requires the following crate features to be activated: `MediaKeySession`, `MediaKeys`*"]
    #[allow(clippy::all)]
    pub fn create_session(&self) -> Result<MediaKeySession, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaKeySession", feature = "MediaKeys",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_session_MediaKeys(
                self_: <&MediaKeys as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaKeySession as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_session_MediaKeys(
            self_: <&MediaKeys as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaKeySession as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaKeys as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_session_MediaKeys(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaKeySession as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "MediaKeySession",
    feature = "MediaKeySessionType",
    feature = "MediaKeys",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_session_with_session_type_MediaKeys() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaKeys as WasmDescribe>::describe();
    <MediaKeySessionType as WasmDescribe>::describe();
    <MediaKeySession as WasmDescribe>::describe();
}
impl MediaKeys {
    #[cfg(all(
        feature = "MediaKeySession",
        feature = "MediaKeySessionType",
        feature = "MediaKeys",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createSession()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/createSession)\n\n*This API requires the following crate features to be activated: `MediaKeySession`, `MediaKeySessionType`, `MediaKeys`*"]
    #[allow(clippy::all)]
    pub fn create_session_with_session_type(
        &self,
        session_type: MediaKeySessionType,
    ) -> Result<MediaKeySession, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "MediaKeySession",
            feature = "MediaKeySessionType",
            feature = "MediaKeys",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_session_with_session_type_MediaKeys(
                self_: <&MediaKeys as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                session_type: <MediaKeySessionType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaKeySession as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_session_with_session_type_MediaKeys(
            self_: <&MediaKeys as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            session_type: <MediaKeySessionType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaKeySession as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(session_type);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaKeys as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let session_type =
                    <MediaKeySessionType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        session_type,
                    );
                __widl_f_create_session_with_session_type_MediaKeys(self_, session_type)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaKeySession as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MediaKeys",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_status_for_policy_MediaKeys() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaKeys as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MediaKeys {
    #[cfg(all(feature = "MediaKeys",))]
    #[allow(bad_style)]
    #[doc = "The `getStatusForPolicy()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/getStatusForPolicy)\n\n*This API requires the following crate features to be activated: `MediaKeys`*"]
    #[allow(clippy::all)]
    pub fn get_status_for_policy(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "MediaKeys",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_status_for_policy_MediaKeys(
                self_: <&MediaKeys as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_status_for_policy_MediaKeys(
            self_: <&MediaKeys as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaKeys as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_status_for_policy_MediaKeys(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeys", feature = "MediaKeysPolicy",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_status_for_policy_with_policy_MediaKeys() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaKeys as WasmDescribe>::describe();
    <&MediaKeysPolicy as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MediaKeys {
    #[cfg(all(feature = "MediaKeys", feature = "MediaKeysPolicy",))]
    #[allow(bad_style)]
    #[doc = "The `getStatusForPolicy()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/getStatusForPolicy)\n\n*This API requires the following crate features to be activated: `MediaKeys`, `MediaKeysPolicy`*"]
    #[allow(clippy::all)]
    pub fn get_status_for_policy_with_policy(&self, policy: &MediaKeysPolicy) -> ::js_sys::Promise {
        #[cfg(all(feature = "MediaKeys", feature = "MediaKeysPolicy",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_status_for_policy_with_policy_MediaKeys(
                self_: <&MediaKeys as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                policy: <&MediaKeysPolicy as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_status_for_policy_with_policy_MediaKeys(
            self_: <&MediaKeys as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            policy: <&MediaKeysPolicy as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(policy);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaKeys as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let policy =
                    <&MediaKeysPolicy as wasm_bindgen::convert::IntoWasmAbi>::into_abi(policy);
                __widl_f_get_status_for_policy_with_policy_MediaKeys(self_, policy)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeys",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_server_certificate_with_buffer_source_MediaKeys()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaKeys as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MediaKeys {
    #[cfg(all(feature = "MediaKeys",))]
    #[allow(bad_style)]
    #[doc = "The `setServerCertificate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/setServerCertificate)\n\n*This API requires the following crate features to be activated: `MediaKeys`*"]
    #[allow(clippy::all)]
    pub fn set_server_certificate_with_buffer_source(
        &self,
        server_certificate: &::js_sys::Object,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "MediaKeys",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_server_certificate_with_buffer_source_MediaKeys(
                self_: <&MediaKeys as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                server_certificate: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_server_certificate_with_buffer_source_MediaKeys(
            self_: <&MediaKeys as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            server_certificate: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(server_certificate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaKeys as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let server_certificate =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        server_certificate,
                    );
                __widl_f_set_server_certificate_with_buffer_source_MediaKeys(
                    self_,
                    server_certificate,
                )
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeys",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_server_certificate_with_u8_array_MediaKeys() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaKeys as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MediaKeys {
    #[cfg(all(feature = "MediaKeys",))]
    #[allow(bad_style)]
    #[doc = "The `setServerCertificate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/setServerCertificate)\n\n*This API requires the following crate features to be activated: `MediaKeys`*"]
    #[allow(clippy::all)]
    pub fn set_server_certificate_with_u8_array(
        &self,
        server_certificate: &mut [u8],
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "MediaKeys",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_server_certificate_with_u8_array_MediaKeys(
                self_: <&MediaKeys as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                server_certificate: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_server_certificate_with_u8_array_MediaKeys(
            self_: <&MediaKeys as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            server_certificate: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(server_certificate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaKeys as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let server_certificate =
                    <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(server_certificate);
                __widl_f_set_server_certificate_with_u8_array_MediaKeys(self_, server_certificate)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeys",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_key_system_MediaKeys() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaKeys as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MediaKeys {
    #[cfg(all(feature = "MediaKeys",))]
    #[allow(bad_style)]
    #[doc = "The `keySystem` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/keySystem)\n\n*This API requires the following crate features to be activated: `MediaKeys`*"]
    #[allow(clippy::all)]
    pub fn key_system(&self) -> String {
        #[cfg(all(feature = "MediaKeys",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_key_system_MediaKeys(
                self_: <&MediaKeys as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_key_system_MediaKeys(
            self_: <&MediaKeys as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaKeys as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_key_system_MediaKeys(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_f561910a1081eb21: [u8; 856usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x16\x03\0\0\0\0\x08\0\0\x02\tMediaKeys\x1B__widl_instanceof_MediaKeys\0\0\0\0!__widl_f_create_session_MediaKeys\x01\0\0\x01\tMediaKeys\x01\0\0\x01\x01\x05self_\rcreateSession\0\0\03__widl_f_create_session_with_session_type_MediaKeys\x01\0\0\x01\tMediaKeys\x01\0\0\x01\x02\x05self_\x0Csession_type\rcreateSession\0\0\0(__widl_f_get_status_for_policy_MediaKeys\0\0\0\x01\tMediaKeys\x01\0\0\x01\x01\x05self_\x12getStatusForPolicy\0\0\04__widl_f_get_status_for_policy_with_policy_MediaKeys\0\0\0\x01\tMediaKeys\x01\0\0\x01\x02\x05self_\x06policy\x12getStatusForPolicy\0\0\0<__widl_f_set_server_certificate_with_buffer_source_MediaKeys\0\0\0\x01\tMediaKeys\x01\0\0\x01\x02\x05self_\x12server_certificate\x14setServerCertificate\0\0\07__widl_f_set_server_certificate_with_u8_array_MediaKeys\0\0\0\x01\tMediaKeys\x01\0\0\x01\x02\x05self_\x12server_certificate\x14setServerCertificate\0\0\0\x1D__widl_f_key_system_MediaKeys\0\0\0\x01\tMediaKeys\x01\0\x01\tkeySystem\x01\x01\x05self_\tkeySystem\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
