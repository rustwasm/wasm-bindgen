use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `FileSystemFileEntry` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry)\n\n*This API requires the following crate features to be activated: `FileSystemFileEntry`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct FileSystemFileEntry {
    obj: FileSystemEntry,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_FileSystemFileEntry: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for FileSystemFileEntry {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
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
            inform(70u32);
            inform(105u32);
            inform(108u32);
            inform(101u32);
            inform(69u32);
            inform(110u32);
            inform(116u32);
            inform(114u32);
            inform(121u32);
        }
    }
    impl core::ops::Deref for FileSystemFileEntry {
        type Target = FileSystemEntry;
        #[inline]
        fn deref(&self) -> &FileSystemEntry {
            &self.obj
        }
    }
    impl IntoWasmAbi for FileSystemFileEntry {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for FileSystemFileEntry {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a FileSystemFileEntry {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for FileSystemFileEntry {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            FileSystemFileEntry {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for FileSystemFileEntry {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a FileSystemFileEntry {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for FileSystemFileEntry {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<FileSystemFileEntry>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(FileSystemFileEntry {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for FileSystemFileEntry {
        #[inline]
        fn from(obj: JsValue) -> FileSystemFileEntry {
            FileSystemFileEntry { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for FileSystemFileEntry {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<FileSystemFileEntry> for FileSystemFileEntry {
        #[inline]
        fn as_ref(&self) -> &FileSystemFileEntry {
            self
        }
    }
    impl From<FileSystemFileEntry> for JsValue {
        #[inline]
        fn from(obj: FileSystemFileEntry) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for FileSystemFileEntry {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_FileSystemFileEntry(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_FileSystemFileEntry(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_FileSystemFileEntry(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            FileSystemFileEntry { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const FileSystemFileEntry) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<FileSystemFileEntry> for FileSystemEntry {
    #[inline]
    fn from(obj: FileSystemFileEntry) -> FileSystemEntry {
        use wasm_bindgen::JsCast;
        FileSystemEntry::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<FileSystemEntry> for FileSystemFileEntry {
    #[inline]
    fn as_ref(&self) -> &FileSystemEntry {
        use wasm_bindgen::JsCast;
        FileSystemEntry::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<FileSystemFileEntry> for ::js_sys::Object {
    #[inline]
    fn from(obj: FileSystemFileEntry) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for FileSystemFileEntry {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "FileSystemFileEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_file_with_callback_FileSystemFileEntry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileSystemFileEntry as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemFileEntry {
    #[cfg(all(feature = "FileSystemFileEntry",))]
    #[allow(bad_style)]
    #[doc = "The `file()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry/file)\n\n*This API requires the following crate features to be activated: `FileSystemFileEntry`*"]
    #[allow(clippy::all)]
    pub fn file_with_callback(&self, success_callback: &::js_sys::Function) {
        #[cfg(all(feature = "FileSystemFileEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_file_with_callback_FileSystemFileEntry(
                self_: <&FileSystemFileEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_file_with_callback_FileSystemFileEntry(
            self_: <&FileSystemFileEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&FileSystemFileEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                __widl_f_file_with_callback_FileSystemFileEntry(self_, success_callback)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileCallback", feature = "FileSystemFileEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_file_with_file_callback_FileSystemFileEntry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileSystemFileEntry as WasmDescribe>::describe();
    <&FileCallback as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemFileEntry {
    #[cfg(all(feature = "FileCallback", feature = "FileSystemFileEntry",))]
    #[allow(bad_style)]
    #[doc = "The `file()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry/file)\n\n*This API requires the following crate features to be activated: `FileCallback`, `FileSystemFileEntry`*"]
    #[allow(clippy::all)]
    pub fn file_with_file_callback(&self, success_callback: &FileCallback) {
        #[cfg(all(feature = "FileCallback", feature = "FileSystemFileEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_file_with_file_callback_FileSystemFileEntry(
                self_: <&FileSystemFileEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&FileCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_file_with_file_callback_FileSystemFileEntry(
            self_: <&FileSystemFileEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&FileCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&FileSystemFileEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let success_callback =
                    <&FileCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                __widl_f_file_with_file_callback_FileSystemFileEntry(self_, success_callback)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileSystemFileEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_file_with_callback_and_callback_FileSystemFileEntry()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&FileSystemFileEntry as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemFileEntry {
    #[cfg(all(feature = "FileSystemFileEntry",))]
    #[allow(bad_style)]
    #[doc = "The `file()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry/file)\n\n*This API requires the following crate features to be activated: `FileSystemFileEntry`*"]
    #[allow(clippy::all)]
    pub fn file_with_callback_and_callback(
        &self,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    ) {
        #[cfg(all(feature = "FileSystemFileEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_file_with_callback_and_callback_FileSystemFileEntry(
                self_: <&FileSystemFileEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_file_with_callback_and_callback_FileSystemFileEntry(
            self_: <&FileSystemFileEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&FileSystemFileEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_file_with_callback_and_callback_FileSystemFileEntry(
                    self_,
                    success_callback,
                    error_callback,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileCallback", feature = "FileSystemFileEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_file_with_file_callback_and_callback_FileSystemFileEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&FileSystemFileEntry as WasmDescribe>::describe();
    <&FileCallback as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemFileEntry {
    #[cfg(all(feature = "FileCallback", feature = "FileSystemFileEntry",))]
    #[allow(bad_style)]
    #[doc = "The `file()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry/file)\n\n*This API requires the following crate features to be activated: `FileCallback`, `FileSystemFileEntry`*"]
    #[allow(clippy::all)]
    pub fn file_with_file_callback_and_callback(
        &self,
        success_callback: &FileCallback,
        error_callback: &::js_sys::Function,
    ) {
        #[cfg(all(feature = "FileCallback", feature = "FileSystemFileEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_file_with_file_callback_and_callback_FileSystemFileEntry(
                self_: <&FileSystemFileEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&FileCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_file_with_file_callback_and_callback_FileSystemFileEntry(
            self_: <&FileSystemFileEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&FileCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&FileSystemFileEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let success_callback =
                    <&FileCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_file_with_file_callback_and_callback_FileSystemFileEntry(
                    self_,
                    success_callback,
                    error_callback,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "ErrorCallback", feature = "FileSystemFileEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_file_with_callback_and_error_callback_FileSystemFileEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&FileSystemFileEntry as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&ErrorCallback as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemFileEntry {
    #[cfg(all(feature = "ErrorCallback", feature = "FileSystemFileEntry",))]
    #[allow(bad_style)]
    #[doc = "The `file()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry/file)\n\n*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemFileEntry`*"]
    #[allow(clippy::all)]
    pub fn file_with_callback_and_error_callback(
        &self,
        success_callback: &::js_sys::Function,
        error_callback: &ErrorCallback,
    ) {
        #[cfg(all(feature = "ErrorCallback", feature = "FileSystemFileEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_file_with_callback_and_error_callback_FileSystemFileEntry(
                self_: <&FileSystemFileEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error_callback: <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_file_with_callback_and_error_callback_FileSystemFileEntry(
            self_: <&FileSystemFileEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&FileSystemFileEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_file_with_callback_and_error_callback_FileSystemFileEntry(
                    self_,
                    success_callback,
                    error_callback,
                )
            };
            ()
        }
    }
}
#[cfg(all(
    feature = "ErrorCallback",
    feature = "FileCallback",
    feature = "FileSystemFileEntry",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_file_with_file_callback_and_error_callback_FileSystemFileEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&FileSystemFileEntry as WasmDescribe>::describe();
    <&FileCallback as WasmDescribe>::describe();
    <&ErrorCallback as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemFileEntry {
    #[cfg(all(
        feature = "ErrorCallback",
        feature = "FileCallback",
        feature = "FileSystemFileEntry",
    ))]
    #[allow(bad_style)]
    #[doc = "The `file()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry/file)\n\n*This API requires the following crate features to be activated: `ErrorCallback`, `FileCallback`, `FileSystemFileEntry`*"]
    #[allow(clippy::all)]
    pub fn file_with_file_callback_and_error_callback(
        &self,
        success_callback: &FileCallback,
        error_callback: &ErrorCallback,
    ) {
        #[cfg(all(
            feature = "ErrorCallback",
            feature = "FileCallback",
            feature = "FileSystemFileEntry",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_file_with_file_callback_and_error_callback_FileSystemFileEntry(
                self_: <&FileSystemFileEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&FileCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error_callback: <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_file_with_file_callback_and_error_callback_FileSystemFileEntry(
            self_: <&FileSystemFileEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&FileCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&FileSystemFileEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let success_callback =
                    <&FileCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_file_with_file_callback_and_error_callback_FileSystemFileEntry(
                    self_,
                    success_callback,
                    error_callback,
                )
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
pub static __WASM_BINDGEN_GENERATED_18235d7549fb9ca7: [u8; 954usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}x\x03\0\0\0\0\x07\0\0\x02\x13FileSystemFileEntry%__widl_instanceof_FileSystemFileEntry\0\0\0\0/__widl_f_file_with_callback_FileSystemFileEntry\0\0\0\x01\x13FileSystemFileEntry\x01\0\0\x01\x02\x05self_\x10success_callback\x04file\0\0\04__widl_f_file_with_file_callback_FileSystemFileEntry\0\0\0\x01\x13FileSystemFileEntry\x01\0\0\x01\x02\x05self_\x10success_callback\x04file\0\0\0<__widl_f_file_with_callback_and_callback_FileSystemFileEntry\0\0\0\x01\x13FileSystemFileEntry\x01\0\0\x01\x03\x05self_\x10success_callback\x0Eerror_callback\x04file\0\0\0A__widl_f_file_with_file_callback_and_callback_FileSystemFileEntry\0\0\0\x01\x13FileSystemFileEntry\x01\0\0\x01\x03\x05self_\x10success_callback\x0Eerror_callback\x04file\0\0\0B__widl_f_file_with_callback_and_error_callback_FileSystemFileEntry\0\0\0\x01\x13FileSystemFileEntry\x01\0\0\x01\x03\x05self_\x10success_callback\x0Eerror_callback\x04file\0\0\0G__widl_f_file_with_file_callback_and_error_callback_FileSystemFileEntry\0\0\0\x01\x13FileSystemFileEntry\x01\0\0\x01\x03\x05self_\x10success_callback\x0Eerror_callback\x04file\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
