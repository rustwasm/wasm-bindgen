use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CheckerboardReportService` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService)\n\n*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CheckerboardReportService {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CheckerboardReportService: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CheckerboardReportService {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(25u32);
            inform(67u32);
            inform(104u32);
            inform(101u32);
            inform(99u32);
            inform(107u32);
            inform(101u32);
            inform(114u32);
            inform(98u32);
            inform(111u32);
            inform(97u32);
            inform(114u32);
            inform(100u32);
            inform(82u32);
            inform(101u32);
            inform(112u32);
            inform(111u32);
            inform(114u32);
            inform(116u32);
            inform(83u32);
            inform(101u32);
            inform(114u32);
            inform(118u32);
            inform(105u32);
            inform(99u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for CheckerboardReportService {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for CheckerboardReportService {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CheckerboardReportService {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CheckerboardReportService {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CheckerboardReportService {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CheckerboardReportService {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CheckerboardReportService {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CheckerboardReportService {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CheckerboardReportService {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CheckerboardReportService>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CheckerboardReportService {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CheckerboardReportService {
        #[inline]
        fn from(obj: JsValue) -> CheckerboardReportService {
            CheckerboardReportService { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CheckerboardReportService {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CheckerboardReportService> for CheckerboardReportService {
        #[inline]
        fn as_ref(&self) -> &CheckerboardReportService {
            self
        }
    }
    impl From<CheckerboardReportService> for JsValue {
        #[inline]
        fn from(obj: CheckerboardReportService) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CheckerboardReportService {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CheckerboardReportService(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CheckerboardReportService(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CheckerboardReportService(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CheckerboardReportService { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CheckerboardReportService) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CheckerboardReportService> for ::js_sys::Object {
    #[inline]
    fn from(obj: CheckerboardReportService) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CheckerboardReportService {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CheckerboardReportService",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_CheckerboardReportService() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <CheckerboardReportService as WasmDescribe>::describe();
}
impl CheckerboardReportService {
    #[cfg(all(feature = "CheckerboardReportService",))]
    #[allow(bad_style)]
    #[doc = "The `new CheckerboardReportService(..)` constructor, creating a new instance of `CheckerboardReportService`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/CheckerboardReportService)\n\n*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<CheckerboardReportService, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CheckerboardReportService",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_CheckerboardReportService(
            ) -> <CheckerboardReportService as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_CheckerboardReportService(
        ) -> <CheckerboardReportService as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_CheckerboardReportService() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<CheckerboardReportService as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CheckerboardReportService",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_flush_active_reports_CheckerboardReportService() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CheckerboardReportService as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CheckerboardReportService {
    #[cfg(all(feature = "CheckerboardReportService",))]
    #[allow(bad_style)]
    #[doc = "The `flushActiveReports()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/flushActiveReports)\n\n*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
    #[allow(clippy::all)]
    pub fn flush_active_reports(&self) {
        #[cfg(all(feature = "CheckerboardReportService",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_flush_active_reports_CheckerboardReportService(
                self_: <&CheckerboardReportService as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_flush_active_reports_CheckerboardReportService(
            self_: <&CheckerboardReportService as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CheckerboardReportService as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_flush_active_reports_CheckerboardReportService(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CheckerboardReportService",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_reports_CheckerboardReportService() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CheckerboardReportService as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl CheckerboardReportService {
    #[cfg(all(feature = "CheckerboardReportService",))]
    #[allow(bad_style)]
    #[doc = "The `getReports()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/getReports)\n\n*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
    #[allow(clippy::all)]
    pub fn get_reports(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "CheckerboardReportService",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_reports_CheckerboardReportService(
                self_: <&CheckerboardReportService as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_reports_CheckerboardReportService(
            self_: <&CheckerboardReportService as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CheckerboardReportService as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_get_reports_CheckerboardReportService(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CheckerboardReportService",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_recording_enabled_CheckerboardReportService() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CheckerboardReportService as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl CheckerboardReportService {
    #[cfg(all(feature = "CheckerboardReportService",))]
    #[allow(bad_style)]
    #[doc = "The `isRecordingEnabled()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/isRecordingEnabled)\n\n*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
    #[allow(clippy::all)]
    pub fn is_recording_enabled(&self) -> bool {
        #[cfg(all(feature = "CheckerboardReportService",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_recording_enabled_CheckerboardReportService(
                self_: <&CheckerboardReportService as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_recording_enabled_CheckerboardReportService(
            self_: <&CheckerboardReportService as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CheckerboardReportService as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_is_recording_enabled_CheckerboardReportService(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CheckerboardReportService",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_recording_enabled_CheckerboardReportService() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CheckerboardReportService as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CheckerboardReportService {
    #[cfg(all(feature = "CheckerboardReportService",))]
    #[allow(bad_style)]
    #[doc = "The `setRecordingEnabled()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/setRecordingEnabled)\n\n*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
    #[allow(clippy::all)]
    pub fn set_recording_enabled(&self, a_enabled: bool) {
        #[cfg(all(feature = "CheckerboardReportService",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_recording_enabled_CheckerboardReportService(
                self_: <&CheckerboardReportService as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_enabled: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_recording_enabled_CheckerboardReportService(
            self_: <&CheckerboardReportService as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_enabled: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(a_enabled);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CheckerboardReportService as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let a_enabled = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_enabled);
                __widl_f_set_recording_enabled_CheckerboardReportService(self_, a_enabled)
            };
            ()
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_54cd8f387ebf0a0c: [u8; 729usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x97\x02\0\0\0\0\x06\0\0\x02\x19CheckerboardReportService+__widl_instanceof_CheckerboardReportService\0\0\0\0&__widl_f_new_CheckerboardReportService\x01\0\0\x01\x19CheckerboardReportService\0\x01\0\x03new\0\0\07__widl_f_flush_active_reports_CheckerboardReportService\0\0\0\x01\x19CheckerboardReportService\x01\0\0\x01\x01\x05self_\x12flushActiveReports\0\0\0.__widl_f_get_reports_CheckerboardReportService\0\0\0\x01\x19CheckerboardReportService\x01\0\0\x01\x01\x05self_\ngetReports\0\0\07__widl_f_is_recording_enabled_CheckerboardReportService\0\0\0\x01\x19CheckerboardReportService\x01\0\0\x01\x01\x05self_\x12isRecordingEnabled\0\0\08__widl_f_set_recording_enabled_CheckerboardReportService\0\0\0\x01\x19CheckerboardReportService\x01\0\0\x01\x02\x05self_\ta_enabled\x13setRecordingEnabled\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
