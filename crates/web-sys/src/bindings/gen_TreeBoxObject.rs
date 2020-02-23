use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `TreeBoxObject` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct TreeBoxObject {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_TreeBoxObject: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for TreeBoxObject {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(84u32);
            inform(114u32);
            inform(101u32);
            inform(101u32);
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
    impl core::ops::Deref for TreeBoxObject {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for TreeBoxObject {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for TreeBoxObject {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a TreeBoxObject {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for TreeBoxObject {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            TreeBoxObject {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for TreeBoxObject {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a TreeBoxObject {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for TreeBoxObject {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<TreeBoxObject>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(TreeBoxObject {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for TreeBoxObject {
        #[inline]
        fn from(obj: JsValue) -> TreeBoxObject {
            TreeBoxObject { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for TreeBoxObject {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<TreeBoxObject> for TreeBoxObject {
        #[inline]
        fn as_ref(&self) -> &TreeBoxObject {
            self
        }
    }
    impl From<TreeBoxObject> for JsValue {
        #[inline]
        fn from(obj: TreeBoxObject) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for TreeBoxObject {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_TreeBoxObject(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_TreeBoxObject(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_TreeBoxObject(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            TreeBoxObject { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const TreeBoxObject) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<TreeBoxObject> for ::js_sys::Object {
    #[inline]
    fn from(obj: TreeBoxObject) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for TreeBoxObject {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_begin_update_batch_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `beginUpdateBatch()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/beginUpdateBatch)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn begin_update_batch(&self) {
        #[cfg(all(feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_begin_update_batch_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_begin_update_batch_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_begin_update_batch_TreeBoxObject(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_style_and_image_caches_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `clearStyleAndImageCaches()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/clearStyleAndImageCaches)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn clear_style_and_image_caches(&self) {
        #[cfg(all(feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_style_and_image_caches_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_style_and_image_caches_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clear_style_and_image_caches_TreeBoxObject(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_end_update_batch_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `endUpdateBatch()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/endUpdateBatch)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn end_update_batch(&self) {
        #[cfg(all(feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_end_update_batch_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_end_update_batch_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_end_update_batch_TreeBoxObject(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ensure_row_is_visible_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `ensureRowIsVisible()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/ensureRowIsVisible)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn ensure_row_is_visible(&self, index: i32) {
        #[cfg(all(feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ensure_row_is_visible_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ensure_row_is_visible_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_ensure_row_is_visible_TreeBoxObject(self_, index)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TreeBoxObject", feature = "TreeCellInfo",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_cell_at_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <TreeCellInfo as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject", feature = "TreeCellInfo",))]
    #[allow(bad_style)]
    #[doc = "The `getCellAt()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getCellAt)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`, `TreeCellInfo`*"]
    #[allow(clippy::all)]
    pub fn get_cell_at(&self, x: i32, y: i32) -> Result<TreeCellInfo, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TreeBoxObject", feature = "TreeCellInfo",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_cell_at_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TreeCellInfo as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_cell_at_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TreeCellInfo as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_get_cell_at_TreeBoxObject(self_, x, y)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TreeCellInfo as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_cell_at_with_row_and_column_and_child_elt_TreeBoxObject(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `getCellAt()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getCellAt)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn get_cell_at_with_row_and_column_and_child_elt(
        &self,
        x: i32,
        y: i32,
        row: &::js_sys::Object,
        column: &::js_sys::Object,
        child_elt: &::js_sys::Object,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_cell_at_with_row_and_column_and_child_elt_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                row: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                column: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                child_elt: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_cell_at_with_row_and_column_and_child_elt_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            row: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            column: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            child_elt: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(row);
            drop(column);
            drop(child_elt);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let row = <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(row);
                let column =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(column);
                let child_elt =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(child_elt);
                __widl_f_get_cell_at_with_row_and_column_and_child_elt_TreeBoxObject(
                    self_, x, y, row, column, child_elt,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_first_visible_row_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `getFirstVisibleRow()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getFirstVisibleRow)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn get_first_visible_row(&self) -> i32 {
        #[cfg(all(feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_first_visible_row_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_first_visible_row_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_first_visible_row_TreeBoxObject(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_last_visible_row_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `getLastVisibleRow()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getLastVisibleRow)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn get_last_visible_row(&self) -> i32 {
        #[cfg(all(feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_last_visible_row_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_last_visible_row_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_last_visible_row_TreeBoxObject(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_page_length_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `getPageLength()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getPageLength)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn get_page_length(&self) -> i32 {
        #[cfg(all(feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_page_length_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_page_length_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_page_length_TreeBoxObject(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_row_at_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `getRowAt()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getRowAt)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn get_row_at(&self, x: i32, y: i32) -> i32 {
        #[cfg(all(feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_row_at_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_row_at_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_get_row_at_TreeBoxObject(self_, x, y)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_invalidate_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `invalidate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/invalidate)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn invalidate(&self) {
        #[cfg(all(feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_invalidate_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_invalidate_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_invalidate_TreeBoxObject(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_invalidate_range_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `invalidateRange()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/invalidateRange)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn invalidate_range(&self, start_index: i32, end_index: i32) {
        #[cfg(all(feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_invalidate_range_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end_index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_invalidate_range_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end_index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(start_index);
            drop(end_index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start_index =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_index);
                let end_index = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end_index);
                __widl_f_invalidate_range_TreeBoxObject(self_, start_index, end_index)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_invalidate_row_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `invalidateRow()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/invalidateRow)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn invalidate_row(&self, index: i32) {
        #[cfg(all(feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_invalidate_row_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_invalidate_row_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_invalidate_row_TreeBoxObject(self_, index)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_row_count_changed_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `rowCountChanged()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/rowCountChanged)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn row_count_changed(&self, index: i32, count: i32) {
        #[cfg(all(feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_row_count_changed_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                count: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_row_count_changed_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            count: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(index);
            drop(count);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                let count = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(count);
                __widl_f_row_count_changed_TreeBoxObject(self_, index, count)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_by_lines_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `scrollByLines()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/scrollByLines)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn scroll_by_lines(&self, num_lines: i32) {
        #[cfg(all(feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_by_lines_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                num_lines: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_by_lines_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let num_lines = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(num_lines);
                __widl_f_scroll_by_lines_TreeBoxObject(self_, num_lines)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_by_pages_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `scrollByPages()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/scrollByPages)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn scroll_by_pages(&self, num_pages: i32) {
        #[cfg(all(feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_by_pages_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                num_pages: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_by_pages_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            num_pages: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(num_pages);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let num_pages = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(num_pages);
                __widl_f_scroll_by_pages_TreeBoxObject(self_, num_pages)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_to_row_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `scrollToRow()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/scrollToRow)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn scroll_to_row(&self, index: i32) {
        #[cfg(all(feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_to_row_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_to_row_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_scroll_to_row_TreeBoxObject(self_, index)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_focused_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `focused` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/focused)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn focused(&self) -> bool {
        #[cfg(all(feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_focused_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_focused_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_focused_TreeBoxObject(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_focused_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `focused` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/focused)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn set_focused(&self, focused: bool) {
        #[cfg(all(feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_focused_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                focused: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_focused_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            focused: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(focused);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let focused = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(focused);
                __widl_f_set_focused_TreeBoxObject(self_, focused)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element", feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tree_body_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "Element", feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `treeBody` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/treeBody)\n\n*This API requires the following crate features to be activated: `Element`, `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn tree_body(&self) -> Option<Element> {
        #[cfg(all(feature = "Element", feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tree_body_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tree_body_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_tree_body_TreeBoxObject(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_row_height_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `rowHeight` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/rowHeight)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn row_height(&self) -> i32 {
        #[cfg(all(feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_row_height_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_row_height_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_row_height_TreeBoxObject(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_row_width_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `rowWidth` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/rowWidth)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn row_width(&self) -> i32 {
        #[cfg(all(feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_row_width_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_row_width_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_row_width_TreeBoxObject(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TreeBoxObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_horizontal_position_TreeBoxObject() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeBoxObject as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl TreeBoxObject {
    #[cfg(all(feature = "TreeBoxObject",))]
    #[allow(bad_style)]
    #[doc = "The `horizontalPosition` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/horizontalPosition)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    #[allow(clippy::all)]
    pub fn horizontal_position(&self) -> i32 {
        #[cfg(all(feature = "TreeBoxObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_horizontal_position_TreeBoxObject(
                self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_horizontal_position_TreeBoxObject(
            self_: <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeBoxObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_horizontal_position_TreeBoxObject(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_ec4a1fda0bdd0678: [u8; 2305usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xBF\x08\0\0\0\0\x18\0\0\x02\rTreeBoxObject\x1F__widl_instanceof_TreeBoxObject\0\0\0\0)__widl_f_begin_update_batch_TreeBoxObject\0\0\0\x01\rTreeBoxObject\x01\0\0\x01\x01\x05self_\x10beginUpdateBatch\0\0\03__widl_f_clear_style_and_image_caches_TreeBoxObject\0\0\0\x01\rTreeBoxObject\x01\0\0\x01\x01\x05self_\x18clearStyleAndImageCaches\0\0\0'__widl_f_end_update_batch_TreeBoxObject\0\0\0\x01\rTreeBoxObject\x01\0\0\x01\x01\x05self_\x0EendUpdateBatch\0\0\0,__widl_f_ensure_row_is_visible_TreeBoxObject\0\0\0\x01\rTreeBoxObject\x01\0\0\x01\x02\x05self_\x05index\x12ensureRowIsVisible\0\0\0\"__widl_f_get_cell_at_TreeBoxObject\x01\0\0\x01\rTreeBoxObject\x01\0\0\x01\x03\x05self_\x01x\x01y\tgetCellAt\0\0\0D__widl_f_get_cell_at_with_row_and_column_and_child_elt_TreeBoxObject\x01\0\0\x01\rTreeBoxObject\x01\0\0\x01\x06\x05self_\x01x\x01y\x03row\x06column\tchild_elt\tgetCellAt\0\0\0,__widl_f_get_first_visible_row_TreeBoxObject\0\0\0\x01\rTreeBoxObject\x01\0\0\x01\x01\x05self_\x12getFirstVisibleRow\0\0\0+__widl_f_get_last_visible_row_TreeBoxObject\0\0\0\x01\rTreeBoxObject\x01\0\0\x01\x01\x05self_\x11getLastVisibleRow\0\0\0&__widl_f_get_page_length_TreeBoxObject\0\0\0\x01\rTreeBoxObject\x01\0\0\x01\x01\x05self_\rgetPageLength\0\0\0!__widl_f_get_row_at_TreeBoxObject\0\0\0\x01\rTreeBoxObject\x01\0\0\x01\x03\x05self_\x01x\x01y\x08getRowAt\0\0\0!__widl_f_invalidate_TreeBoxObject\0\0\0\x01\rTreeBoxObject\x01\0\0\x01\x01\x05self_\ninvalidate\0\0\0'__widl_f_invalidate_range_TreeBoxObject\0\0\0\x01\rTreeBoxObject\x01\0\0\x01\x03\x05self_\x0Bstart_index\tend_index\x0FinvalidateRange\0\0\0%__widl_f_invalidate_row_TreeBoxObject\0\0\0\x01\rTreeBoxObject\x01\0\0\x01\x02\x05self_\x05index\rinvalidateRow\0\0\0(__widl_f_row_count_changed_TreeBoxObject\0\0\0\x01\rTreeBoxObject\x01\0\0\x01\x03\x05self_\x05index\x05count\x0FrowCountChanged\0\0\0&__widl_f_scroll_by_lines_TreeBoxObject\0\0\0\x01\rTreeBoxObject\x01\0\0\x01\x02\x05self_\tnum_lines\rscrollByLines\0\0\0&__widl_f_scroll_by_pages_TreeBoxObject\0\0\0\x01\rTreeBoxObject\x01\0\0\x01\x02\x05self_\tnum_pages\rscrollByPages\0\0\0$__widl_f_scroll_to_row_TreeBoxObject\0\0\0\x01\rTreeBoxObject\x01\0\0\x01\x02\x05self_\x05index\x0BscrollToRow\0\0\0\x1E__widl_f_focused_TreeBoxObject\0\0\0\x01\rTreeBoxObject\x01\0\x01\x07focused\x01\x01\x05self_\x07focused\0\0\0\"__widl_f_set_focused_TreeBoxObject\0\0\0\x01\rTreeBoxObject\x01\0\x02\x07focused\x01\x02\x05self_\x07focused\x07focused\0\0\0 __widl_f_tree_body_TreeBoxObject\0\0\0\x01\rTreeBoxObject\x01\0\x01\x08treeBody\x01\x01\x05self_\x08treeBody\0\0\0!__widl_f_row_height_TreeBoxObject\0\0\0\x01\rTreeBoxObject\x01\0\x01\trowHeight\x01\x01\x05self_\trowHeight\0\0\0 __widl_f_row_width_TreeBoxObject\0\0\0\x01\rTreeBoxObject\x01\0\x01\x08rowWidth\x01\x01\x05self_\x08rowWidth\0\0\0*__widl_f_horizontal_position_TreeBoxObject\0\0\0\x01\rTreeBoxObject\x01\0\x01\x12horizontalPosition\x01\x01\x05self_\x12horizontalPosition\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
