use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `GetUserMediaRequest` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest)\n\n*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct GetUserMediaRequest {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_GetUserMediaRequest: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for GetUserMediaRequest {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(71u32);
            inform(101u32);
            inform(116u32);
            inform(85u32);
            inform(115u32);
            inform(101u32);
            inform(114u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(82u32);
            inform(101u32);
            inform(113u32);
            inform(117u32);
            inform(101u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for GetUserMediaRequest {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for GetUserMediaRequest {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for GetUserMediaRequest {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a GetUserMediaRequest {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for GetUserMediaRequest {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            GetUserMediaRequest {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for GetUserMediaRequest {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a GetUserMediaRequest {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for GetUserMediaRequest {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<GetUserMediaRequest>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(GetUserMediaRequest {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for GetUserMediaRequest {
        #[inline]
        fn from(obj: JsValue) -> GetUserMediaRequest {
            GetUserMediaRequest { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for GetUserMediaRequest {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<GetUserMediaRequest> for GetUserMediaRequest {
        #[inline]
        fn as_ref(&self) -> &GetUserMediaRequest {
            self
        }
    }
    impl From<GetUserMediaRequest> for JsValue {
        #[inline]
        fn from(obj: GetUserMediaRequest) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for GetUserMediaRequest {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_GetUserMediaRequest(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_GetUserMediaRequest(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_GetUserMediaRequest(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            GetUserMediaRequest { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const GetUserMediaRequest) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<GetUserMediaRequest> for ::js_sys::Object {
    #[inline]
    fn from(obj: GetUserMediaRequest) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for GetUserMediaRequest {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "GetUserMediaRequest", feature = "MediaStreamConstraints",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_constraints_GetUserMediaRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GetUserMediaRequest as WasmDescribe>::describe();
    <MediaStreamConstraints as WasmDescribe>::describe();
}
impl GetUserMediaRequest {
    #[cfg(all(feature = "GetUserMediaRequest", feature = "MediaStreamConstraints",))]
    #[allow(bad_style)]
    #[doc = "The `getConstraints()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/getConstraints)\n\n*This API requires the following crate features to be activated: `GetUserMediaRequest`, `MediaStreamConstraints`*"]
    #[allow(clippy::all)]
    pub fn get_constraints(&self) -> MediaStreamConstraints {
        #[cfg(all(feature = "GetUserMediaRequest", feature = "MediaStreamConstraints",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_constraints_GetUserMediaRequest(
                self_: <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaStreamConstraints as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_constraints_GetUserMediaRequest(
            self_: <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaStreamConstraints as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_constraints_GetUserMediaRequest(self_)
            };
            <MediaStreamConstraints as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "GetUserMediaRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_window_id_GetUserMediaRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GetUserMediaRequest as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl GetUserMediaRequest {
    #[cfg(all(feature = "GetUserMediaRequest",))]
    #[allow(bad_style)]
    #[doc = "The `windowID` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/windowID)\n\n*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
    #[allow(clippy::all)]
    pub fn window_id(&self) -> f64 {
        #[cfg(all(feature = "GetUserMediaRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_window_id_GetUserMediaRequest(
                self_: <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_window_id_GetUserMediaRequest(
            self_: <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_window_id_GetUserMediaRequest(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "GetUserMediaRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_inner_window_id_GetUserMediaRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GetUserMediaRequest as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl GetUserMediaRequest {
    #[cfg(all(feature = "GetUserMediaRequest",))]
    #[allow(bad_style)]
    #[doc = "The `innerWindowID` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/innerWindowID)\n\n*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
    #[allow(clippy::all)]
    pub fn inner_window_id(&self) -> f64 {
        #[cfg(all(feature = "GetUserMediaRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_inner_window_id_GetUserMediaRequest(
                self_: <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_inner_window_id_GetUserMediaRequest(
            self_: <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_inner_window_id_GetUserMediaRequest(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "GetUserMediaRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_call_id_GetUserMediaRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GetUserMediaRequest as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl GetUserMediaRequest {
    #[cfg(all(feature = "GetUserMediaRequest",))]
    #[allow(bad_style)]
    #[doc = "The `callID` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/callID)\n\n*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
    #[allow(clippy::all)]
    pub fn call_id(&self) -> String {
        #[cfg(all(feature = "GetUserMediaRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_call_id_GetUserMediaRequest(
                self_: <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_call_id_GetUserMediaRequest(
            self_: <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_call_id_GetUserMediaRequest(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "GetUserMediaRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_raw_id_GetUserMediaRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GetUserMediaRequest as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl GetUserMediaRequest {
    #[cfg(all(feature = "GetUserMediaRequest",))]
    #[allow(bad_style)]
    #[doc = "The `rawID` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/rawID)\n\n*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
    #[allow(clippy::all)]
    pub fn raw_id(&self) -> String {
        #[cfg(all(feature = "GetUserMediaRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_raw_id_GetUserMediaRequest(
                self_: <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_raw_id_GetUserMediaRequest(
            self_: <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_raw_id_GetUserMediaRequest(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "GetUserMediaRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_media_source_GetUserMediaRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GetUserMediaRequest as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl GetUserMediaRequest {
    #[cfg(all(feature = "GetUserMediaRequest",))]
    #[allow(bad_style)]
    #[doc = "The `mediaSource` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/mediaSource)\n\n*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
    #[allow(clippy::all)]
    pub fn media_source(&self) -> String {
        #[cfg(all(feature = "GetUserMediaRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_media_source_GetUserMediaRequest(
                self_: <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_media_source_GetUserMediaRequest(
            self_: <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_media_source_GetUserMediaRequest(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "GetUserMediaRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_secure_GetUserMediaRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GetUserMediaRequest as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl GetUserMediaRequest {
    #[cfg(all(feature = "GetUserMediaRequest",))]
    #[allow(bad_style)]
    #[doc = "The `isSecure` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/isSecure)\n\n*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
    #[allow(clippy::all)]
    pub fn is_secure(&self) -> bool {
        #[cfg(all(feature = "GetUserMediaRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_secure_GetUserMediaRequest(
                self_: <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_secure_GetUserMediaRequest(
            self_: <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_is_secure_GetUserMediaRequest(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "GetUserMediaRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_handling_user_input_GetUserMediaRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GetUserMediaRequest as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl GetUserMediaRequest {
    #[cfg(all(feature = "GetUserMediaRequest",))]
    #[allow(bad_style)]
    #[doc = "The `isHandlingUserInput` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/isHandlingUserInput)\n\n*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
    #[allow(clippy::all)]
    pub fn is_handling_user_input(&self) -> bool {
        #[cfg(all(feature = "GetUserMediaRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_handling_user_input_GetUserMediaRequest(
                self_: <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_handling_user_input_GetUserMediaRequest(
            self_: <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GetUserMediaRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_is_handling_user_input_GetUserMediaRequest(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_bdc3803a7eb8eb7c: [u8; 975usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x8D\x03\0\0\0\0\t\0\0\x02\x13GetUserMediaRequest%__widl_instanceof_GetUserMediaRequest\0\0\0\0,__widl_f_get_constraints_GetUserMediaRequest\0\0\0\x01\x13GetUserMediaRequest\x01\0\0\x01\x01\x05self_\x0EgetConstraints\0\0\0&__widl_f_window_id_GetUserMediaRequest\0\0\0\x01\x13GetUserMediaRequest\x01\0\x01\x08windowID\x01\x01\x05self_\x08windowID\0\0\0,__widl_f_inner_window_id_GetUserMediaRequest\0\0\0\x01\x13GetUserMediaRequest\x01\0\x01\rinnerWindowID\x01\x01\x05self_\rinnerWindowID\0\0\0$__widl_f_call_id_GetUserMediaRequest\0\0\0\x01\x13GetUserMediaRequest\x01\0\x01\x06callID\x01\x01\x05self_\x06callID\0\0\0#__widl_f_raw_id_GetUserMediaRequest\0\0\0\x01\x13GetUserMediaRequest\x01\0\x01\x05rawID\x01\x01\x05self_\x05rawID\0\0\0)__widl_f_media_source_GetUserMediaRequest\0\0\0\x01\x13GetUserMediaRequest\x01\0\x01\x0BmediaSource\x01\x01\x05self_\x0BmediaSource\0\0\0&__widl_f_is_secure_GetUserMediaRequest\0\0\0\x01\x13GetUserMediaRequest\x01\0\x01\x08isSecure\x01\x01\x05self_\x08isSecure\0\0\03__widl_f_is_handling_user_input_GetUserMediaRequest\0\0\0\x01\x13GetUserMediaRequest\x01\0\x01\x13isHandlingUserInput\x01\x01\x05self_\x13isHandlingUserInput\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
