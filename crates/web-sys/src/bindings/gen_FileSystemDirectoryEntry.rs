use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `FileSystemDirectoryEntry` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry)\n\n*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct FileSystemDirectoryEntry {
    obj: FileSystemEntry,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_FileSystemDirectoryEntry: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for FileSystemDirectoryEntry {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(24u32);
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
            inform(69u32);
            inform(110u32);
            inform(116u32);
            inform(114u32);
            inform(121u32);
        }
    }
    impl core::ops::Deref for FileSystemDirectoryEntry {
        type Target = FileSystemEntry;
        #[inline]
        fn deref(&self) -> &FileSystemEntry {
            &self.obj
        }
    }
    impl IntoWasmAbi for FileSystemDirectoryEntry {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for FileSystemDirectoryEntry {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a FileSystemDirectoryEntry {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for FileSystemDirectoryEntry {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            FileSystemDirectoryEntry {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for FileSystemDirectoryEntry {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a FileSystemDirectoryEntry {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for FileSystemDirectoryEntry {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<FileSystemDirectoryEntry>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(FileSystemDirectoryEntry {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for FileSystemDirectoryEntry {
        #[inline]
        fn from(obj: JsValue) -> FileSystemDirectoryEntry {
            FileSystemDirectoryEntry { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for FileSystemDirectoryEntry {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<FileSystemDirectoryEntry> for FileSystemDirectoryEntry {
        #[inline]
        fn as_ref(&self) -> &FileSystemDirectoryEntry {
            self
        }
    }
    impl From<FileSystemDirectoryEntry> for JsValue {
        #[inline]
        fn from(obj: FileSystemDirectoryEntry) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for FileSystemDirectoryEntry {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_FileSystemDirectoryEntry(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_FileSystemDirectoryEntry(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_FileSystemDirectoryEntry(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            FileSystemDirectoryEntry { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const FileSystemDirectoryEntry) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<FileSystemDirectoryEntry> for FileSystemEntry {
    #[inline]
    fn from(obj: FileSystemDirectoryEntry) -> FileSystemEntry {
        use wasm_bindgen::JsCast;
        FileSystemEntry::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<FileSystemEntry> for FileSystemDirectoryEntry {
    #[inline]
    fn as_ref(&self) -> &FileSystemEntry {
        use wasm_bindgen::JsCast;
        FileSystemEntry::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<FileSystemDirectoryEntry> for ::js_sys::Object {
    #[inline]
    fn from(obj: FileSystemDirectoryEntry) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for FileSystemDirectoryEntry {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(
    feature = "FileSystemDirectoryEntry",
    feature = "FileSystemDirectoryReader",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_reader_FileSystemDirectoryEntry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FileSystemDirectoryEntry as WasmDescribe>::describe();
    <FileSystemDirectoryReader as WasmDescribe>::describe();
}
impl FileSystemDirectoryEntry {
    #[cfg(all(
        feature = "FileSystemDirectoryEntry",
        feature = "FileSystemDirectoryReader",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createReader()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/createReader)\n\n*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemDirectoryReader`*"]
    #[allow(clippy::all)]
    pub fn create_reader(&self) -> FileSystemDirectoryReader {
        #[cfg(all(
            feature = "FileSystemDirectoryEntry",
            feature = "FileSystemDirectoryReader",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_reader_FileSystemDirectoryEntry(
                self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <FileSystemDirectoryReader as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_reader_FileSystemDirectoryEntry(
            self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <FileSystemDirectoryReader as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_create_reader_FileSystemDirectoryEntry(self_)
            };
            <FileSystemDirectoryReader as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FileSystemDirectoryEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_directory_FileSystemDirectoryEntry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FileSystemDirectoryEntry as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryEntry {
    #[cfg(all(feature = "FileSystemDirectoryEntry",))]
    #[allow(bad_style)]
    #[doc = "The `getDirectory()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)\n\n*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`*"]
    #[allow(clippy::all)]
    pub fn get_directory(&self) {
        #[cfg(all(feature = "FileSystemDirectoryEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_directory_FileSystemDirectoryEntry(
                self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_directory_FileSystemDirectoryEntry(
            self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_get_directory_FileSystemDirectoryEntry(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileSystemDirectoryEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_directory_with_path_FileSystemDirectoryEntry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileSystemDirectoryEntry as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryEntry {
    #[cfg(all(feature = "FileSystemDirectoryEntry",))]
    #[allow(bad_style)]
    #[doc = "The `getDirectory()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)\n\n*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`*"]
    #[allow(clippy::all)]
    pub fn get_directory_with_path(&self, path: Option<&str>) {
        #[cfg(all(feature = "FileSystemDirectoryEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_directory_with_path_FileSystemDirectoryEntry(
                self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_directory_with_path_FileSystemDirectoryEntry(
            self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                __widl_f_get_directory_with_path_FileSystemDirectoryEntry(self_, path)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileSystemDirectoryEntry", feature = "FileSystemFlags",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_directory_with_path_and_options_FileSystemDirectoryEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&FileSystemDirectoryEntry as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&FileSystemFlags as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryEntry {
    #[cfg(all(feature = "FileSystemDirectoryEntry", feature = "FileSystemFlags",))]
    #[allow(bad_style)]
    #[doc = "The `getDirectory()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)\n\n*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
    #[allow(clippy::all)]
    pub fn get_directory_with_path_and_options(
        &self,
        path: Option<&str>,
        options: &FileSystemFlags,
    ) {
        #[cfg(all(feature = "FileSystemDirectoryEntry", feature = "FileSystemFlags",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_directory_with_path_and_options_FileSystemDirectoryEntry(
                self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_directory_with_path_and_options_FileSystemDirectoryEntry(
            self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                let options =
                    <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_get_directory_with_path_and_options_FileSystemDirectoryEntry(
                    self_, path, options,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileSystemDirectoryEntry", feature = "FileSystemFlags",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_directory_with_path_and_options_and_callback_FileSystemDirectoryEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&FileSystemDirectoryEntry as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&FileSystemFlags as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryEntry {
    #[cfg(all(feature = "FileSystemDirectoryEntry", feature = "FileSystemFlags",))]
    #[allow(bad_style)]
    #[doc = "The `getDirectory()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)\n\n*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
    #[allow(clippy::all)]
    pub fn get_directory_with_path_and_options_and_callback(
        &self,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &::js_sys::Function,
    ) {
        #[cfg(all(feature = "FileSystemDirectoryEntry", feature = "FileSystemFlags",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_directory_with_path_and_options_and_callback_FileSystemDirectoryEntry(
                self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_directory_with_path_and_options_and_callback_FileSystemDirectoryEntry(
            self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            drop(options);
            drop(success_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                let options =
                    <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                __widl_f_get_directory_with_path_and_options_and_callback_FileSystemDirectoryEntry(
                    self_,
                    path,
                    options,
                    success_callback,
                )
            };
            ()
        }
    }
}
#[cfg(all(
    feature = "FileSystemDirectoryEntry",
    feature = "FileSystemEntryCallback",
    feature = "FileSystemFlags",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_directory_with_path_and_options_and_file_system_entry_callback_FileSystemDirectoryEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&FileSystemDirectoryEntry as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&FileSystemFlags as WasmDescribe>::describe();
    <&FileSystemEntryCallback as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryEntry {
    #[cfg(all(
        feature = "FileSystemDirectoryEntry",
        feature = "FileSystemEntryCallback",
        feature = "FileSystemFlags",
    ))]
    #[allow(bad_style)]
    #[doc = "The `getDirectory()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)\n\n*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemEntryCallback`, `FileSystemFlags`*"]
    #[allow(clippy::all)]
    pub fn get_directory_with_path_and_options_and_file_system_entry_callback(
        &self,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &FileSystemEntryCallback,
    ) {
        #[cfg(all(
            feature = "FileSystemDirectoryEntry",
            feature = "FileSystemEntryCallback",
            feature = "FileSystemFlags",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_directory_with_path_and_options_and_file_system_entry_callback_FileSystemDirectoryEntry(
                self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback : < & FileSystemEntryCallback as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_directory_with_path_and_options_and_file_system_entry_callback_FileSystemDirectoryEntry(
            self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&FileSystemEntryCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            drop(options);
            drop(success_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                let options =
                    <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                let success_callback =
                    <&FileSystemEntryCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                __widl_f_get_directory_with_path_and_options_and_file_system_entry_callback_FileSystemDirectoryEntry ( self_ , path , options , success_callback )
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileSystemDirectoryEntry", feature = "FileSystemFlags",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_directory_with_path_and_options_and_callback_and_callback_FileSystemDirectoryEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&FileSystemDirectoryEntry as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&FileSystemFlags as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryEntry {
    #[cfg(all(feature = "FileSystemDirectoryEntry", feature = "FileSystemFlags",))]
    #[allow(bad_style)]
    #[doc = "The `getDirectory()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)\n\n*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
    #[allow(clippy::all)]
    pub fn get_directory_with_path_and_options_and_callback_and_callback(
        &self,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    ) {
        #[cfg(all(feature = "FileSystemDirectoryEntry", feature = "FileSystemFlags",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_directory_with_path_and_options_and_callback_and_callback_FileSystemDirectoryEntry(
                self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_directory_with_path_and_options_and_callback_and_callback_FileSystemDirectoryEntry(
            self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            error_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            drop(options);
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
                    <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                let options =
                    <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_get_directory_with_path_and_options_and_callback_and_callback_FileSystemDirectoryEntry ( self_ , path , options , success_callback , error_callback )
            };
            ()
        }
    }
}
#[cfg(all(
    feature = "FileSystemDirectoryEntry",
    feature = "FileSystemEntryCallback",
    feature = "FileSystemFlags",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_directory_with_path_and_options_and_file_system_entry_callback_and_callback_FileSystemDirectoryEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&FileSystemDirectoryEntry as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&FileSystemFlags as WasmDescribe>::describe();
    <&FileSystemEntryCallback as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryEntry {
    #[cfg(all(
        feature = "FileSystemDirectoryEntry",
        feature = "FileSystemEntryCallback",
        feature = "FileSystemFlags",
    ))]
    #[allow(bad_style)]
    #[doc = "The `getDirectory()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)\n\n*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemEntryCallback`, `FileSystemFlags`*"]
    #[allow(clippy::all)]
    pub fn get_directory_with_path_and_options_and_file_system_entry_callback_and_callback(
        &self,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &FileSystemEntryCallback,
        error_callback: &::js_sys::Function,
    ) {
        #[cfg(all(
            feature = "FileSystemDirectoryEntry",
            feature = "FileSystemEntryCallback",
            feature = "FileSystemFlags",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_directory_with_path_and_options_and_file_system_entry_callback_and_callback_FileSystemDirectoryEntry(
                self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback : < & FileSystemEntryCallback as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                error_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_directory_with_path_and_options_and_file_system_entry_callback_and_callback_FileSystemDirectoryEntry(
            self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&FileSystemEntryCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            error_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            drop(options);
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
                    <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                let options =
                    <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                let success_callback =
                    <&FileSystemEntryCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_get_directory_with_path_and_options_and_file_system_entry_callback_and_callback_FileSystemDirectoryEntry ( self_ , path , options , success_callback , error_callback )
            };
            ()
        }
    }
}
#[cfg(all(
    feature = "ErrorCallback",
    feature = "FileSystemDirectoryEntry",
    feature = "FileSystemFlags",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_directory_with_path_and_options_and_callback_and_error_callback_FileSystemDirectoryEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&FileSystemDirectoryEntry as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&FileSystemFlags as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&ErrorCallback as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryEntry {
    #[cfg(all(
        feature = "ErrorCallback",
        feature = "FileSystemDirectoryEntry",
        feature = "FileSystemFlags",
    ))]
    #[allow(bad_style)]
    #[doc = "The `getDirectory()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)\n\n*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
    #[allow(clippy::all)]
    pub fn get_directory_with_path_and_options_and_callback_and_error_callback(
        &self,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &::js_sys::Function,
        error_callback: &ErrorCallback,
    ) {
        #[cfg(all(
            feature = "ErrorCallback",
            feature = "FileSystemDirectoryEntry",
            feature = "FileSystemFlags",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_directory_with_path_and_options_and_callback_and_error_callback_FileSystemDirectoryEntry(
                self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error_callback: <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_directory_with_path_and_options_and_callback_and_error_callback_FileSystemDirectoryEntry(
            self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            error_callback: <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            drop(options);
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
                    <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                let options =
                    <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_get_directory_with_path_and_options_and_callback_and_error_callback_FileSystemDirectoryEntry ( self_ , path , options , success_callback , error_callback )
            };
            ()
        }
    }
}
#[cfg(all(
    feature = "ErrorCallback",
    feature = "FileSystemDirectoryEntry",
    feature = "FileSystemEntryCallback",
    feature = "FileSystemFlags",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_directory_with_path_and_options_and_file_system_entry_callback_and_error_callback_FileSystemDirectoryEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&FileSystemDirectoryEntry as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&FileSystemFlags as WasmDescribe>::describe();
    <&FileSystemEntryCallback as WasmDescribe>::describe();
    <&ErrorCallback as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryEntry {
    #[cfg(all(
        feature = "ErrorCallback",
        feature = "FileSystemDirectoryEntry",
        feature = "FileSystemEntryCallback",
        feature = "FileSystemFlags",
    ))]
    #[allow(bad_style)]
    #[doc = "The `getDirectory()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)\n\n*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemDirectoryEntry`, `FileSystemEntryCallback`, `FileSystemFlags`*"]
    #[allow(clippy::all)]
    pub fn get_directory_with_path_and_options_and_file_system_entry_callback_and_error_callback(
        &self,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &FileSystemEntryCallback,
        error_callback: &ErrorCallback,
    ) {
        #[cfg(all(
            feature = "ErrorCallback",
            feature = "FileSystemDirectoryEntry",
            feature = "FileSystemEntryCallback",
            feature = "FileSystemFlags",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_directory_with_path_and_options_and_file_system_entry_callback_and_error_callback_FileSystemDirectoryEntry(
                self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback : < & FileSystemEntryCallback as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                error_callback: <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_directory_with_path_and_options_and_file_system_entry_callback_and_error_callback_FileSystemDirectoryEntry(
            self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&FileSystemEntryCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            error_callback: <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            drop(options);
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
                    <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                let options =
                    <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                let success_callback =
                    <&FileSystemEntryCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_get_directory_with_path_and_options_and_file_system_entry_callback_and_error_callback_FileSystemDirectoryEntry ( self_ , path , options , success_callback , error_callback )
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileSystemDirectoryEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_file_FileSystemDirectoryEntry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FileSystemDirectoryEntry as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryEntry {
    #[cfg(all(feature = "FileSystemDirectoryEntry",))]
    #[allow(bad_style)]
    #[doc = "The `getFile()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)\n\n*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`*"]
    #[allow(clippy::all)]
    pub fn get_file(&self) {
        #[cfg(all(feature = "FileSystemDirectoryEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_file_FileSystemDirectoryEntry(
                self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_file_FileSystemDirectoryEntry(
            self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_get_file_FileSystemDirectoryEntry(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileSystemDirectoryEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_file_with_path_FileSystemDirectoryEntry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileSystemDirectoryEntry as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryEntry {
    #[cfg(all(feature = "FileSystemDirectoryEntry",))]
    #[allow(bad_style)]
    #[doc = "The `getFile()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)\n\n*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`*"]
    #[allow(clippy::all)]
    pub fn get_file_with_path(&self, path: Option<&str>) {
        #[cfg(all(feature = "FileSystemDirectoryEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_file_with_path_FileSystemDirectoryEntry(
                self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_file_with_path_FileSystemDirectoryEntry(
            self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                __widl_f_get_file_with_path_FileSystemDirectoryEntry(self_, path)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileSystemDirectoryEntry", feature = "FileSystemFlags",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_file_with_path_and_options_FileSystemDirectoryEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&FileSystemDirectoryEntry as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&FileSystemFlags as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryEntry {
    #[cfg(all(feature = "FileSystemDirectoryEntry", feature = "FileSystemFlags",))]
    #[allow(bad_style)]
    #[doc = "The `getFile()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)\n\n*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
    #[allow(clippy::all)]
    pub fn get_file_with_path_and_options(&self, path: Option<&str>, options: &FileSystemFlags) {
        #[cfg(all(feature = "FileSystemDirectoryEntry", feature = "FileSystemFlags",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_file_with_path_and_options_FileSystemDirectoryEntry(
                self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_file_with_path_and_options_FileSystemDirectoryEntry(
            self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                let options =
                    <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_get_file_with_path_and_options_FileSystemDirectoryEntry(
                    self_, path, options,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileSystemDirectoryEntry", feature = "FileSystemFlags",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_file_with_path_and_options_and_callback_FileSystemDirectoryEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&FileSystemDirectoryEntry as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&FileSystemFlags as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryEntry {
    #[cfg(all(feature = "FileSystemDirectoryEntry", feature = "FileSystemFlags",))]
    #[allow(bad_style)]
    #[doc = "The `getFile()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)\n\n*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
    #[allow(clippy::all)]
    pub fn get_file_with_path_and_options_and_callback(
        &self,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &::js_sys::Function,
    ) {
        #[cfg(all(feature = "FileSystemDirectoryEntry", feature = "FileSystemFlags",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_file_with_path_and_options_and_callback_FileSystemDirectoryEntry(
                self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_file_with_path_and_options_and_callback_FileSystemDirectoryEntry(
            self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            drop(options);
            drop(success_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                let options =
                    <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                __widl_f_get_file_with_path_and_options_and_callback_FileSystemDirectoryEntry(
                    self_,
                    path,
                    options,
                    success_callback,
                )
            };
            ()
        }
    }
}
#[cfg(all(
    feature = "FileSystemDirectoryEntry",
    feature = "FileSystemEntryCallback",
    feature = "FileSystemFlags",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_file_with_path_and_options_and_file_system_entry_callback_FileSystemDirectoryEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&FileSystemDirectoryEntry as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&FileSystemFlags as WasmDescribe>::describe();
    <&FileSystemEntryCallback as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryEntry {
    #[cfg(all(
        feature = "FileSystemDirectoryEntry",
        feature = "FileSystemEntryCallback",
        feature = "FileSystemFlags",
    ))]
    #[allow(bad_style)]
    #[doc = "The `getFile()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)\n\n*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemEntryCallback`, `FileSystemFlags`*"]
    #[allow(clippy::all)]
    pub fn get_file_with_path_and_options_and_file_system_entry_callback(
        &self,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &FileSystemEntryCallback,
    ) {
        #[cfg(all(
            feature = "FileSystemDirectoryEntry",
            feature = "FileSystemEntryCallback",
            feature = "FileSystemFlags",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_file_with_path_and_options_and_file_system_entry_callback_FileSystemDirectoryEntry(
                self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback : < & FileSystemEntryCallback as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_file_with_path_and_options_and_file_system_entry_callback_FileSystemDirectoryEntry(
            self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&FileSystemEntryCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            drop(options);
            drop(success_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                let options =
                    <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                let success_callback =
                    <&FileSystemEntryCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                __widl_f_get_file_with_path_and_options_and_file_system_entry_callback_FileSystemDirectoryEntry ( self_ , path , options , success_callback )
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileSystemDirectoryEntry", feature = "FileSystemFlags",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_file_with_path_and_options_and_callback_and_callback_FileSystemDirectoryEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&FileSystemDirectoryEntry as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&FileSystemFlags as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryEntry {
    #[cfg(all(feature = "FileSystemDirectoryEntry", feature = "FileSystemFlags",))]
    #[allow(bad_style)]
    #[doc = "The `getFile()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)\n\n*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
    #[allow(clippy::all)]
    pub fn get_file_with_path_and_options_and_callback_and_callback(
        &self,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    ) {
        #[cfg(all(feature = "FileSystemDirectoryEntry", feature = "FileSystemFlags",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_file_with_path_and_options_and_callback_and_callback_FileSystemDirectoryEntry(
                self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_file_with_path_and_options_and_callback_and_callback_FileSystemDirectoryEntry(
            self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            error_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            drop(options);
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
                    <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                let options =
                    <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_get_file_with_path_and_options_and_callback_and_callback_FileSystemDirectoryEntry ( self_ , path , options , success_callback , error_callback )
            };
            ()
        }
    }
}
#[cfg(all(
    feature = "FileSystemDirectoryEntry",
    feature = "FileSystemEntryCallback",
    feature = "FileSystemFlags",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_file_with_path_and_options_and_file_system_entry_callback_and_callback_FileSystemDirectoryEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&FileSystemDirectoryEntry as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&FileSystemFlags as WasmDescribe>::describe();
    <&FileSystemEntryCallback as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryEntry {
    #[cfg(all(
        feature = "FileSystemDirectoryEntry",
        feature = "FileSystemEntryCallback",
        feature = "FileSystemFlags",
    ))]
    #[allow(bad_style)]
    #[doc = "The `getFile()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)\n\n*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemEntryCallback`, `FileSystemFlags`*"]
    #[allow(clippy::all)]
    pub fn get_file_with_path_and_options_and_file_system_entry_callback_and_callback(
        &self,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &FileSystemEntryCallback,
        error_callback: &::js_sys::Function,
    ) {
        #[cfg(all(
            feature = "FileSystemDirectoryEntry",
            feature = "FileSystemEntryCallback",
            feature = "FileSystemFlags",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_file_with_path_and_options_and_file_system_entry_callback_and_callback_FileSystemDirectoryEntry(
                self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback : < & FileSystemEntryCallback as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                error_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_file_with_path_and_options_and_file_system_entry_callback_and_callback_FileSystemDirectoryEntry(
            self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&FileSystemEntryCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            error_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            drop(options);
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
                    <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                let options =
                    <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                let success_callback =
                    <&FileSystemEntryCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_get_file_with_path_and_options_and_file_system_entry_callback_and_callback_FileSystemDirectoryEntry ( self_ , path , options , success_callback , error_callback )
            };
            ()
        }
    }
}
#[cfg(all(
    feature = "ErrorCallback",
    feature = "FileSystemDirectoryEntry",
    feature = "FileSystemFlags",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_file_with_path_and_options_and_callback_and_error_callback_FileSystemDirectoryEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&FileSystemDirectoryEntry as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&FileSystemFlags as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&ErrorCallback as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryEntry {
    #[cfg(all(
        feature = "ErrorCallback",
        feature = "FileSystemDirectoryEntry",
        feature = "FileSystemFlags",
    ))]
    #[allow(bad_style)]
    #[doc = "The `getFile()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)\n\n*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
    #[allow(clippy::all)]
    pub fn get_file_with_path_and_options_and_callback_and_error_callback(
        &self,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &::js_sys::Function,
        error_callback: &ErrorCallback,
    ) {
        #[cfg(all(
            feature = "ErrorCallback",
            feature = "FileSystemDirectoryEntry",
            feature = "FileSystemFlags",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_file_with_path_and_options_and_callback_and_error_callback_FileSystemDirectoryEntry(
                self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error_callback: <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_file_with_path_and_options_and_callback_and_error_callback_FileSystemDirectoryEntry(
            self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            error_callback: <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            drop(options);
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
                    <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                let options =
                    <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_get_file_with_path_and_options_and_callback_and_error_callback_FileSystemDirectoryEntry ( self_ , path , options , success_callback , error_callback )
            };
            ()
        }
    }
}
#[cfg(all(
    feature = "ErrorCallback",
    feature = "FileSystemDirectoryEntry",
    feature = "FileSystemEntryCallback",
    feature = "FileSystemFlags",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_file_with_path_and_options_and_file_system_entry_callback_and_error_callback_FileSystemDirectoryEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&FileSystemDirectoryEntry as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&FileSystemFlags as WasmDescribe>::describe();
    <&FileSystemEntryCallback as WasmDescribe>::describe();
    <&ErrorCallback as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemDirectoryEntry {
    #[cfg(all(
        feature = "ErrorCallback",
        feature = "FileSystemDirectoryEntry",
        feature = "FileSystemEntryCallback",
        feature = "FileSystemFlags",
    ))]
    #[allow(bad_style)]
    #[doc = "The `getFile()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)\n\n*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemDirectoryEntry`, `FileSystemEntryCallback`, `FileSystemFlags`*"]
    #[allow(clippy::all)]
    pub fn get_file_with_path_and_options_and_file_system_entry_callback_and_error_callback(
        &self,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &FileSystemEntryCallback,
        error_callback: &ErrorCallback,
    ) {
        #[cfg(all(
            feature = "ErrorCallback",
            feature = "FileSystemDirectoryEntry",
            feature = "FileSystemEntryCallback",
            feature = "FileSystemFlags",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_file_with_path_and_options_and_file_system_entry_callback_and_error_callback_FileSystemDirectoryEntry(
                self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback : < & FileSystemEntryCallback as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                error_callback: <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_file_with_path_and_options_and_file_system_entry_callback_and_error_callback_FileSystemDirectoryEntry(
            self_: <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            path: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&FileSystemEntryCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            error_callback: <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(path);
            drop(options);
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
                    <&FileSystemDirectoryEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let path = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(path);
                let options =
                    <&FileSystemFlags as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                let success_callback =
                    <&FileSystemEntryCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_get_file_with_path_and_options_and_file_system_entry_callback_and_error_callback_FileSystemDirectoryEntry ( self_ , path , options , success_callback , error_callback )
            };
            ()
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_076aace49a4eaddb: [u8; 3299usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xA1\x0C\0\0\0\0\x14\0\0\x02\x18FileSystemDirectoryEntry*__widl_instanceof_FileSystemDirectoryEntry\0\0\0\0/__widl_f_create_reader_FileSystemDirectoryEntry\0\0\0\x01\x18FileSystemDirectoryEntry\x01\0\0\x01\x01\x05self_\x0CcreateReader\0\0\0/__widl_f_get_directory_FileSystemDirectoryEntry\0\0\0\x01\x18FileSystemDirectoryEntry\x01\0\0\x01\x01\x05self_\x0CgetDirectory\0\0\09__widl_f_get_directory_with_path_FileSystemDirectoryEntry\0\0\0\x01\x18FileSystemDirectoryEntry\x01\0\0\x01\x02\x05self_\x04path\x0CgetDirectory\0\0\0E__widl_f_get_directory_with_path_and_options_FileSystemDirectoryEntry\0\0\0\x01\x18FileSystemDirectoryEntry\x01\0\0\x01\x03\x05self_\x04path\x07options\x0CgetDirectory\0\0\0R__widl_f_get_directory_with_path_and_options_and_callback_FileSystemDirectoryEntry\0\0\0\x01\x18FileSystemDirectoryEntry\x01\0\0\x01\x04\x05self_\x04path\x07options\x10success_callback\x0CgetDirectory\0\0\0d__widl_f_get_directory_with_path_and_options_and_file_system_entry_callback_FileSystemDirectoryEntry\0\0\0\x01\x18FileSystemDirectoryEntry\x01\0\0\x01\x04\x05self_\x04path\x07options\x10success_callback\x0CgetDirectory\0\0\0___widl_f_get_directory_with_path_and_options_and_callback_and_callback_FileSystemDirectoryEntry\0\0\0\x01\x18FileSystemDirectoryEntry\x01\0\0\x01\x05\x05self_\x04path\x07options\x10success_callback\x0Eerror_callback\x0CgetDirectory\0\0\0q__widl_f_get_directory_with_path_and_options_and_file_system_entry_callback_and_callback_FileSystemDirectoryEntry\0\0\0\x01\x18FileSystemDirectoryEntry\x01\0\0\x01\x05\x05self_\x04path\x07options\x10success_callback\x0Eerror_callback\x0CgetDirectory\0\0\0e__widl_f_get_directory_with_path_and_options_and_callback_and_error_callback_FileSystemDirectoryEntry\0\0\0\x01\x18FileSystemDirectoryEntry\x01\0\0\x01\x05\x05self_\x04path\x07options\x10success_callback\x0Eerror_callback\x0CgetDirectory\0\0\0w__widl_f_get_directory_with_path_and_options_and_file_system_entry_callback_and_error_callback_FileSystemDirectoryEntry\0\0\0\x01\x18FileSystemDirectoryEntry\x01\0\0\x01\x05\x05self_\x04path\x07options\x10success_callback\x0Eerror_callback\x0CgetDirectory\0\0\0*__widl_f_get_file_FileSystemDirectoryEntry\0\0\0\x01\x18FileSystemDirectoryEntry\x01\0\0\x01\x01\x05self_\x07getFile\0\0\04__widl_f_get_file_with_path_FileSystemDirectoryEntry\0\0\0\x01\x18FileSystemDirectoryEntry\x01\0\0\x01\x02\x05self_\x04path\x07getFile\0\0\0@__widl_f_get_file_with_path_and_options_FileSystemDirectoryEntry\0\0\0\x01\x18FileSystemDirectoryEntry\x01\0\0\x01\x03\x05self_\x04path\x07options\x07getFile\0\0\0M__widl_f_get_file_with_path_and_options_and_callback_FileSystemDirectoryEntry\0\0\0\x01\x18FileSystemDirectoryEntry\x01\0\0\x01\x04\x05self_\x04path\x07options\x10success_callback\x07getFile\0\0\0___widl_f_get_file_with_path_and_options_and_file_system_entry_callback_FileSystemDirectoryEntry\0\0\0\x01\x18FileSystemDirectoryEntry\x01\0\0\x01\x04\x05self_\x04path\x07options\x10success_callback\x07getFile\0\0\0Z__widl_f_get_file_with_path_and_options_and_callback_and_callback_FileSystemDirectoryEntry\0\0\0\x01\x18FileSystemDirectoryEntry\x01\0\0\x01\x05\x05self_\x04path\x07options\x10success_callback\x0Eerror_callback\x07getFile\0\0\0l__widl_f_get_file_with_path_and_options_and_file_system_entry_callback_and_callback_FileSystemDirectoryEntry\0\0\0\x01\x18FileSystemDirectoryEntry\x01\0\0\x01\x05\x05self_\x04path\x07options\x10success_callback\x0Eerror_callback\x07getFile\0\0\0`__widl_f_get_file_with_path_and_options_and_callback_and_error_callback_FileSystemDirectoryEntry\0\0\0\x01\x18FileSystemDirectoryEntry\x01\0\0\x01\x05\x05self_\x04path\x07options\x10success_callback\x0Eerror_callback\x07getFile\0\0\0r__widl_f_get_file_with_path_and_options_and_file_system_entry_callback_and_error_callback_FileSystemDirectoryEntry\0\0\0\x01\x18FileSystemDirectoryEntry\x01\0\0\x01\x05\x05self_\x04path\x07options\x10success_callback\x0Eerror_callback\x07getFile\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
