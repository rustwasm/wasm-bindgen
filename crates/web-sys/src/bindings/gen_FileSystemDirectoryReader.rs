use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `FileSystemDirectoryReader` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader)\n\n*This API requires the following crate features to be activated: `FileSystemDirectoryReader`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct FileSystemDirectoryReader {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_FileSystemDirectoryReader: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for FileSystemDirectoryReader {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(25u32);
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
            inform(68u32);
            inform(105u32);
            inform(114u32);
            inform(101u32);
            inform(99u32);
            inform(116u32);
            inform(111u32);
            inform(114u32);
            inform(121u32);
            inform(82u32);
            inform(101u32);
            inform(97u32);
            inform(100u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for FileSystemDirectoryReader {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for FileSystemDirectoryReader {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for FileSystemDirectoryReader {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a FileSystemDirectoryReader {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for FileSystemDirectoryReader {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            FileSystemDirectoryReader {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for FileSystemDirectoryReader {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a FileSystemDirectoryReader {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for FileSystemDirectoryReader {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<FileSystemDirectoryReader>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(FileSystemDirectoryReader {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for FileSystemDirectoryReader {
        #[inline]
        fn from(obj: JsValue) -> FileSystemDirectoryReader {
            FileSystemDirectoryReader { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for FileSystemDirectoryReader {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<FileSystemDirectoryReader> for FileSystemDirectoryReader {
        #[inline]
        fn as_ref(&self) -> &FileSystemDirectoryReader {
            self
        }
    }
    impl From<FileSystemDirectoryReader> for JsValue {
        #[inline]
        fn from(obj: FileSystemDirectoryReader) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for FileSystemDirectoryReader {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_FileSystemDirectoryReader(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_FileSystemDirectoryReader(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_FileSystemDirectoryReader(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            FileSystemDirectoryReader { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const FileSystemDirectoryReader) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<FileSystemDirectoryReader> for ::js_sys::Object {
    #[inline]
    fn from(obj: FileSystemDirectoryReader) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for FileSystemDirectoryReader {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "FileSystemDirectoryReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_entries_with_callback_FileSystemDirectoryReader(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileSystemDirectoryReader as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryReader {
    #[cfg(all(feature = "FileSystemDirectoryReader",))]
    #[allow(bad_style)]
    #[doc = "The `readEntries()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader/readEntries)\n\n*This API requires the following crate features to be activated: `FileSystemDirectoryReader`*"]
    #[allow(clippy::all)]
    pub fn read_entries_with_callback(
        &self,
        success_callback: &::js_sys::Function,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FileSystemDirectoryReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_entries_with_callback_FileSystemDirectoryReader(
                self_: <&FileSystemDirectoryReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_entries_with_callback_FileSystemDirectoryReader(
            self_: <&FileSystemDirectoryReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(success_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&FileSystemDirectoryReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                __widl_f_read_entries_with_callback_FileSystemDirectoryReader(
                    self_,
                    success_callback,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(
    feature = "FileSystemDirectoryReader",
    feature = "FileSystemEntriesCallback",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_entries_with_file_system_entries_callback_FileSystemDirectoryReader(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileSystemDirectoryReader as WasmDescribe>::describe();
    <&FileSystemEntriesCallback as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryReader {
    #[cfg(all(
        feature = "FileSystemDirectoryReader",
        feature = "FileSystemEntriesCallback",
    ))]
    #[allow(bad_style)]
    #[doc = "The `readEntries()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader/readEntries)\n\n*This API requires the following crate features to be activated: `FileSystemDirectoryReader`, `FileSystemEntriesCallback`*"]
    #[allow(clippy::all)]
    pub fn read_entries_with_file_system_entries_callback(
        &self,
        success_callback: &FileSystemEntriesCallback,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "FileSystemDirectoryReader",
            feature = "FileSystemEntriesCallback",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_entries_with_file_system_entries_callback_FileSystemDirectoryReader(
                self_: <&FileSystemDirectoryReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback : < & FileSystemEntriesCallback as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_entries_with_file_system_entries_callback_FileSystemDirectoryReader(
            self_: <&FileSystemDirectoryReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback : < & FileSystemEntriesCallback as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(success_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&FileSystemDirectoryReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let success_callback =
                    <&FileSystemEntriesCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                __widl_f_read_entries_with_file_system_entries_callback_FileSystemDirectoryReader(
                    self_,
                    success_callback,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "FileSystemDirectoryReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_entries_with_callback_and_callback_FileSystemDirectoryReader(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&FileSystemDirectoryReader as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryReader {
    #[cfg(all(feature = "FileSystemDirectoryReader",))]
    #[allow(bad_style)]
    #[doc = "The `readEntries()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader/readEntries)\n\n*This API requires the following crate features to be activated: `FileSystemDirectoryReader`*"]
    #[allow(clippy::all)]
    pub fn read_entries_with_callback_and_callback(
        &self,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FileSystemDirectoryReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_entries_with_callback_and_callback_FileSystemDirectoryReader(
                self_: <&FileSystemDirectoryReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_entries_with_callback_and_callback_FileSystemDirectoryReader(
            self_: <&FileSystemDirectoryReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            error_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(success_callback);
            drop(error_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&FileSystemDirectoryReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_read_entries_with_callback_and_callback_FileSystemDirectoryReader(
                    self_,
                    success_callback,
                    error_callback,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(
    feature = "FileSystemDirectoryReader",
    feature = "FileSystemEntriesCallback",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_entries_with_file_system_entries_callback_and_callback_FileSystemDirectoryReader(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&FileSystemDirectoryReader as WasmDescribe>::describe();
    <&FileSystemEntriesCallback as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryReader {
    #[cfg(all(
        feature = "FileSystemDirectoryReader",
        feature = "FileSystemEntriesCallback",
    ))]
    #[allow(bad_style)]
    #[doc = "The `readEntries()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader/readEntries)\n\n*This API requires the following crate features to be activated: `FileSystemDirectoryReader`, `FileSystemEntriesCallback`*"]
    #[allow(clippy::all)]
    pub fn read_entries_with_file_system_entries_callback_and_callback(
        &self,
        success_callback: &FileSystemEntriesCallback,
        error_callback: &::js_sys::Function,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "FileSystemDirectoryReader",
            feature = "FileSystemEntriesCallback",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_entries_with_file_system_entries_callback_and_callback_FileSystemDirectoryReader(
                self_: <&FileSystemDirectoryReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback : < & FileSystemEntriesCallback as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                error_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_entries_with_file_system_entries_callback_and_callback_FileSystemDirectoryReader(
            self_: <&FileSystemDirectoryReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback : < & FileSystemEntriesCallback as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            error_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(success_callback);
            drop(error_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&FileSystemDirectoryReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let success_callback =
                    <&FileSystemEntriesCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_read_entries_with_file_system_entries_callback_and_callback_FileSystemDirectoryReader ( self_ , success_callback , error_callback )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "ErrorCallback", feature = "FileSystemDirectoryReader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_entries_with_callback_and_error_callback_FileSystemDirectoryReader(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&FileSystemDirectoryReader as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&ErrorCallback as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryReader {
    #[cfg(all(feature = "ErrorCallback", feature = "FileSystemDirectoryReader",))]
    #[allow(bad_style)]
    #[doc = "The `readEntries()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader/readEntries)\n\n*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemDirectoryReader`*"]
    #[allow(clippy::all)]
    pub fn read_entries_with_callback_and_error_callback(
        &self,
        success_callback: &::js_sys::Function,
        error_callback: &ErrorCallback,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ErrorCallback", feature = "FileSystemDirectoryReader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_entries_with_callback_and_error_callback_FileSystemDirectoryReader(
                self_: <&FileSystemDirectoryReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error_callback: <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_entries_with_callback_and_error_callback_FileSystemDirectoryReader(
            self_: <&FileSystemDirectoryReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            error_callback: <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(success_callback);
            drop(error_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&FileSystemDirectoryReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_read_entries_with_callback_and_error_callback_FileSystemDirectoryReader(
                    self_,
                    success_callback,
                    error_callback,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(
    feature = "ErrorCallback",
    feature = "FileSystemDirectoryReader",
    feature = "FileSystemEntriesCallback",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_entries_with_file_system_entries_callback_and_error_callback_FileSystemDirectoryReader(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&FileSystemDirectoryReader as WasmDescribe>::describe();
    <&FileSystemEntriesCallback as WasmDescribe>::describe();
    <&ErrorCallback as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryReader {
    #[cfg(all(
        feature = "ErrorCallback",
        feature = "FileSystemDirectoryReader",
        feature = "FileSystemEntriesCallback",
    ))]
    #[allow(bad_style)]
    #[doc = "The `readEntries()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader/readEntries)\n\n*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemDirectoryReader`, `FileSystemEntriesCallback`*"]
    #[allow(clippy::all)]
    pub fn read_entries_with_file_system_entries_callback_and_error_callback(
        &self,
        success_callback: &FileSystemEntriesCallback,
        error_callback: &ErrorCallback,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ErrorCallback",
            feature = "FileSystemDirectoryReader",
            feature = "FileSystemEntriesCallback",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_entries_with_file_system_entries_callback_and_error_callback_FileSystemDirectoryReader(
                self_: <&FileSystemDirectoryReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback : < & FileSystemEntriesCallback as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                error_callback: <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_entries_with_file_system_entries_callback_and_error_callback_FileSystemDirectoryReader(
            self_: <&FileSystemDirectoryReader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback : < & FileSystemEntriesCallback as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            error_callback: <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(success_callback);
            drop(error_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&FileSystemDirectoryReader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let success_callback =
                    <&FileSystemEntriesCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_read_entries_with_file_system_entries_callback_and_error_callback_FileSystemDirectoryReader ( self_ , success_callback , error_callback )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_77998acab878b670: [u8; 1173usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}S\x04\0\0\0\0\x07\0\0\x02\x19FileSystemDirectoryReader+__widl_instanceof_FileSystemDirectoryReader\0\0\0\0=__widl_f_read_entries_with_callback_FileSystemDirectoryReader\x01\0\0\x01\x19FileSystemDirectoryReader\x01\0\0\x01\x02\x05self_\x10success_callback\x0BreadEntries\0\0\0Q__widl_f_read_entries_with_file_system_entries_callback_FileSystemDirectoryReader\x01\0\0\x01\x19FileSystemDirectoryReader\x01\0\0\x01\x02\x05self_\x10success_callback\x0BreadEntries\0\0\0J__widl_f_read_entries_with_callback_and_callback_FileSystemDirectoryReader\x01\0\0\x01\x19FileSystemDirectoryReader\x01\0\0\x01\x03\x05self_\x10success_callback\x0Eerror_callback\x0BreadEntries\0\0\0^__widl_f_read_entries_with_file_system_entries_callback_and_callback_FileSystemDirectoryReader\x01\0\0\x01\x19FileSystemDirectoryReader\x01\0\0\x01\x03\x05self_\x10success_callback\x0Eerror_callback\x0BreadEntries\0\0\0P__widl_f_read_entries_with_callback_and_error_callback_FileSystemDirectoryReader\x01\0\0\x01\x19FileSystemDirectoryReader\x01\0\0\x01\x03\x05self_\x10success_callback\x0Eerror_callback\x0BreadEntries\0\0\0d__widl_f_read_entries_with_file_system_entries_callback_and_error_callback_FileSystemDirectoryReader\x01\0\0\x01\x19FileSystemDirectoryReader\x01\0\0\x01\x03\x05self_\x10success_callback\x0Eerror_callback\x0BreadEntries\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
