use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CacheStorage` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage)\n\n*This API requires the following crate features to be activated: `CacheStorage`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CacheStorage {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CacheStorage: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CacheStorage {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(67u32);
            inform(97u32);
            inform(99u32);
            inform(104u32);
            inform(101u32);
            inform(83u32);
            inform(116u32);
            inform(111u32);
            inform(114u32);
            inform(97u32);
            inform(103u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for CacheStorage {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for CacheStorage {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CacheStorage {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CacheStorage {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CacheStorage {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CacheStorage {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CacheStorage {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CacheStorage {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CacheStorage {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CacheStorage>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CacheStorage {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CacheStorage {
        #[inline]
        fn from(obj: JsValue) -> CacheStorage {
            CacheStorage { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CacheStorage {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CacheStorage> for CacheStorage {
        #[inline]
        fn as_ref(&self) -> &CacheStorage {
            self
        }
    }
    impl From<CacheStorage> for JsValue {
        #[inline]
        fn from(obj: CacheStorage) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CacheStorage {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CacheStorage(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CacheStorage(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CacheStorage(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CacheStorage { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CacheStorage) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CacheStorage> for ::js_sys::Object {
    #[inline]
    fn from(obj: CacheStorage) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CacheStorage {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CacheStorage",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_CacheStorage() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CacheStorage as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl CacheStorage {
    #[cfg(all(feature = "CacheStorage",))]
    #[allow(bad_style)]
    #[doc = "The `delete()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/delete)\n\n*This API requires the following crate features to be activated: `CacheStorage`*"]
    #[allow(clippy::all)]
    pub fn delete(&self, cache_name: &str) -> ::js_sys::Promise {
        #[cfg(all(feature = "CacheStorage",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_CacheStorage(
                self_: <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cache_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_CacheStorage(
            self_: <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cache_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(cache_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cache_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cache_name);
                __widl_f_delete_CacheStorage(self_, cache_name)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CacheStorage",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_CacheStorage() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CacheStorage as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl CacheStorage {
    #[cfg(all(feature = "CacheStorage",))]
    #[allow(bad_style)]
    #[doc = "The `has()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/has)\n\n*This API requires the following crate features to be activated: `CacheStorage`*"]
    #[allow(clippy::all)]
    pub fn has(&self, cache_name: &str) -> ::js_sys::Promise {
        #[cfg(all(feature = "CacheStorage",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_CacheStorage(
                self_: <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cache_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_CacheStorage(
            self_: <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cache_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(cache_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cache_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cache_name);
                __widl_f_has_CacheStorage(self_, cache_name)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CacheStorage",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_keys_CacheStorage() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CacheStorage as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl CacheStorage {
    #[cfg(all(feature = "CacheStorage",))]
    #[allow(bad_style)]
    #[doc = "The `keys()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/keys)\n\n*This API requires the following crate features to be activated: `CacheStorage`*"]
    #[allow(clippy::all)]
    pub fn keys(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "CacheStorage",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_keys_CacheStorage(
                self_: <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_keys_CacheStorage(
            self_: <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_keys_CacheStorage(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CacheStorage", feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_match_with_request_CacheStorage() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CacheStorage as WasmDescribe>::describe();
    <&Request as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl CacheStorage {
    #[cfg(all(feature = "CacheStorage", feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `match()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match)\n\n*This API requires the following crate features to be activated: `CacheStorage`, `Request`*"]
    #[allow(clippy::all)]
    pub fn match_with_request(&self, request: &Request) -> ::js_sys::Promise {
        #[cfg(all(feature = "CacheStorage", feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_match_with_request_CacheStorage(
                self_: <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_match_with_request_CacheStorage(
            self_: <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            request: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(request);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                __widl_f_match_with_request_CacheStorage(self_, request)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CacheStorage",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_match_with_str_CacheStorage() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CacheStorage as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl CacheStorage {
    #[cfg(all(feature = "CacheStorage",))]
    #[allow(bad_style)]
    #[doc = "The `match()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match)\n\n*This API requires the following crate features to be activated: `CacheStorage`*"]
    #[allow(clippy::all)]
    pub fn match_with_str(&self, request: &str) -> ::js_sys::Promise {
        #[cfg(all(feature = "CacheStorage",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_match_with_str_CacheStorage(
                self_: <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_match_with_str_CacheStorage(
            self_: <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            request: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(request);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                __widl_f_match_with_str_CacheStorage(self_, request)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "CacheQueryOptions",
    feature = "CacheStorage",
    feature = "Request",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_match_with_request_and_options_CacheStorage() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CacheStorage as WasmDescribe>::describe();
    <&Request as WasmDescribe>::describe();
    <&CacheQueryOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl CacheStorage {
    #[cfg(all(
        feature = "CacheQueryOptions",
        feature = "CacheStorage",
        feature = "Request",
    ))]
    #[allow(bad_style)]
    #[doc = "The `match()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match)\n\n*This API requires the following crate features to be activated: `CacheQueryOptions`, `CacheStorage`, `Request`*"]
    #[allow(clippy::all)]
    pub fn match_with_request_and_options(
        &self,
        request: &Request,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise {
        #[cfg(all(
            feature = "CacheQueryOptions",
            feature = "CacheStorage",
            feature = "Request",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_match_with_request_and_options_CacheStorage(
                self_: <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_match_with_request_and_options_CacheStorage(
            self_: <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            request: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(request);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                let options =
                    <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_match_with_request_and_options_CacheStorage(self_, request, options)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CacheQueryOptions", feature = "CacheStorage",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_match_with_str_and_options_CacheStorage() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CacheStorage as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&CacheQueryOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl CacheStorage {
    #[cfg(all(feature = "CacheQueryOptions", feature = "CacheStorage",))]
    #[allow(bad_style)]
    #[doc = "The `match()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match)\n\n*This API requires the following crate features to be activated: `CacheQueryOptions`, `CacheStorage`*"]
    #[allow(clippy::all)]
    pub fn match_with_str_and_options(
        &self,
        request: &str,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "CacheQueryOptions", feature = "CacheStorage",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_match_with_str_and_options_CacheStorage(
                self_: <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_match_with_str_and_options_CacheStorage(
            self_: <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            request: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(request);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                let options =
                    <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_match_with_str_and_options_CacheStorage(self_, request, options)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CacheStorage",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_CacheStorage() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CacheStorage as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl CacheStorage {
    #[cfg(all(feature = "CacheStorage",))]
    #[allow(bad_style)]
    #[doc = "The `open()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/open)\n\n*This API requires the following crate features to be activated: `CacheStorage`*"]
    #[allow(clippy::all)]
    pub fn open(&self, cache_name: &str) -> ::js_sys::Promise {
        #[cfg(all(feature = "CacheStorage",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_CacheStorage(
                self_: <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cache_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_CacheStorage(
            self_: <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cache_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(cache_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CacheStorage as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cache_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cache_name);
                __widl_f_open_CacheStorage(self_, cache_name)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_aa9dcd8b51d6f884: [u8; 816usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xEE\x02\0\0\0\0\t\0\0\x02\x0CCacheStorage\x1E__widl_instanceof_CacheStorage\0\0\0\0\x1C__widl_f_delete_CacheStorage\0\0\0\x01\x0CCacheStorage\x01\0\0\x01\x02\x05self_\ncache_name\x06delete\0\0\0\x19__widl_f_has_CacheStorage\0\0\0\x01\x0CCacheStorage\x01\0\0\x01\x02\x05self_\ncache_name\x03has\0\0\0\x1A__widl_f_keys_CacheStorage\0\0\0\x01\x0CCacheStorage\x01\0\0\x01\x01\x05self_\x04keys\0\0\0(__widl_f_match_with_request_CacheStorage\0\0\0\x01\x0CCacheStorage\x01\0\0\x01\x02\x05self_\x07request\x05match\0\0\0$__widl_f_match_with_str_CacheStorage\0\0\0\x01\x0CCacheStorage\x01\0\0\x01\x02\x05self_\x07request\x05match\0\0\04__widl_f_match_with_request_and_options_CacheStorage\0\0\0\x01\x0CCacheStorage\x01\0\0\x01\x03\x05self_\x07request\x07options\x05match\0\0\00__widl_f_match_with_str_and_options_CacheStorage\0\0\0\x01\x0CCacheStorage\x01\0\0\x01\x03\x05self_\x07request\x07options\x05match\0\0\0\x1A__widl_f_open_CacheStorage\0\0\0\x01\x0CCacheStorage\x01\0\0\x01\x02\x05self_\ncache_name\x04open\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
