use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ListBoxObject` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject)\n\n*This API requires the following crate features to be activated: `ListBoxObject`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ListBoxObject {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ListBoxObject: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ListBoxObject {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(76u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
            inform(66u32);
            inform(111u32);
            inform(120u32);
            inform(79u32);
            inform(98u32);
            inform(106u32);
            inform(101u32);
            inform(99u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for ListBoxObject {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for ListBoxObject {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ListBoxObject {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ListBoxObject {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ListBoxObject {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ListBoxObject {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ListBoxObject {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ListBoxObject {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ListBoxObject {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ListBoxObject>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ListBoxObject {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ListBoxObject {
        #[inline]
        fn from(obj: JsValue) -> ListBoxObject {
            ListBoxObject { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ListBoxObject {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ListBoxObject> for ListBoxObject {
        #[inline]
        fn as_ref(&self) -> &ListBoxObject {
            self
        }
    }
    impl From<ListBoxObject> for JsValue {
        #[inline]
        fn from(obj: ListBoxObject) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ListBoxObject {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ListBoxObject(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ListBoxObject(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ListBoxObject(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ListBoxObject { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ListBoxObject) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ListBoxObject> for ::js_sys::Object {
    #[inline]
    fn from(obj: ListBoxObject) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ListBoxObject {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ListBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ensure_index_is_visible_ListBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ListBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ListBoxObject {
    #[cfg(all(feature = "ListBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `ensureIndexIsVisible()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/ensureIndexIsVisible)\n\n*This API requires the following crate features to be activated: `ListBoxObject`*"]
    #[allow(clippy::all)]
    pub fn ensure_index_is_visible(&self, row_index: i32) {
        #[cfg(all(feature = "ListBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ensure_index_is_visible_ListBoxObject(
                self_: <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                row_index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ensure_index_is_visible_ListBoxObject(
            self_: <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            row_index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(row_index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let row_index = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(row_index);
                __widl_f_ensure_index_is_visible_ListBoxObject(self_, row_index)
            };
            ()
        }
    }
}
#[cfg(all(feature = "ListBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_index_of_first_visible_row_ListBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ListBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl ListBoxObject {
    #[cfg(all(feature = "ListBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `getIndexOfFirstVisibleRow()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getIndexOfFirstVisibleRow)\n\n*This API requires the following crate features to be activated: `ListBoxObject`*"]
    #[allow(clippy::all)]
    pub fn get_index_of_first_visible_row(&self) -> i32 {
        #[cfg(all(feature = "ListBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_index_of_first_visible_row_ListBoxObject(
                self_: <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_index_of_first_visible_row_ListBoxObject(
            self_: <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_index_of_first_visible_row_ListBoxObject(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element", feature = "ListBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_index_of_item_ListBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ListBoxObject as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl ListBoxObject {
    #[cfg(all(feature = "Element", feature = "ListBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `getIndexOfItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getIndexOfItem)\n\n*This API requires the following crate features to be activated: `Element`, `ListBoxObject`*"]
    #[allow(clippy::all)]
    pub fn get_index_of_item(&self, item: &Element) -> i32 {
        #[cfg(all(feature = "Element", feature = "ListBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_index_of_item_ListBoxObject(
                self_: <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                item: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_index_of_item_ListBoxObject(
            self_: <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            item: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(item);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let item = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(item);
                __widl_f_get_index_of_item_ListBoxObject(self_, item)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element", feature = "ListBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_item_at_index_ListBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ListBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl ListBoxObject {
    #[cfg(all(feature = "Element", feature = "ListBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `getItemAtIndex()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getItemAtIndex)\n\n*This API requires the following crate features to be activated: `Element`, `ListBoxObject`*"]
    #[allow(clippy::all)]
    pub fn get_item_at_index(&self, index: i32) -> Option<Element> {
        #[cfg(all(feature = "Element", feature = "ListBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_item_at_index_ListBoxObject(
                self_: <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_item_at_index_ListBoxObject(
            self_: <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_item_at_index_ListBoxObject(self_, index)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ListBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_number_of_visible_rows_ListBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ListBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl ListBoxObject {
    #[cfg(all(feature = "ListBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `getNumberOfVisibleRows()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getNumberOfVisibleRows)\n\n*This API requires the following crate features to be activated: `ListBoxObject`*"]
    #[allow(clippy::all)]
    pub fn get_number_of_visible_rows(&self) -> i32 {
        #[cfg(all(feature = "ListBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_number_of_visible_rows_ListBoxObject(
                self_: <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_number_of_visible_rows_ListBoxObject(
            self_: <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_number_of_visible_rows_ListBoxObject(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ListBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_row_count_ListBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ListBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl ListBoxObject {
    #[cfg(all(feature = "ListBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `getRowCount()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getRowCount)\n\n*This API requires the following crate features to be activated: `ListBoxObject`*"]
    #[allow(clippy::all)]
    pub fn get_row_count(&self) -> i32 {
        #[cfg(all(feature = "ListBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_row_count_ListBoxObject(
                self_: <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_row_count_ListBoxObject(
            self_: <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_row_count_ListBoxObject(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ListBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_row_height_ListBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ListBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl ListBoxObject {
    #[cfg(all(feature = "ListBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `getRowHeight()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getRowHeight)\n\n*This API requires the following crate features to be activated: `ListBoxObject`*"]
    #[allow(clippy::all)]
    pub fn get_row_height(&self) -> i32 {
        #[cfg(all(feature = "ListBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_row_height_ListBoxObject(
                self_: <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_row_height_ListBoxObject(
            self_: <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_row_height_ListBoxObject(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ListBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_by_lines_ListBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ListBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ListBoxObject {
    #[cfg(all(feature = "ListBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `scrollByLines()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/scrollByLines)\n\n*This API requires the following crate features to be activated: `ListBoxObject`*"]
    #[allow(clippy::all)]
    pub fn scroll_by_lines(&self, num_lines: i32) {
        #[cfg(all(feature = "ListBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_by_lines_ListBoxObject(
                self_: <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                num_lines: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_by_lines_ListBoxObject(
            self_: <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            num_lines: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(num_lines);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let num_lines = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(num_lines);
                __widl_f_scroll_by_lines_ListBoxObject(self_, num_lines)
            };
            ()
        }
    }
}
#[cfg(all(feature = "ListBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_to_index_ListBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ListBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ListBoxObject {
    #[cfg(all(feature = "ListBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `scrollToIndex()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/scrollToIndex)\n\n*This API requires the following crate features to be activated: `ListBoxObject`*"]
    #[allow(clippy::all)]
    pub fn scroll_to_index(&self, row_index: i32) {
        #[cfg(all(feature = "ListBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_to_index_ListBoxObject(
                self_: <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                row_index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_to_index_ListBoxObject(
            self_: <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            row_index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(row_index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ListBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let row_index = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(row_index);
                __widl_f_scroll_to_index_ListBoxObject(self_, row_index)
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
pub static __WASM_BINDGEN_GENERATED_7fbaef7cafa37ce2: [u8; 1023usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xBD\x03\0\0\0\0\n\0\0\x02\rListBoxObject\x1F__widl_instanceof_ListBoxObject\0\0\0\0.__widl_f_ensure_index_is_visible_ListBoxObject\0\0\0\x01\rListBoxObject\x01\0\0\x01\x02\x05self_\trow_index\x14ensureIndexIsVisible\0\0\05__widl_f_get_index_of_first_visible_row_ListBoxObject\0\0\0\x01\rListBoxObject\x01\0\0\x01\x01\x05self_\x19getIndexOfFirstVisibleRow\0\0\0(__widl_f_get_index_of_item_ListBoxObject\0\0\0\x01\rListBoxObject\x01\0\0\x01\x02\x05self_\x04item\x0EgetIndexOfItem\0\0\0(__widl_f_get_item_at_index_ListBoxObject\0\0\0\x01\rListBoxObject\x01\0\0\x01\x02\x05self_\x05index\x0EgetItemAtIndex\0\0\01__widl_f_get_number_of_visible_rows_ListBoxObject\0\0\0\x01\rListBoxObject\x01\0\0\x01\x01\x05self_\x16getNumberOfVisibleRows\0\0\0$__widl_f_get_row_count_ListBoxObject\0\0\0\x01\rListBoxObject\x01\0\0\x01\x01\x05self_\x0BgetRowCount\0\0\0%__widl_f_get_row_height_ListBoxObject\0\0\0\x01\rListBoxObject\x01\0\0\x01\x01\x05self_\x0CgetRowHeight\0\0\0&__widl_f_scroll_by_lines_ListBoxObject\0\0\0\x01\rListBoxObject\x01\0\0\x01\x02\x05self_\tnum_lines\rscrollByLines\0\0\0&__widl_f_scroll_to_index_ListBoxObject\0\0\0\x01\rListBoxObject\x01\0\0\x01\x02\x05self_\trow_index\rscrollToIndex\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
