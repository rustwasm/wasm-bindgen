use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DataTransfer` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DataTransfer {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DataTransfer: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DataTransfer {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
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
        }
    }
    impl core::ops::Deref for DataTransfer {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for DataTransfer {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DataTransfer {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DataTransfer {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DataTransfer {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DataTransfer {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DataTransfer {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DataTransfer {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DataTransfer {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DataTransfer>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DataTransfer {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DataTransfer {
        #[inline]
        fn from(obj: JsValue) -> DataTransfer {
            DataTransfer { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DataTransfer {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DataTransfer> for DataTransfer {
        #[inline]
        fn as_ref(&self) -> &DataTransfer {
            self
        }
    }
    impl From<DataTransfer> for JsValue {
        #[inline]
        fn from(obj: DataTransfer) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DataTransfer {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DataTransfer(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DataTransfer(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DataTransfer(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DataTransfer { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DataTransfer) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DataTransfer> for ::js_sys::Object {
    #[inline]
    fn from(obj: DataTransfer) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DataTransfer {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DataTransfer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_DataTransfer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <DataTransfer as WasmDescribe>::describe();
}
impl DataTransfer {
    #[cfg(all(feature = "DataTransfer",))]
    #[allow(bad_style)]
    #[doc = "The `new DataTransfer(..)` constructor, creating a new instance of `DataTransfer`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/DataTransfer)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<DataTransfer, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DataTransfer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_DataTransfer(
            ) -> <DataTransfer as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_DataTransfer(
        ) -> <DataTransfer as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_DataTransfer() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DataTransfer as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DataTransfer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_data_DataTransfer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DataTransfer as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DataTransfer {
    #[cfg(all(feature = "DataTransfer",))]
    #[allow(bad_style)]
    #[doc = "The `clearData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/clearData)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    #[allow(clippy::all)]
    pub fn clear_data(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DataTransfer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_data_DataTransfer(
                self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_data_DataTransfer(
            self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clear_data_DataTransfer(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DataTransfer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_data_with_format_DataTransfer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DataTransfer as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DataTransfer {
    #[cfg(all(feature = "DataTransfer",))]
    #[allow(bad_style)]
    #[doc = "The `clearData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/clearData)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    #[allow(clippy::all)]
    pub fn clear_data_with_format(&self, format: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DataTransfer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_data_with_format_DataTransfer(
                self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_data_with_format_DataTransfer(
            self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(format);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let format = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                __widl_f_clear_data_with_format_DataTransfer(self_, format)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DataTransfer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_data_DataTransfer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DataTransfer as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl DataTransfer {
    #[cfg(all(feature = "DataTransfer",))]
    #[allow(bad_style)]
    #[doc = "The `getData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getData)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    #[allow(clippy::all)]
    pub fn get_data(&self, format: &str) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DataTransfer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_data_DataTransfer(
                self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_data_DataTransfer(
            self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(format);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let format = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                __widl_f_get_data_DataTransfer(self_, format)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DataTransfer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_files_DataTransfer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DataTransfer as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl DataTransfer {
    #[cfg(all(feature = "DataTransfer",))]
    #[allow(bad_style)]
    #[doc = "The `getFiles()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getFiles)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    #[allow(clippy::all)]
    pub fn get_files(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DataTransfer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_files_DataTransfer(
                self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_files_DataTransfer(
            self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_files_DataTransfer(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DataTransfer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_files_with_recursive_flag_DataTransfer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DataTransfer as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl DataTransfer {
    #[cfg(all(feature = "DataTransfer",))]
    #[allow(bad_style)]
    #[doc = "The `getFiles()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getFiles)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    #[allow(clippy::all)]
    pub fn get_files_with_recursive_flag(
        &self,
        recursive_flag: bool,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DataTransfer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_files_with_recursive_flag_DataTransfer(
                self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                recursive_flag: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_files_with_recursive_flag_DataTransfer(
            self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            recursive_flag: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(recursive_flag);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let recursive_flag =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(recursive_flag);
                __widl_f_get_files_with_recursive_flag_DataTransfer(self_, recursive_flag)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DataTransfer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_files_and_directories_DataTransfer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DataTransfer as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl DataTransfer {
    #[cfg(all(feature = "DataTransfer",))]
    #[allow(bad_style)]
    #[doc = "The `getFilesAndDirectories()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getFilesAndDirectories)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    #[allow(clippy::all)]
    pub fn get_files_and_directories(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DataTransfer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_files_and_directories_DataTransfer(
                self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_files_and_directories_DataTransfer(
            self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_files_and_directories_DataTransfer(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DataTransfer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_data_DataTransfer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DataTransfer as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DataTransfer {
    #[cfg(all(feature = "DataTransfer",))]
    #[allow(bad_style)]
    #[doc = "The `setData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/setData)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    #[allow(clippy::all)]
    pub fn set_data(&self, format: &str, data: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DataTransfer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_data_DataTransfer(
                self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_data_DataTransfer(
            self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(format);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let format = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let data = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_set_data_DataTransfer(self_, format, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DataTransfer", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_drag_image_DataTransfer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DataTransfer as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DataTransfer {
    #[cfg(all(feature = "DataTransfer", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `setDragImage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/setDragImage)\n\n*This API requires the following crate features to be activated: `DataTransfer`, `Element`*"]
    #[allow(clippy::all)]
    pub fn set_drag_image(&self, image: &Element, x: i32, y: i32) {
        #[cfg(all(feature = "DataTransfer", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_drag_image_DataTransfer(
                self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_drag_image_DataTransfer(
            self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(image);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let image = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                let x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_set_drag_image_DataTransfer(self_, image, x, y)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DataTransfer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_drop_effect_DataTransfer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DataTransfer as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl DataTransfer {
    #[cfg(all(feature = "DataTransfer",))]
    #[allow(bad_style)]
    #[doc = "The `dropEffect` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/dropEffect)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    #[allow(clippy::all)]
    pub fn drop_effect(&self) -> String {
        #[cfg(all(feature = "DataTransfer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_drop_effect_DataTransfer(
                self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_drop_effect_DataTransfer(
            self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_drop_effect_DataTransfer(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DataTransfer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_drop_effect_DataTransfer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DataTransfer as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DataTransfer {
    #[cfg(all(feature = "DataTransfer",))]
    #[allow(bad_style)]
    #[doc = "The `dropEffect` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/dropEffect)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    #[allow(clippy::all)]
    pub fn set_drop_effect(&self, drop_effect: &str) {
        #[cfg(all(feature = "DataTransfer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_drop_effect_DataTransfer(
                self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                drop_effect: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_drop_effect_DataTransfer(
            self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            drop_effect: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(drop_effect);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let drop_effect =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(drop_effect);
                __widl_f_set_drop_effect_DataTransfer(self_, drop_effect)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DataTransfer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_effect_allowed_DataTransfer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DataTransfer as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl DataTransfer {
    #[cfg(all(feature = "DataTransfer",))]
    #[allow(bad_style)]
    #[doc = "The `effectAllowed` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/effectAllowed)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    #[allow(clippy::all)]
    pub fn effect_allowed(&self) -> String {
        #[cfg(all(feature = "DataTransfer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_effect_allowed_DataTransfer(
                self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_effect_allowed_DataTransfer(
            self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_effect_allowed_DataTransfer(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DataTransfer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_effect_allowed_DataTransfer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DataTransfer as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DataTransfer {
    #[cfg(all(feature = "DataTransfer",))]
    #[allow(bad_style)]
    #[doc = "The `effectAllowed` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/effectAllowed)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    #[allow(clippy::all)]
    pub fn set_effect_allowed(&self, effect_allowed: &str) {
        #[cfg(all(feature = "DataTransfer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_effect_allowed_DataTransfer(
                self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                effect_allowed: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_effect_allowed_DataTransfer(
            self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            effect_allowed: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(effect_allowed);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let effect_allowed =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(effect_allowed);
                __widl_f_set_effect_allowed_DataTransfer(self_, effect_allowed)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DataTransfer", feature = "DataTransferItemList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_items_DataTransfer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DataTransfer as WasmDescribe>::describe();
    <DataTransferItemList as WasmDescribe>::describe();
}
impl DataTransfer {
    #[cfg(all(feature = "DataTransfer", feature = "DataTransferItemList",))]
    #[allow(bad_style)]
    #[doc = "The `items` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/items)\n\n*This API requires the following crate features to be activated: `DataTransfer`, `DataTransferItemList`*"]
    #[allow(clippy::all)]
    pub fn items(&self) -> DataTransferItemList {
        #[cfg(all(feature = "DataTransfer", feature = "DataTransferItemList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_items_DataTransfer(
                self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DataTransferItemList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_items_DataTransfer(
            self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DataTransferItemList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_items_DataTransfer(self_)
            };
            <DataTransferItemList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DataTransfer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_types_DataTransfer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DataTransfer as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl DataTransfer {
    #[cfg(all(feature = "DataTransfer",))]
    #[allow(bad_style)]
    #[doc = "The `types` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/types)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    #[allow(clippy::all)]
    pub fn types(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "DataTransfer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_types_DataTransfer(
                self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_types_DataTransfer(
            self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_types_DataTransfer(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DataTransfer", feature = "FileList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_files_DataTransfer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DataTransfer as WasmDescribe>::describe();
    <Option<FileList> as WasmDescribe>::describe();
}
impl DataTransfer {
    #[cfg(all(feature = "DataTransfer", feature = "FileList",))]
    #[allow(bad_style)]
    #[doc = "The `files` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/files)\n\n*This API requires the following crate features to be activated: `DataTransfer`, `FileList`*"]
    #[allow(clippy::all)]
    pub fn files(&self) -> Option<FileList> {
        #[cfg(all(feature = "DataTransfer", feature = "FileList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_files_DataTransfer(
                self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<FileList> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_files_DataTransfer(
            self_: <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<FileList> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DataTransfer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_files_DataTransfer(self_)
            };
            <Option<FileList> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8b6d9bf0f93c7825: [u8; 1518usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xAC\x05\0\0\0\0\x11\0\0\x02\x0CDataTransfer\x1E__widl_instanceof_DataTransfer\0\0\0\0\x19__widl_f_new_DataTransfer\x01\0\0\x01\x0CDataTransfer\0\x01\0\x03new\0\0\0 __widl_f_clear_data_DataTransfer\x01\0\0\x01\x0CDataTransfer\x01\0\0\x01\x01\x05self_\tclearData\0\0\0,__widl_f_clear_data_with_format_DataTransfer\x01\0\0\x01\x0CDataTransfer\x01\0\0\x01\x02\x05self_\x06format\tclearData\0\0\0\x1E__widl_f_get_data_DataTransfer\x01\0\0\x01\x0CDataTransfer\x01\0\0\x01\x02\x05self_\x06format\x07getData\0\0\0\x1F__widl_f_get_files_DataTransfer\x01\0\0\x01\x0CDataTransfer\x01\0\0\x01\x01\x05self_\x08getFiles\0\0\03__widl_f_get_files_with_recursive_flag_DataTransfer\x01\0\0\x01\x0CDataTransfer\x01\0\0\x01\x02\x05self_\x0Erecursive_flag\x08getFiles\0\0\0/__widl_f_get_files_and_directories_DataTransfer\x01\0\0\x01\x0CDataTransfer\x01\0\0\x01\x01\x05self_\x16getFilesAndDirectories\0\0\0\x1E__widl_f_set_data_DataTransfer\x01\0\0\x01\x0CDataTransfer\x01\0\0\x01\x03\x05self_\x06format\x04data\x07setData\0\0\0$__widl_f_set_drag_image_DataTransfer\0\0\0\x01\x0CDataTransfer\x01\0\0\x01\x04\x05self_\x05image\x01x\x01y\x0CsetDragImage\0\0\0!__widl_f_drop_effect_DataTransfer\0\0\0\x01\x0CDataTransfer\x01\0\x01\ndropEffect\x01\x01\x05self_\ndropEffect\0\0\0%__widl_f_set_drop_effect_DataTransfer\0\0\0\x01\x0CDataTransfer\x01\0\x02\ndropEffect\x01\x02\x05self_\x0Bdrop_effect\ndropEffect\0\0\0$__widl_f_effect_allowed_DataTransfer\0\0\0\x01\x0CDataTransfer\x01\0\x01\reffectAllowed\x01\x01\x05self_\reffectAllowed\0\0\0(__widl_f_set_effect_allowed_DataTransfer\0\0\0\x01\x0CDataTransfer\x01\0\x02\reffectAllowed\x01\x02\x05self_\x0Eeffect_allowed\reffectAllowed\0\0\0\x1B__widl_f_items_DataTransfer\0\0\0\x01\x0CDataTransfer\x01\0\x01\x05items\x01\x01\x05self_\x05items\0\0\0\x1B__widl_f_types_DataTransfer\0\0\0\x01\x0CDataTransfer\x01\0\x01\x05types\x01\x01\x05self_\x05types\0\0\0\x1B__widl_f_files_DataTransfer\0\0\0\x01\x0CDataTransfer\x01\0\x01\x05files\x01\x01\x05self_\x05files\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
