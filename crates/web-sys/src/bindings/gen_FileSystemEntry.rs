use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `FileSystemEntry` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry)\n\n*This API requires the following crate features to be activated: `FileSystemEntry`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct FileSystemEntry {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_FileSystemEntry: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for FileSystemEntry {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
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
            inform(69u32);
            inform(110u32);
            inform(116u32);
            inform(114u32);
            inform(121u32);
        }
    }
    impl core::ops::Deref for FileSystemEntry {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for FileSystemEntry {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for FileSystemEntry {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a FileSystemEntry {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for FileSystemEntry {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            FileSystemEntry {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for FileSystemEntry {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a FileSystemEntry {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for FileSystemEntry {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<FileSystemEntry>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(FileSystemEntry {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for FileSystemEntry {
        #[inline]
        fn from(obj: JsValue) -> FileSystemEntry {
            FileSystemEntry { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for FileSystemEntry {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<FileSystemEntry> for FileSystemEntry {
        #[inline]
        fn as_ref(&self) -> &FileSystemEntry {
            self
        }
    }
    impl From<FileSystemEntry> for JsValue {
        #[inline]
        fn from(obj: FileSystemEntry) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for FileSystemEntry {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_FileSystemEntry(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_FileSystemEntry(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_FileSystemEntry(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            FileSystemEntry { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const FileSystemEntry) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<FileSystemEntry> for ::js_sys::Object {
    #[inline]
    fn from(obj: FileSystemEntry) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for FileSystemEntry {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "FileSystemEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_parent_FileSystemEntry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FileSystemEntry as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemEntry {
    #[cfg(all(feature = "FileSystemEntry",))]
    #[allow(bad_style)]
    #[doc = "The `getParent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)\n\n*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    #[allow(clippy::all)]
    pub fn get_parent(&self) {
        #[cfg(all(feature = "FileSystemEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_parent_FileSystemEntry(
                self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_parent_FileSystemEntry(
            self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_parent_FileSystemEntry(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileSystemEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_parent_with_callback_FileSystemEntry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileSystemEntry as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemEntry {
    #[cfg(all(feature = "FileSystemEntry",))]
    #[allow(bad_style)]
    #[doc = "The `getParent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)\n\n*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    #[allow(clippy::all)]
    pub fn get_parent_with_callback(&self, success_callback: &::js_sys::Function) {
        #[cfg(all(feature = "FileSystemEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_parent_with_callback_FileSystemEntry(
                self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_parent_with_callback_FileSystemEntry(
            self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                __widl_f_get_parent_with_callback_FileSystemEntry(self_, success_callback)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileSystemEntry", feature = "FileSystemEntryCallback",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_parent_with_file_system_entry_callback_FileSystemEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FileSystemEntry as WasmDescribe>::describe();
    <&FileSystemEntryCallback as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemEntry {
    #[cfg(all(feature = "FileSystemEntry", feature = "FileSystemEntryCallback",))]
    #[allow(bad_style)]
    #[doc = "The `getParent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)\n\n*This API requires the following crate features to be activated: `FileSystemEntry`, `FileSystemEntryCallback`*"]
    #[allow(clippy::all)]
    pub fn get_parent_with_file_system_entry_callback(
        &self,
        success_callback: &FileSystemEntryCallback,
    ) {
        #[cfg(all(feature = "FileSystemEntry", feature = "FileSystemEntryCallback",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_parent_with_file_system_entry_callback_FileSystemEntry(
                self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback : < & FileSystemEntryCallback as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_parent_with_file_system_entry_callback_FileSystemEntry(
            self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&FileSystemEntryCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let success_callback =
                    <&FileSystemEntryCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                __widl_f_get_parent_with_file_system_entry_callback_FileSystemEntry(
                    self_,
                    success_callback,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileSystemEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_parent_with_callback_and_callback_FileSystemEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&FileSystemEntry as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemEntry {
    #[cfg(all(feature = "FileSystemEntry",))]
    #[allow(bad_style)]
    #[doc = "The `getParent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)\n\n*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    #[allow(clippy::all)]
    pub fn get_parent_with_callback_and_callback(
        &self,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    ) {
        #[cfg(all(feature = "FileSystemEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_parent_with_callback_and_callback_FileSystemEntry(
                self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_parent_with_callback_and_callback_FileSystemEntry(
            self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_get_parent_with_callback_and_callback_FileSystemEntry(
                    self_,
                    success_callback,
                    error_callback,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileSystemEntry", feature = "FileSystemEntryCallback",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_parent_with_file_system_entry_callback_and_callback_FileSystemEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&FileSystemEntry as WasmDescribe>::describe();
    <&FileSystemEntryCallback as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemEntry {
    #[cfg(all(feature = "FileSystemEntry", feature = "FileSystemEntryCallback",))]
    #[allow(bad_style)]
    #[doc = "The `getParent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)\n\n*This API requires the following crate features to be activated: `FileSystemEntry`, `FileSystemEntryCallback`*"]
    #[allow(clippy::all)]
    pub fn get_parent_with_file_system_entry_callback_and_callback(
        &self,
        success_callback: &FileSystemEntryCallback,
        error_callback: &::js_sys::Function,
    ) {
        #[cfg(all(feature = "FileSystemEntry", feature = "FileSystemEntryCallback",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_parent_with_file_system_entry_callback_and_callback_FileSystemEntry(
                self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback : < & FileSystemEntryCallback as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                error_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_parent_with_file_system_entry_callback_and_callback_FileSystemEntry(
            self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&FileSystemEntryCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let success_callback =
                    <&FileSystemEntryCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_get_parent_with_file_system_entry_callback_and_callback_FileSystemEntry(
                    self_,
                    success_callback,
                    error_callback,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "ErrorCallback", feature = "FileSystemEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_parent_with_callback_and_error_callback_FileSystemEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&FileSystemEntry as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&ErrorCallback as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemEntry {
    #[cfg(all(feature = "ErrorCallback", feature = "FileSystemEntry",))]
    #[allow(bad_style)]
    #[doc = "The `getParent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)\n\n*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemEntry`*"]
    #[allow(clippy::all)]
    pub fn get_parent_with_callback_and_error_callback(
        &self,
        success_callback: &::js_sys::Function,
        error_callback: &ErrorCallback,
    ) {
        #[cfg(all(feature = "ErrorCallback", feature = "FileSystemEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_parent_with_callback_and_error_callback_FileSystemEntry(
                self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error_callback: <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_parent_with_callback_and_error_callback_FileSystemEntry(
            self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_get_parent_with_callback_and_error_callback_FileSystemEntry(
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
    feature = "FileSystemEntry",
    feature = "FileSystemEntryCallback",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_parent_with_file_system_entry_callback_and_error_callback_FileSystemEntry(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&FileSystemEntry as WasmDescribe>::describe();
    <&FileSystemEntryCallback as WasmDescribe>::describe();
    <&ErrorCallback as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FileSystemEntry {
    #[cfg(all(
        feature = "ErrorCallback",
        feature = "FileSystemEntry",
        feature = "FileSystemEntryCallback",
    ))]
    #[allow(bad_style)]
    #[doc = "The `getParent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)\n\n*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemEntry`, `FileSystemEntryCallback`*"]
    #[allow(clippy::all)]
    pub fn get_parent_with_file_system_entry_callback_and_error_callback(
        &self,
        success_callback: &FileSystemEntryCallback,
        error_callback: &ErrorCallback,
    ) {
        #[cfg(all(
            feature = "ErrorCallback",
            feature = "FileSystemEntry",
            feature = "FileSystemEntryCallback",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_parent_with_file_system_entry_callback_and_error_callback_FileSystemEntry(
                self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback : < & FileSystemEntryCallback as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                error_callback: <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_parent_with_file_system_entry_callback_and_error_callback_FileSystemEntry(
            self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&FileSystemEntryCallback as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let success_callback =
                    <&FileSystemEntryCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&ErrorCallback as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_get_parent_with_file_system_entry_callback_and_error_callback_FileSystemEntry ( self_ , success_callback , error_callback )
            };
            ()
        }
    }
}
#[cfg(all(feature = "FileSystemEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_file_FileSystemEntry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FileSystemEntry as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl FileSystemEntry {
    #[cfg(all(feature = "FileSystemEntry",))]
    #[allow(bad_style)]
    #[doc = "The `isFile` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/isFile)\n\n*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    #[allow(clippy::all)]
    pub fn is_file(&self) -> bool {
        #[cfg(all(feature = "FileSystemEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_file_FileSystemEntry(
                self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_file_FileSystemEntry(
            self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_is_file_FileSystemEntry(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FileSystemEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_directory_FileSystemEntry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FileSystemEntry as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl FileSystemEntry {
    #[cfg(all(feature = "FileSystemEntry",))]
    #[allow(bad_style)]
    #[doc = "The `isDirectory` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/isDirectory)\n\n*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    #[allow(clippy::all)]
    pub fn is_directory(&self) -> bool {
        #[cfg(all(feature = "FileSystemEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_directory_FileSystemEntry(
                self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_directory_FileSystemEntry(
            self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_is_directory_FileSystemEntry(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FileSystemEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_FileSystemEntry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FileSystemEntry as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl FileSystemEntry {
    #[cfg(all(feature = "FileSystemEntry",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/name)\n\n*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "FileSystemEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_FileSystemEntry(
                self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_FileSystemEntry(
            self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_FileSystemEntry(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FileSystemEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_full_path_FileSystemEntry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FileSystemEntry as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl FileSystemEntry {
    #[cfg(all(feature = "FileSystemEntry",))]
    #[allow(bad_style)]
    #[doc = "The `fullPath` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/fullPath)\n\n*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    #[allow(clippy::all)]
    pub fn full_path(&self) -> String {
        #[cfg(all(feature = "FileSystemEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_full_path_FileSystemEntry(
                self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_full_path_FileSystemEntry(
            self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_full_path_FileSystemEntry(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FileSystem", feature = "FileSystemEntry",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_filesystem_FileSystemEntry() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FileSystemEntry as WasmDescribe>::describe();
    <FileSystem as WasmDescribe>::describe();
}
impl FileSystemEntry {
    #[cfg(all(feature = "FileSystem", feature = "FileSystemEntry",))]
    #[allow(bad_style)]
    #[doc = "The `filesystem` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/filesystem)\n\n*This API requires the following crate features to be activated: `FileSystem`, `FileSystemEntry`*"]
    #[allow(clippy::all)]
    pub fn filesystem(&self) -> FileSystem {
        #[cfg(all(feature = "FileSystem", feature = "FileSystemEntry",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_filesystem_FileSystemEntry(
                self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <FileSystem as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_filesystem_FileSystemEntry(
            self_: <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <FileSystem as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&FileSystemEntry as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_filesystem_FileSystemEntry(self_)
            };
            <FileSystem as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_e2032e12eda767e1: [u8; 1513usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xA7\x05\0\0\0\0\r\0\0\x02\x0FFileSystemEntry!__widl_instanceof_FileSystemEntry\0\0\0\0#__widl_f_get_parent_FileSystemEntry\0\0\0\x01\x0FFileSystemEntry\x01\0\0\x01\x01\x05self_\tgetParent\0\0\01__widl_f_get_parent_with_callback_FileSystemEntry\0\0\0\x01\x0FFileSystemEntry\x01\0\0\x01\x02\x05self_\x10success_callback\tgetParent\0\0\0C__widl_f_get_parent_with_file_system_entry_callback_FileSystemEntry\0\0\0\x01\x0FFileSystemEntry\x01\0\0\x01\x02\x05self_\x10success_callback\tgetParent\0\0\0>__widl_f_get_parent_with_callback_and_callback_FileSystemEntry\0\0\0\x01\x0FFileSystemEntry\x01\0\0\x01\x03\x05self_\x10success_callback\x0Eerror_callback\tgetParent\0\0\0P__widl_f_get_parent_with_file_system_entry_callback_and_callback_FileSystemEntry\0\0\0\x01\x0FFileSystemEntry\x01\0\0\x01\x03\x05self_\x10success_callback\x0Eerror_callback\tgetParent\0\0\0D__widl_f_get_parent_with_callback_and_error_callback_FileSystemEntry\0\0\0\x01\x0FFileSystemEntry\x01\0\0\x01\x03\x05self_\x10success_callback\x0Eerror_callback\tgetParent\0\0\0V__widl_f_get_parent_with_file_system_entry_callback_and_error_callback_FileSystemEntry\0\0\0\x01\x0FFileSystemEntry\x01\0\0\x01\x03\x05self_\x10success_callback\x0Eerror_callback\tgetParent\0\0\0 __widl_f_is_file_FileSystemEntry\0\0\0\x01\x0FFileSystemEntry\x01\0\x01\x06isFile\x01\x01\x05self_\x06isFile\0\0\0%__widl_f_is_directory_FileSystemEntry\0\0\0\x01\x0FFileSystemEntry\x01\0\x01\x0BisDirectory\x01\x01\x05self_\x0BisDirectory\0\0\0\x1D__widl_f_name_FileSystemEntry\0\0\0\x01\x0FFileSystemEntry\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0\"__widl_f_full_path_FileSystemEntry\0\0\0\x01\x0FFileSystemEntry\x01\0\x01\x08fullPath\x01\x01\x05self_\x08fullPath\0\0\0#__widl_f_filesystem_FileSystemEntry\0\0\0\x01\x0FFileSystemEntry\x01\0\x01\nfilesystem\x01\x01\x05self_\nfilesystem\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
