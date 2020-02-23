use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Storage` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage)\n\n*This API requires the following crate features to be activated: `Storage`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Storage {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Storage: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Storage {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(7u32);
            inform(83u32);
            inform(116u32);
            inform(111u32);
            inform(114u32);
            inform(97u32);
            inform(103u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for Storage {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Storage {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Storage {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Storage {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Storage {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Storage {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Storage {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Storage {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Storage {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Storage>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Storage {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Storage {
        #[inline]
        fn from(obj: JsValue) -> Storage {
            Storage { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Storage {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Storage> for Storage {
        #[inline]
        fn as_ref(&self) -> &Storage {
            self
        }
    }
    impl From<Storage> for JsValue {
        #[inline]
        fn from(obj: Storage) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Storage {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Storage(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Storage(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Storage(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Storage { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Storage) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Storage> for ::js_sys::Object {
    #[inline]
    fn from(obj: Storage) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Storage {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Storage",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_Storage() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Storage as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Storage {
    #[cfg(all(feature = "Storage",))]
    #[allow(bad_style)]
    #[doc = "The `clear()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/clear)\n\n*This API requires the following crate features to be activated: `Storage`*"]
    #[allow(clippy::all)]
    pub fn clear(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Storage",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_Storage(
                self_: <&Storage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_Storage(
            self_: <&Storage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Storage as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clear_Storage(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Storage",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_item_Storage() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Storage as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl Storage {
    #[cfg(all(feature = "Storage",))]
    #[allow(bad_style)]
    #[doc = "The `getItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/getItem)\n\n*This API requires the following crate features to be activated: `Storage`*"]
    #[allow(clippy::all)]
    pub fn get_item(&self, key: &str) -> Result<Option<String>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Storage",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_item_Storage(
                self_: <&Storage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_item_Storage(
            self_: <&Storage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Storage as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_get_item_Storage(self_, key)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Storage",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_key_Storage() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Storage as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl Storage {
    #[cfg(all(feature = "Storage",))]
    #[allow(bad_style)]
    #[doc = "The `key()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/key)\n\n*This API requires the following crate features to be activated: `Storage`*"]
    #[allow(clippy::all)]
    pub fn key(&self, index: u32) -> Result<Option<String>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Storage",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_key_Storage(
                self_: <&Storage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_key_Storage(
            self_: <&Storage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Storage as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_key_Storage(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Storage",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_item_Storage() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Storage as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Storage {
    #[cfg(all(feature = "Storage",))]
    #[allow(bad_style)]
    #[doc = "The `removeItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/removeItem)\n\n*This API requires the following crate features to be activated: `Storage`*"]
    #[allow(clippy::all)]
    pub fn remove_item(&self, key: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Storage",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_item_Storage(
                self_: <&Storage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_item_Storage(
            self_: <&Storage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Storage as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_remove_item_Storage(self_, key)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Storage",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_item_Storage() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Storage as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Storage {
    #[cfg(all(feature = "Storage",))]
    #[allow(bad_style)]
    #[doc = "The `setItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/setItem)\n\n*This API requires the following crate features to be activated: `Storage`*"]
    #[allow(clippy::all)]
    pub fn set_item(&self, key: &str, value: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Storage",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_item_Storage(
                self_: <&Storage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_item_Storage(
            self_: <&Storage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(key);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Storage as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_set_item_Storage(self_, key, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Storage",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_Storage() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Storage as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl Storage {
    #[cfg(all(feature = "Storage",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `Storage`*"]
    #[allow(clippy::all)]
    pub fn get(&self, key: &str) -> Result<Option<String>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Storage",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_Storage(
                self_: <&Storage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_Storage(
            self_: <&Storage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Storage as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_get_Storage(self_, key)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Storage",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_Storage() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Storage as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Storage {
    #[cfg(all(feature = "Storage",))]
    #[allow(bad_style)]
    #[doc = "The indexing setter\n\n\n\n*This API requires the following crate features to be activated: `Storage`*"]
    #[allow(clippy::all)]
    pub fn set(&self, key: &str, value: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Storage",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_Storage(
                self_: <&Storage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_Storage(
            self_: <&Storage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(key);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Storage as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_set_Storage(self_, key, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Storage",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_Storage() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Storage as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Storage {
    #[cfg(all(feature = "Storage",))]
    #[allow(bad_style)]
    #[doc = "The indexing deleter\n\n\n\n*This API requires the following crate features to be activated: `Storage`*"]
    #[allow(clippy::all)]
    pub fn delete(&self, key: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Storage",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_Storage(
                self_: <&Storage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_Storage(
            self_: <&Storage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Storage as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_delete_Storage(self_, key)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Storage",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_Storage() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Storage as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl Storage {
    #[cfg(all(feature = "Storage",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/length)\n\n*This API requires the following crate features to be activated: `Storage`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> Result<u32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Storage",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_Storage(
                self_: <&Storage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_Storage(
            self_: <&Storage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Storage as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_Storage(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_f75481b9497fc9a9: [u8; 700usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}z\x02\0\0\0\0\n\0\0\x02\x07Storage\x19__widl_instanceof_Storage\0\0\0\0\x16__widl_f_clear_Storage\x01\0\0\x01\x07Storage\x01\0\0\x01\x01\x05self_\x05clear\0\0\0\x19__widl_f_get_item_Storage\x01\0\0\x01\x07Storage\x01\0\0\x01\x02\x05self_\x03key\x07getItem\0\0\0\x14__widl_f_key_Storage\x01\0\0\x01\x07Storage\x01\0\0\x01\x02\x05self_\x05index\x03key\0\0\0\x1C__widl_f_remove_item_Storage\x01\0\0\x01\x07Storage\x01\0\0\x01\x02\x05self_\x03key\nremoveItem\0\0\0\x19__widl_f_set_item_Storage\x01\0\0\x01\x07Storage\x01\0\0\x01\x03\x05self_\x03key\x05value\x07setItem\0\0\0\x14__widl_f_get_Storage\x01\0\0\x01\x07Storage\x01\0\x03\x01\x02\x05self_\x03key\x03get\0\0\0\x14__widl_f_set_Storage\x01\0\0\x01\x07Storage\x01\0\x04\x01\x03\x05self_\x03key\x05value\x03set\0\0\0\x17__widl_f_delete_Storage\x01\0\0\x01\x07Storage\x01\0\x05\x01\x02\x05self_\x03key\x06delete\0\0\0\x17__widl_f_length_Storage\x01\0\0\x01\x07Storage\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
