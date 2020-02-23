use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WorkerNavigator` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WorkerNavigator {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WorkerNavigator: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WorkerNavigator {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(87u32);
            inform(111u32);
            inform(114u32);
            inform(107u32);
            inform(101u32);
            inform(114u32);
            inform(78u32);
            inform(97u32);
            inform(118u32);
            inform(105u32);
            inform(103u32);
            inform(97u32);
            inform(116u32);
            inform(111u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for WorkerNavigator {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for WorkerNavigator {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WorkerNavigator {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WorkerNavigator {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WorkerNavigator {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WorkerNavigator {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WorkerNavigator {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WorkerNavigator {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WorkerNavigator {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WorkerNavigator>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WorkerNavigator {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WorkerNavigator {
        #[inline]
        fn from(obj: JsValue) -> WorkerNavigator {
            WorkerNavigator { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WorkerNavigator {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WorkerNavigator> for WorkerNavigator {
        #[inline]
        fn as_ref(&self) -> &WorkerNavigator {
            self
        }
    }
    impl From<WorkerNavigator> for JsValue {
        #[inline]
        fn from(obj: WorkerNavigator) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WorkerNavigator {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WorkerNavigator(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WorkerNavigator(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WorkerNavigator(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WorkerNavigator { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WorkerNavigator) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WorkerNavigator> for ::js_sys::Object {
    #[inline]
    fn from(obj: WorkerNavigator) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WorkerNavigator {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "NetworkInformation", feature = "WorkerNavigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_connection_WorkerNavigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerNavigator as WasmDescribe>::describe();
    <NetworkInformation as WasmDescribe>::describe();
}
impl WorkerNavigator {
    #[cfg(all(feature = "NetworkInformation", feature = "WorkerNavigator",))]
    #[allow(bad_style)]
    #[doc = "The `connection` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/connection)\n\n*This API requires the following crate features to be activated: `NetworkInformation`, `WorkerNavigator`*"]
    #[allow(clippy::all)]
    pub fn connection(&self) -> Result<NetworkInformation, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "NetworkInformation", feature = "WorkerNavigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_connection_WorkerNavigator(
                self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <NetworkInformation as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_connection_WorkerNavigator(
            self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <NetworkInformation as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_connection_WorkerNavigator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<NetworkInformation as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MediaCapabilities", feature = "WorkerNavigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_media_capabilities_WorkerNavigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerNavigator as WasmDescribe>::describe();
    <MediaCapabilities as WasmDescribe>::describe();
}
impl WorkerNavigator {
    #[cfg(all(feature = "MediaCapabilities", feature = "WorkerNavigator",))]
    #[allow(bad_style)]
    #[doc = "The `mediaCapabilities` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/mediaCapabilities)\n\n*This API requires the following crate features to be activated: `MediaCapabilities`, `WorkerNavigator`*"]
    #[allow(clippy::all)]
    pub fn media_capabilities(&self) -> MediaCapabilities {
        #[cfg(all(feature = "MediaCapabilities", feature = "WorkerNavigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_media_capabilities_WorkerNavigator(
                self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaCapabilities as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_media_capabilities_WorkerNavigator(
            self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaCapabilities as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_media_capabilities_WorkerNavigator(self_)
            };
            <MediaCapabilities as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerNavigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_hardware_concurrency_WorkerNavigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerNavigator as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl WorkerNavigator {
    #[cfg(all(feature = "WorkerNavigator",))]
    #[allow(bad_style)]
    #[doc = "The `hardwareConcurrency` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/hardwareConcurrency)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    #[allow(clippy::all)]
    pub fn hardware_concurrency(&self) -> f64 {
        #[cfg(all(feature = "WorkerNavigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_hardware_concurrency_WorkerNavigator(
                self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_hardware_concurrency_WorkerNavigator(
            self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_hardware_concurrency_WorkerNavigator(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerNavigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_taint_enabled_WorkerNavigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerNavigator as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl WorkerNavigator {
    #[cfg(all(feature = "WorkerNavigator",))]
    #[allow(bad_style)]
    #[doc = "The `taintEnabled()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/taintEnabled)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    #[allow(clippy::all)]
    pub fn taint_enabled(&self) -> bool {
        #[cfg(all(feature = "WorkerNavigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_taint_enabled_WorkerNavigator(
                self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_taint_enabled_WorkerNavigator(
            self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_taint_enabled_WorkerNavigator(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerNavigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_app_code_name_WorkerNavigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerNavigator as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WorkerNavigator {
    #[cfg(all(feature = "WorkerNavigator",))]
    #[allow(bad_style)]
    #[doc = "The `appCodeName` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/appCodeName)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    #[allow(clippy::all)]
    pub fn app_code_name(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerNavigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_app_code_name_WorkerNavigator(
                self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_app_code_name_WorkerNavigator(
            self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_app_code_name_WorkerNavigator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "WorkerNavigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_app_name_WorkerNavigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerNavigator as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WorkerNavigator {
    #[cfg(all(feature = "WorkerNavigator",))]
    #[allow(bad_style)]
    #[doc = "The `appName` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/appName)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    #[allow(clippy::all)]
    pub fn app_name(&self) -> String {
        #[cfg(all(feature = "WorkerNavigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_app_name_WorkerNavigator(
                self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_app_name_WorkerNavigator(
            self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_app_name_WorkerNavigator(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerNavigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_app_version_WorkerNavigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerNavigator as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WorkerNavigator {
    #[cfg(all(feature = "WorkerNavigator",))]
    #[allow(bad_style)]
    #[doc = "The `appVersion` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/appVersion)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    #[allow(clippy::all)]
    pub fn app_version(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerNavigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_app_version_WorkerNavigator(
                self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_app_version_WorkerNavigator(
            self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_app_version_WorkerNavigator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "WorkerNavigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_platform_WorkerNavigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerNavigator as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WorkerNavigator {
    #[cfg(all(feature = "WorkerNavigator",))]
    #[allow(bad_style)]
    #[doc = "The `platform` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/platform)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    #[allow(clippy::all)]
    pub fn platform(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerNavigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_platform_WorkerNavigator(
                self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_platform_WorkerNavigator(
            self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_platform_WorkerNavigator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "WorkerNavigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_user_agent_WorkerNavigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerNavigator as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WorkerNavigator {
    #[cfg(all(feature = "WorkerNavigator",))]
    #[allow(bad_style)]
    #[doc = "The `userAgent` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/userAgent)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    #[allow(clippy::all)]
    pub fn user_agent(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerNavigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_user_agent_WorkerNavigator(
                self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_user_agent_WorkerNavigator(
            self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_user_agent_WorkerNavigator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "WorkerNavigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_product_WorkerNavigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerNavigator as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WorkerNavigator {
    #[cfg(all(feature = "WorkerNavigator",))]
    #[allow(bad_style)]
    #[doc = "The `product` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/product)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    #[allow(clippy::all)]
    pub fn product(&self) -> String {
        #[cfg(all(feature = "WorkerNavigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_product_WorkerNavigator(
                self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_product_WorkerNavigator(
            self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_product_WorkerNavigator(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerNavigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_language_WorkerNavigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerNavigator as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl WorkerNavigator {
    #[cfg(all(feature = "WorkerNavigator",))]
    #[allow(bad_style)]
    #[doc = "The `language` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/language)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    #[allow(clippy::all)]
    pub fn language(&self) -> Option<String> {
        #[cfg(all(feature = "WorkerNavigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_language_WorkerNavigator(
                self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_language_WorkerNavigator(
            self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_language_WorkerNavigator(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerNavigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_languages_WorkerNavigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerNavigator as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl WorkerNavigator {
    #[cfg(all(feature = "WorkerNavigator",))]
    #[allow(bad_style)]
    #[doc = "The `languages` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/languages)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    #[allow(clippy::all)]
    pub fn languages(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "WorkerNavigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_languages_WorkerNavigator(
                self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_languages_WorkerNavigator(
            self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_languages_WorkerNavigator(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerNavigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_on_line_WorkerNavigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerNavigator as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl WorkerNavigator {
    #[cfg(all(feature = "WorkerNavigator",))]
    #[allow(bad_style)]
    #[doc = "The `onLine` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/onLine)\n\n*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    #[allow(clippy::all)]
    pub fn on_line(&self) -> bool {
        #[cfg(all(feature = "WorkerNavigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_on_line_WorkerNavigator(
                self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_on_line_WorkerNavigator(
            self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_on_line_WorkerNavigator(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "StorageManager", feature = "WorkerNavigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_storage_WorkerNavigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerNavigator as WasmDescribe>::describe();
    <StorageManager as WasmDescribe>::describe();
}
impl WorkerNavigator {
    #[cfg(all(feature = "StorageManager", feature = "WorkerNavigator",))]
    #[allow(bad_style)]
    #[doc = "The `storage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/storage)\n\n*This API requires the following crate features to be activated: `StorageManager`, `WorkerNavigator`*"]
    #[allow(clippy::all)]
    pub fn storage(&self) -> StorageManager {
        #[cfg(all(feature = "StorageManager", feature = "WorkerNavigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_storage_WorkerNavigator(
                self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <StorageManager as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_storage_WorkerNavigator(
            self_: <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <StorageManager as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerNavigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_storage_WorkerNavigator(self_)
            };
            <StorageManager as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_0a7fafb8342217ea: [u8; 1443usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}a\x05\0\0\0\0\x0F\0\0\x02\x0FWorkerNavigator!__widl_instanceof_WorkerNavigator\0\0\0\0#__widl_f_connection_WorkerNavigator\x01\0\0\x01\x0FWorkerNavigator\x01\0\x01\nconnection\x01\x01\x05self_\nconnection\0\0\0+__widl_f_media_capabilities_WorkerNavigator\0\0\0\x01\x0FWorkerNavigator\x01\0\x01\x11mediaCapabilities\x01\x01\x05self_\x11mediaCapabilities\0\0\0-__widl_f_hardware_concurrency_WorkerNavigator\0\0\0\x01\x0FWorkerNavigator\x01\0\x01\x13hardwareConcurrency\x01\x01\x05self_\x13hardwareConcurrency\0\0\0&__widl_f_taint_enabled_WorkerNavigator\0\0\0\x01\x0FWorkerNavigator\x01\0\0\x01\x01\x05self_\x0CtaintEnabled\0\0\0&__widl_f_app_code_name_WorkerNavigator\x01\0\0\x01\x0FWorkerNavigator\x01\0\x01\x0BappCodeName\x01\x01\x05self_\x0BappCodeName\0\0\0!__widl_f_app_name_WorkerNavigator\0\0\0\x01\x0FWorkerNavigator\x01\0\x01\x07appName\x01\x01\x05self_\x07appName\0\0\0$__widl_f_app_version_WorkerNavigator\x01\0\0\x01\x0FWorkerNavigator\x01\0\x01\nappVersion\x01\x01\x05self_\nappVersion\0\0\0!__widl_f_platform_WorkerNavigator\x01\0\0\x01\x0FWorkerNavigator\x01\0\x01\x08platform\x01\x01\x05self_\x08platform\0\0\0#__widl_f_user_agent_WorkerNavigator\x01\0\0\x01\x0FWorkerNavigator\x01\0\x01\tuserAgent\x01\x01\x05self_\tuserAgent\0\0\0 __widl_f_product_WorkerNavigator\0\0\0\x01\x0FWorkerNavigator\x01\0\x01\x07product\x01\x01\x05self_\x07product\0\0\0!__widl_f_language_WorkerNavigator\0\0\0\x01\x0FWorkerNavigator\x01\0\x01\x08language\x01\x01\x05self_\x08language\0\0\0\"__widl_f_languages_WorkerNavigator\0\0\0\x01\x0FWorkerNavigator\x01\0\x01\tlanguages\x01\x01\x05self_\tlanguages\0\0\0 __widl_f_on_line_WorkerNavigator\0\0\0\x01\x0FWorkerNavigator\x01\0\x01\x06onLine\x01\x01\x05self_\x06onLine\0\0\0 __widl_f_storage_WorkerNavigator\0\0\0\x01\x0FWorkerNavigator\x01\0\x01\x07storage\x01\x01\x05self_\x07storage\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
