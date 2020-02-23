use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `FileReaderSync` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync)\n\n*This API requires the following crate features to be activated: `FileReaderSync`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct FileReaderSync {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_FileReaderSync: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for FileReaderSync {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(70u32);
            inform(105u32);
            inform(108u32);
            inform(101u32);
            inform(82u32);
            inform(101u32);
            inform(97u32);
            inform(100u32);
            inform(101u32);
            inform(114u32);
            inform(83u32);
            inform(121u32);
            inform(110u32);
            inform(99u32);
        }
    }
    impl core::ops::Deref for FileReaderSync {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for FileReaderSync {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for FileReaderSync {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a FileReaderSync {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for FileReaderSync {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            FileReaderSync {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for FileReaderSync {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a FileReaderSync {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for FileReaderSync {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<FileReaderSync>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(FileReaderSync {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for FileReaderSync {
        #[inline]
        fn from(obj: JsValue) -> FileReaderSync {
            FileReaderSync { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for FileReaderSync {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<FileReaderSync> for FileReaderSync {
        #[inline]
        fn as_ref(&self) -> &FileReaderSync {
            self
        }
    }
    impl From<FileReaderSync> for JsValue {
        #[inline]
        fn from(obj: FileReaderSync) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for FileReaderSync {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_FileReaderSync(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_FileReaderSync(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_FileReaderSync(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            FileReaderSync { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const FileReaderSync) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<FileReaderSync> for ::js_sys::Object {
    #[inline]
    fn from(obj: FileReaderSync) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for FileReaderSync {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "FileReaderSync",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_FileReaderSync() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <FileReaderSync as WasmDescribe>::describe();
}
impl FileReaderSync {
    #[cfg(all(feature = "FileReaderSync",))]
    #[allow(bad_style)]
    #[doc = "The `new FileReaderSync(..)` constructor, creating a new instance of `FileReaderSync`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/FileReaderSync)\n\n*This API requires the following crate features to be activated: `FileReaderSync`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<FileReaderSync, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FileReaderSync",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_FileReaderSync(
            ) -> <FileReaderSync as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_FileReaderSync(
        ) -> <FileReaderSync as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_FileReaderSync() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<FileReaderSync as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob", feature = "FileReaderSync",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_as_array_buffer_FileReaderSync() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileReaderSync as WasmDescribe>::describe();
    <&Blob as WasmDescribe>::describe();
    <::js_sys::ArrayBuffer as WasmDescribe>::describe();
}
impl FileReaderSync {
    #[cfg(all(feature = "Blob", feature = "FileReaderSync",))]
    #[allow(bad_style)]
    #[doc = "The `readAsArrayBuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsArrayBuffer)\n\n*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*"]
    #[allow(clippy::all)]
    pub fn read_as_array_buffer(
        &self,
        blob: &Blob,
    ) -> Result<::js_sys::ArrayBuffer, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "FileReaderSync",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_as_array_buffer_FileReaderSync(
                self_: <&FileReaderSync as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                blob: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::ArrayBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_as_array_buffer_FileReaderSync(
            self_: <&FileReaderSync as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            blob: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::ArrayBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(blob);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReaderSync as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let blob = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(blob);
                __widl_f_read_as_array_buffer_FileReaderSync(self_, blob)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::ArrayBuffer as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob", feature = "FileReaderSync",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_as_binary_string_FileReaderSync() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileReaderSync as WasmDescribe>::describe();
    <&Blob as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl FileReaderSync {
    #[cfg(all(feature = "Blob", feature = "FileReaderSync",))]
    #[allow(bad_style)]
    #[doc = "The `readAsBinaryString()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsBinaryString)\n\n*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*"]
    #[allow(clippy::all)]
    pub fn read_as_binary_string(&self, blob: &Blob) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "FileReaderSync",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_as_binary_string_FileReaderSync(
                self_: <&FileReaderSync as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                blob: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_as_binary_string_FileReaderSync(
            self_: <&FileReaderSync as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            blob: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(blob);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReaderSync as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let blob = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(blob);
                __widl_f_read_as_binary_string_FileReaderSync(self_, blob)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Blob", feature = "FileReaderSync",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_as_data_url_FileReaderSync() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileReaderSync as WasmDescribe>::describe();
    <&Blob as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl FileReaderSync {
    #[cfg(all(feature = "Blob", feature = "FileReaderSync",))]
    #[allow(bad_style)]
    #[doc = "The `readAsDataURL()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsDataURL)\n\n*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*"]
    #[allow(clippy::all)]
    pub fn read_as_data_url(&self, blob: &Blob) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "FileReaderSync",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_as_data_url_FileReaderSync(
                self_: <&FileReaderSync as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                blob: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_as_data_url_FileReaderSync(
            self_: <&FileReaderSync as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            blob: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(blob);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReaderSync as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let blob = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(blob);
                __widl_f_read_as_data_url_FileReaderSync(self_, blob)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Blob", feature = "FileReaderSync",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_as_text_FileReaderSync() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileReaderSync as WasmDescribe>::describe();
    <&Blob as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl FileReaderSync {
    #[cfg(all(feature = "Blob", feature = "FileReaderSync",))]
    #[allow(bad_style)]
    #[doc = "The `readAsText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsText)\n\n*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*"]
    #[allow(clippy::all)]
    pub fn read_as_text(&self, blob: &Blob) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "FileReaderSync",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_as_text_FileReaderSync(
                self_: <&FileReaderSync as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                blob: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_as_text_FileReaderSync(
            self_: <&FileReaderSync as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            blob: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(blob);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReaderSync as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let blob = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(blob);
                __widl_f_read_as_text_FileReaderSync(self_, blob)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Blob", feature = "FileReaderSync",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_as_text_with_encoding_FileReaderSync() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&FileReaderSync as WasmDescribe>::describe();
    <&Blob as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl FileReaderSync {
    #[cfg(all(feature = "Blob", feature = "FileReaderSync",))]
    #[allow(bad_style)]
    #[doc = "The `readAsText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsText)\n\n*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*"]
    #[allow(clippy::all)]
    pub fn read_as_text_with_encoding(
        &self,
        blob: &Blob,
        encoding: &str,
    ) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "FileReaderSync",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_as_text_with_encoding_FileReaderSync(
                self_: <&FileReaderSync as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                blob: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                encoding: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_as_text_with_encoding_FileReaderSync(
            self_: <&FileReaderSync as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            blob: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            encoding: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(blob);
            drop(encoding);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReaderSync as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let blob = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(blob);
                let encoding = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(encoding);
                __widl_f_read_as_text_with_encoding_FileReaderSync(self_, blob, encoding)
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
pub static __WASM_BINDGEN_GENERATED_e5fb898828a0e7d0: [u8; 706usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x80\x02\0\0\0\0\x07\0\0\x02\x0EFileReaderSync __widl_instanceof_FileReaderSync\0\0\0\0\x1B__widl_f_new_FileReaderSync\x01\0\0\x01\x0EFileReaderSync\0\x01\0\x03new\0\0\0,__widl_f_read_as_array_buffer_FileReaderSync\x01\0\0\x01\x0EFileReaderSync\x01\0\0\x01\x02\x05self_\x04blob\x11readAsArrayBuffer\0\0\0-__widl_f_read_as_binary_string_FileReaderSync\x01\0\0\x01\x0EFileReaderSync\x01\0\0\x01\x02\x05self_\x04blob\x12readAsBinaryString\0\0\0(__widl_f_read_as_data_url_FileReaderSync\x01\0\0\x01\x0EFileReaderSync\x01\0\0\x01\x02\x05self_\x04blob\rreadAsDataURL\0\0\0$__widl_f_read_as_text_FileReaderSync\x01\0\0\x01\x0EFileReaderSync\x01\0\0\x01\x02\x05self_\x04blob\nreadAsText\0\0\02__widl_f_read_as_text_with_encoding_FileReaderSync\x01\0\0\x01\x0EFileReaderSync\x01\0\0\x01\x03\x05self_\x04blob\x08encoding\nreadAsText\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
