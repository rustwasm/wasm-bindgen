use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DataTransferItemList` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList)\n\n*This API requires the following crate features to be activated: `DataTransferItemList`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DataTransferItemList {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DataTransferItemList: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DataTransferItemList {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(20u32);
            inform(68u32);
            inform(97u32);
            inform(116u32);
            inform(97u32);
            inform(84u32);
            inform(114u32);
            inform(97u32);
            inform(110u32);
            inform(115u32);
            inform(102u32);
            inform(101u32);
            inform(114u32);
            inform(73u32);
            inform(116u32);
            inform(101u32);
            inform(109u32);
            inform(76u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for DataTransferItemList {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for DataTransferItemList {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DataTransferItemList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DataTransferItemList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DataTransferItemList {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DataTransferItemList {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DataTransferItemList {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DataTransferItemList {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DataTransferItemList {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DataTransferItemList>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DataTransferItemList {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DataTransferItemList {
        #[inline]
        fn from(obj: JsValue) -> DataTransferItemList {
            DataTransferItemList { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DataTransferItemList {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DataTransferItemList> for DataTransferItemList {
        #[inline]
        fn as_ref(&self) -> &DataTransferItemList {
            self
        }
    }
    impl From<DataTransferItemList> for JsValue {
        #[inline]
        fn from(obj: DataTransferItemList) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DataTransferItemList {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DataTransferItemList(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DataTransferItemList(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DataTransferItemList(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DataTransferItemList { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DataTransferItemList) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DataTransferItemList> for ::js_sys::Object {
    #[inline]
    fn from(obj: DataTransferItemList) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DataTransferItemList {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DataTransferItem", feature = "DataTransferItemList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_with_str_and_type_DataTransferItemList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DataTransferItemList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<DataTransferItem> as WasmDescribe>::describe();
}
impl DataTransferItemList {
    #[cfg(all(feature = "DataTransferItem", feature = "DataTransferItemList",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/add)\n\n*This API requires the following crate features to be activated: `DataTransferItem`, `DataTransferItemList`*"]
    #[allow(clippy::all)]
    pub fn add_with_str_and_type(
        &self,
        data: &str,
        type_: &str,
    ) -> Result<Option<DataTransferItem>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DataTransferItem", feature = "DataTransferItemList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_with_str_and_type_DataTransferItemList(
                self_: <&DataTransferItemList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<DataTransferItem> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_with_str_and_type_DataTransferItemList(
            self_: <&DataTransferItemList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<DataTransferItem> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(data);
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DataTransferItemList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_add_with_str_and_type_DataTransferItemList(self_, data, type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<DataTransferItem> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "DataTransferItem",
    feature = "DataTransferItemList",
    feature = "File",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_with_file_DataTransferItemList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DataTransferItemList as WasmDescribe>::describe();
    <&File as WasmDescribe>::describe();
    <Option<DataTransferItem> as WasmDescribe>::describe();
}
impl DataTransferItemList {
    #[cfg(all(
        feature = "DataTransferItem",
        feature = "DataTransferItemList",
        feature = "File",
    ))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/add)\n\n*This API requires the following crate features to be activated: `DataTransferItem`, `DataTransferItemList`, `File`*"]
    #[allow(clippy::all)]
    pub fn add_with_file(
        &self,
        data: &File,
    ) -> Result<Option<DataTransferItem>, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "DataTransferItem",
            feature = "DataTransferItemList",
            feature = "File",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_with_file_DataTransferItemList(
                self_: <&DataTransferItemList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&File as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<DataTransferItem> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_with_file_DataTransferItemList(
            self_: <&DataTransferItemList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&File as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<DataTransferItem> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DataTransferItemList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data = <&File as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_add_with_file_DataTransferItemList(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<DataTransferItem> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DataTransferItemList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_DataTransferItemList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DataTransferItemList as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DataTransferItemList {
    #[cfg(all(feature = "DataTransferItemList",))]
    #[allow(bad_style)]
    #[doc = "The `clear()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/clear)\n\n*This API requires the following crate features to be activated: `DataTransferItemList`*"]
    #[allow(clippy::all)]
    pub fn clear(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DataTransferItemList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_DataTransferItemList(
                self_: <&DataTransferItemList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_DataTransferItemList(
            self_: <&DataTransferItemList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DataTransferItemList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clear_DataTransferItemList(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DataTransferItemList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_DataTransferItemList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DataTransferItemList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DataTransferItemList {
    #[cfg(all(feature = "DataTransferItemList",))]
    #[allow(bad_style)]
    #[doc = "The `remove()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/remove)\n\n*This API requires the following crate features to be activated: `DataTransferItemList`*"]
    #[allow(clippy::all)]
    pub fn remove(&self, index: u32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DataTransferItemList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_DataTransferItemList(
                self_: <&DataTransferItemList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_DataTransferItemList(
            self_: <&DataTransferItemList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ =
                    <&DataTransferItemList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_remove_DataTransferItemList(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DataTransferItem", feature = "DataTransferItemList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_DataTransferItemList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DataTransferItemList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<DataTransferItem> as WasmDescribe>::describe();
}
impl DataTransferItemList {
    #[cfg(all(feature = "DataTransferItem", feature = "DataTransferItemList",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `DataTransferItem`, `DataTransferItemList`*"]
    #[allow(clippy::all)]
    pub fn get(&self, index: u32) -> Option<DataTransferItem> {
        #[cfg(all(feature = "DataTransferItem", feature = "DataTransferItemList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_DataTransferItemList(
                self_: <&DataTransferItemList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<DataTransferItem> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_DataTransferItemList(
            self_: <&DataTransferItemList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<DataTransferItem> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DataTransferItemList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_DataTransferItemList(self_, index)
            };
            <Option<DataTransferItem> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DataTransferItemList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_DataTransferItemList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DataTransferItemList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl DataTransferItemList {
    #[cfg(all(feature = "DataTransferItemList",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/length)\n\n*This API requires the following crate features to be activated: `DataTransferItemList`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> u32 {
        #[cfg(all(feature = "DataTransferItemList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_DataTransferItemList(
                self_: <&DataTransferItemList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_DataTransferItemList(
            self_: <&DataTransferItemList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DataTransferItemList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_DataTransferItemList(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_22d7e92a5d501d74: [u8; 710usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x84\x02\0\0\0\0\x07\0\0\x02\x14DataTransferItemList&__widl_instanceof_DataTransferItemList\0\0\0\03__widl_f_add_with_str_and_type_DataTransferItemList\x01\0\0\x01\x14DataTransferItemList\x01\0\0\x01\x03\x05self_\x04data\x05type_\x03add\0\0\0+__widl_f_add_with_file_DataTransferItemList\x01\0\0\x01\x14DataTransferItemList\x01\0\0\x01\x02\x05self_\x04data\x03add\0\0\0#__widl_f_clear_DataTransferItemList\x01\0\0\x01\x14DataTransferItemList\x01\0\0\x01\x01\x05self_\x05clear\0\0\0$__widl_f_remove_DataTransferItemList\x01\0\0\x01\x14DataTransferItemList\x01\0\0\x01\x02\x05self_\x05index\x06remove\0\0\0!__widl_f_get_DataTransferItemList\0\0\0\x01\x14DataTransferItemList\x01\0\x03\x01\x02\x05self_\x05index\x03get\0\0\0$__widl_f_length_DataTransferItemList\0\0\0\x01\x14DataTransferItemList\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
