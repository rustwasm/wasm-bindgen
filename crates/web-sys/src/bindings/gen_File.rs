use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `File` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File)\n\n*This API requires the following crate features to be activated: `File`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct File {
    obj: Blob,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_File: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for File {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(4u32);
            inform(70u32);
            inform(105u32);
            inform(108u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for File {
        type Target = Blob;
        #[inline]
        fn deref(&self) -> &Blob {
            &self.obj
        }
    }
    impl IntoWasmAbi for File {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for File {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a File {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for File {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            File {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for File {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a File {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for File {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<File>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(File {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for File {
        #[inline]
        fn from(obj: JsValue) -> File {
            File { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for File {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<File> for File {
        #[inline]
        fn as_ref(&self) -> &File {
            self
        }
    }
    impl From<File> for JsValue {
        #[inline]
        fn from(obj: File) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for File {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_File(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_File(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_File(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            File { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const File) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<File> for Blob {
    #[inline]
    fn from(obj: File) -> Blob {
        use wasm_bindgen::JsCast;
        Blob::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Blob> for File {
    #[inline]
    fn as_ref(&self) -> &Blob {
        use wasm_bindgen::JsCast;
        Blob::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<File> for ::js_sys::Object {
    #[inline]
    fn from(obj: File) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for File {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "File",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_buffer_source_sequence_File() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <File as WasmDescribe>::describe();
}
impl File {
    #[cfg(all(feature = "File",))]
    #[allow(bad_style)]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)\n\n*This API requires the following crate features to be activated: `File`*"]
    #[allow(clippy::all)]
    pub fn new_with_buffer_source_sequence(
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
    ) -> Result<File, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "File",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_buffer_source_sequence_File(
                file_bits: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                file_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <File as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_buffer_source_sequence_File(
            file_bits: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            file_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <File as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(file_bits);
            drop(file_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let file_bits =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        file_bits,
                    );
                let file_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(file_name);
                __widl_f_new_with_buffer_source_sequence_File(file_bits, file_name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<File as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "File",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_u8_array_sequence_File() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <File as WasmDescribe>::describe();
}
impl File {
    #[cfg(all(feature = "File",))]
    #[allow(bad_style)]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)\n\n*This API requires the following crate features to be activated: `File`*"]
    #[allow(clippy::all)]
    pub fn new_with_u8_array_sequence(
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
    ) -> Result<File, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "File",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_u8_array_sequence_File(
                file_bits: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                file_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <File as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_u8_array_sequence_File(
            file_bits: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            file_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <File as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(file_bits);
            drop(file_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let file_bits =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        file_bits,
                    );
                let file_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(file_name);
                __widl_f_new_with_u8_array_sequence_File(file_bits, file_name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<File as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "File",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_blob_sequence_File() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <File as WasmDescribe>::describe();
}
impl File {
    #[cfg(all(feature = "File",))]
    #[allow(bad_style)]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)\n\n*This API requires the following crate features to be activated: `File`*"]
    #[allow(clippy::all)]
    pub fn new_with_blob_sequence(
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
    ) -> Result<File, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "File",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_blob_sequence_File(
                file_bits: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                file_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <File as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_blob_sequence_File(
            file_bits: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            file_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <File as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(file_bits);
            drop(file_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let file_bits =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        file_bits,
                    );
                let file_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(file_name);
                __widl_f_new_with_blob_sequence_File(file_bits, file_name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<File as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "File",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_str_sequence_File() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <File as WasmDescribe>::describe();
}
impl File {
    #[cfg(all(feature = "File",))]
    #[allow(bad_style)]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)\n\n*This API requires the following crate features to be activated: `File`*"]
    #[allow(clippy::all)]
    pub fn new_with_str_sequence(
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
    ) -> Result<File, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "File",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_str_sequence_File(
                file_bits: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                file_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <File as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_str_sequence_File(
            file_bits: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            file_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <File as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(file_bits);
            drop(file_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let file_bits =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        file_bits,
                    );
                let file_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(file_name);
                __widl_f_new_with_str_sequence_File(file_bits, file_name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<File as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "File", feature = "FilePropertyBag",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_buffer_source_sequence_and_options_File() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&FilePropertyBag as WasmDescribe>::describe();
    <File as WasmDescribe>::describe();
}
impl File {
    #[cfg(all(feature = "File", feature = "FilePropertyBag",))]
    #[allow(bad_style)]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)\n\n*This API requires the following crate features to be activated: `File`, `FilePropertyBag`*"]
    #[allow(clippy::all)]
    pub fn new_with_buffer_source_sequence_and_options(
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
        options: &FilePropertyBag,
    ) -> Result<File, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "File", feature = "FilePropertyBag",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_buffer_source_sequence_and_options_File(
                file_bits: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                file_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&FilePropertyBag as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <File as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_buffer_source_sequence_and_options_File(
            file_bits: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            file_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&FilePropertyBag as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <File as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(file_bits);
            drop(file_name);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let file_bits =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        file_bits,
                    );
                let file_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(file_name);
                let options =
                    <&FilePropertyBag as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_buffer_source_sequence_and_options_File(
                    file_bits, file_name, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<File as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "File", feature = "FilePropertyBag",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_u8_array_sequence_and_options_File() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&FilePropertyBag as WasmDescribe>::describe();
    <File as WasmDescribe>::describe();
}
impl File {
    #[cfg(all(feature = "File", feature = "FilePropertyBag",))]
    #[allow(bad_style)]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)\n\n*This API requires the following crate features to be activated: `File`, `FilePropertyBag`*"]
    #[allow(clippy::all)]
    pub fn new_with_u8_array_sequence_and_options(
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
        options: &FilePropertyBag,
    ) -> Result<File, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "File", feature = "FilePropertyBag",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_u8_array_sequence_and_options_File(
                file_bits: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                file_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&FilePropertyBag as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <File as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_u8_array_sequence_and_options_File(
            file_bits: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            file_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&FilePropertyBag as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <File as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(file_bits);
            drop(file_name);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let file_bits =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        file_bits,
                    );
                let file_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(file_name);
                let options =
                    <&FilePropertyBag as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_u8_array_sequence_and_options_File(file_bits, file_name, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<File as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "File", feature = "FilePropertyBag",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_blob_sequence_and_options_File() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&FilePropertyBag as WasmDescribe>::describe();
    <File as WasmDescribe>::describe();
}
impl File {
    #[cfg(all(feature = "File", feature = "FilePropertyBag",))]
    #[allow(bad_style)]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)\n\n*This API requires the following crate features to be activated: `File`, `FilePropertyBag`*"]
    #[allow(clippy::all)]
    pub fn new_with_blob_sequence_and_options(
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
        options: &FilePropertyBag,
    ) -> Result<File, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "File", feature = "FilePropertyBag",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_blob_sequence_and_options_File(
                file_bits: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                file_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&FilePropertyBag as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <File as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_blob_sequence_and_options_File(
            file_bits: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            file_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&FilePropertyBag as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <File as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(file_bits);
            drop(file_name);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let file_bits =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        file_bits,
                    );
                let file_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(file_name);
                let options =
                    <&FilePropertyBag as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_blob_sequence_and_options_File(file_bits, file_name, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<File as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "File", feature = "FilePropertyBag",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_str_sequence_and_options_File() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&FilePropertyBag as WasmDescribe>::describe();
    <File as WasmDescribe>::describe();
}
impl File {
    #[cfg(all(feature = "File", feature = "FilePropertyBag",))]
    #[allow(bad_style)]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)\n\n*This API requires the following crate features to be activated: `File`, `FilePropertyBag`*"]
    #[allow(clippy::all)]
    pub fn new_with_str_sequence_and_options(
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
        options: &FilePropertyBag,
    ) -> Result<File, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "File", feature = "FilePropertyBag",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_str_sequence_and_options_File(
                file_bits: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                file_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&FilePropertyBag as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <File as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_str_sequence_and_options_File(
            file_bits: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            file_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&FilePropertyBag as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <File as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(file_bits);
            drop(file_name);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let file_bits =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        file_bits,
                    );
                let file_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(file_name);
                let options =
                    <&FilePropertyBag as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_str_sequence_and_options_File(file_bits, file_name, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<File as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "File",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_File() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&File as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl File {
    #[cfg(all(feature = "File",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/name)\n\n*This API requires the following crate features to be activated: `File`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "File",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_File(
                self_: <&File as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_File(
            self_: <&File as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&File as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_File(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "File",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_last_modified_File() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&File as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl File {
    #[cfg(all(feature = "File",))]
    #[allow(bad_style)]
    #[doc = "The `lastModified` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/lastModified)\n\n*This API requires the following crate features to be activated: `File`*"]
    #[allow(clippy::all)]
    pub fn last_modified(&self) -> f64 {
        #[cfg(all(feature = "File",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_last_modified_File(
                self_: <&File as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_last_modified_File(
            self_: <&File as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&File as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_last_modified_File(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_fecb70b4232da10f: [u8; 978usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x90\x03\0\0\0\0\x0B\0\0\x02\x04File\x16__widl_instanceof_File\0\0\0\0-__widl_f_new_with_buffer_source_sequence_File\x01\0\0\x01\x04File\0\x01\x02\tfile_bits\tfile_name\x03new\0\0\0(__widl_f_new_with_u8_array_sequence_File\x01\0\0\x01\x04File\0\x01\x02\tfile_bits\tfile_name\x03new\0\0\0$__widl_f_new_with_blob_sequence_File\x01\0\0\x01\x04File\0\x01\x02\tfile_bits\tfile_name\x03new\0\0\0#__widl_f_new_with_str_sequence_File\x01\0\0\x01\x04File\0\x01\x02\tfile_bits\tfile_name\x03new\0\0\09__widl_f_new_with_buffer_source_sequence_and_options_File\x01\0\0\x01\x04File\0\x01\x03\tfile_bits\tfile_name\x07options\x03new\0\0\04__widl_f_new_with_u8_array_sequence_and_options_File\x01\0\0\x01\x04File\0\x01\x03\tfile_bits\tfile_name\x07options\x03new\0\0\00__widl_f_new_with_blob_sequence_and_options_File\x01\0\0\x01\x04File\0\x01\x03\tfile_bits\tfile_name\x07options\x03new\0\0\0/__widl_f_new_with_str_sequence_and_options_File\x01\0\0\x01\x04File\0\x01\x03\tfile_bits\tfile_name\x07options\x03new\0\0\0\x12__widl_f_name_File\0\0\0\x01\x04File\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0\x1B__widl_f_last_modified_File\0\0\0\x01\x04File\x01\0\x01\x0ClastModified\x01\x01\x05self_\x0ClastModified\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
