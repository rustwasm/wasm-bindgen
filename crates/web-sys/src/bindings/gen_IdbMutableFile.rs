use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `IDBMutableFile` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile)\n\n*This API requires the following crate features to be activated: `IdbMutableFile`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct IdbMutableFile {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_IdbMutableFile: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for IdbMutableFile {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(73u32);
            inform(68u32);
            inform(66u32);
            inform(77u32);
            inform(117u32);
            inform(116u32);
            inform(97u32);
            inform(98u32);
            inform(108u32);
            inform(101u32);
            inform(70u32);
            inform(105u32);
            inform(108u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for IdbMutableFile {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for IdbMutableFile {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for IdbMutableFile {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IdbMutableFile {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for IdbMutableFile {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            IdbMutableFile {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for IdbMutableFile {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a IdbMutableFile {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for IdbMutableFile {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<IdbMutableFile>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(IdbMutableFile {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for IdbMutableFile {
        #[inline]
        fn from(obj: JsValue) -> IdbMutableFile {
            IdbMutableFile { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for IdbMutableFile {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<IdbMutableFile> for IdbMutableFile {
        #[inline]
        fn as_ref(&self) -> &IdbMutableFile {
            self
        }
    }
    impl From<IdbMutableFile> for JsValue {
        #[inline]
        fn from(obj: IdbMutableFile) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for IdbMutableFile {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_IDBMutableFile(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_IDBMutableFile(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_IDBMutableFile(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IdbMutableFile { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IdbMutableFile) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<IdbMutableFile> for EventTarget {
    #[inline]
    fn from(obj: IdbMutableFile) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for IdbMutableFile {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<IdbMutableFile> for ::js_sys::Object {
    #[inline]
    fn from(obj: IdbMutableFile) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for IdbMutableFile {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DomRequest", feature = "IdbMutableFile",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_file_IDBMutableFile() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbMutableFile as WasmDescribe>::describe();
    <DomRequest as WasmDescribe>::describe();
}
impl IdbMutableFile {
    #[cfg(all(feature = "DomRequest", feature = "IdbMutableFile",))]
    #[allow(bad_style)]
    #[doc = "The `getFile()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/getFile)\n\n*This API requires the following crate features to be activated: `DomRequest`, `IdbMutableFile`*"]
    #[allow(clippy::all)]
    pub fn get_file(&self) -> Result<DomRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomRequest", feature = "IdbMutableFile",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_file_IDBMutableFile(
                self_: <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_file_IDBMutableFile(
            self_: <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_file_IDBMutableFile(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbMutableFile",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_IDBMutableFile() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbMutableFile as WasmDescribe>::describe();
    <IdbFileHandle as WasmDescribe>::describe();
}
impl IdbMutableFile {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbMutableFile",))]
    #[allow(bad_style)]
    #[doc = "The `open()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/open)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbMutableFile`*"]
    #[allow(clippy::all)]
    pub fn open(&self) -> Result<IdbFileHandle, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbMutableFile",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_IDBMutableFile(
                self_: <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbFileHandle as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_IDBMutableFile(
            self_: <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbFileHandle as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_open_IDBMutableFile(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbFileHandle as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbMutableFile",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_IDBMutableFile() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbMutableFile as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl IdbMutableFile {
    #[cfg(all(feature = "IdbMutableFile",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/name)\n\n*This API requires the following crate features to be activated: `IdbMutableFile`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "IdbMutableFile",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_IDBMutableFile(
                self_: <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_IDBMutableFile(
            self_: <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_IDBMutableFile(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbMutableFile",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_IDBMutableFile() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbMutableFile as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl IdbMutableFile {
    #[cfg(all(feature = "IdbMutableFile",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/type)\n\n*This API requires the following crate features to be activated: `IdbMutableFile`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "IdbMutableFile",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_IDBMutableFile(
                self_: <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_IDBMutableFile(
            self_: <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_IDBMutableFile(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbDatabase", feature = "IdbMutableFile",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_database_IDBMutableFile() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbMutableFile as WasmDescribe>::describe();
    <IdbDatabase as WasmDescribe>::describe();
}
impl IdbMutableFile {
    #[cfg(all(feature = "IdbDatabase", feature = "IdbMutableFile",))]
    #[allow(bad_style)]
    #[doc = "The `database` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/database)\n\n*This API requires the following crate features to be activated: `IdbDatabase`, `IdbMutableFile`*"]
    #[allow(clippy::all)]
    pub fn database(&self) -> IdbDatabase {
        #[cfg(all(feature = "IdbDatabase", feature = "IdbMutableFile",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_database_IDBMutableFile(
                self_: <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbDatabase as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_database_IDBMutableFile(
            self_: <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbDatabase as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_database_IDBMutableFile(self_)
            };
            <IdbDatabase as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbMutableFile",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onabort_IDBMutableFile() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbMutableFile as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl IdbMutableFile {
    #[cfg(all(feature = "IdbMutableFile",))]
    #[allow(bad_style)]
    #[doc = "The `onabort` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/onabort)\n\n*This API requires the following crate features to be activated: `IdbMutableFile`*"]
    #[allow(clippy::all)]
    pub fn onabort(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "IdbMutableFile",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onabort_IDBMutableFile(
                self_: <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onabort_IDBMutableFile(
            self_: <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onabort_IDBMutableFile(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbMutableFile",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onabort_IDBMutableFile() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbMutableFile as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbMutableFile {
    #[cfg(all(feature = "IdbMutableFile",))]
    #[allow(bad_style)]
    #[doc = "The `onabort` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/onabort)\n\n*This API requires the following crate features to be activated: `IdbMutableFile`*"]
    #[allow(clippy::all)]
    pub fn set_onabort(&self, onabort: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "IdbMutableFile",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onabort_IDBMutableFile(
                self_: <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onabort: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onabort_IDBMutableFile(
            self_: <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onabort =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onabort,
                    );
                __widl_f_set_onabort_IDBMutableFile(self_, onabort)
            };
            ()
        }
    }
}
#[cfg(all(feature = "IdbMutableFile",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_IDBMutableFile() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbMutableFile as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl IdbMutableFile {
    #[cfg(all(feature = "IdbMutableFile",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/onerror)\n\n*This API requires the following crate features to be activated: `IdbMutableFile`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "IdbMutableFile",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_IDBMutableFile(
                self_: <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_IDBMutableFile(
            self_: <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_IDBMutableFile(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbMutableFile",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_IDBMutableFile() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbMutableFile as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbMutableFile {
    #[cfg(all(feature = "IdbMutableFile",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/onerror)\n\n*This API requires the following crate features to be activated: `IdbMutableFile`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "IdbMutableFile",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_IDBMutableFile(
                self_: <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_IDBMutableFile(
            self_: <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbMutableFile as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_IDBMutableFile(self_, onerror)
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
pub static __WASM_BINDGEN_GENERATED_b7ae78f5b3c998bf: [u8; 874usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}(\x03\0\0\0\0\n\0\0\x02\x0EIDBMutableFile __widl_instanceof_IDBMutableFile\0\0\0\0 __widl_f_get_file_IDBMutableFile\x01\0\0\x01\x0EIDBMutableFile\x01\0\0\x01\x01\x05self_\x07getFile\0\0\0\x1C__widl_f_open_IDBMutableFile\x01\0\0\x01\x0EIDBMutableFile\x01\0\0\x01\x01\x05self_\x04open\0\0\0\x1C__widl_f_name_IDBMutableFile\0\0\0\x01\x0EIDBMutableFile\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0\x1C__widl_f_type_IDBMutableFile\0\0\0\x01\x0EIDBMutableFile\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0 __widl_f_database_IDBMutableFile\0\0\0\x01\x0EIDBMutableFile\x01\0\x01\x08database\x01\x01\x05self_\x08database\0\0\0\x1F__widl_f_onabort_IDBMutableFile\0\0\0\x01\x0EIDBMutableFile\x01\0\x01\x07onabort\x01\x01\x05self_\x07onabort\0\0\0#__widl_f_set_onabort_IDBMutableFile\0\0\0\x01\x0EIDBMutableFile\x01\0\x02\x07onabort\x01\x02\x05self_\x07onabort\x07onabort\0\0\0\x1F__widl_f_onerror_IDBMutableFile\0\0\0\x01\x0EIDBMutableFile\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0#__widl_f_set_onerror_IDBMutableFile\0\0\0\x01\x0EIDBMutableFile\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
