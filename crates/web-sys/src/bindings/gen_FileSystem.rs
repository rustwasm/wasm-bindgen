use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `FileSystem` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystem)\n\n*This API requires the following crate features to be activated: `FileSystem`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct FileSystem {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_FileSystem: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for FileSystem {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(70u32);
            inform(105u32);
            inform(108u32);
            inform(101u32);
            inform(83u32);
            inform(121u32);
            inform(115u32);
            inform(116u32);
            inform(101u32);
            inform(109u32);
        }
    }
    impl core::ops::Deref for FileSystem {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for FileSystem {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for FileSystem {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a FileSystem {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for FileSystem {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            FileSystem {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for FileSystem {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a FileSystem {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for FileSystem {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<FileSystem>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(FileSystem {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for FileSystem {
        #[inline]
        fn from(obj: JsValue) -> FileSystem {
            FileSystem { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for FileSystem {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<FileSystem> for FileSystem {
        #[inline]
        fn as_ref(&self) -> &FileSystem {
            self
        }
    }
    impl From<FileSystem> for JsValue {
        #[inline]
        fn from(obj: FileSystem) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for FileSystem {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_FileSystem(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_FileSystem(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_FileSystem(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            FileSystem { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const FileSystem) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<FileSystem> for ::js_sys::Object {
    #[inline]
    fn from(obj: FileSystem) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for FileSystem {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "FileSystem",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_FileSystem() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FileSystem as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl FileSystem {
    #[cfg(all(feature = "FileSystem",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystem/name)\n\n*This API requires the following crate features to be activated: `FileSystem`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "FileSystem",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_FileSystem(
                self_: <&FileSystem as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_FileSystem(
            self_: <&FileSystem as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileSystem as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_FileSystem(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FileSystem", feature = "FileSystemDirectoryEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_root_FileSystem() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FileSystem as WasmDescribe>::describe();
    <FileSystemDirectoryEntry as WasmDescribe>::describe();
}
impl FileSystem {
    #[cfg(all(feature = "FileSystem", feature = "FileSystemDirectoryEntry",))]
    #[allow(bad_style)]
    #[doc = "The `root` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystem/root)\n\n*This API requires the following crate features to be activated: `FileSystem`, `FileSystemDirectoryEntry`*"]
    #[allow(clippy::all)]
    pub fn root(&self) -> FileSystemDirectoryEntry {
        #[cfg(all(feature = "FileSystem", feature = "FileSystemDirectoryEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_root_FileSystem(
                self_: <&FileSystem as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <FileSystemDirectoryEntry as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_root_FileSystem(
            self_: <&FileSystem as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <FileSystemDirectoryEntry as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileSystem as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_root_FileSystem(self_)
            };
            <FileSystemDirectoryEntry as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_b9fbd67ff61bc5dd: [u8; 277usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xD3\0\0\0\0\0\x03\0\0\x02\nFileSystem\x1C__widl_instanceof_FileSystem\0\0\0\0\x18__widl_f_name_FileSystem\0\0\0\x01\nFileSystem\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0\x18__widl_f_root_FileSystem\0\0\0\x01\nFileSystem\x01\0\x01\x04root\x01\x01\x05self_\x04root\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
