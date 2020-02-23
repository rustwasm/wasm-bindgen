use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `TreeView` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct TreeView {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_TreeView: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for TreeView {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(8u32);
            inform(84u32);
            inform(114u32);
            inform(101u32);
            inform(101u32);
            inform(86u32);
            inform(105u32);
            inform(101u32);
            inform(119u32);
        }
    }
    impl core::ops::Deref for TreeView {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for TreeView {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for TreeView {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a TreeView {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for TreeView {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            TreeView {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for TreeView {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a TreeView {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for TreeView {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<TreeView>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(TreeView {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for TreeView {
        #[inline]
        fn from(obj: JsValue) -> TreeView {
            TreeView { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for TreeView {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<TreeView> for TreeView {
        #[inline]
        fn as_ref(&self) -> &TreeView {
            self
        }
    }
    impl From<TreeView> for JsValue {
        #[inline]
        fn from(obj: TreeView) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for TreeView {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_TreeView(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_TreeView(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_TreeView(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            TreeView { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const TreeView) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<TreeView> for ::js_sys::Object {
    #[inline]
    fn from(obj: TreeView) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for TreeView {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DataTransfer", feature = "TreeView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_can_drop_TreeView() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&TreeView as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <Option<&DataTransfer> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl TreeView {
    #[cfg(all(feature = "DataTransfer", feature = "TreeView",))]
    #[allow(bad_style)]
    #[doc = "The `canDrop()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/canDrop)\n\n*This API requires the following crate features to be activated: `DataTransfer`, `TreeView`*"]
    #[allow(clippy::all)]
    pub fn can_drop(
        &self,
        row: i32,
        orientation: i32,
        data_transfer: Option<&DataTransfer>,
    ) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DataTransfer", feature = "TreeView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_can_drop_TreeView(
                self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                orientation: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_transfer: <Option<&DataTransfer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_can_drop_TreeView(
            self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            orientation: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_transfer: <Option<&DataTransfer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(row);
            drop(orientation);
            drop(data_transfer);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let row = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(row);
                let orientation =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(orientation);
                let data_transfer =
                    <Option<&DataTransfer> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_transfer,
                    );
                __widl_f_can_drop_TreeView(self_, row, orientation, data_transfer)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DataTransfer", feature = "TreeView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_drop_TreeView() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&TreeView as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <Option<&DataTransfer> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TreeView {
    #[cfg(all(feature = "DataTransfer", feature = "TreeView",))]
    #[allow(bad_style)]
    #[doc = "The `drop()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/drop)\n\n*This API requires the following crate features to be activated: `DataTransfer`, `TreeView`*"]
    #[allow(clippy::all)]
    pub fn drop(
        &self,
        row: i32,
        orientation: i32,
        data_transfer: Option<&DataTransfer>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DataTransfer", feature = "TreeView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_drop_TreeView(
                self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                orientation: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_transfer: <Option<&DataTransfer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_drop_TreeView(
            self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            orientation: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_transfer: <Option<&DataTransfer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(row);
            drop(orientation);
            drop(data_transfer);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let row = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(row);
                let orientation =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(orientation);
                let data_transfer =
                    <Option<&DataTransfer> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_transfer,
                    );
                __widl_f_drop_TreeView(self_, row, orientation, data_transfer)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "TreeView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_level_TreeView() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TreeView as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl TreeView {
    #[cfg(all(feature = "TreeView",))]
    #[allow(bad_style)]
    #[doc = "The `getLevel()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/getLevel)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    #[allow(clippy::all)]
    pub fn get_level(&self, row: i32) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TreeView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_level_TreeView(
                self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_level_TreeView(
            self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(row);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let row = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(row);
                __widl_f_get_level_TreeView(self_, row)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TreeView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_parent_index_TreeView() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TreeView as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl TreeView {
    #[cfg(all(feature = "TreeView",))]
    #[allow(bad_style)]
    #[doc = "The `getParentIndex()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/getParentIndex)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    #[allow(clippy::all)]
    pub fn get_parent_index(&self, row: i32) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TreeView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_parent_index_TreeView(
                self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_parent_index_TreeView(
            self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(row);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let row = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(row);
                __widl_f_get_parent_index_TreeView(self_, row)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TreeView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_row_properties_TreeView() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TreeView as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl TreeView {
    #[cfg(all(feature = "TreeView",))]
    #[allow(bad_style)]
    #[doc = "The `getRowProperties()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/getRowProperties)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    #[allow(clippy::all)]
    pub fn get_row_properties(&self, row: i32) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TreeView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_row_properties_TreeView(
                self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_row_properties_TreeView(
            self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(row);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let row = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(row);
                __widl_f_get_row_properties_TreeView(self_, row)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "TreeView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_next_sibling_TreeView() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&TreeView as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl TreeView {
    #[cfg(all(feature = "TreeView",))]
    #[allow(bad_style)]
    #[doc = "The `hasNextSibling()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/hasNextSibling)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    #[allow(clippy::all)]
    pub fn has_next_sibling(
        &self,
        row: i32,
        after_index: i32,
    ) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TreeView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_next_sibling_TreeView(
                self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                after_index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_next_sibling_TreeView(
            self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            after_index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(row);
            drop(after_index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let row = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(row);
                let after_index =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(after_index);
                __widl_f_has_next_sibling_TreeView(self_, row, after_index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TreeView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_container_TreeView() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TreeView as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl TreeView {
    #[cfg(all(feature = "TreeView",))]
    #[allow(bad_style)]
    #[doc = "The `isContainer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isContainer)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    #[allow(clippy::all)]
    pub fn is_container(&self, row: i32) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TreeView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_container_TreeView(
                self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_container_TreeView(
            self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(row);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let row = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(row);
                __widl_f_is_container_TreeView(self_, row)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TreeView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_container_empty_TreeView() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TreeView as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl TreeView {
    #[cfg(all(feature = "TreeView",))]
    #[allow(bad_style)]
    #[doc = "The `isContainerEmpty()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isContainerEmpty)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    #[allow(clippy::all)]
    pub fn is_container_empty(&self, row: i32) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TreeView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_container_empty_TreeView(
                self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_container_empty_TreeView(
            self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(row);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let row = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(row);
                __widl_f_is_container_empty_TreeView(self_, row)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TreeView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_container_open_TreeView() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TreeView as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl TreeView {
    #[cfg(all(feature = "TreeView",))]
    #[allow(bad_style)]
    #[doc = "The `isContainerOpen()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isContainerOpen)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    #[allow(clippy::all)]
    pub fn is_container_open(&self, row: i32) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TreeView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_container_open_TreeView(
                self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_container_open_TreeView(
            self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(row);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let row = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(row);
                __widl_f_is_container_open_TreeView(self_, row)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TreeView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_separator_TreeView() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TreeView as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl TreeView {
    #[cfg(all(feature = "TreeView",))]
    #[allow(bad_style)]
    #[doc = "The `isSeparator()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isSeparator)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    #[allow(clippy::all)]
    pub fn is_separator(&self, row: i32) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TreeView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_separator_TreeView(
                self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_separator_TreeView(
            self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(row);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let row = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(row);
                __widl_f_is_separator_TreeView(self_, row)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TreeView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_sorted_TreeView() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeView as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl TreeView {
    #[cfg(all(feature = "TreeView",))]
    #[allow(bad_style)]
    #[doc = "The `isSorted()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isSorted)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    #[allow(clippy::all)]
    pub fn is_sorted(&self) -> bool {
        #[cfg(all(feature = "TreeView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_sorted_TreeView(
                self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_sorted_TreeView(
            self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_is_sorted_TreeView(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TreeView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_perform_action_TreeView() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TreeView as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TreeView {
    #[cfg(all(feature = "TreeView",))]
    #[allow(bad_style)]
    #[doc = "The `performAction()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/performAction)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    #[allow(clippy::all)]
    pub fn perform_action(&self, action: &str) {
        #[cfg(all(feature = "TreeView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_perform_action_TreeView(
                self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                action: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_perform_action_TreeView(
            self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            action: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(action);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let action = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(action);
                __widl_f_perform_action_TreeView(self_, action)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TreeView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_perform_action_on_row_TreeView() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&TreeView as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TreeView {
    #[cfg(all(feature = "TreeView",))]
    #[allow(bad_style)]
    #[doc = "The `performActionOnRow()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/performActionOnRow)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    #[allow(clippy::all)]
    pub fn perform_action_on_row(&self, action: &str, row: i32) {
        #[cfg(all(feature = "TreeView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_perform_action_on_row_TreeView(
                self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                action: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_perform_action_on_row_TreeView(
            self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            action: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(action);
            drop(row);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let action = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(action);
                let row = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(row);
                __widl_f_perform_action_on_row_TreeView(self_, action, row)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TreeView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_selection_changed_TreeView() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeView as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TreeView {
    #[cfg(all(feature = "TreeView",))]
    #[allow(bad_style)]
    #[doc = "The `selectionChanged()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/selectionChanged)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    #[allow(clippy::all)]
    pub fn selection_changed(&self) {
        #[cfg(all(feature = "TreeView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_selection_changed_TreeView(
                self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_selection_changed_TreeView(
            self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_selection_changed_TreeView(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TreeBoxObject", feature = "TreeView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_tree_TreeView() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TreeView as WasmDescribe>::describe();
    <Option<&TreeBoxObject> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TreeView {
    #[cfg(all(feature = "TreeBoxObject", feature = "TreeView",))]
    #[allow(bad_style)]
    #[doc = "The `setTree()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/setTree)\n\n*This API requires the following crate features to be activated: `TreeBoxObject`, `TreeView`*"]
    #[allow(clippy::all)]
    pub fn set_tree(&self, tree: Option<&TreeBoxObject>) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TreeBoxObject", feature = "TreeView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_tree_TreeView(
                self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tree: <Option<&TreeBoxObject> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_tree_TreeView(
            self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tree: <Option<&TreeBoxObject> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tree);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tree =
                    <Option<&TreeBoxObject> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tree);
                __widl_f_set_tree_TreeView(self_, tree)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "TreeView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_toggle_open_state_TreeView() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TreeView as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TreeView {
    #[cfg(all(feature = "TreeView",))]
    #[allow(bad_style)]
    #[doc = "The `toggleOpenState()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/toggleOpenState)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    #[allow(clippy::all)]
    pub fn toggle_open_state(&self, row: i32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TreeView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_toggle_open_state_TreeView(
                self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_toggle_open_state_TreeView(
            self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            row: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(row);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let row = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(row);
                __widl_f_toggle_open_state_TreeView(self_, row)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "TreeView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_row_count_TreeView() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TreeView as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl TreeView {
    #[cfg(all(feature = "TreeView",))]
    #[allow(bad_style)]
    #[doc = "The `rowCount` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/rowCount)\n\n*This API requires the following crate features to be activated: `TreeView`*"]
    #[allow(clippy::all)]
    pub fn row_count(&self) -> i32 {
        #[cfg(all(feature = "TreeView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_row_count_TreeView(
                self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_row_count_TreeView(
            self_: <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TreeView as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_row_count_TreeView(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl TreeView {
    pub const DROP_BEFORE: i16 = -1i64 as i16;
}
impl TreeView {
    pub const DROP_ON: i16 = 0i64 as i16;
}
impl TreeView {
    pub const DROP_AFTER: i16 = 1u64 as i16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_4c9f458492be715c: [u8; 1510usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xA4\x05\0\0\0\0\x12\0\0\x02\x08TreeView\x1A__widl_instanceof_TreeView\0\0\0\0\x1A__widl_f_can_drop_TreeView\x01\0\0\x01\x08TreeView\x01\0\0\x01\x04\x05self_\x03row\x0Borientation\rdata_transfer\x07canDrop\0\0\0\x16__widl_f_drop_TreeView\x01\0\0\x01\x08TreeView\x01\0\0\x01\x04\x05self_\x03row\x0Borientation\rdata_transfer\x04drop\0\0\0\x1B__widl_f_get_level_TreeView\x01\0\0\x01\x08TreeView\x01\0\0\x01\x02\x05self_\x03row\x08getLevel\0\0\0\"__widl_f_get_parent_index_TreeView\x01\0\0\x01\x08TreeView\x01\0\0\x01\x02\x05self_\x03row\x0EgetParentIndex\0\0\0$__widl_f_get_row_properties_TreeView\x01\0\0\x01\x08TreeView\x01\0\0\x01\x02\x05self_\x03row\x10getRowProperties\0\0\0\"__widl_f_has_next_sibling_TreeView\x01\0\0\x01\x08TreeView\x01\0\0\x01\x03\x05self_\x03row\x0Bafter_index\x0EhasNextSibling\0\0\0\x1E__widl_f_is_container_TreeView\x01\0\0\x01\x08TreeView\x01\0\0\x01\x02\x05self_\x03row\x0BisContainer\0\0\0$__widl_f_is_container_empty_TreeView\x01\0\0\x01\x08TreeView\x01\0\0\x01\x02\x05self_\x03row\x10isContainerEmpty\0\0\0#__widl_f_is_container_open_TreeView\x01\0\0\x01\x08TreeView\x01\0\0\x01\x02\x05self_\x03row\x0FisContainerOpen\0\0\0\x1E__widl_f_is_separator_TreeView\x01\0\0\x01\x08TreeView\x01\0\0\x01\x02\x05self_\x03row\x0BisSeparator\0\0\0\x1B__widl_f_is_sorted_TreeView\0\0\0\x01\x08TreeView\x01\0\0\x01\x01\x05self_\x08isSorted\0\0\0 __widl_f_perform_action_TreeView\0\0\0\x01\x08TreeView\x01\0\0\x01\x02\x05self_\x06action\rperformAction\0\0\0'__widl_f_perform_action_on_row_TreeView\0\0\0\x01\x08TreeView\x01\0\0\x01\x03\x05self_\x06action\x03row\x12performActionOnRow\0\0\0#__widl_f_selection_changed_TreeView\0\0\0\x01\x08TreeView\x01\0\0\x01\x01\x05self_\x10selectionChanged\0\0\0\x1A__widl_f_set_tree_TreeView\x01\0\0\x01\x08TreeView\x01\0\0\x01\x02\x05self_\x04tree\x07setTree\0\0\0#__widl_f_toggle_open_state_TreeView\x01\0\0\x01\x08TreeView\x01\0\0\x01\x02\x05self_\x03row\x0FtoggleOpenState\0\0\0\x1B__widl_f_row_count_TreeView\0\0\0\x01\x08TreeView\x01\0\x01\x08rowCount\x01\x01\x05self_\x08rowCount\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
