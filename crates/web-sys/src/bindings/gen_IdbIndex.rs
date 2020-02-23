use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `IDBIndex` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex)\n\n*This API requires the following crate features to be activated: `IdbIndex`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct IdbIndex {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_IdbIndex: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for IdbIndex {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(8u32);
            inform(73u32);
            inform(68u32);
            inform(66u32);
            inform(73u32);
            inform(110u32);
            inform(100u32);
            inform(101u32);
            inform(120u32);
        }
    }
    impl core::ops::Deref for IdbIndex {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for IdbIndex {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for IdbIndex {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IdbIndex {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for IdbIndex {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            IdbIndex {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for IdbIndex {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a IdbIndex {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for IdbIndex {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<IdbIndex>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(IdbIndex {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for IdbIndex {
        #[inline]
        fn from(obj: JsValue) -> IdbIndex {
            IdbIndex { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for IdbIndex {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<IdbIndex> for IdbIndex {
        #[inline]
        fn as_ref(&self) -> &IdbIndex {
            self
        }
    }
    impl From<IdbIndex> for JsValue {
        #[inline]
        fn from(obj: IdbIndex) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for IdbIndex {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_IDBIndex(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_IDBIndex(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_IDBIndex(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IdbIndex { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IdbIndex) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<IdbIndex> for ::js_sys::Object {
    #[inline]
    fn from(obj: IdbIndex) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for IdbIndex {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_count_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbIndex as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `count()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/count)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn count(&self) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_count_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_count_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_count_IDBIndex(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_count_with_key_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbIndex as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `count()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/count)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn count_with_key(
        &self,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_count_with_key_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_count_with_key_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_count_with_key_IDBIndex(self_, key)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbIndex as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `get()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/get)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn get(
        &self,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_get_IDBIndex(self_, key)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_all_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbIndex as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `getAll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAll)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn get_all(&self) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_all_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_all_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_all_IDBIndex(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_all_with_key_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbIndex as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `getAll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAll)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn get_all_with_key(
        &self,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_all_with_key_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_all_with_key_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_get_all_with_key_IDBIndex(self_, key)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_all_with_key_and_limit_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbIndex as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `getAll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAll)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn get_all_with_key_and_limit(
        &self,
        key: &::wasm_bindgen::JsValue,
        limit: u32,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_all_with_key_and_limit_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                limit: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_all_with_key_and_limit_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            limit: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(key);
            drop(limit);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let limit = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(limit);
                __widl_f_get_all_with_key_and_limit_IDBIndex(self_, key, limit)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_all_keys_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbIndex as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `getAllKeys()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAllKeys)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn get_all_keys(&self) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_all_keys_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_all_keys_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_all_keys_IDBIndex(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_all_keys_with_key_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbIndex as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `getAllKeys()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAllKeys)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn get_all_keys_with_key(
        &self,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_all_keys_with_key_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_all_keys_with_key_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_get_all_keys_with_key_IDBIndex(self_, key)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_all_keys_with_key_and_limit_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbIndex as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `getAllKeys()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAllKeys)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn get_all_keys_with_key_and_limit(
        &self,
        key: &::wasm_bindgen::JsValue,
        limit: u32,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_all_keys_with_key_and_limit_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                limit: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_all_keys_with_key_and_limit_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            limit: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(key);
            drop(limit);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let limit = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(limit);
                __widl_f_get_all_keys_with_key_and_limit_IDBIndex(self_, key, limit)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_key_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbIndex as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `getKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getKey)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn get_key(
        &self,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_key_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_key_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_get_key_IDBIndex(self_, key)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_cursor_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbIndex as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `openCursor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openCursor)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn open_cursor(&self) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_cursor_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_cursor_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_open_cursor_IDBIndex(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_cursor_with_range_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbIndex as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `openCursor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openCursor)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn open_cursor_with_range(
        &self,
        range: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_cursor_with_range_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                range: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_cursor_with_range_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            range: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(range);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let range =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        range,
                    );
                __widl_f_open_cursor_with_range_IDBIndex(self_, range)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "IdbCursorDirection",
    feature = "IdbIndex",
    feature = "IdbRequest",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_cursor_with_range_and_direction_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbIndex as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbCursorDirection as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(
        feature = "IdbCursorDirection",
        feature = "IdbIndex",
        feature = "IdbRequest",
    ))]
    #[allow(bad_style)]
    #[doc = "The `openCursor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openCursor)\n\n*This API requires the following crate features to be activated: `IdbCursorDirection`, `IdbIndex`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn open_cursor_with_range_and_direction(
        &self,
        range: &::wasm_bindgen::JsValue,
        direction: IdbCursorDirection,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "IdbCursorDirection",
            feature = "IdbIndex",
            feature = "IdbRequest",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_cursor_with_range_and_direction_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                range: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                direction: <IdbCursorDirection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_cursor_with_range_and_direction_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            range: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            direction: <IdbCursorDirection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(range);
            drop(direction);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let range =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        range,
                    );
                let direction =
                    <IdbCursorDirection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(direction);
                __widl_f_open_cursor_with_range_and_direction_IDBIndex(self_, range, direction)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_key_cursor_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbIndex as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `openKeyCursor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openKeyCursor)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn open_key_cursor(&self) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_key_cursor_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_key_cursor_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_open_key_cursor_IDBIndex(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_key_cursor_with_range_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbIndex as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `openKeyCursor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openKeyCursor)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn open_key_cursor_with_range(
        &self,
        range: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbIndex", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_key_cursor_with_range_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                range: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_key_cursor_with_range_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            range: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(range);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let range =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        range,
                    );
                __widl_f_open_key_cursor_with_range_IDBIndex(self_, range)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "IdbCursorDirection",
    feature = "IdbIndex",
    feature = "IdbRequest",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_key_cursor_with_range_and_direction_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbIndex as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbCursorDirection as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(
        feature = "IdbCursorDirection",
        feature = "IdbIndex",
        feature = "IdbRequest",
    ))]
    #[allow(bad_style)]
    #[doc = "The `openKeyCursor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openKeyCursor)\n\n*This API requires the following crate features to be activated: `IdbCursorDirection`, `IdbIndex`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn open_key_cursor_with_range_and_direction(
        &self,
        range: &::wasm_bindgen::JsValue,
        direction: IdbCursorDirection,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "IdbCursorDirection",
            feature = "IdbIndex",
            feature = "IdbRequest",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_key_cursor_with_range_and_direction_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                range: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                direction: <IdbCursorDirection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_key_cursor_with_range_and_direction_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            range: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            direction: <IdbCursorDirection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(range);
            drop(direction);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let range =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        range,
                    );
                let direction =
                    <IdbCursorDirection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(direction);
                __widl_f_open_key_cursor_with_range_and_direction_IDBIndex(self_, range, direction)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbIndex",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbIndex as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/name)\n\n*This API requires the following crate features to be activated: `IdbIndex`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "IdbIndex",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_IDBIndex(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbIndex",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_name_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbIndex as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex",))]
    #[allow(bad_style)]
    #[doc = "The `name` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/name)\n\n*This API requires the following crate features to be activated: `IdbIndex`*"]
    #[allow(clippy::all)]
    pub fn set_name(&self, name: &str) {
        #[cfg(all(feature = "IdbIndex",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_name_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_name_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_set_name_IDBIndex(self_, name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "IdbIndex", feature = "IdbObjectStore",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_object_store_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbIndex as WasmDescribe>::describe();
    <IdbObjectStore as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex", feature = "IdbObjectStore",))]
    #[allow(bad_style)]
    #[doc = "The `objectStore` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/objectStore)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbObjectStore`*"]
    #[allow(clippy::all)]
    pub fn object_store(&self) -> IdbObjectStore {
        #[cfg(all(feature = "IdbIndex", feature = "IdbObjectStore",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_object_store_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbObjectStore as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_object_store_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbObjectStore as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_object_store_IDBIndex(self_)
            };
            <IdbObjectStore as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbIndex",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_key_path_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbIndex as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex",))]
    #[allow(bad_style)]
    #[doc = "The `keyPath` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/keyPath)\n\n*This API requires the following crate features to be activated: `IdbIndex`*"]
    #[allow(clippy::all)]
    pub fn key_path(&self) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbIndex",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_key_path_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_key_path_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_key_path_IDBIndex(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbIndex",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_multi_entry_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbIndex as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex",))]
    #[allow(bad_style)]
    #[doc = "The `multiEntry` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/multiEntry)\n\n*This API requires the following crate features to be activated: `IdbIndex`*"]
    #[allow(clippy::all)]
    pub fn multi_entry(&self) -> bool {
        #[cfg(all(feature = "IdbIndex",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_multi_entry_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_multi_entry_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_multi_entry_IDBIndex(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbIndex",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unique_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbIndex as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex",))]
    #[allow(bad_style)]
    #[doc = "The `unique` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/unique)\n\n*This API requires the following crate features to be activated: `IdbIndex`*"]
    #[allow(clippy::all)]
    pub fn unique(&self) -> bool {
        #[cfg(all(feature = "IdbIndex",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unique_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unique_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_unique_IDBIndex(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbIndex",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_locale_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbIndex as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex",))]
    #[allow(bad_style)]
    #[doc = "The `locale` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/locale)\n\n*This API requires the following crate features to be activated: `IdbIndex`*"]
    #[allow(clippy::all)]
    pub fn locale(&self) -> Option<String> {
        #[cfg(all(feature = "IdbIndex",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_locale_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_locale_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_locale_IDBIndex(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbIndex",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_auto_locale_IDBIndex() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbIndex as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl IdbIndex {
    #[cfg(all(feature = "IdbIndex",))]
    #[allow(bad_style)]
    #[doc = "The `isAutoLocale` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/isAutoLocale)\n\n*This API requires the following crate features to be activated: `IdbIndex`*"]
    #[allow(clippy::all)]
    pub fn is_auto_locale(&self) -> bool {
        #[cfg(all(feature = "IdbIndex",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_auto_locale_IDBIndex(
                self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_auto_locale_IDBIndex(
            self_: <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbIndex as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_is_auto_locale_IDBIndex(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_97a2db01c6901d93: [u8; 1987usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x81\x07\0\0\0\0\x19\0\0\x02\x08IDBIndex\x1A__widl_instanceof_IDBIndex\0\0\0\0\x17__widl_f_count_IDBIndex\x01\0\0\x01\x08IDBIndex\x01\0\0\x01\x01\x05self_\x05count\0\0\0 __widl_f_count_with_key_IDBIndex\x01\0\0\x01\x08IDBIndex\x01\0\0\x01\x02\x05self_\x03key\x05count\0\0\0\x15__widl_f_get_IDBIndex\x01\0\0\x01\x08IDBIndex\x01\0\0\x01\x02\x05self_\x03key\x03get\0\0\0\x19__widl_f_get_all_IDBIndex\x01\0\0\x01\x08IDBIndex\x01\0\0\x01\x01\x05self_\x06getAll\0\0\0\"__widl_f_get_all_with_key_IDBIndex\x01\0\0\x01\x08IDBIndex\x01\0\0\x01\x02\x05self_\x03key\x06getAll\0\0\0,__widl_f_get_all_with_key_and_limit_IDBIndex\x01\0\0\x01\x08IDBIndex\x01\0\0\x01\x03\x05self_\x03key\x05limit\x06getAll\0\0\0\x1E__widl_f_get_all_keys_IDBIndex\x01\0\0\x01\x08IDBIndex\x01\0\0\x01\x01\x05self_\ngetAllKeys\0\0\0'__widl_f_get_all_keys_with_key_IDBIndex\x01\0\0\x01\x08IDBIndex\x01\0\0\x01\x02\x05self_\x03key\ngetAllKeys\0\0\01__widl_f_get_all_keys_with_key_and_limit_IDBIndex\x01\0\0\x01\x08IDBIndex\x01\0\0\x01\x03\x05self_\x03key\x05limit\ngetAllKeys\0\0\0\x19__widl_f_get_key_IDBIndex\x01\0\0\x01\x08IDBIndex\x01\0\0\x01\x02\x05self_\x03key\x06getKey\0\0\0\x1D__widl_f_open_cursor_IDBIndex\x01\0\0\x01\x08IDBIndex\x01\0\0\x01\x01\x05self_\nopenCursor\0\0\0(__widl_f_open_cursor_with_range_IDBIndex\x01\0\0\x01\x08IDBIndex\x01\0\0\x01\x02\x05self_\x05range\nopenCursor\0\0\06__widl_f_open_cursor_with_range_and_direction_IDBIndex\x01\0\0\x01\x08IDBIndex\x01\0\0\x01\x03\x05self_\x05range\tdirection\nopenCursor\0\0\0!__widl_f_open_key_cursor_IDBIndex\x01\0\0\x01\x08IDBIndex\x01\0\0\x01\x01\x05self_\ropenKeyCursor\0\0\0,__widl_f_open_key_cursor_with_range_IDBIndex\x01\0\0\x01\x08IDBIndex\x01\0\0\x01\x02\x05self_\x05range\ropenKeyCursor\0\0\0:__widl_f_open_key_cursor_with_range_and_direction_IDBIndex\x01\0\0\x01\x08IDBIndex\x01\0\0\x01\x03\x05self_\x05range\tdirection\ropenKeyCursor\0\0\0\x16__widl_f_name_IDBIndex\0\0\0\x01\x08IDBIndex\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0\x1A__widl_f_set_name_IDBIndex\0\0\0\x01\x08IDBIndex\x01\0\x02\x04name\x01\x02\x05self_\x04name\x04name\0\0\0\x1E__widl_f_object_store_IDBIndex\0\0\0\x01\x08IDBIndex\x01\0\x01\x0BobjectStore\x01\x01\x05self_\x0BobjectStore\0\0\0\x1A__widl_f_key_path_IDBIndex\x01\0\0\x01\x08IDBIndex\x01\0\x01\x07keyPath\x01\x01\x05self_\x07keyPath\0\0\0\x1D__widl_f_multi_entry_IDBIndex\0\0\0\x01\x08IDBIndex\x01\0\x01\nmultiEntry\x01\x01\x05self_\nmultiEntry\0\0\0\x18__widl_f_unique_IDBIndex\0\0\0\x01\x08IDBIndex\x01\0\x01\x06unique\x01\x01\x05self_\x06unique\0\0\0\x18__widl_f_locale_IDBIndex\0\0\0\x01\x08IDBIndex\x01\0\x01\x06locale\x01\x01\x05self_\x06locale\0\0\0 __widl_f_is_auto_locale_IDBIndex\0\0\0\x01\x08IDBIndex\x01\0\x01\x0CisAutoLocale\x01\x01\x05self_\x0CisAutoLocale\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
