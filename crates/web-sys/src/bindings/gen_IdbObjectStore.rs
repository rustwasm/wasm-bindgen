use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `IDBObjectStore` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct IdbObjectStore {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_IdbObjectStore: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for IdbObjectStore {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(73u32);
            inform(68u32);
            inform(66u32);
            inform(79u32);
            inform(98u32);
            inform(106u32);
            inform(101u32);
            inform(99u32);
            inform(116u32);
            inform(83u32);
            inform(116u32);
            inform(111u32);
            inform(114u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for IdbObjectStore {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for IdbObjectStore {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for IdbObjectStore {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IdbObjectStore {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for IdbObjectStore {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            IdbObjectStore {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for IdbObjectStore {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a IdbObjectStore {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for IdbObjectStore {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<IdbObjectStore>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(IdbObjectStore {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for IdbObjectStore {
        #[inline]
        fn from(obj: JsValue) -> IdbObjectStore {
            IdbObjectStore { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for IdbObjectStore {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<IdbObjectStore> for IdbObjectStore {
        #[inline]
        fn as_ref(&self) -> &IdbObjectStore {
            self
        }
    }
    impl From<IdbObjectStore> for JsValue {
        #[inline]
        fn from(obj: IdbObjectStore) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for IdbObjectStore {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_IDBObjectStore(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_IDBObjectStore(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_IDBObjectStore(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IdbObjectStore { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IdbObjectStore) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<IdbObjectStore> for ::js_sys::Object {
    #[inline]
    fn from(obj: IdbObjectStore) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for IdbObjectStore {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/add)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn add(
        &self,
        value: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        value,
                    );
                __widl_f_add_IDBObjectStore(self_, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_with_key_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/add)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn add_with_key(
        &self,
        value: &::wasm_bindgen::JsValue,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_with_key_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_with_key_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(value);
            drop(key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        value,
                    );
                let key =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_add_with_key_IDBObjectStore(self_, value, key)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `clear()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/clear)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn clear(&self) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clear_IDBObjectStore(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_count_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `count()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/count)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn count(&self) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_count_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_count_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_count_IDBObjectStore(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_count_with_key_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `count()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/count)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn count_with_key(
        &self,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_count_with_key_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_count_with_key_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_count_with_key_IDBObjectStore(self_, key)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbIndex", feature = "IdbObjectStore",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_index_with_str_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <IdbIndex as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbIndex", feature = "IdbObjectStore",))]
    #[allow(bad_style)]
    #[doc = "The `createIndex()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbObjectStore`*"]
    #[allow(clippy::all)]
    pub fn create_index_with_str(
        &self,
        name: &str,
        key_path: &str,
    ) -> Result<IdbIndex, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbIndex", feature = "IdbObjectStore",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_index_with_str_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_path: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbIndex as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_index_with_str_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_path: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbIndex as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            drop(key_path);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let key_path = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key_path);
                __widl_f_create_index_with_str_IDBObjectStore(self_, name, key_path)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbIndex as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "IdbIndex", feature = "IdbObjectStore",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_index_with_str_sequence_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbIndex as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbIndex", feature = "IdbObjectStore",))]
    #[allow(bad_style)]
    #[doc = "The `createIndex()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbObjectStore`*"]
    #[allow(clippy::all)]
    pub fn create_index_with_str_sequence(
        &self,
        name: &str,
        key_path: &::wasm_bindgen::JsValue,
    ) -> Result<IdbIndex, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbIndex", feature = "IdbObjectStore",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_index_with_str_sequence_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_path: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbIndex as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_index_with_str_sequence_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_path: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbIndex as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            drop(key_path);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let key_path =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        key_path,
                    );
                __widl_f_create_index_with_str_sequence_IDBObjectStore(self_, name, key_path)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbIndex as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "IdbIndex",
    feature = "IdbIndexParameters",
    feature = "IdbObjectStore",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_index_with_str_and_optional_parameters_IDBObjectStore(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&IdbIndexParameters as WasmDescribe>::describe();
    <IdbIndex as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(
        feature = "IdbIndex",
        feature = "IdbIndexParameters",
        feature = "IdbObjectStore",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createIndex()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbIndexParameters`, `IdbObjectStore`*"]
    #[allow(clippy::all)]
    pub fn create_index_with_str_and_optional_parameters(
        &self,
        name: &str,
        key_path: &str,
        optional_parameters: &IdbIndexParameters,
    ) -> Result<IdbIndex, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "IdbIndex",
            feature = "IdbIndexParameters",
            feature = "IdbObjectStore",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_index_with_str_and_optional_parameters_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_path: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                optional_parameters : < & IdbIndexParameters as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <IdbIndex as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_index_with_str_and_optional_parameters_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_path: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            optional_parameters: <&IdbIndexParameters as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbIndex as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            drop(key_path);
            drop(optional_parameters);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let key_path = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key_path);
                let optional_parameters =
                    <&IdbIndexParameters as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        optional_parameters,
                    );
                __widl_f_create_index_with_str_and_optional_parameters_IDBObjectStore(
                    self_,
                    name,
                    key_path,
                    optional_parameters,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbIndex as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "IdbIndex",
    feature = "IdbIndexParameters",
    feature = "IdbObjectStore",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_index_with_str_sequence_and_optional_parameters_IDBObjectStore(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&IdbIndexParameters as WasmDescribe>::describe();
    <IdbIndex as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(
        feature = "IdbIndex",
        feature = "IdbIndexParameters",
        feature = "IdbObjectStore",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createIndex()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbIndexParameters`, `IdbObjectStore`*"]
    #[allow(clippy::all)]
    pub fn create_index_with_str_sequence_and_optional_parameters(
        &self,
        name: &str,
        key_path: &::wasm_bindgen::JsValue,
        optional_parameters: &IdbIndexParameters,
    ) -> Result<IdbIndex, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "IdbIndex",
            feature = "IdbIndexParameters",
            feature = "IdbObjectStore",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_index_with_str_sequence_and_optional_parameters_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_path: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                optional_parameters : < & IdbIndexParameters as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <IdbIndex as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_index_with_str_sequence_and_optional_parameters_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_path: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            optional_parameters: <&IdbIndexParameters as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbIndex as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            drop(key_path);
            drop(optional_parameters);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let key_path =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        key_path,
                    );
                let optional_parameters =
                    <&IdbIndexParameters as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        optional_parameters,
                    );
                __widl_f_create_index_with_str_sequence_and_optional_parameters_IDBObjectStore(
                    self_,
                    name,
                    key_path,
                    optional_parameters,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbIndex as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `delete()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/delete)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn delete(
        &self,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_delete_IDBObjectStore(self_, key)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbObjectStore",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_index_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore",))]
    #[allow(bad_style)]
    #[doc = "The `deleteIndex()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/deleteIndex)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`*"]
    #[allow(clippy::all)]
    pub fn delete_index(&self, index_name: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_index_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_index_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(index_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index_name);
                __widl_f_delete_index_IDBObjectStore(self_, index_name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `get()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/get)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn get(
        &self,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_get_IDBObjectStore(self_, key)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_all_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `getAll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAll)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn get_all(&self) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_all_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_all_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_all_IDBObjectStore(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_all_with_key_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `getAll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAll)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn get_all_with_key(
        &self,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_all_with_key_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_all_with_key_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_get_all_with_key_IDBObjectStore(self_, key)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_all_with_key_and_limit_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `getAll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAll)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn get_all_with_key_and_limit(
        &self,
        key: &::wasm_bindgen::JsValue,
        limit: u32,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_all_with_key_and_limit_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                limit: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_all_with_key_and_limit_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let limit = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(limit);
                __widl_f_get_all_with_key_and_limit_IDBObjectStore(self_, key, limit)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_all_keys_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `getAllKeys()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn get_all_keys(&self) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_all_keys_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_all_keys_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_all_keys_IDBObjectStore(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_all_keys_with_key_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `getAllKeys()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn get_all_keys_with_key(
        &self,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_all_keys_with_key_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_all_keys_with_key_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_get_all_keys_with_key_IDBObjectStore(self_, key)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_all_keys_with_key_and_limit_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `getAllKeys()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn get_all_keys_with_key_and_limit(
        &self,
        key: &::wasm_bindgen::JsValue,
        limit: u32,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_all_keys_with_key_and_limit_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                limit: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_all_keys_with_key_and_limit_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let limit = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(limit);
                __widl_f_get_all_keys_with_key_and_limit_IDBObjectStore(self_, key, limit)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_key_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `getKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getKey)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn get_key(
        &self,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_key_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_key_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_get_key_IDBObjectStore(self_, key)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbIndex", feature = "IdbObjectStore",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_index_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <IdbIndex as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbIndex", feature = "IdbObjectStore",))]
    #[allow(bad_style)]
    #[doc = "The `index()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/index)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbObjectStore`*"]
    #[allow(clippy::all)]
    pub fn index(&self, name: &str) -> Result<IdbIndex, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbIndex", feature = "IdbObjectStore",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_index_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbIndex as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_index_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbIndex as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_index_IDBObjectStore(self_, name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbIndex as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_cursor_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `openCursor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openCursor)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn open_cursor(&self) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_cursor_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_cursor_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_open_cursor_IDBObjectStore(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_cursor_with_range_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `openCursor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openCursor)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn open_cursor_with_range(
        &self,
        range: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_cursor_with_range_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                range: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_cursor_with_range_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let range =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        range,
                    );
                __widl_f_open_cursor_with_range_IDBObjectStore(self_, range)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "IdbCursorDirection",
    feature = "IdbObjectStore",
    feature = "IdbRequest",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_cursor_with_range_and_direction_IDBObjectStore()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbCursorDirection as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(
        feature = "IdbCursorDirection",
        feature = "IdbObjectStore",
        feature = "IdbRequest",
    ))]
    #[allow(bad_style)]
    #[doc = "The `openCursor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openCursor)\n\n*This API requires the following crate features to be activated: `IdbCursorDirection`, `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn open_cursor_with_range_and_direction(
        &self,
        range: &::wasm_bindgen::JsValue,
        direction: IdbCursorDirection,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "IdbCursorDirection",
            feature = "IdbObjectStore",
            feature = "IdbRequest",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_cursor_with_range_and_direction_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                range: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                direction: <IdbCursorDirection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_cursor_with_range_and_direction_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let range =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        range,
                    );
                let direction =
                    <IdbCursorDirection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(direction);
                __widl_f_open_cursor_with_range_and_direction_IDBObjectStore(
                    self_, range, direction,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_key_cursor_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `openKeyCursor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openKeyCursor)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn open_key_cursor(&self) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_key_cursor_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_key_cursor_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_open_key_cursor_IDBObjectStore(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_key_cursor_with_range_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `openKeyCursor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openKeyCursor)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn open_key_cursor_with_range(
        &self,
        range: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_key_cursor_with_range_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                range: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_key_cursor_with_range_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let range =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        range,
                    );
                __widl_f_open_key_cursor_with_range_IDBObjectStore(self_, range)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "IdbCursorDirection",
    feature = "IdbObjectStore",
    feature = "IdbRequest",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_key_cursor_with_range_and_direction_IDBObjectStore(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbCursorDirection as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(
        feature = "IdbCursorDirection",
        feature = "IdbObjectStore",
        feature = "IdbRequest",
    ))]
    #[allow(bad_style)]
    #[doc = "The `openKeyCursor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openKeyCursor)\n\n*This API requires the following crate features to be activated: `IdbCursorDirection`, `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn open_key_cursor_with_range_and_direction(
        &self,
        range: &::wasm_bindgen::JsValue,
        direction: IdbCursorDirection,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "IdbCursorDirection",
            feature = "IdbObjectStore",
            feature = "IdbRequest",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_key_cursor_with_range_and_direction_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                range: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                direction: <IdbCursorDirection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_key_cursor_with_range_and_direction_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let range =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        range,
                    );
                let direction =
                    <IdbCursorDirection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(direction);
                __widl_f_open_key_cursor_with_range_and_direction_IDBObjectStore(
                    self_, range, direction,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_put_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `put()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/put)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn put(
        &self,
        value: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_put_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_put_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        value,
                    );
                __widl_f_put_IDBObjectStore(self_, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_put_with_key_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `put()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/put)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn put_with_key(
        &self,
        value: &::wasm_bindgen::JsValue,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_put_with_key_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_put_with_key_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(value);
            drop(key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        value,
                    );
                let key =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_put_with_key_IDBObjectStore(self_, value, key)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbObjectStore",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/name)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "IdbObjectStore",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_IDBObjectStore(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbObjectStore",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_name_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore",))]
    #[allow(bad_style)]
    #[doc = "The `name` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/name)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`*"]
    #[allow(clippy::all)]
    pub fn set_name(&self, name: &str) {
        #[cfg(all(feature = "IdbObjectStore",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_name_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_name_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_set_name_IDBObjectStore(self_, name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "IdbObjectStore",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_key_path_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore",))]
    #[allow(bad_style)]
    #[doc = "The `keyPath` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/keyPath)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`*"]
    #[allow(clippy::all)]
    pub fn key_path(&self) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_key_path_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_key_path_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_key_path_IDBObjectStore(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomStringList", feature = "IdbObjectStore",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_index_names_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <DomStringList as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "DomStringList", feature = "IdbObjectStore",))]
    #[allow(bad_style)]
    #[doc = "The `indexNames` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/indexNames)\n\n*This API requires the following crate features to be activated: `DomStringList`, `IdbObjectStore`*"]
    #[allow(clippy::all)]
    pub fn index_names(&self) -> DomStringList {
        #[cfg(all(feature = "DomStringList", feature = "IdbObjectStore",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_index_names_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomStringList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_index_names_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomStringList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_index_names_IDBObjectStore(self_)
            };
            <DomStringList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbTransaction",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transaction_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <IdbTransaction as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbTransaction",))]
    #[allow(bad_style)]
    #[doc = "The `transaction` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/transaction)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbTransaction`*"]
    #[allow(clippy::all)]
    pub fn transaction(&self) -> IdbTransaction {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbTransaction",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transaction_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbTransaction as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transaction_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbTransaction as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_transaction_IDBObjectStore(self_)
            };
            <IdbTransaction as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbObjectStore",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_auto_increment_IDBObjectStore() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbObjectStore as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl IdbObjectStore {
    #[cfg(all(feature = "IdbObjectStore",))]
    #[allow(bad_style)]
    #[doc = "The `autoIncrement` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/autoIncrement)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`*"]
    #[allow(clippy::all)]
    pub fn auto_increment(&self) -> bool {
        #[cfg(all(feature = "IdbObjectStore",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_auto_increment_IDBObjectStore(
                self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_auto_increment_IDBObjectStore(
            self_: <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbObjectStore as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_auto_increment_IDBObjectStore(self_)
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
pub static __WASM_BINDGEN_GENERATED_627897be6bd57b66: [u8; 3279usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x8D\x0C\0\0\0\0#\0\0\x02\x0EIDBObjectStore __widl_instanceof_IDBObjectStore\0\0\0\0\x1B__widl_f_add_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x02\x05self_\x05value\x03add\0\0\0$__widl_f_add_with_key_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x03\x05self_\x05value\x03key\x03add\0\0\0\x1D__widl_f_clear_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x01\x05self_\x05clear\0\0\0\x1D__widl_f_count_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x01\x05self_\x05count\0\0\0&__widl_f_count_with_key_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x02\x05self_\x03key\x05count\0\0\0-__widl_f_create_index_with_str_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x03\x05self_\x04name\x08key_path\x0BcreateIndex\0\0\06__widl_f_create_index_with_str_sequence_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x03\x05self_\x04name\x08key_path\x0BcreateIndex\0\0\0E__widl_f_create_index_with_str_and_optional_parameters_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x04\x05self_\x04name\x08key_path\x13optional_parameters\x0BcreateIndex\0\0\0N__widl_f_create_index_with_str_sequence_and_optional_parameters_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x04\x05self_\x04name\x08key_path\x13optional_parameters\x0BcreateIndex\0\0\0\x1E__widl_f_delete_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x02\x05self_\x03key\x06delete\0\0\0$__widl_f_delete_index_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x02\x05self_\nindex_name\x0BdeleteIndex\0\0\0\x1B__widl_f_get_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x02\x05self_\x03key\x03get\0\0\0\x1F__widl_f_get_all_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x01\x05self_\x06getAll\0\0\0(__widl_f_get_all_with_key_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x02\x05self_\x03key\x06getAll\0\0\02__widl_f_get_all_with_key_and_limit_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x03\x05self_\x03key\x05limit\x06getAll\0\0\0$__widl_f_get_all_keys_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x01\x05self_\ngetAllKeys\0\0\0-__widl_f_get_all_keys_with_key_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x02\x05self_\x03key\ngetAllKeys\0\0\07__widl_f_get_all_keys_with_key_and_limit_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x03\x05self_\x03key\x05limit\ngetAllKeys\0\0\0\x1F__widl_f_get_key_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x02\x05self_\x03key\x06getKey\0\0\0\x1D__widl_f_index_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x02\x05self_\x04name\x05index\0\0\0#__widl_f_open_cursor_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x01\x05self_\nopenCursor\0\0\0.__widl_f_open_cursor_with_range_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x02\x05self_\x05range\nopenCursor\0\0\0<__widl_f_open_cursor_with_range_and_direction_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x03\x05self_\x05range\tdirection\nopenCursor\0\0\0'__widl_f_open_key_cursor_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x01\x05self_\ropenKeyCursor\0\0\02__widl_f_open_key_cursor_with_range_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x02\x05self_\x05range\ropenKeyCursor\0\0\0@__widl_f_open_key_cursor_with_range_and_direction_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x03\x05self_\x05range\tdirection\ropenKeyCursor\0\0\0\x1B__widl_f_put_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x02\x05self_\x05value\x03put\0\0\0$__widl_f_put_with_key_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\0\x01\x03\x05self_\x05value\x03key\x03put\0\0\0\x1C__widl_f_name_IDBObjectStore\0\0\0\x01\x0EIDBObjectStore\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0 __widl_f_set_name_IDBObjectStore\0\0\0\x01\x0EIDBObjectStore\x01\0\x02\x04name\x01\x02\x05self_\x04name\x04name\0\0\0 __widl_f_key_path_IDBObjectStore\x01\0\0\x01\x0EIDBObjectStore\x01\0\x01\x07keyPath\x01\x01\x05self_\x07keyPath\0\0\0#__widl_f_index_names_IDBObjectStore\0\0\0\x01\x0EIDBObjectStore\x01\0\x01\nindexNames\x01\x01\x05self_\nindexNames\0\0\0#__widl_f_transaction_IDBObjectStore\0\0\0\x01\x0EIDBObjectStore\x01\0\x01\x0Btransaction\x01\x01\x05self_\x0Btransaction\0\0\0&__widl_f_auto_increment_IDBObjectStore\0\0\0\x01\x0EIDBObjectStore\x01\0\x01\rautoIncrement\x01\x01\x05self_\rautoIncrement\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
