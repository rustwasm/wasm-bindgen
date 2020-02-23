use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `StorageManager` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager)\n\n*This API requires the following crate features to be activated: `StorageManager`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct StorageManager {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_StorageManager: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for StorageManager {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(83u32);
            inform(116u32);
            inform(111u32);
            inform(114u32);
            inform(97u32);
            inform(103u32);
            inform(101u32);
            inform(77u32);
            inform(97u32);
            inform(110u32);
            inform(97u32);
            inform(103u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for StorageManager {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for StorageManager {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for StorageManager {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a StorageManager {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for StorageManager {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            StorageManager {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for StorageManager {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a StorageManager {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for StorageManager {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<StorageManager>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(StorageManager {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for StorageManager {
        #[inline]
        fn from(obj: JsValue) -> StorageManager {
            StorageManager { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for StorageManager {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<StorageManager> for StorageManager {
        #[inline]
        fn as_ref(&self) -> &StorageManager {
            self
        }
    }
    impl From<StorageManager> for JsValue {
        #[inline]
        fn from(obj: StorageManager) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for StorageManager {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_StorageManager(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_StorageManager(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_StorageManager(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            StorageManager { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const StorageManager) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<StorageManager> for ::js_sys::Object {
    #[inline]
    fn from(obj: StorageManager) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for StorageManager {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "StorageManager",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_estimate_StorageManager() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&StorageManager as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl StorageManager {
    #[cfg(all(feature = "StorageManager",))]
    #[allow(bad_style)]
    #[doc = "The `estimate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/estimate)\n\n*This API requires the following crate features to be activated: `StorageManager`*"]
    #[allow(clippy::all)]
    pub fn estimate(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "StorageManager",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_estimate_StorageManager(
                self_: <&StorageManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_estimate_StorageManager(
            self_: <&StorageManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StorageManager as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_estimate_StorageManager(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "StorageManager",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_persist_StorageManager() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&StorageManager as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl StorageManager {
    #[cfg(all(feature = "StorageManager",))]
    #[allow(bad_style)]
    #[doc = "The `persist()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/persist)\n\n*This API requires the following crate features to be activated: `StorageManager`*"]
    #[allow(clippy::all)]
    pub fn persist(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "StorageManager",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_persist_StorageManager(
                self_: <&StorageManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_persist_StorageManager(
            self_: <&StorageManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StorageManager as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_persist_StorageManager(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "StorageManager",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_persisted_StorageManager() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&StorageManager as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl StorageManager {
    #[cfg(all(feature = "StorageManager",))]
    #[allow(bad_style)]
    #[doc = "The `persisted()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/persisted)\n\n*This API requires the following crate features to be activated: `StorageManager`*"]
    #[allow(clippy::all)]
    pub fn persisted(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "StorageManager",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_persisted_StorageManager(
                self_: <&StorageManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_persisted_StorageManager(
            self_: <&StorageManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StorageManager as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_persisted_StorageManager(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_74707745146c3f6a: [u8; 382usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}<\x01\0\0\0\0\x04\0\0\x02\x0EStorageManager __widl_instanceof_StorageManager\0\0\0\0 __widl_f_estimate_StorageManager\x01\0\0\x01\x0EStorageManager\x01\0\0\x01\x01\x05self_\x08estimate\0\0\0\x1F__widl_f_persist_StorageManager\x01\0\0\x01\x0EStorageManager\x01\0\0\x01\x01\x05self_\x07persist\0\0\0!__widl_f_persisted_StorageManager\x01\0\0\x01\x0EStorageManager\x01\0\0\x01\x01\x05self_\tpersisted\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
