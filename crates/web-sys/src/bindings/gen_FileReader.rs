use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `FileReader` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct FileReader {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_FileReader: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for FileReader {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
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
        }
    }
    impl core::ops::Deref for FileReader {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for FileReader {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for FileReader {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a FileReader {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for FileReader {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            FileReader {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for FileReader {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a FileReader {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for FileReader {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<FileReader>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(FileReader {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for FileReader {
        #[inline]
        fn from(obj: JsValue) -> FileReader {
            FileReader { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for FileReader {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<FileReader> for FileReader {
        #[inline]
        fn as_ref(&self) -> &FileReader {
            self
        }
    }
    impl From<FileReader> for JsValue {
        #[inline]
        fn from(obj: FileReader) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for FileReader {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_FileReader(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_FileReader(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_FileReader(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            FileReader { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const FileReader) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<FileReader> for EventTarget {
    #[inline]
    fn from(obj: FileReader) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for FileReader {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<FileReader> for ::js_sys::Object {
    #[inline]
    fn from(obj: FileReader) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for FileReader {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <FileReader as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `new FileReader(..)` constructor, creating a new instance of `FileReader`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/FileReader)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<FileReader, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_FileReader() -> <FileReader as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_FileReader(
        ) -> <FileReader as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_FileReader() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<FileReader as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_abort_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FileReader as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `abort()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/abort)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    #[allow(clippy::all)]
    pub fn abort(&self) {
        #[cfg(all(feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_abort_FileReader(
                self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_abort_FileReader(
            self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_abort_FileReader(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Blob", feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_as_array_buffer_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileReader as WasmDescribe>::describe();
    <&Blob as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "Blob", feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `readAsArrayBuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsArrayBuffer)\n\n*This API requires the following crate features to be activated: `Blob`, `FileReader`*"]
    #[allow(clippy::all)]
    pub fn read_as_array_buffer(&self, blob: &Blob) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_as_array_buffer_FileReader(
                self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                blob: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_as_array_buffer_FileReader(
            self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            blob: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(blob);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let blob = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(blob);
                __widl_f_read_as_array_buffer_FileReader(self_, blob)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Blob", feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_as_binary_string_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileReader as WasmDescribe>::describe();
    <&Blob as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "Blob", feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `readAsBinaryString()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsBinaryString)\n\n*This API requires the following crate features to be activated: `Blob`, `FileReader`*"]
    #[allow(clippy::all)]
    pub fn read_as_binary_string(&self, filedata: &Blob) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_as_binary_string_FileReader(
                self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                filedata: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_as_binary_string_FileReader(
            self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            filedata: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(filedata);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let filedata = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(filedata);
                __widl_f_read_as_binary_string_FileReader(self_, filedata)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Blob", feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_as_data_url_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileReader as WasmDescribe>::describe();
    <&Blob as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "Blob", feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `readAsDataURL()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsDataURL)\n\n*This API requires the following crate features to be activated: `Blob`, `FileReader`*"]
    #[allow(clippy::all)]
    pub fn read_as_data_url(&self, blob: &Blob) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_as_data_url_FileReader(
                self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                blob: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_as_data_url_FileReader(
            self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            blob: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(blob);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let blob = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(blob);
                __widl_f_read_as_data_url_FileReader(self_, blob)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Blob", feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_as_text_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileReader as WasmDescribe>::describe();
    <&Blob as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "Blob", feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `readAsText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsText)\n\n*This API requires the following crate features to be activated: `Blob`, `FileReader`*"]
    #[allow(clippy::all)]
    pub fn read_as_text(&self, blob: &Blob) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_as_text_FileReader(
                self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                blob: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_as_text_FileReader(
            self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            blob: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(blob);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let blob = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(blob);
                __widl_f_read_as_text_FileReader(self_, blob)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Blob", feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_as_text_with_label_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&FileReader as WasmDescribe>::describe();
    <&Blob as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "Blob", feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `readAsText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsText)\n\n*This API requires the following crate features to be activated: `Blob`, `FileReader`*"]
    #[allow(clippy::all)]
    pub fn read_as_text_with_label(
        &self,
        blob: &Blob,
        label: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_as_text_with_label_FileReader(
                self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                blob: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_as_text_with_label_FileReader(
            self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            blob: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(blob);
            drop(label);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let blob = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(blob);
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_read_as_text_with_label_FileReader(self_, blob, label)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ready_state_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FileReader as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `readyState` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readyState)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    #[allow(clippy::all)]
    pub fn ready_state(&self) -> u16 {
        #[cfg(all(feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ready_state_FileReader(
                self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ready_state_FileReader(
            self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ready_state_FileReader(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_result_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FileReader as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `result` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/result)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    #[allow(clippy::all)]
    pub fn result(&self) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_result_FileReader(
                self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_result_FileReader(
            self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_result_FileReader(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomException", feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_error_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FileReader as WasmDescribe>::describe();
    <Option<DomException> as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "DomException", feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `error` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/error)\n\n*This API requires the following crate features to be activated: `DomException`, `FileReader`*"]
    #[allow(clippy::all)]
    pub fn error(&self) -> Option<DomException> {
        #[cfg(all(feature = "DomException", feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_FileReader(
                self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<DomException> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_FileReader(
            self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<DomException> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_error_FileReader(self_)
            };
            <Option<DomException> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onloadstart_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FileReader as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `onloadstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadstart)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    #[allow(clippy::all)]
    pub fn onloadstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onloadstart_FileReader(
                self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onloadstart_FileReader(
            self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onloadstart_FileReader(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onloadstart_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileReader as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `onloadstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadstart)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    #[allow(clippy::all)]
    pub fn set_onloadstart(&self, onloadstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onloadstart_FileReader(
                self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onloadstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onloadstart_FileReader(
            self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onloadstart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onloadstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onloadstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onloadstart,
                    );
                __widl_f_set_onloadstart_FileReader(self_, onloadstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onprogress_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FileReader as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `onprogress` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onprogress)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    #[allow(clippy::all)]
    pub fn onprogress(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onprogress_FileReader(
                self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onprogress_FileReader(
            self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onprogress_FileReader(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onprogress_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileReader as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `onprogress` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onprogress)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    #[allow(clippy::all)]
    pub fn set_onprogress(&self, onprogress: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onprogress_FileReader(
                self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onprogress : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onprogress_FileReader(
            self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onprogress: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onprogress);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onprogress =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onprogress,
                    );
                __widl_f_set_onprogress_FileReader(self_, onprogress)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onload_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FileReader as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `onload` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onload)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    #[allow(clippy::all)]
    pub fn onload(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onload_FileReader(
                self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onload_FileReader(
            self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onload_FileReader(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onload_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileReader as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `onload` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onload)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    #[allow(clippy::all)]
    pub fn set_onload(&self, onload: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onload_FileReader(
                self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onload: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onload_FileReader(
            self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onload: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onload);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onload =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onload,
                    );
                __widl_f_set_onload_FileReader(self_, onload)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onabort_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FileReader as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `onabort` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onabort)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    #[allow(clippy::all)]
    pub fn onabort(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onabort_FileReader(
                self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onabort_FileReader(
            self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onabort_FileReader(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onabort_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileReader as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `onabort` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onabort)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    #[allow(clippy::all)]
    pub fn set_onabort(&self, onabort: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onabort_FileReader(
                self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onabort: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onabort_FileReader(
            self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onabort: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onabort);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onabort =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onabort,
                    );
                __widl_f_set_onabort_FileReader(self_, onabort)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FileReader as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onerror)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_FileReader(
                self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_FileReader(
            self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_FileReader(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileReader as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onerror)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_FileReader(
                self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_FileReader(
            self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onerror);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_FileReader(self_, onerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onloadend_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FileReader as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `onloadend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadend)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    #[allow(clippy::all)]
    pub fn onloadend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onloadend_FileReader(
                self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onloadend_FileReader(
            self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onloadend_FileReader(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FileReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onloadend_FileReader() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileReader as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileReader {
    #[cfg(all(feature = "FileReader",))]
    #[allow(bad_style)]
    #[doc = "The `onloadend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadend)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    #[allow(clippy::all)]
    pub fn set_onloadend(&self, onloadend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "FileReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onloadend_FileReader(
                self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onloadend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onloadend_FileReader(
            self_: <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onloadend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onloadend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FileReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onloadend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onloadend,
                    );
                __widl_f_set_onloadend_FileReader(self_, onloadend)
            };
            ()
        }
    }
}
impl FileReader {
    pub const EMPTY: u16 = 0i64 as u16;
}
impl FileReader {
    pub const LOADING: u16 = 1u64 as u16;
}
impl FileReader {
    pub const DONE: u16 = 2u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_94c607162f8c34e0: [u8; 1933usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}K\x07\0\0\0\0\x17\0\0\x02\nFileReader\x1C__widl_instanceof_FileReader\0\0\0\0\x17__widl_f_new_FileReader\x01\0\0\x01\nFileReader\0\x01\0\x03new\0\0\0\x19__widl_f_abort_FileReader\0\0\0\x01\nFileReader\x01\0\0\x01\x01\x05self_\x05abort\0\0\0(__widl_f_read_as_array_buffer_FileReader\x01\0\0\x01\nFileReader\x01\0\0\x01\x02\x05self_\x04blob\x11readAsArrayBuffer\0\0\0)__widl_f_read_as_binary_string_FileReader\x01\0\0\x01\nFileReader\x01\0\0\x01\x02\x05self_\x08filedata\x12readAsBinaryString\0\0\0$__widl_f_read_as_data_url_FileReader\x01\0\0\x01\nFileReader\x01\0\0\x01\x02\x05self_\x04blob\rreadAsDataURL\0\0\0 __widl_f_read_as_text_FileReader\x01\0\0\x01\nFileReader\x01\0\0\x01\x02\x05self_\x04blob\nreadAsText\0\0\0+__widl_f_read_as_text_with_label_FileReader\x01\0\0\x01\nFileReader\x01\0\0\x01\x03\x05self_\x04blob\x05label\nreadAsText\0\0\0\x1F__widl_f_ready_state_FileReader\0\0\0\x01\nFileReader\x01\0\x01\nreadyState\x01\x01\x05self_\nreadyState\0\0\0\x1A__widl_f_result_FileReader\x01\0\0\x01\nFileReader\x01\0\x01\x06result\x01\x01\x05self_\x06result\0\0\0\x19__widl_f_error_FileReader\0\0\0\x01\nFileReader\x01\0\x01\x05error\x01\x01\x05self_\x05error\0\0\0\x1F__widl_f_onloadstart_FileReader\0\0\0\x01\nFileReader\x01\0\x01\x0Bonloadstart\x01\x01\x05self_\x0Bonloadstart\0\0\0#__widl_f_set_onloadstart_FileReader\0\0\0\x01\nFileReader\x01\0\x02\x0Bonloadstart\x01\x02\x05self_\x0Bonloadstart\x0Bonloadstart\0\0\0\x1E__widl_f_onprogress_FileReader\0\0\0\x01\nFileReader\x01\0\x01\nonprogress\x01\x01\x05self_\nonprogress\0\0\0\"__widl_f_set_onprogress_FileReader\0\0\0\x01\nFileReader\x01\0\x02\nonprogress\x01\x02\x05self_\nonprogress\nonprogress\0\0\0\x1A__widl_f_onload_FileReader\0\0\0\x01\nFileReader\x01\0\x01\x06onload\x01\x01\x05self_\x06onload\0\0\0\x1E__widl_f_set_onload_FileReader\0\0\0\x01\nFileReader\x01\0\x02\x06onload\x01\x02\x05self_\x06onload\x06onload\0\0\0\x1B__widl_f_onabort_FileReader\0\0\0\x01\nFileReader\x01\0\x01\x07onabort\x01\x01\x05self_\x07onabort\0\0\0\x1F__widl_f_set_onabort_FileReader\0\0\0\x01\nFileReader\x01\0\x02\x07onabort\x01\x02\x05self_\x07onabort\x07onabort\0\0\0\x1B__widl_f_onerror_FileReader\0\0\0\x01\nFileReader\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0\x1F__widl_f_set_onerror_FileReader\0\0\0\x01\nFileReader\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0\x1D__widl_f_onloadend_FileReader\0\0\0\x01\nFileReader\x01\0\x01\tonloadend\x01\x01\x05self_\tonloadend\0\0\0!__widl_f_set_onloadend_FileReader\0\0\0\x01\nFileReader\x01\0\x02\tonloadend\x01\x02\x05self_\tonloadend\tonloadend\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
