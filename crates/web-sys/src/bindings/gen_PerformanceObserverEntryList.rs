use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PerformanceObserverEntryList` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList)\n\n*This API requires the following crate features to be activated: `PerformanceObserverEntryList`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PerformanceObserverEntryList {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PerformanceObserverEntryList: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PerformanceObserverEntryList {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(28u32);
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
            inform(79u32);
            inform(98u32);
            inform(115u32);
            inform(101u32);
            inform(114u32);
            inform(118u32);
            inform(101u32);
            inform(114u32);
            inform(69u32);
            inform(110u32);
            inform(116u32);
            inform(114u32);
            inform(121u32);
            inform(76u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for PerformanceObserverEntryList {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for PerformanceObserverEntryList {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PerformanceObserverEntryList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PerformanceObserverEntryList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PerformanceObserverEntryList {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PerformanceObserverEntryList {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PerformanceObserverEntryList {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PerformanceObserverEntryList {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PerformanceObserverEntryList {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PerformanceObserverEntryList>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PerformanceObserverEntryList {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PerformanceObserverEntryList {
        #[inline]
        fn from(obj: JsValue) -> PerformanceObserverEntryList {
            PerformanceObserverEntryList { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PerformanceObserverEntryList {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PerformanceObserverEntryList> for PerformanceObserverEntryList {
        #[inline]
        fn as_ref(&self) -> &PerformanceObserverEntryList {
            self
        }
    }
    impl From<PerformanceObserverEntryList> for JsValue {
        #[inline]
        fn from(obj: PerformanceObserverEntryList) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PerformanceObserverEntryList {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PerformanceObserverEntryList(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PerformanceObserverEntryList(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PerformanceObserverEntryList(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PerformanceObserverEntryList { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PerformanceObserverEntryList) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PerformanceObserverEntryList> for ::js_sys::Object {
    #[inline]
    fn from(obj: PerformanceObserverEntryList) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PerformanceObserverEntryList {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PerformanceObserverEntryList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_entries_PerformanceObserverEntryList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceObserverEntryList as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl PerformanceObserverEntryList {
    #[cfg(all(feature = "PerformanceObserverEntryList",))]
    #[allow(bad_style)]
    #[doc = "The `getEntries()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntries)\n\n*This API requires the following crate features to be activated: `PerformanceObserverEntryList`*"]
    #[allow(clippy::all)]
    pub fn get_entries(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "PerformanceObserverEntryList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_entries_PerformanceObserverEntryList(
                self_: <&PerformanceObserverEntryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_entries_PerformanceObserverEntryList(
            self_: <&PerformanceObserverEntryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceObserverEntryList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_get_entries_PerformanceObserverEntryList(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "PerformanceEntryFilterOptions",
    feature = "PerformanceObserverEntryList",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_entries_with_filter_PerformanceObserverEntryList(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PerformanceObserverEntryList as WasmDescribe>::describe();
    <&PerformanceEntryFilterOptions as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl PerformanceObserverEntryList {
    #[cfg(all(
        feature = "PerformanceEntryFilterOptions",
        feature = "PerformanceObserverEntryList",
    ))]
    #[allow(bad_style)]
    #[doc = "The `getEntries()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntries)\n\n*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`, `PerformanceObserverEntryList`*"]
    #[allow(clippy::all)]
    pub fn get_entries_with_filter(
        &self,
        filter: &PerformanceEntryFilterOptions,
    ) -> ::js_sys::Array {
        #[cfg(all(
            feature = "PerformanceEntryFilterOptions",
            feature = "PerformanceObserverEntryList",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_entries_with_filter_PerformanceObserverEntryList(
                self_: <&PerformanceObserverEntryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                filter: <&PerformanceEntryFilterOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_entries_with_filter_PerformanceObserverEntryList(
            self_: <&PerformanceObserverEntryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            filter: <&PerformanceEntryFilterOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(filter);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceObserverEntryList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let filter = < & PerformanceEntryFilterOptions as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( filter ) ;
                __widl_f_get_entries_with_filter_PerformanceObserverEntryList(self_, filter)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceObserverEntryList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_entries_by_name_PerformanceObserverEntryList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PerformanceObserverEntryList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl PerformanceObserverEntryList {
    #[cfg(all(feature = "PerformanceObserverEntryList",))]
    #[allow(bad_style)]
    #[doc = "The `getEntriesByName()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntriesByName)\n\n*This API requires the following crate features to be activated: `PerformanceObserverEntryList`*"]
    #[allow(clippy::all)]
    pub fn get_entries_by_name(&self, name: &str) -> ::js_sys::Array {
        #[cfg(all(feature = "PerformanceObserverEntryList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_entries_by_name_PerformanceObserverEntryList(
                self_: <&PerformanceObserverEntryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_entries_by_name_PerformanceObserverEntryList(
            self_: <&PerformanceObserverEntryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ =
                    <&PerformanceObserverEntryList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_get_entries_by_name_PerformanceObserverEntryList(self_, name)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceObserverEntryList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_entries_by_name_with_entry_type_PerformanceObserverEntryList(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&PerformanceObserverEntryList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl PerformanceObserverEntryList {
    #[cfg(all(feature = "PerformanceObserverEntryList",))]
    #[allow(bad_style)]
    #[doc = "The `getEntriesByName()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntriesByName)\n\n*This API requires the following crate features to be activated: `PerformanceObserverEntryList`*"]
    #[allow(clippy::all)]
    pub fn get_entries_by_name_with_entry_type(
        &self,
        name: &str,
        entry_type: &str,
    ) -> ::js_sys::Array {
        #[cfg(all(feature = "PerformanceObserverEntryList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_entries_by_name_with_entry_type_PerformanceObserverEntryList(
                self_: <&PerformanceObserverEntryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                entry_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_entries_by_name_with_entry_type_PerformanceObserverEntryList(
            self_: <&PerformanceObserverEntryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ =
                    <&PerformanceObserverEntryList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let entry_type = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(entry_type);
                __widl_f_get_entries_by_name_with_entry_type_PerformanceObserverEntryList(
                    self_, name, entry_type,
                )
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceObserverEntryList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_entries_by_type_PerformanceObserverEntryList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PerformanceObserverEntryList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl PerformanceObserverEntryList {
    #[cfg(all(feature = "PerformanceObserverEntryList",))]
    #[allow(bad_style)]
    #[doc = "The `getEntriesByType()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntriesByType)\n\n*This API requires the following crate features to be activated: `PerformanceObserverEntryList`*"]
    #[allow(clippy::all)]
    pub fn get_entries_by_type(&self, entry_type: &str) -> ::js_sys::Array {
        #[cfg(all(feature = "PerformanceObserverEntryList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_entries_by_type_PerformanceObserverEntryList(
                self_: <&PerformanceObserverEntryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                entry_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_entries_by_type_PerformanceObserverEntryList(
            self_: <&PerformanceObserverEntryList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ =
                    <&PerformanceObserverEntryList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let entry_type = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(entry_type);
                __widl_f_get_entries_by_type_PerformanceObserverEntryList(self_, entry_type)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_dd4496e24d00bba8: [u8; 834usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\0\x03\0\0\0\0\x06\0\0\x02\x1CPerformanceObserverEntryList.__widl_instanceof_PerformanceObserverEntryList\0\0\0\01__widl_f_get_entries_PerformanceObserverEntryList\0\0\0\x01\x1CPerformanceObserverEntryList\x01\0\0\x01\x01\x05self_\ngetEntries\0\0\0=__widl_f_get_entries_with_filter_PerformanceObserverEntryList\0\0\0\x01\x1CPerformanceObserverEntryList\x01\0\0\x01\x02\x05self_\x06filter\ngetEntries\0\0\09__widl_f_get_entries_by_name_PerformanceObserverEntryList\0\0\0\x01\x1CPerformanceObserverEntryList\x01\0\0\x01\x02\x05self_\x04name\x10getEntriesByName\0\0\0I__widl_f_get_entries_by_name_with_entry_type_PerformanceObserverEntryList\0\0\0\x01\x1CPerformanceObserverEntryList\x01\0\0\x01\x03\x05self_\x04name\nentry_type\x10getEntriesByName\0\0\09__widl_f_get_entries_by_type_PerformanceObserverEntryList\0\0\0\x01\x1CPerformanceObserverEntryList\x01\0\0\x01\x02\x05self_\nentry_type\x10getEntriesByType\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
