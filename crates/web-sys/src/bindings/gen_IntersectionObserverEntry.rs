use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `IntersectionObserverEntry` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry)\n\n*This API requires the following crate features to be activated: `IntersectionObserverEntry`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct IntersectionObserverEntry {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_IntersectionObserverEntry: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for IntersectionObserverEntry {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(25u32);
            inform(73u32);
            inform(110u32);
            inform(116u32);
            inform(101u32);
            inform(114u32);
            inform(115u32);
            inform(101u32);
            inform(99u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
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
        }
    }
    impl core::ops::Deref for IntersectionObserverEntry {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for IntersectionObserverEntry {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for IntersectionObserverEntry {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IntersectionObserverEntry {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for IntersectionObserverEntry {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            IntersectionObserverEntry {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for IntersectionObserverEntry {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a IntersectionObserverEntry {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for IntersectionObserverEntry {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<IntersectionObserverEntry>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(IntersectionObserverEntry {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for IntersectionObserverEntry {
        #[inline]
        fn from(obj: JsValue) -> IntersectionObserverEntry {
            IntersectionObserverEntry { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for IntersectionObserverEntry {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<IntersectionObserverEntry> for IntersectionObserverEntry {
        #[inline]
        fn as_ref(&self) -> &IntersectionObserverEntry {
            self
        }
    }
    impl From<IntersectionObserverEntry> for JsValue {
        #[inline]
        fn from(obj: IntersectionObserverEntry) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for IntersectionObserverEntry {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_IntersectionObserverEntry(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_IntersectionObserverEntry(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_IntersectionObserverEntry(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IntersectionObserverEntry { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IntersectionObserverEntry) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<IntersectionObserverEntry> for ::js_sys::Object {
    #[inline]
    fn from(obj: IntersectionObserverEntry) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for IntersectionObserverEntry {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "IntersectionObserverEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_time_IntersectionObserverEntry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IntersectionObserverEntry as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl IntersectionObserverEntry {
    #[cfg(all(feature = "IntersectionObserverEntry",))]
    #[allow(bad_style)]
    #[doc = "The `time` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/time)\n\n*This API requires the following crate features to be activated: `IntersectionObserverEntry`*"]
    #[allow(clippy::all)]
    pub fn time(&self) -> f64 {
        #[cfg(all(feature = "IntersectionObserverEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_IntersectionObserverEntry(
                self_: <&IntersectionObserverEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_IntersectionObserverEntry(
            self_: <&IntersectionObserverEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&IntersectionObserverEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_time_IntersectionObserverEntry(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRectReadOnly", feature = "IntersectionObserverEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_root_bounds_IntersectionObserverEntry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IntersectionObserverEntry as WasmDescribe>::describe();
    <Option<DomRectReadOnly> as WasmDescribe>::describe();
}
impl IntersectionObserverEntry {
    #[cfg(all(feature = "DomRectReadOnly", feature = "IntersectionObserverEntry",))]
    #[allow(bad_style)]
    #[doc = "The `rootBounds` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/rootBounds)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`, `IntersectionObserverEntry`*"]
    #[allow(clippy::all)]
    pub fn root_bounds(&self) -> Option<DomRectReadOnly> {
        #[cfg(all(feature = "DomRectReadOnly", feature = "IntersectionObserverEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_root_bounds_IntersectionObserverEntry(
                self_: <&IntersectionObserverEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<DomRectReadOnly> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_root_bounds_IntersectionObserverEntry(
            self_: <&IntersectionObserverEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<DomRectReadOnly> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&IntersectionObserverEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_root_bounds_IntersectionObserverEntry(self_)
            };
            <Option<DomRectReadOnly> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRectReadOnly", feature = "IntersectionObserverEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bounding_client_rect_IntersectionObserverEntry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IntersectionObserverEntry as WasmDescribe>::describe();
    <DomRectReadOnly as WasmDescribe>::describe();
}
impl IntersectionObserverEntry {
    #[cfg(all(feature = "DomRectReadOnly", feature = "IntersectionObserverEntry",))]
    #[allow(bad_style)]
    #[doc = "The `boundingClientRect` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/boundingClientRect)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`, `IntersectionObserverEntry`*"]
    #[allow(clippy::all)]
    pub fn bounding_client_rect(&self) -> DomRectReadOnly {
        #[cfg(all(feature = "DomRectReadOnly", feature = "IntersectionObserverEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bounding_client_rect_IntersectionObserverEntry(
                self_: <&IntersectionObserverEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bounding_client_rect_IntersectionObserverEntry(
            self_: <&IntersectionObserverEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&IntersectionObserverEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_bounding_client_rect_IntersectionObserverEntry(self_)
            };
            <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRectReadOnly", feature = "IntersectionObserverEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_intersection_rect_IntersectionObserverEntry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IntersectionObserverEntry as WasmDescribe>::describe();
    <DomRectReadOnly as WasmDescribe>::describe();
}
impl IntersectionObserverEntry {
    #[cfg(all(feature = "DomRectReadOnly", feature = "IntersectionObserverEntry",))]
    #[allow(bad_style)]
    #[doc = "The `intersectionRect` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/intersectionRect)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`, `IntersectionObserverEntry`*"]
    #[allow(clippy::all)]
    pub fn intersection_rect(&self) -> DomRectReadOnly {
        #[cfg(all(feature = "DomRectReadOnly", feature = "IntersectionObserverEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_intersection_rect_IntersectionObserverEntry(
                self_: <&IntersectionObserverEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_intersection_rect_IntersectionObserverEntry(
            self_: <&IntersectionObserverEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&IntersectionObserverEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_intersection_rect_IntersectionObserverEntry(self_)
            };
            <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IntersectionObserverEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_intersecting_IntersectionObserverEntry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IntersectionObserverEntry as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl IntersectionObserverEntry {
    #[cfg(all(feature = "IntersectionObserverEntry",))]
    #[allow(bad_style)]
    #[doc = "The `isIntersecting` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/isIntersecting)\n\n*This API requires the following crate features to be activated: `IntersectionObserverEntry`*"]
    #[allow(clippy::all)]
    pub fn is_intersecting(&self) -> bool {
        #[cfg(all(feature = "IntersectionObserverEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_intersecting_IntersectionObserverEntry(
                self_: <&IntersectionObserverEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_intersecting_IntersectionObserverEntry(
            self_: <&IntersectionObserverEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&IntersectionObserverEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_is_intersecting_IntersectionObserverEntry(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IntersectionObserverEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_intersection_ratio_IntersectionObserverEntry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IntersectionObserverEntry as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl IntersectionObserverEntry {
    #[cfg(all(feature = "IntersectionObserverEntry",))]
    #[allow(bad_style)]
    #[doc = "The `intersectionRatio` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/intersectionRatio)\n\n*This API requires the following crate features to be activated: `IntersectionObserverEntry`*"]
    #[allow(clippy::all)]
    pub fn intersection_ratio(&self) -> f64 {
        #[cfg(all(feature = "IntersectionObserverEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_intersection_ratio_IntersectionObserverEntry(
                self_: <&IntersectionObserverEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_intersection_ratio_IntersectionObserverEntry(
            self_: <&IntersectionObserverEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&IntersectionObserverEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_intersection_ratio_IntersectionObserverEntry(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element", feature = "IntersectionObserverEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_target_IntersectionObserverEntry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IntersectionObserverEntry as WasmDescribe>::describe();
    <Element as WasmDescribe>::describe();
}
impl IntersectionObserverEntry {
    #[cfg(all(feature = "Element", feature = "IntersectionObserverEntry",))]
    #[allow(bad_style)]
    #[doc = "The `target` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/target)\n\n*This API requires the following crate features to be activated: `Element`, `IntersectionObserverEntry`*"]
    #[allow(clippy::all)]
    pub fn target(&self) -> Element {
        #[cfg(all(feature = "Element", feature = "IntersectionObserverEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_target_IntersectionObserverEntry(
                self_: <&IntersectionObserverEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Element as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_target_IntersectionObserverEntry(
            self_: <&IntersectionObserverEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Element as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&IntersectionObserverEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_target_IntersectionObserverEntry(self_)
            };
            <Element as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_4c1645b310b51867: [u8; 1014usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xB4\x03\0\0\0\0\x08\0\0\x02\x19IntersectionObserverEntry+__widl_instanceof_IntersectionObserverEntry\0\0\0\0'__widl_f_time_IntersectionObserverEntry\0\0\0\x01\x19IntersectionObserverEntry\x01\0\x01\x04time\x01\x01\x05self_\x04time\0\0\0.__widl_f_root_bounds_IntersectionObserverEntry\0\0\0\x01\x19IntersectionObserverEntry\x01\0\x01\nrootBounds\x01\x01\x05self_\nrootBounds\0\0\07__widl_f_bounding_client_rect_IntersectionObserverEntry\0\0\0\x01\x19IntersectionObserverEntry\x01\0\x01\x12boundingClientRect\x01\x01\x05self_\x12boundingClientRect\0\0\04__widl_f_intersection_rect_IntersectionObserverEntry\0\0\0\x01\x19IntersectionObserverEntry\x01\0\x01\x10intersectionRect\x01\x01\x05self_\x10intersectionRect\0\0\02__widl_f_is_intersecting_IntersectionObserverEntry\0\0\0\x01\x19IntersectionObserverEntry\x01\0\x01\x0EisIntersecting\x01\x01\x05self_\x0EisIntersecting\0\0\05__widl_f_intersection_ratio_IntersectionObserverEntry\0\0\0\x01\x19IntersectionObserverEntry\x01\0\x01\x11intersectionRatio\x01\x01\x05self_\x11intersectionRatio\0\0\0)__widl_f_target_IntersectionObserverEntry\0\0\0\x01\x19IntersectionObserverEntry\x01\0\x01\x06target\x01\x01\x05self_\x06target\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
