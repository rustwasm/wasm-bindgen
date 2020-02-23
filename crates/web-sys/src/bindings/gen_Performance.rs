use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Performance` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance)\n\n*This API requires the following crate features to be activated: `Performance`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Performance {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Performance: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Performance {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
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
        }
    }
    impl core::ops::Deref for Performance {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for Performance {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Performance {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Performance {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Performance {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Performance {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Performance {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Performance {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Performance {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Performance>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Performance {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Performance {
        #[inline]
        fn from(obj: JsValue) -> Performance {
            Performance { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Performance {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Performance> for Performance {
        #[inline]
        fn as_ref(&self) -> &Performance {
            self
        }
    }
    impl From<Performance> for JsValue {
        #[inline]
        fn from(obj: Performance) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Performance {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Performance(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Performance(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Performance(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Performance { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Performance) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Performance> for EventTarget {
    #[inline]
    fn from(obj: Performance) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for Performance {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<Performance> for ::js_sys::Object {
    #[inline]
    fn from(obj: Performance) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Performance {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Performance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_marks_Performance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Performance as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Performance {
    #[cfg(all(feature = "Performance",))]
    #[allow(bad_style)]
    #[doc = "The `clearMarks()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearMarks)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    #[allow(clippy::all)]
    pub fn clear_marks(&self) {
        #[cfg(all(feature = "Performance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_marks_Performance(
                self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_marks_Performance(
            self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Performance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clear_marks_Performance(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Performance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_marks_with_mark_name_Performance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Performance as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Performance {
    #[cfg(all(feature = "Performance",))]
    #[allow(bad_style)]
    #[doc = "The `clearMarks()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearMarks)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    #[allow(clippy::all)]
    pub fn clear_marks_with_mark_name(&self, mark_name: &str) {
        #[cfg(all(feature = "Performance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_marks_with_mark_name_Performance(
                self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mark_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_marks_with_mark_name_Performance(
            self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mark_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(mark_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Performance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let mark_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mark_name);
                __widl_f_clear_marks_with_mark_name_Performance(self_, mark_name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Performance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_measures_Performance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Performance as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Performance {
    #[cfg(all(feature = "Performance",))]
    #[allow(bad_style)]
    #[doc = "The `clearMeasures()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearMeasures)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    #[allow(clippy::all)]
    pub fn clear_measures(&self) {
        #[cfg(all(feature = "Performance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_measures_Performance(
                self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_measures_Performance(
            self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Performance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clear_measures_Performance(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Performance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_measures_with_measure_name_Performance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Performance as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Performance {
    #[cfg(all(feature = "Performance",))]
    #[allow(bad_style)]
    #[doc = "The `clearMeasures()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearMeasures)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    #[allow(clippy::all)]
    pub fn clear_measures_with_measure_name(&self, measure_name: &str) {
        #[cfg(all(feature = "Performance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_measures_with_measure_name_Performance(
                self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                measure_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_measures_with_measure_name_Performance(
            self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            measure_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(measure_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Performance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let measure_name =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(measure_name);
                __widl_f_clear_measures_with_measure_name_Performance(self_, measure_name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Performance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_resource_timings_Performance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Performance as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Performance {
    #[cfg(all(feature = "Performance",))]
    #[allow(bad_style)]
    #[doc = "The `clearResourceTimings()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearResourceTimings)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    #[allow(clippy::all)]
    pub fn clear_resource_timings(&self) {
        #[cfg(all(feature = "Performance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_resource_timings_Performance(
                self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_resource_timings_Performance(
            self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Performance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clear_resource_timings_Performance(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Performance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_entries_Performance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Performance as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl Performance {
    #[cfg(all(feature = "Performance",))]
    #[allow(bad_style)]
    #[doc = "The `getEntries()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/getEntries)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    #[allow(clippy::all)]
    pub fn get_entries(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "Performance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_entries_Performance(
                self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_entries_Performance(
            self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Performance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_entries_Performance(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Performance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_entries_by_name_Performance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Performance as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl Performance {
    #[cfg(all(feature = "Performance",))]
    #[allow(bad_style)]
    #[doc = "The `getEntriesByName()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/getEntriesByName)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    #[allow(clippy::all)]
    pub fn get_entries_by_name(&self, name: &str) -> ::js_sys::Array {
        #[cfg(all(feature = "Performance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_entries_by_name_Performance(
                self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_entries_by_name_Performance(
            self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Performance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_get_entries_by_name_Performance(self_, name)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Performance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_entries_by_name_with_entry_type_Performance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Performance as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl Performance {
    #[cfg(all(feature = "Performance",))]
    #[allow(bad_style)]
    #[doc = "The `getEntriesByName()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/getEntriesByName)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    #[allow(clippy::all)]
    pub fn get_entries_by_name_with_entry_type(
        &self,
        name: &str,
        entry_type: &str,
    ) -> ::js_sys::Array {
        #[cfg(all(feature = "Performance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_entries_by_name_with_entry_type_Performance(
                self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                entry_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_entries_by_name_with_entry_type_Performance(
            self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            entry_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            drop(entry_type);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Performance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let entry_type = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(entry_type);
                __widl_f_get_entries_by_name_with_entry_type_Performance(self_, name, entry_type)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Performance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_entries_by_type_Performance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Performance as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl Performance {
    #[cfg(all(feature = "Performance",))]
    #[allow(bad_style)]
    #[doc = "The `getEntriesByType()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/getEntriesByType)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    #[allow(clippy::all)]
    pub fn get_entries_by_type(&self, entry_type: &str) -> ::js_sys::Array {
        #[cfg(all(feature = "Performance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_entries_by_type_Performance(
                self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                entry_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_entries_by_type_Performance(
            self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            entry_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(entry_type);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Performance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let entry_type = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(entry_type);
                __widl_f_get_entries_by_type_Performance(self_, entry_type)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Performance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_mark_Performance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Performance as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Performance {
    #[cfg(all(feature = "Performance",))]
    #[allow(bad_style)]
    #[doc = "The `mark()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/mark)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    #[allow(clippy::all)]
    pub fn mark(&self, mark_name: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Performance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_mark_Performance(
                self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mark_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_mark_Performance(
            self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mark_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(mark_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Performance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let mark_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mark_name);
                __widl_f_mark_Performance(self_, mark_name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Performance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_measure_Performance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Performance as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Performance {
    #[cfg(all(feature = "Performance",))]
    #[allow(bad_style)]
    #[doc = "The `measure()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/measure)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    #[allow(clippy::all)]
    pub fn measure(&self, measure_name: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Performance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_measure_Performance(
                self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                measure_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_measure_Performance(
            self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            measure_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(measure_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Performance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let measure_name =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(measure_name);
                __widl_f_measure_Performance(self_, measure_name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Performance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_measure_with_start_mark_Performance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Performance as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Performance {
    #[cfg(all(feature = "Performance",))]
    #[allow(bad_style)]
    #[doc = "The `measure()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/measure)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    #[allow(clippy::all)]
    pub fn measure_with_start_mark(
        &self,
        measure_name: &str,
        start_mark: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Performance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_measure_with_start_mark_Performance(
                self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                measure_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_mark: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_measure_with_start_mark_Performance(
            self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            measure_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_mark: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(measure_name);
            drop(start_mark);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Performance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let measure_name =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(measure_name);
                let start_mark = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_mark);
                __widl_f_measure_with_start_mark_Performance(self_, measure_name, start_mark)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Performance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_measure_with_start_mark_and_end_mark_Performance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Performance as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Performance {
    #[cfg(all(feature = "Performance",))]
    #[allow(bad_style)]
    #[doc = "The `measure()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/measure)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    #[allow(clippy::all)]
    pub fn measure_with_start_mark_and_end_mark(
        &self,
        measure_name: &str,
        start_mark: &str,
        end_mark: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Performance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_measure_with_start_mark_and_end_mark_Performance(
                self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                measure_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_mark: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end_mark: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_measure_with_start_mark_and_end_mark_Performance(
            self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            measure_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_mark: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end_mark: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(measure_name);
            drop(start_mark);
            drop(end_mark);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Performance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let measure_name =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(measure_name);
                let start_mark = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_mark);
                let end_mark = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end_mark);
                __widl_f_measure_with_start_mark_and_end_mark_Performance(
                    self_,
                    measure_name,
                    start_mark,
                    end_mark,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Performance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_now_Performance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Performance as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl Performance {
    #[cfg(all(feature = "Performance",))]
    #[allow(bad_style)]
    #[doc = "The `now()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/now)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    #[allow(clippy::all)]
    pub fn now(&self) -> f64 {
        #[cfg(all(feature = "Performance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_now_Performance(
                self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_now_Performance(
            self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Performance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_now_Performance(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Performance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_resource_timing_buffer_size_Performance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Performance as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Performance {
    #[cfg(all(feature = "Performance",))]
    #[allow(bad_style)]
    #[doc = "The `setResourceTimingBufferSize()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/setResourceTimingBufferSize)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    #[allow(clippy::all)]
    pub fn set_resource_timing_buffer_size(&self, max_size: u32) {
        #[cfg(all(feature = "Performance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_resource_timing_buffer_size_Performance(
                self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                max_size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_resource_timing_buffer_size_Performance(
            self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            max_size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(max_size);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Performance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let max_size = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(max_size);
                __widl_f_set_resource_timing_buffer_size_Performance(self_, max_size)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Performance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_json_Performance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Performance as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl Performance {
    #[cfg(all(feature = "Performance",))]
    #[allow(bad_style)]
    #[doc = "The `toJSON()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/toJSON)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    #[allow(clippy::all)]
    pub fn to_json(&self) -> ::js_sys::Object {
        #[cfg(all(feature = "Performance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_json_Performance(
                self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_json_Performance(
            self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Performance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_to_json_Performance(self_)
            };
            <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Performance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_time_origin_Performance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Performance as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl Performance {
    #[cfg(all(feature = "Performance",))]
    #[allow(bad_style)]
    #[doc = "The `timeOrigin` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/timeOrigin)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    #[allow(clippy::all)]
    pub fn time_origin(&self) -> f64 {
        #[cfg(all(feature = "Performance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_origin_Performance(
                self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_origin_Performance(
            self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Performance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_time_origin_Performance(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Performance", feature = "PerformanceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_timing_Performance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Performance as WasmDescribe>::describe();
    <PerformanceTiming as WasmDescribe>::describe();
}
impl Performance {
    #[cfg(all(feature = "Performance", feature = "PerformanceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `timing` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/timing)\n\n*This API requires the following crate features to be activated: `Performance`, `PerformanceTiming`*"]
    #[allow(clippy::all)]
    pub fn timing(&self) -> PerformanceTiming {
        #[cfg(all(feature = "Performance", feature = "PerformanceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_timing_Performance(
                self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PerformanceTiming as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_timing_Performance(
            self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PerformanceTiming as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Performance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_timing_Performance(self_)
            };
            <PerformanceTiming as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Performance", feature = "PerformanceNavigation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_navigation_Performance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Performance as WasmDescribe>::describe();
    <PerformanceNavigation as WasmDescribe>::describe();
}
impl Performance {
    #[cfg(all(feature = "Performance", feature = "PerformanceNavigation",))]
    #[allow(bad_style)]
    #[doc = "The `navigation` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/navigation)\n\n*This API requires the following crate features to be activated: `Performance`, `PerformanceNavigation`*"]
    #[allow(clippy::all)]
    pub fn navigation(&self) -> PerformanceNavigation {
        #[cfg(all(feature = "Performance", feature = "PerformanceNavigation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_navigation_Performance(
                self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PerformanceNavigation as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_navigation_Performance(
            self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PerformanceNavigation as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Performance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_navigation_Performance(self_)
            };
            <PerformanceNavigation as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Performance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onresourcetimingbufferfull_Performance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Performance as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Performance {
    #[cfg(all(feature = "Performance",))]
    #[allow(bad_style)]
    #[doc = "The `onresourcetimingbufferfull` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/onresourcetimingbufferfull)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    #[allow(clippy::all)]
    pub fn onresourcetimingbufferfull(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Performance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onresourcetimingbufferfull_Performance(
                self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onresourcetimingbufferfull_Performance(
            self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Performance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onresourcetimingbufferfull_Performance(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Performance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onresourcetimingbufferfull_Performance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Performance as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Performance {
    #[cfg(all(feature = "Performance",))]
    #[allow(bad_style)]
    #[doc = "The `onresourcetimingbufferfull` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/onresourcetimingbufferfull)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    #[allow(clippy::all)]
    pub fn set_onresourcetimingbufferfull(
        &self,
        onresourcetimingbufferfull: Option<&::js_sys::Function>,
    ) {
        #[cfg(all(feature = "Performance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onresourcetimingbufferfull_Performance(
                self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onresourcetimingbufferfull : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onresourcetimingbufferfull_Performance(
            self_: <&Performance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onresourcetimingbufferfull : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onresourcetimingbufferfull);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Performance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onresourcetimingbufferfull =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onresourcetimingbufferfull,
                    );
                __widl_f_set_onresourcetimingbufferfull_Performance(
                    self_,
                    onresourcetimingbufferfull,
                )
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
pub static __WASM_BINDGEN_GENERATED_76fd74395da46783: [u8; 2164usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}2\x08\0\0\0\0\x16\0\0\x02\x0BPerformance\x1D__widl_instanceof_Performance\0\0\0\0 __widl_f_clear_marks_Performance\0\0\0\x01\x0BPerformance\x01\0\0\x01\x01\x05self_\nclearMarks\0\0\0/__widl_f_clear_marks_with_mark_name_Performance\0\0\0\x01\x0BPerformance\x01\0\0\x01\x02\x05self_\tmark_name\nclearMarks\0\0\0#__widl_f_clear_measures_Performance\0\0\0\x01\x0BPerformance\x01\0\0\x01\x01\x05self_\rclearMeasures\0\0\05__widl_f_clear_measures_with_measure_name_Performance\0\0\0\x01\x0BPerformance\x01\0\0\x01\x02\x05self_\x0Cmeasure_name\rclearMeasures\0\0\0+__widl_f_clear_resource_timings_Performance\0\0\0\x01\x0BPerformance\x01\0\0\x01\x01\x05self_\x14clearResourceTimings\0\0\0 __widl_f_get_entries_Performance\0\0\0\x01\x0BPerformance\x01\0\0\x01\x01\x05self_\ngetEntries\0\0\0(__widl_f_get_entries_by_name_Performance\0\0\0\x01\x0BPerformance\x01\0\0\x01\x02\x05self_\x04name\x10getEntriesByName\0\0\08__widl_f_get_entries_by_name_with_entry_type_Performance\0\0\0\x01\x0BPerformance\x01\0\0\x01\x03\x05self_\x04name\nentry_type\x10getEntriesByName\0\0\0(__widl_f_get_entries_by_type_Performance\0\0\0\x01\x0BPerformance\x01\0\0\x01\x02\x05self_\nentry_type\x10getEntriesByType\0\0\0\x19__widl_f_mark_Performance\x01\0\0\x01\x0BPerformance\x01\0\0\x01\x02\x05self_\tmark_name\x04mark\0\0\0\x1C__widl_f_measure_Performance\x01\0\0\x01\x0BPerformance\x01\0\0\x01\x02\x05self_\x0Cmeasure_name\x07measure\0\0\0,__widl_f_measure_with_start_mark_Performance\x01\0\0\x01\x0BPerformance\x01\0\0\x01\x03\x05self_\x0Cmeasure_name\nstart_mark\x07measure\0\0\09__widl_f_measure_with_start_mark_and_end_mark_Performance\x01\0\0\x01\x0BPerformance\x01\0\0\x01\x04\x05self_\x0Cmeasure_name\nstart_mark\x08end_mark\x07measure\0\0\0\x18__widl_f_now_Performance\0\0\0\x01\x0BPerformance\x01\0\0\x01\x01\x05self_\x03now\0\0\04__widl_f_set_resource_timing_buffer_size_Performance\0\0\0\x01\x0BPerformance\x01\0\0\x01\x02\x05self_\x08max_size\x1BsetResourceTimingBufferSize\0\0\0\x1C__widl_f_to_json_Performance\0\0\0\x01\x0BPerformance\x01\0\0\x01\x01\x05self_\x06toJSON\0\0\0 __widl_f_time_origin_Performance\0\0\0\x01\x0BPerformance\x01\0\x01\ntimeOrigin\x01\x01\x05self_\ntimeOrigin\0\0\0\x1B__widl_f_timing_Performance\0\0\0\x01\x0BPerformance\x01\0\x01\x06timing\x01\x01\x05self_\x06timing\0\0\0\x1F__widl_f_navigation_Performance\0\0\0\x01\x0BPerformance\x01\0\x01\nnavigation\x01\x01\x05self_\nnavigation\0\0\0/__widl_f_onresourcetimingbufferfull_Performance\0\0\0\x01\x0BPerformance\x01\0\x01\x1Aonresourcetimingbufferfull\x01\x01\x05self_\x1Aonresourcetimingbufferfull\0\0\03__widl_f_set_onresourcetimingbufferfull_Performance\0\0\0\x01\x0BPerformance\x01\0\x02\x1Aonresourcetimingbufferfull\x01\x02\x05self_\x1Aonresourcetimingbufferfull\x1Aonresourcetimingbufferfull\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
