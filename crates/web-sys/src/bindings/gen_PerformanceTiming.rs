use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PerformanceTiming` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PerformanceTiming {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PerformanceTiming: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PerformanceTiming {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(80u32);
            inform(101u32);
            inform(114u32);
            inform(102u32);
            inform(111u32);
            inform(114u32);
            inform(109u32);
            inform(97u32);
            inform(110u32);
            inform(99u32);
            inform(101u32);
            inform(84u32);
            inform(105u32);
            inform(109u32);
            inform(105u32);
            inform(110u32);
            inform(103u32);
        }
    }
    impl core::ops::Deref for PerformanceTiming {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for PerformanceTiming {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PerformanceTiming {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PerformanceTiming {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PerformanceTiming {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PerformanceTiming {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PerformanceTiming {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PerformanceTiming {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PerformanceTiming {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PerformanceTiming>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PerformanceTiming {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PerformanceTiming {
        #[inline]
        fn from(obj: JsValue) -> PerformanceTiming {
            PerformanceTiming { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PerformanceTiming {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PerformanceTiming> for PerformanceTiming {
        #[inline]
        fn as_ref(&self) -> &PerformanceTiming {
            self
        }
    }
    impl From<PerformanceTiming> for JsValue {
        #[inline]
        fn from(obj: PerformanceTiming) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PerformanceTiming {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PerformanceTiming(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PerformanceTiming(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PerformanceTiming(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PerformanceTiming { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PerformanceTiming) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PerformanceTiming> for ::js_sys::Object {
    #[inline]
    fn from(obj: PerformanceTiming) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PerformanceTiming {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_json_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `toJSON()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/toJSON)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn to_json(&self) -> ::js_sys::Object {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_json_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_json_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_to_json_PerformanceTiming(self_)
            };
            <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_navigation_start_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `navigationStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/navigationStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn navigation_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_navigation_start_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_navigation_start_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_navigation_start_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unload_event_start_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `unloadEventStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/unloadEventStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn unload_event_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unload_event_start_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unload_event_start_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_unload_event_start_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unload_event_end_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `unloadEventEnd` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/unloadEventEnd)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn unload_event_end(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unload_event_end_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unload_event_end_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_unload_event_end_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_redirect_start_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `redirectStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/redirectStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn redirect_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_redirect_start_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_redirect_start_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_redirect_start_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_redirect_end_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `redirectEnd` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/redirectEnd)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn redirect_end(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_redirect_end_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_redirect_end_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_redirect_end_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fetch_start_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `fetchStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/fetchStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn fetch_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fetch_start_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fetch_start_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_fetch_start_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_domain_lookup_start_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `domainLookupStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domainLookupStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn domain_lookup_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_domain_lookup_start_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_domain_lookup_start_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_domain_lookup_start_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_domain_lookup_end_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `domainLookupEnd` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domainLookupEnd)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn domain_lookup_end(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_domain_lookup_end_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_domain_lookup_end_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_domain_lookup_end_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_connect_start_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `connectStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/connectStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn connect_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_connect_start_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_connect_start_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_connect_start_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_connect_end_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `connectEnd` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/connectEnd)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn connect_end(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_connect_end_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_connect_end_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_connect_end_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_secure_connection_start_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `secureConnectionStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/secureConnectionStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn secure_connection_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_secure_connection_start_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_secure_connection_start_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_secure_connection_start_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_request_start_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `requestStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/requestStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn request_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_request_start_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_request_start_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_request_start_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_response_start_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `responseStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/responseStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn response_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_response_start_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_response_start_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_response_start_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_response_end_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `responseEnd` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/responseEnd)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn response_end(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_response_end_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_response_end_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_response_end_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dom_loading_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `domLoading` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domLoading)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn dom_loading(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dom_loading_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dom_loading_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_dom_loading_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dom_interactive_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `domInteractive` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domInteractive)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn dom_interactive(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dom_interactive_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dom_interactive_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_dom_interactive_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dom_content_loaded_event_start_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `domContentLoadedEventStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domContentLoadedEventStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn dom_content_loaded_event_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dom_content_loaded_event_start_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dom_content_loaded_event_start_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_dom_content_loaded_event_start_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dom_content_loaded_event_end_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `domContentLoadedEventEnd` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domContentLoadedEventEnd)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn dom_content_loaded_event_end(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dom_content_loaded_event_end_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dom_content_loaded_event_end_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_dom_content_loaded_event_end_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dom_complete_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `domComplete` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domComplete)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn dom_complete(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dom_complete_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dom_complete_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_dom_complete_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_load_event_start_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `loadEventStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/loadEventStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn load_event_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_load_event_start_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_load_event_start_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_load_event_start_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_load_event_end_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `loadEventEnd` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/loadEventEnd)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn load_event_end(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_load_event_end_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_load_event_end_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_load_event_end_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_time_to_non_blank_paint_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `timeToNonBlankPaint` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/timeToNonBlankPaint)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn time_to_non_blank_paint(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_to_non_blank_paint_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_to_non_blank_paint_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_time_to_non_blank_paint_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_time_to_dom_content_flushed_PerformanceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceTiming {
    #[cfg(all(feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `timeToDOMContentFlushed` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/timeToDOMContentFlushed)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn time_to_dom_content_flushed(&self) -> f64 {
        #[cfg(all(feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_to_dom_content_flushed_PerformanceTiming(
                self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_to_dom_content_flushed_PerformanceTiming(
            self_: <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_time_to_dom_content_flushed_PerformanceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_d1520f63a8d9806b: [u8; 2830usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xCC\n\0\0\0\0\x19\0\0\x02\x11PerformanceTiming#__widl_instanceof_PerformanceTiming\0\0\0\0\"__widl_f_to_json_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\0\x01\x01\x05self_\x06toJSON\0\0\0+__widl_f_navigation_start_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\x0FnavigationStart\x01\x01\x05self_\x0FnavigationStart\0\0\0-__widl_f_unload_event_start_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\x10unloadEventStart\x01\x01\x05self_\x10unloadEventStart\0\0\0+__widl_f_unload_event_end_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\x0EunloadEventEnd\x01\x01\x05self_\x0EunloadEventEnd\0\0\0)__widl_f_redirect_start_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\rredirectStart\x01\x01\x05self_\rredirectStart\0\0\0'__widl_f_redirect_end_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\x0BredirectEnd\x01\x01\x05self_\x0BredirectEnd\0\0\0&__widl_f_fetch_start_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\nfetchStart\x01\x01\x05self_\nfetchStart\0\0\0.__widl_f_domain_lookup_start_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\x11domainLookupStart\x01\x01\x05self_\x11domainLookupStart\0\0\0,__widl_f_domain_lookup_end_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\x0FdomainLookupEnd\x01\x01\x05self_\x0FdomainLookupEnd\0\0\0(__widl_f_connect_start_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\x0CconnectStart\x01\x01\x05self_\x0CconnectStart\0\0\0&__widl_f_connect_end_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\nconnectEnd\x01\x01\x05self_\nconnectEnd\0\0\02__widl_f_secure_connection_start_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\x15secureConnectionStart\x01\x01\x05self_\x15secureConnectionStart\0\0\0(__widl_f_request_start_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\x0CrequestStart\x01\x01\x05self_\x0CrequestStart\0\0\0)__widl_f_response_start_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\rresponseStart\x01\x01\x05self_\rresponseStart\0\0\0'__widl_f_response_end_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\x0BresponseEnd\x01\x01\x05self_\x0BresponseEnd\0\0\0&__widl_f_dom_loading_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\ndomLoading\x01\x01\x05self_\ndomLoading\0\0\0*__widl_f_dom_interactive_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\x0EdomInteractive\x01\x01\x05self_\x0EdomInteractive\0\0\09__widl_f_dom_content_loaded_event_start_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\x1AdomContentLoadedEventStart\x01\x01\x05self_\x1AdomContentLoadedEventStart\0\0\07__widl_f_dom_content_loaded_event_end_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\x18domContentLoadedEventEnd\x01\x01\x05self_\x18domContentLoadedEventEnd\0\0\0'__widl_f_dom_complete_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\x0BdomComplete\x01\x01\x05self_\x0BdomComplete\0\0\0+__widl_f_load_event_start_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\x0EloadEventStart\x01\x01\x05self_\x0EloadEventStart\0\0\0)__widl_f_load_event_end_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\x0CloadEventEnd\x01\x01\x05self_\x0CloadEventEnd\0\0\02__widl_f_time_to_non_blank_paint_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\x13timeToNonBlankPaint\x01\x01\x05self_\x13timeToNonBlankPaint\0\0\06__widl_f_time_to_dom_content_flushed_PerformanceTiming\0\0\0\x01\x11PerformanceTiming\x01\0\x01\x17timeToDOMContentFlushed\x01\x01\x05self_\x17timeToDOMContentFlushed\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
