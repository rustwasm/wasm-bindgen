use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Range` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range)\n\n*This API requires the following crate features to be activated: `Range`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Range {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Range: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Range {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(5u32);
            inform(82u32);
            inform(97u32);
            inform(110u32);
            inform(103u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for Range {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Range {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Range {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Range {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Range {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Range {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Range {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Range {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Range {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Range>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Range {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Range {
        #[inline]
        fn from(obj: JsValue) -> Range {
            Range { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Range {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Range> for Range {
        #[inline]
        fn as_ref(&self) -> &Range {
            self
        }
    }
    impl From<Range> for JsValue {
        #[inline]
        fn from(obj: Range) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Range {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Range(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Range(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Range(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Range { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Range) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Range> for ::js_sys::Object {
    #[inline]
    fn from(obj: Range) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Range {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <Range as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `new Range(..)` constructor, creating a new instance of `Range`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/Range)\n\n*This API requires the following crate features to be activated: `Range`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<Range, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_Range() -> <Range as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_Range() -> <Range as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_Range() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Range as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clone_contents_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Range as WasmDescribe>::describe();
    <DocumentFragment as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "DocumentFragment", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `cloneContents()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/cloneContents)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `Range`*"]
    #[allow(clippy::all)]
    pub fn clone_contents(&self) -> Result<DocumentFragment, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clone_contents_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DocumentFragment as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clone_contents_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DocumentFragment as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clone_contents_Range(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DocumentFragment as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clone_range_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Range as WasmDescribe>::describe();
    <Range as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `cloneRange()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/cloneRange)\n\n*This API requires the following crate features to be activated: `Range`*"]
    #[allow(clippy::all)]
    pub fn clone_range(&self) -> Range {
        #[cfg(all(feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clone_range_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Range as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clone_range_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Range as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clone_range_Range(self_)
            };
            <Range as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_collapse_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Range as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `collapse()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/collapse)\n\n*This API requires the following crate features to be activated: `Range`*"]
    #[allow(clippy::all)]
    pub fn collapse(&self) {
        #[cfg(all(feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_collapse_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_collapse_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_collapse_Range(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_collapse_with_to_start_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Range as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `collapse()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/collapse)\n\n*This API requires the following crate features to be activated: `Range`*"]
    #[allow(clippy::all)]
    pub fn collapse_with_to_start(&self, to_start: bool) {
        #[cfg(all(feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_collapse_with_to_start_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                to_start: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_collapse_with_to_start_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            to_start: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(to_start);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let to_start = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(to_start);
                __widl_f_collapse_with_to_start_Range(self_, to_start)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_compare_boundary_points_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Range as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <&Range as WasmDescribe>::describe();
    <i16 as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `compareBoundaryPoints()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/compareBoundaryPoints)\n\n*This API requires the following crate features to be activated: `Range`*"]
    #[allow(clippy::all)]
    pub fn compare_boundary_points(
        &self,
        how: u16,
        source_range: &Range,
    ) -> Result<i16, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_compare_boundary_points_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                how: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                source_range: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_compare_boundary_points_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            how: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            source_range: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(how);
            drop(source_range);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let how = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(how);
                let source_range =
                    <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(source_range);
                __widl_f_compare_boundary_points_Range(self_, how, source_range)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_compare_point_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Range as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i16 as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Node", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `comparePoint()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/comparePoint)\n\n*This API requires the following crate features to be activated: `Node`, `Range`*"]
    #[allow(clippy::all)]
    pub fn compare_point(&self, node: &Node, offset: u32) -> Result<i16, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_compare_point_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_compare_point_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(node);
            drop(offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                let offset = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                __widl_f_compare_point_Range(self_, node, offset)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_contextual_fragment_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Range as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <DocumentFragment as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "DocumentFragment", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `createContextualFragment()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/createContextualFragment)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `Range`*"]
    #[allow(clippy::all)]
    pub fn create_contextual_fragment(
        &self,
        fragment: &str,
    ) -> Result<DocumentFragment, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_contextual_fragment_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                fragment: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DocumentFragment as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_contextual_fragment_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            fragment: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DocumentFragment as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(fragment);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let fragment = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(fragment);
                __widl_f_create_contextual_fragment_Range(self_, fragment)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DocumentFragment as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_contents_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Range as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `deleteContents()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/deleteContents)\n\n*This API requires the following crate features to be activated: `Range`*"]
    #[allow(clippy::all)]
    pub fn delete_contents(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_contents_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_contents_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_delete_contents_Range(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_detach_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Range as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `detach()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/detach)\n\n*This API requires the following crate features to be activated: `Range`*"]
    #[allow(clippy::all)]
    pub fn detach(&self) {
        #[cfg(all(feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_detach_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_detach_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_detach_Range(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_extract_contents_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Range as WasmDescribe>::describe();
    <DocumentFragment as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "DocumentFragment", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `extractContents()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/extractContents)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `Range`*"]
    #[allow(clippy::all)]
    pub fn extract_contents(&self) -> Result<DocumentFragment, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_extract_contents_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DocumentFragment as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_extract_contents_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DocumentFragment as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_extract_contents_Range(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DocumentFragment as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomRect", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_bounding_client_rect_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Range as WasmDescribe>::describe();
    <DomRect as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "DomRect", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `getBoundingClientRect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/getBoundingClientRect)\n\n*This API requires the following crate features to be activated: `DomRect`, `Range`*"]
    #[allow(clippy::all)]
    pub fn get_bounding_client_rect(&self) -> DomRect {
        #[cfg(all(feature = "DomRect", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_bounding_client_rect_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomRect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_bounding_client_rect_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomRect as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_bounding_client_rect_Range(self_)
            };
            <DomRect as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRectList", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_client_rects_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Range as WasmDescribe>::describe();
    <Option<DomRectList> as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "DomRectList", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `getClientRects()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/getClientRects)\n\n*This API requires the following crate features to be activated: `DomRectList`, `Range`*"]
    #[allow(clippy::all)]
    pub fn get_client_rects(&self) -> Option<DomRectList> {
        #[cfg(all(feature = "DomRectList", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_client_rects_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<DomRectList> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_client_rects_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<DomRectList> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_client_rects_Range(self_)
            };
            <Option<DomRectList> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_insert_node_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Range as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Node", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `insertNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/insertNode)\n\n*This API requires the following crate features to be activated: `Node`, `Range`*"]
    #[allow(clippy::all)]
    pub fn insert_node(&self, node: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_insert_node_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_insert_node_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(node);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                __widl_f_insert_node_Range(self_, node)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Node", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_intersects_node_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Range as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Node", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `intersectsNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/intersectsNode)\n\n*This API requires the following crate features to be activated: `Node`, `Range`*"]
    #[allow(clippy::all)]
    pub fn intersects_node(&self, node: &Node) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_intersects_node_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_intersects_node_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(node);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                __widl_f_intersects_node_Range(self_, node)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_point_in_range_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Range as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Node", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `isPointInRange()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/isPointInRange)\n\n*This API requires the following crate features to be activated: `Node`, `Range`*"]
    #[allow(clippy::all)]
    pub fn is_point_in_range(
        &self,
        node: &Node,
        offset: u32,
    ) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_point_in_range_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_point_in_range_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(node);
            drop(offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                let offset = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                __widl_f_is_point_in_range_Range(self_, node, offset)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_select_node_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Range as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Node", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `selectNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/selectNode)\n\n*This API requires the following crate features to be activated: `Node`, `Range`*"]
    #[allow(clippy::all)]
    pub fn select_node(&self, ref_node: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_select_node_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ref_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_select_node_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ref_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ref_node);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ref_node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ref_node);
                __widl_f_select_node_Range(self_, ref_node)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Node", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_select_node_contents_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Range as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Node", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `selectNodeContents()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/selectNodeContents)\n\n*This API requires the following crate features to be activated: `Node`, `Range`*"]
    #[allow(clippy::all)]
    pub fn select_node_contents(&self, ref_node: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_select_node_contents_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ref_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_select_node_contents_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ref_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ref_node);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ref_node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ref_node);
                __widl_f_select_node_contents_Range(self_, ref_node)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Node", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_end_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Range as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Node", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `setEnd()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setEnd)\n\n*This API requires the following crate features to be activated: `Node`, `Range`*"]
    #[allow(clippy::all)]
    pub fn set_end(&self, ref_node: &Node, offset: u32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_end_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ref_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_end_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ref_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ref_node);
            drop(offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ref_node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ref_node);
                let offset = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                __widl_f_set_end_Range(self_, ref_node, offset)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Node", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_end_after_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Range as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Node", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `setEndAfter()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setEndAfter)\n\n*This API requires the following crate features to be activated: `Node`, `Range`*"]
    #[allow(clippy::all)]
    pub fn set_end_after(&self, ref_node: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_end_after_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ref_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_end_after_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ref_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ref_node);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ref_node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ref_node);
                __widl_f_set_end_after_Range(self_, ref_node)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Node", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_end_before_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Range as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Node", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `setEndBefore()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setEndBefore)\n\n*This API requires the following crate features to be activated: `Node`, `Range`*"]
    #[allow(clippy::all)]
    pub fn set_end_before(&self, ref_node: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_end_before_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ref_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_end_before_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ref_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ref_node);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ref_node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ref_node);
                __widl_f_set_end_before_Range(self_, ref_node)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Node", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_start_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Range as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Node", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `setStart()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setStart)\n\n*This API requires the following crate features to be activated: `Node`, `Range`*"]
    #[allow(clippy::all)]
    pub fn set_start(&self, ref_node: &Node, offset: u32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_start_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ref_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_start_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ref_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ref_node);
            drop(offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ref_node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ref_node);
                let offset = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                __widl_f_set_start_Range(self_, ref_node, offset)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Node", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_start_after_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Range as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Node", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `setStartAfter()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setStartAfter)\n\n*This API requires the following crate features to be activated: `Node`, `Range`*"]
    #[allow(clippy::all)]
    pub fn set_start_after(&self, ref_node: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_start_after_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ref_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_start_after_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ref_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ref_node);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ref_node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ref_node);
                __widl_f_set_start_after_Range(self_, ref_node)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Node", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_start_before_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Range as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Node", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `setStartBefore()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setStartBefore)\n\n*This API requires the following crate features to be activated: `Node`, `Range`*"]
    #[allow(clippy::all)]
    pub fn set_start_before(&self, ref_node: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_start_before_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ref_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_start_before_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ref_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ref_node);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ref_node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ref_node);
                __widl_f_set_start_before_Range(self_, ref_node)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Node", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_surround_contents_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Range as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Node", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `surroundContents()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/surroundContents)\n\n*This API requires the following crate features to be activated: `Node`, `Range`*"]
    #[allow(clippy::all)]
    pub fn surround_contents(&self, new_parent: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_surround_contents_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                new_parent: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_surround_contents_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            new_parent: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(new_parent);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let new_parent =
                    <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(new_parent);
                __widl_f_surround_contents_Range(self_, new_parent)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Node", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_container_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Range as WasmDescribe>::describe();
    <Node as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Node", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `startContainer` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/startContainer)\n\n*This API requires the following crate features to be activated: `Node`, `Range`*"]
    #[allow(clippy::all)]
    pub fn start_container(&self) -> Result<Node, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_container_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_container_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_start_container_Range(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Node as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_offset_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Range as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `startOffset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/startOffset)\n\n*This API requires the following crate features to be activated: `Range`*"]
    #[allow(clippy::all)]
    pub fn start_offset(&self) -> Result<u32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_offset_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_offset_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_start_offset_Range(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_end_container_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Range as WasmDescribe>::describe();
    <Node as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Node", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `endContainer` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/endContainer)\n\n*This API requires the following crate features to be activated: `Node`, `Range`*"]
    #[allow(clippy::all)]
    pub fn end_container(&self) -> Result<Node, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_end_container_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_end_container_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_end_container_Range(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Node as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_end_offset_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Range as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `endOffset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/endOffset)\n\n*This API requires the following crate features to be activated: `Range`*"]
    #[allow(clippy::all)]
    pub fn end_offset(&self) -> Result<u32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_end_offset_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_end_offset_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_end_offset_Range(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_collapsed_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Range as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `collapsed` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/collapsed)\n\n*This API requires the following crate features to be activated: `Range`*"]
    #[allow(clippy::all)]
    pub fn collapsed(&self) -> bool {
        #[cfg(all(feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_collapsed_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_collapsed_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_collapsed_Range(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_common_ancestor_container_Range() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Range as WasmDescribe>::describe();
    <Node as WasmDescribe>::describe();
}
impl Range {
    #[cfg(all(feature = "Node", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `commonAncestorContainer` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/commonAncestorContainer)\n\n*This API requires the following crate features to be activated: `Node`, `Range`*"]
    #[allow(clippy::all)]
    pub fn common_ancestor_container(&self) -> Result<Node, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_common_ancestor_container_Range(
                self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_common_ancestor_container_Range(
            self_: <&Range as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Range as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_common_ancestor_container_Range(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Node as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
impl Range {
    pub const START_TO_START: u16 = 0i64 as u16;
}
impl Range {
    pub const START_TO_END: u16 = 1u64 as u16;
}
impl Range {
    pub const END_TO_END: u16 = 2u64 as u16;
}
impl Range {
    pub const END_TO_START: u16 = 3u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_e05614a336c488cd: [u8; 2491usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}y\t\0\0\0\0 \0\0\x02\x05Range\x17__widl_instanceof_Range\0\0\0\0\x12__widl_f_new_Range\x01\0\0\x01\x05Range\0\x01\0\x03new\0\0\0\x1D__widl_f_clone_contents_Range\x01\0\0\x01\x05Range\x01\0\0\x01\x01\x05self_\rcloneContents\0\0\0\x1A__widl_f_clone_range_Range\0\0\0\x01\x05Range\x01\0\0\x01\x01\x05self_\ncloneRange\0\0\0\x17__widl_f_collapse_Range\0\0\0\x01\x05Range\x01\0\0\x01\x01\x05self_\x08collapse\0\0\0%__widl_f_collapse_with_to_start_Range\0\0\0\x01\x05Range\x01\0\0\x01\x02\x05self_\x08to_start\x08collapse\0\0\0&__widl_f_compare_boundary_points_Range\x01\0\0\x01\x05Range\x01\0\0\x01\x03\x05self_\x03how\x0Csource_range\x15compareBoundaryPoints\0\0\0\x1C__widl_f_compare_point_Range\x01\0\0\x01\x05Range\x01\0\0\x01\x03\x05self_\x04node\x06offset\x0CcomparePoint\0\0\0)__widl_f_create_contextual_fragment_Range\x01\0\0\x01\x05Range\x01\0\0\x01\x02\x05self_\x08fragment\x18createContextualFragment\0\0\0\x1E__widl_f_delete_contents_Range\x01\0\0\x01\x05Range\x01\0\0\x01\x01\x05self_\x0EdeleteContents\0\0\0\x15__widl_f_detach_Range\0\0\0\x01\x05Range\x01\0\0\x01\x01\x05self_\x06detach\0\0\0\x1F__widl_f_extract_contents_Range\x01\0\0\x01\x05Range\x01\0\0\x01\x01\x05self_\x0FextractContents\0\0\0'__widl_f_get_bounding_client_rect_Range\0\0\0\x01\x05Range\x01\0\0\x01\x01\x05self_\x15getBoundingClientRect\0\0\0\x1F__widl_f_get_client_rects_Range\0\0\0\x01\x05Range\x01\0\0\x01\x01\x05self_\x0EgetClientRects\0\0\0\x1A__widl_f_insert_node_Range\x01\0\0\x01\x05Range\x01\0\0\x01\x02\x05self_\x04node\ninsertNode\0\0\0\x1E__widl_f_intersects_node_Range\x01\0\0\x01\x05Range\x01\0\0\x01\x02\x05self_\x04node\x0EintersectsNode\0\0\0 __widl_f_is_point_in_range_Range\x01\0\0\x01\x05Range\x01\0\0\x01\x03\x05self_\x04node\x06offset\x0EisPointInRange\0\0\0\x1A__widl_f_select_node_Range\x01\0\0\x01\x05Range\x01\0\0\x01\x02\x05self_\x08ref_node\nselectNode\0\0\0#__widl_f_select_node_contents_Range\x01\0\0\x01\x05Range\x01\0\0\x01\x02\x05self_\x08ref_node\x12selectNodeContents\0\0\0\x16__widl_f_set_end_Range\x01\0\0\x01\x05Range\x01\0\0\x01\x03\x05self_\x08ref_node\x06offset\x06setEnd\0\0\0\x1C__widl_f_set_end_after_Range\x01\0\0\x01\x05Range\x01\0\0\x01\x02\x05self_\x08ref_node\x0BsetEndAfter\0\0\0\x1D__widl_f_set_end_before_Range\x01\0\0\x01\x05Range\x01\0\0\x01\x02\x05self_\x08ref_node\x0CsetEndBefore\0\0\0\x18__widl_f_set_start_Range\x01\0\0\x01\x05Range\x01\0\0\x01\x03\x05self_\x08ref_node\x06offset\x08setStart\0\0\0\x1E__widl_f_set_start_after_Range\x01\0\0\x01\x05Range\x01\0\0\x01\x02\x05self_\x08ref_node\rsetStartAfter\0\0\0\x1F__widl_f_set_start_before_Range\x01\0\0\x01\x05Range\x01\0\0\x01\x02\x05self_\x08ref_node\x0EsetStartBefore\0\0\0 __widl_f_surround_contents_Range\x01\0\0\x01\x05Range\x01\0\0\x01\x02\x05self_\nnew_parent\x10surroundContents\0\0\0\x1E__widl_f_start_container_Range\x01\0\0\x01\x05Range\x01\0\x01\x0EstartContainer\x01\x01\x05self_\x0EstartContainer\0\0\0\x1B__widl_f_start_offset_Range\x01\0\0\x01\x05Range\x01\0\x01\x0BstartOffset\x01\x01\x05self_\x0BstartOffset\0\0\0\x1C__widl_f_end_container_Range\x01\0\0\x01\x05Range\x01\0\x01\x0CendContainer\x01\x01\x05self_\x0CendContainer\0\0\0\x19__widl_f_end_offset_Range\x01\0\0\x01\x05Range\x01\0\x01\tendOffset\x01\x01\x05self_\tendOffset\0\0\0\x18__widl_f_collapsed_Range\0\0\0\x01\x05Range\x01\0\x01\tcollapsed\x01\x01\x05self_\tcollapsed\0\0\0(__widl_f_common_ancestor_container_Range\x01\0\0\x01\x05Range\x01\0\x01\x17commonAncestorContainer\x01\x01\x05self_\x17commonAncestorContainer\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
