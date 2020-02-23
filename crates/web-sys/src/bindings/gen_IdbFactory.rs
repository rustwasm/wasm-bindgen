use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `IDBFactory` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory)\n\n*This API requires the following crate features to be activated: `IdbFactory`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct IdbFactory {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_IdbFactory: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for IdbFactory {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(73u32);
            inform(68u32);
            inform(66u32);
            inform(70u32);
            inform(97u32);
            inform(99u32);
            inform(116u32);
            inform(111u32);
            inform(114u32);
            inform(121u32);
        }
    }
    impl core::ops::Deref for IdbFactory {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for IdbFactory {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for IdbFactory {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IdbFactory {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for IdbFactory {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            IdbFactory {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for IdbFactory {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a IdbFactory {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for IdbFactory {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<IdbFactory>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(IdbFactory {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for IdbFactory {
        #[inline]
        fn from(obj: JsValue) -> IdbFactory {
            IdbFactory { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for IdbFactory {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<IdbFactory> for IdbFactory {
        #[inline]
        fn as_ref(&self) -> &IdbFactory {
            self
        }
    }
    impl From<IdbFactory> for JsValue {
        #[inline]
        fn from(obj: IdbFactory) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for IdbFactory {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_IDBFactory(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_IDBFactory(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_IDBFactory(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IdbFactory { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IdbFactory) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<IdbFactory> for ::js_sys::Object {
    #[inline]
    fn from(obj: IdbFactory) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for IdbFactory {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "IdbFactory",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cmp_IDBFactory() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbFactory as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i16 as WasmDescribe>::describe();
}
impl IdbFactory {
    #[cfg(all(feature = "IdbFactory",))]
    #[allow(bad_style)]
    #[doc = "The `cmp()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/cmp)\n\n*This API requires the following crate features to be activated: `IdbFactory`*"]
    #[allow(clippy::all)]
    pub fn cmp(
        &self,
        first: &::wasm_bindgen::JsValue,
        second: &::wasm_bindgen::JsValue,
    ) -> Result<i16, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFactory",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cmp_IDBFactory(
                self_: <&IdbFactory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                first: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                second: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cmp_IDBFactory(
            self_: <&IdbFactory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            first: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            second: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(first);
            drop(second);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFactory as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let first =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        first,
                    );
                let second =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        second,
                    );
                __widl_f_cmp_IDBFactory(self_, first, second)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFactory", feature = "IdbOpenDbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_database_IDBFactory() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFactory as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <IdbOpenDbRequest as WasmDescribe>::describe();
}
impl IdbFactory {
    #[cfg(all(feature = "IdbFactory", feature = "IdbOpenDbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `deleteDatabase()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/deleteDatabase)\n\n*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbRequest`*"]
    #[allow(clippy::all)]
    pub fn delete_database(&self, name: &str) -> Result<IdbOpenDbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFactory", feature = "IdbOpenDbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_database_IDBFactory(
                self_: <&IdbFactory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbOpenDbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_database_IDBFactory(
            self_: <&IdbFactory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbOpenDbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFactory as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_delete_database_IDBFactory(self_, name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbOpenDbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "IdbFactory",
    feature = "IdbOpenDbOptions",
    feature = "IdbOpenDbRequest",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_database_with_options_IDBFactory() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbFactory as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&IdbOpenDbOptions as WasmDescribe>::describe();
    <IdbOpenDbRequest as WasmDescribe>::describe();
}
impl IdbFactory {
    #[cfg(all(
        feature = "IdbFactory",
        feature = "IdbOpenDbOptions",
        feature = "IdbOpenDbRequest",
    ))]
    #[allow(bad_style)]
    #[doc = "The `deleteDatabase()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/deleteDatabase)\n\n*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbOptions`, `IdbOpenDbRequest`*"]
    #[allow(clippy::all)]
    pub fn delete_database_with_options(
        &self,
        name: &str,
        options: &IdbOpenDbOptions,
    ) -> Result<IdbOpenDbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "IdbFactory",
            feature = "IdbOpenDbOptions",
            feature = "IdbOpenDbRequest",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_database_with_options_IDBFactory(
                self_: <&IdbFactory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&IdbOpenDbOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbOpenDbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_database_with_options_IDBFactory(
            self_: <&IdbFactory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&IdbOpenDbOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbOpenDbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFactory as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let options =
                    <&IdbOpenDbOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_delete_database_with_options_IDBFactory(self_, name, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbOpenDbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFactory", feature = "IdbOpenDbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_with_u32_IDBFactory() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbFactory as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <IdbOpenDbRequest as WasmDescribe>::describe();
}
impl IdbFactory {
    #[cfg(all(feature = "IdbFactory", feature = "IdbOpenDbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `open()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open)\n\n*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbRequest`*"]
    #[allow(clippy::all)]
    pub fn open_with_u32(
        &self,
        name: &str,
        version: u32,
    ) -> Result<IdbOpenDbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFactory", feature = "IdbOpenDbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_with_u32_IDBFactory(
                self_: <&IdbFactory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                version: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbOpenDbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_with_u32_IDBFactory(
            self_: <&IdbFactory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            version: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbOpenDbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            drop(version);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFactory as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let version = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(version);
                __widl_f_open_with_u32_IDBFactory(self_, name, version)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbOpenDbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFactory", feature = "IdbOpenDbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_with_f64_IDBFactory() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbFactory as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <IdbOpenDbRequest as WasmDescribe>::describe();
}
impl IdbFactory {
    #[cfg(all(feature = "IdbFactory", feature = "IdbOpenDbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `open()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open)\n\n*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbRequest`*"]
    #[allow(clippy::all)]
    pub fn open_with_f64(
        &self,
        name: &str,
        version: f64,
    ) -> Result<IdbOpenDbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFactory", feature = "IdbOpenDbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_with_f64_IDBFactory(
                self_: <&IdbFactory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                version: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbOpenDbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_with_f64_IDBFactory(
            self_: <&IdbFactory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            version: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbOpenDbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            drop(version);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFactory as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let version = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(version);
                __widl_f_open_with_f64_IDBFactory(self_, name, version)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbOpenDbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFactory", feature = "IdbOpenDbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_IDBFactory() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFactory as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <IdbOpenDbRequest as WasmDescribe>::describe();
}
impl IdbFactory {
    #[cfg(all(feature = "IdbFactory", feature = "IdbOpenDbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `open()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open)\n\n*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbRequest`*"]
    #[allow(clippy::all)]
    pub fn open(&self, name: &str) -> Result<IdbOpenDbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFactory", feature = "IdbOpenDbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_IDBFactory(
                self_: <&IdbFactory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbOpenDbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_IDBFactory(
            self_: <&IdbFactory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbOpenDbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFactory as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_open_IDBFactory(self_, name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbOpenDbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "IdbFactory",
    feature = "IdbOpenDbOptions",
    feature = "IdbOpenDbRequest",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_with_idb_open_db_options_IDBFactory() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbFactory as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&IdbOpenDbOptions as WasmDescribe>::describe();
    <IdbOpenDbRequest as WasmDescribe>::describe();
}
impl IdbFactory {
    #[cfg(all(
        feature = "IdbFactory",
        feature = "IdbOpenDbOptions",
        feature = "IdbOpenDbRequest",
    ))]
    #[allow(bad_style)]
    #[doc = "The `open()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open)\n\n*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbOptions`, `IdbOpenDbRequest`*"]
    #[allow(clippy::all)]
    pub fn open_with_idb_open_db_options(
        &self,
        name: &str,
        options: &IdbOpenDbOptions,
    ) -> Result<IdbOpenDbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "IdbFactory",
            feature = "IdbOpenDbOptions",
            feature = "IdbOpenDbRequest",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_with_idb_open_db_options_IDBFactory(
                self_: <&IdbFactory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&IdbOpenDbOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbOpenDbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_with_idb_open_db_options_IDBFactory(
            self_: <&IdbFactory as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&IdbOpenDbOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbOpenDbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFactory as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let options =
                    <&IdbOpenDbOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_open_with_idb_open_db_options_IDBFactory(self_, name, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbOpenDbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_59787ec52f075819: [u8; 733usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x9B\x02\0\0\0\0\x08\0\0\x02\nIDBFactory\x1C__widl_instanceof_IDBFactory\0\0\0\0\x17__widl_f_cmp_IDBFactory\x01\0\0\x01\nIDBFactory\x01\0\0\x01\x03\x05self_\x05first\x06second\x03cmp\0\0\0#__widl_f_delete_database_IDBFactory\x01\0\0\x01\nIDBFactory\x01\0\0\x01\x02\x05self_\x04name\x0EdeleteDatabase\0\0\00__widl_f_delete_database_with_options_IDBFactory\x01\0\0\x01\nIDBFactory\x01\0\0\x01\x03\x05self_\x04name\x07options\x0EdeleteDatabase\0\0\0!__widl_f_open_with_u32_IDBFactory\x01\0\0\x01\nIDBFactory\x01\0\0\x01\x03\x05self_\x04name\x07version\x04open\0\0\0!__widl_f_open_with_f64_IDBFactory\x01\0\0\x01\nIDBFactory\x01\0\0\x01\x03\x05self_\x04name\x07version\x04open\0\0\0\x18__widl_f_open_IDBFactory\x01\0\0\x01\nIDBFactory\x01\0\0\x01\x02\x05self_\x04name\x04open\0\0\01__widl_f_open_with_idb_open_db_options_IDBFactory\x01\0\0\x01\nIDBFactory\x01\0\0\x01\x03\x05self_\x04name\x07options\x04open\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
