use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DataTransferItem` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem)\n\n*This API requires the following crate features to be activated: `DataTransferItem`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DataTransferItem {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DataTransferItem: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DataTransferItem {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
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
        }
    }
    impl core::ops::Deref for DataTransferItem {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for DataTransferItem {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DataTransferItem {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DataTransferItem {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DataTransferItem {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DataTransferItem {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DataTransferItem {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DataTransferItem {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DataTransferItem {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DataTransferItem>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DataTransferItem {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DataTransferItem {
        #[inline]
        fn from(obj: JsValue) -> DataTransferItem {
            DataTransferItem { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DataTransferItem {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DataTransferItem> for DataTransferItem {
        #[inline]
        fn as_ref(&self) -> &DataTransferItem {
            self
        }
    }
    impl From<DataTransferItem> for JsValue {
        #[inline]
        fn from(obj: DataTransferItem) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DataTransferItem {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DataTransferItem(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DataTransferItem(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DataTransferItem(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DataTransferItem { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DataTransferItem) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DataTransferItem> for ::js_sys::Object {
    #[inline]
    fn from(obj: DataTransferItem) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DataTransferItem {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DataTransferItem", feature = "File",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_as_file_DataTransferItem() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DataTransferItem as WasmDescribe>::describe();
    <Option<File> as WasmDescribe>::describe();
}
impl DataTransferItem {
    #[cfg(all(feature = "DataTransferItem", feature = "File",))]
    #[allow(bad_style)]
    #[doc = "The `getAsFile()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/getAsFile)\n\n*This API requires the following crate features to be activated: `DataTransferItem`, `File`*"]
    #[allow(clippy::all)]
    pub fn get_as_file(&self) -> Result<Option<File>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DataTransferItem", feature = "File",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_as_file_DataTransferItem(
                self_: <&DataTransferItem as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<File> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_as_file_DataTransferItem(
            self_: <&DataTransferItem as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<File> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DataTransferItem as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_as_file_DataTransferItem(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<File> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DataTransferItem",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_as_string_DataTransferItem() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DataTransferItem as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DataTransferItem {
    #[cfg(all(feature = "DataTransferItem",))]
    #[allow(bad_style)]
    #[doc = "The `getAsString()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/getAsString)\n\n*This API requires the following crate features to be activated: `DataTransferItem`*"]
    #[allow(clippy::all)]
    pub fn get_as_string(
        &self,
        callback: Option<&::js_sys::Function>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DataTransferItem",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_as_string_DataTransferItem(
                self_: <&DataTransferItem as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                callback: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_as_string_DataTransferItem(
            self_: <&DataTransferItem as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            callback: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DataTransferItem as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let callback =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        callback,
                    );
                __widl_f_get_as_string_DataTransferItem(self_, callback)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DataTransferItem", feature = "FileSystemEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_webkit_get_as_entry_DataTransferItem() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DataTransferItem as WasmDescribe>::describe();
    <Option<FileSystemEntry> as WasmDescribe>::describe();
}
impl DataTransferItem {
    #[cfg(all(feature = "DataTransferItem", feature = "FileSystemEntry",))]
    #[allow(bad_style)]
    #[doc = "The `webkitGetAsEntry()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/webkitGetAsEntry)\n\n*This API requires the following crate features to be activated: `DataTransferItem`, `FileSystemEntry`*"]
    #[allow(clippy::all)]
    pub fn webkit_get_as_entry(&self) -> Result<Option<FileSystemEntry>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DataTransferItem", feature = "FileSystemEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_webkit_get_as_entry_DataTransferItem(
                self_: <&DataTransferItem as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<FileSystemEntry> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_webkit_get_as_entry_DataTransferItem(
            self_: <&DataTransferItem as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<FileSystemEntry> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DataTransferItem as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_webkit_get_as_entry_DataTransferItem(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<FileSystemEntry> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DataTransferItem",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_kind_DataTransferItem() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DataTransferItem as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl DataTransferItem {
    #[cfg(all(feature = "DataTransferItem",))]
    #[allow(bad_style)]
    #[doc = "The `kind` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/kind)\n\n*This API requires the following crate features to be activated: `DataTransferItem`*"]
    #[allow(clippy::all)]
    pub fn kind(&self) -> String {
        #[cfg(all(feature = "DataTransferItem",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_kind_DataTransferItem(
                self_: <&DataTransferItem as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_kind_DataTransferItem(
            self_: <&DataTransferItem as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DataTransferItem as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_kind_DataTransferItem(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DataTransferItem",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_DataTransferItem() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DataTransferItem as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl DataTransferItem {
    #[cfg(all(feature = "DataTransferItem",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/type)\n\n*This API requires the following crate features to be activated: `DataTransferItem`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "DataTransferItem",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_DataTransferItem(
                self_: <&DataTransferItem as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_DataTransferItem(
            self_: <&DataTransferItem as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DataTransferItem as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_DataTransferItem(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_05be8ebf0f4a7a54: [u8; 590usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x0C\x02\0\0\0\0\x06\0\0\x02\x10DataTransferItem\"__widl_instanceof_DataTransferItem\0\0\0\0%__widl_f_get_as_file_DataTransferItem\x01\0\0\x01\x10DataTransferItem\x01\0\0\x01\x01\x05self_\tgetAsFile\0\0\0'__widl_f_get_as_string_DataTransferItem\x01\0\0\x01\x10DataTransferItem\x01\0\0\x01\x02\x05self_\x08callback\x0BgetAsString\0\0\0-__widl_f_webkit_get_as_entry_DataTransferItem\x01\0\0\x01\x10DataTransferItem\x01\0\0\x01\x01\x05self_\x10webkitGetAsEntry\0\0\0\x1E__widl_f_kind_DataTransferItem\0\0\0\x01\x10DataTransferItem\x01\0\x01\x04kind\x01\x01\x05self_\x04kind\0\0\0\x1E__widl_f_type_DataTransferItem\0\0\0\x01\x10DataTransferItem\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
