use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Directory` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory)\n\n*This API requires the following crate features to be activated: `Directory`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Directory {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Directory: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Directory {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(68u32);
            inform(105u32);
            inform(114u32);
            inform(101u32);
            inform(99u32);
            inform(116u32);
            inform(111u32);
            inform(114u32);
            inform(121u32);
        }
    }
    impl core::ops::Deref for Directory {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Directory {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Directory {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Directory {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Directory {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Directory {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Directory {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Directory {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Directory {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Directory>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Directory {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Directory {
        #[inline]
        fn from(obj: JsValue) -> Directory {
            Directory { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Directory {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Directory> for Directory {
        #[inline]
        fn as_ref(&self) -> &Directory {
            self
        }
    }
    impl From<Directory> for JsValue {
        #[inline]
        fn from(obj: Directory) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Directory {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Directory(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Directory(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Directory(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Directory { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Directory) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Directory> for ::js_sys::Object {
    #[inline]
    fn from(obj: Directory) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Directory {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Directory",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_files_Directory() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Directory as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Directory {
    #[cfg(all(feature = "Directory",))]
    #[allow(bad_style)]
    #[doc = "The `getFiles()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/getFiles)\n\n*This API requires the following crate features to be activated: `Directory`*"]
    #[allow(clippy::all)]
    pub fn get_files(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Directory",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_files_Directory(
                self_: <&Directory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_files_Directory(
            self_: <&Directory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Directory as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_files_Directory(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Directory",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_files_with_recursive_flag_Directory() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Directory as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Directory {
    #[cfg(all(feature = "Directory",))]
    #[allow(bad_style)]
    #[doc = "The `getFiles()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/getFiles)\n\n*This API requires the following crate features to be activated: `Directory`*"]
    #[allow(clippy::all)]
    pub fn get_files_with_recursive_flag(
        &self,
        recursive_flag: bool,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Directory",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_files_with_recursive_flag_Directory(
                self_: <&Directory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                recursive_flag: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_files_with_recursive_flag_Directory(
            self_: <&Directory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Directory as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let recursive_flag =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(recursive_flag);
                __widl_f_get_files_with_recursive_flag_Directory(self_, recursive_flag)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Directory",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_files_and_directories_Directory() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Directory as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Directory {
    #[cfg(all(feature = "Directory",))]
    #[allow(bad_style)]
    #[doc = "The `getFilesAndDirectories()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/getFilesAndDirectories)\n\n*This API requires the following crate features to be activated: `Directory`*"]
    #[allow(clippy::all)]
    pub fn get_files_and_directories(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Directory",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_files_and_directories_Directory(
                self_: <&Directory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_files_and_directories_Directory(
            self_: <&Directory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Directory as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_files_and_directories_Directory(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Directory",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_Directory() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Directory as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Directory {
    #[cfg(all(feature = "Directory",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/name)\n\n*This API requires the following crate features to be activated: `Directory`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Directory",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_Directory(
                self_: <&Directory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_Directory(
            self_: <&Directory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Directory as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_Directory(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Directory",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_path_Directory() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Directory as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Directory {
    #[cfg(all(feature = "Directory",))]
    #[allow(bad_style)]
    #[doc = "The `path` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/path)\n\n*This API requires the following crate features to be activated: `Directory`*"]
    #[allow(clippy::all)]
    pub fn path(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Directory",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_path_Directory(
                self_: <&Directory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_path_Directory(
            self_: <&Directory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Directory as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_path_Directory(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_e0574a8ab47b8c8e: [u8; 534usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xD4\x01\0\0\0\0\x06\0\0\x02\tDirectory\x1B__widl_instanceof_Directory\0\0\0\0\x1C__widl_f_get_files_Directory\x01\0\0\x01\tDirectory\x01\0\0\x01\x01\x05self_\x08getFiles\0\0\00__widl_f_get_files_with_recursive_flag_Directory\x01\0\0\x01\tDirectory\x01\0\0\x01\x02\x05self_\x0Erecursive_flag\x08getFiles\0\0\0,__widl_f_get_files_and_directories_Directory\x01\0\0\x01\tDirectory\x01\0\0\x01\x01\x05self_\x16getFilesAndDirectories\0\0\0\x17__widl_f_name_Directory\x01\0\0\x01\tDirectory\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0\x17__widl_f_path_Directory\x01\0\0\x01\tDirectory\x01\0\x01\x04path\x01\x01\x05self_\x04path\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
