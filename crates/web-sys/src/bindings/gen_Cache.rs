use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Cache` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache)\n\n*This API requires the following crate features to be activated: `Cache`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Cache {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Cache: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Cache {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(5u32);
            inform(67u32);
            inform(97u32);
            inform(99u32);
            inform(104u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for Cache {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Cache {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Cache {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Cache {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Cache {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Cache {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Cache {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Cache {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Cache {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Cache>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Cache {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Cache {
        #[inline]
        fn from(obj: JsValue) -> Cache {
            Cache { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Cache {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Cache> for Cache {
        #[inline]
        fn as_ref(&self) -> &Cache {
            self
        }
    }
    impl From<Cache> for JsValue {
        #[inline]
        fn from(obj: Cache) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Cache {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Cache(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Cache(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Cache(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Cache { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Cache) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Cache> for ::js_sys::Object {
    #[inline]
    fn from(obj: Cache) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Cache {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Cache", feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_with_request_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Cache as WasmDescribe>::describe();
    <&Request as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache", feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/add)\n\n*This API requires the following crate features to be activated: `Cache`, `Request`*"]
    #[allow(clippy::all)]
    pub fn add_with_request(&self, request: &Request) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache", feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_with_request_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_with_request_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                __widl_f_add_with_request_Cache(self_, request)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_with_str_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Cache as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/add)\n\n*This API requires the following crate features to be activated: `Cache`*"]
    #[allow(clippy::all)]
    pub fn add_with_str(&self, request: &str) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_with_str_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_with_str_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                __widl_f_add_with_str_Cache(self_, request)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_all_with_request_sequence_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Cache as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache",))]
    #[allow(bad_style)]
    #[doc = "The `addAll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/addAll)\n\n*This API requires the following crate features to be activated: `Cache`*"]
    #[allow(clippy::all)]
    pub fn add_all_with_request_sequence(
        &self,
        requests: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_all_with_request_sequence_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                requests: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_all_with_request_sequence_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            requests: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(requests);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let requests =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        requests,
                    );
                __widl_f_add_all_with_request_sequence_Cache(self_, requests)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_all_with_str_sequence_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Cache as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache",))]
    #[allow(bad_style)]
    #[doc = "The `addAll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/addAll)\n\n*This API requires the following crate features to be activated: `Cache`*"]
    #[allow(clippy::all)]
    pub fn add_all_with_str_sequence(
        &self,
        requests: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_all_with_str_sequence_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                requests: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_all_with_str_sequence_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            requests: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(requests);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let requests =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        requests,
                    );
                __widl_f_add_all_with_str_sequence_Cache(self_, requests)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache", feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_with_request_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Cache as WasmDescribe>::describe();
    <&Request as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache", feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `delete()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/delete)\n\n*This API requires the following crate features to be activated: `Cache`, `Request`*"]
    #[allow(clippy::all)]
    pub fn delete_with_request(&self, request: &Request) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache", feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_with_request_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_with_request_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                __widl_f_delete_with_request_Cache(self_, request)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_with_str_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Cache as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache",))]
    #[allow(bad_style)]
    #[doc = "The `delete()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/delete)\n\n*This API requires the following crate features to be activated: `Cache`*"]
    #[allow(clippy::all)]
    pub fn delete_with_str(&self, request: &str) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_with_str_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_with_str_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                __widl_f_delete_with_str_Cache(self_, request)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache", feature = "CacheQueryOptions", feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_with_request_and_options_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Cache as WasmDescribe>::describe();
    <&Request as WasmDescribe>::describe();
    <&CacheQueryOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache", feature = "CacheQueryOptions", feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `delete()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/delete)\n\n*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`, `Request`*"]
    #[allow(clippy::all)]
    pub fn delete_with_request_and_options(
        &self,
        request: &Request,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache", feature = "CacheQueryOptions", feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_with_request_and_options_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_with_request_and_options_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                let options =
                    <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_delete_with_request_and_options_Cache(self_, request, options)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache", feature = "CacheQueryOptions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_with_str_and_options_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Cache as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&CacheQueryOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache", feature = "CacheQueryOptions",))]
    #[allow(bad_style)]
    #[doc = "The `delete()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/delete)\n\n*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`*"]
    #[allow(clippy::all)]
    pub fn delete_with_str_and_options(
        &self,
        request: &str,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache", feature = "CacheQueryOptions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_with_str_and_options_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_with_str_and_options_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                let options =
                    <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_delete_with_str_and_options_Cache(self_, request, options)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_keys_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Cache as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache",))]
    #[allow(bad_style)]
    #[doc = "The `keys()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)\n\n*This API requires the following crate features to be activated: `Cache`*"]
    #[allow(clippy::all)]
    pub fn keys(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_keys_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_keys_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_keys_Cache(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache", feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_keys_with_request_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Cache as WasmDescribe>::describe();
    <&Request as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache", feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `keys()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)\n\n*This API requires the following crate features to be activated: `Cache`, `Request`*"]
    #[allow(clippy::all)]
    pub fn keys_with_request(&self, request: &Request) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache", feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_keys_with_request_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_keys_with_request_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                __widl_f_keys_with_request_Cache(self_, request)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_keys_with_str_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Cache as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache",))]
    #[allow(bad_style)]
    #[doc = "The `keys()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)\n\n*This API requires the following crate features to be activated: `Cache`*"]
    #[allow(clippy::all)]
    pub fn keys_with_str(&self, request: &str) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_keys_with_str_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_keys_with_str_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                __widl_f_keys_with_str_Cache(self_, request)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache", feature = "CacheQueryOptions", feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_keys_with_request_and_options_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Cache as WasmDescribe>::describe();
    <&Request as WasmDescribe>::describe();
    <&CacheQueryOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache", feature = "CacheQueryOptions", feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `keys()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)\n\n*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`, `Request`*"]
    #[allow(clippy::all)]
    pub fn keys_with_request_and_options(
        &self,
        request: &Request,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache", feature = "CacheQueryOptions", feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_keys_with_request_and_options_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_keys_with_request_and_options_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                let options =
                    <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_keys_with_request_and_options_Cache(self_, request, options)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache", feature = "CacheQueryOptions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_keys_with_str_and_options_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Cache as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&CacheQueryOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache", feature = "CacheQueryOptions",))]
    #[allow(bad_style)]
    #[doc = "The `keys()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)\n\n*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`*"]
    #[allow(clippy::all)]
    pub fn keys_with_str_and_options(
        &self,
        request: &str,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache", feature = "CacheQueryOptions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_keys_with_str_and_options_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_keys_with_str_and_options_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                let options =
                    <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_keys_with_str_and_options_Cache(self_, request, options)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache", feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_match_with_request_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Cache as WasmDescribe>::describe();
    <&Request as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache", feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `match()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/match)\n\n*This API requires the following crate features to be activated: `Cache`, `Request`*"]
    #[allow(clippy::all)]
    pub fn match_with_request(&self, request: &Request) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache", feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_match_with_request_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_match_with_request_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                __widl_f_match_with_request_Cache(self_, request)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_match_with_str_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Cache as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache",))]
    #[allow(bad_style)]
    #[doc = "The `match()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/match)\n\n*This API requires the following crate features to be activated: `Cache`*"]
    #[allow(clippy::all)]
    pub fn match_with_str(&self, request: &str) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_match_with_str_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_match_with_str_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                __widl_f_match_with_str_Cache(self_, request)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache", feature = "CacheQueryOptions", feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_match_with_request_and_options_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Cache as WasmDescribe>::describe();
    <&Request as WasmDescribe>::describe();
    <&CacheQueryOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache", feature = "CacheQueryOptions", feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `match()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/match)\n\n*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`, `Request`*"]
    #[allow(clippy::all)]
    pub fn match_with_request_and_options(
        &self,
        request: &Request,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache", feature = "CacheQueryOptions", feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_match_with_request_and_options_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_match_with_request_and_options_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                let options =
                    <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_match_with_request_and_options_Cache(self_, request, options)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache", feature = "CacheQueryOptions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_match_with_str_and_options_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Cache as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&CacheQueryOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache", feature = "CacheQueryOptions",))]
    #[allow(bad_style)]
    #[doc = "The `match()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/match)\n\n*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`*"]
    #[allow(clippy::all)]
    pub fn match_with_str_and_options(
        &self,
        request: &str,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache", feature = "CacheQueryOptions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_match_with_str_and_options_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_match_with_str_and_options_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                let options =
                    <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_match_with_str_and_options_Cache(self_, request, options)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_match_all_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Cache as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache",))]
    #[allow(bad_style)]
    #[doc = "The `matchAll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)\n\n*This API requires the following crate features to be activated: `Cache`*"]
    #[allow(clippy::all)]
    pub fn match_all(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_match_all_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_match_all_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_match_all_Cache(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache", feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_match_all_with_request_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Cache as WasmDescribe>::describe();
    <&Request as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache", feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `matchAll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)\n\n*This API requires the following crate features to be activated: `Cache`, `Request`*"]
    #[allow(clippy::all)]
    pub fn match_all_with_request(&self, request: &Request) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache", feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_match_all_with_request_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_match_all_with_request_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                __widl_f_match_all_with_request_Cache(self_, request)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_match_all_with_str_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Cache as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache",))]
    #[allow(bad_style)]
    #[doc = "The `matchAll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)\n\n*This API requires the following crate features to be activated: `Cache`*"]
    #[allow(clippy::all)]
    pub fn match_all_with_str(&self, request: &str) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_match_all_with_str_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_match_all_with_str_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                __widl_f_match_all_with_str_Cache(self_, request)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache", feature = "CacheQueryOptions", feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_match_all_with_request_and_options_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Cache as WasmDescribe>::describe();
    <&Request as WasmDescribe>::describe();
    <&CacheQueryOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache", feature = "CacheQueryOptions", feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `matchAll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)\n\n*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`, `Request`*"]
    #[allow(clippy::all)]
    pub fn match_all_with_request_and_options(
        &self,
        request: &Request,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache", feature = "CacheQueryOptions", feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_match_all_with_request_and_options_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_match_all_with_request_and_options_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                let options =
                    <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_match_all_with_request_and_options_Cache(self_, request, options)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache", feature = "CacheQueryOptions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_match_all_with_str_and_options_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Cache as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&CacheQueryOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache", feature = "CacheQueryOptions",))]
    #[allow(bad_style)]
    #[doc = "The `matchAll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)\n\n*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`*"]
    #[allow(clippy::all)]
    pub fn match_all_with_str_and_options(
        &self,
        request: &str,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache", feature = "CacheQueryOptions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_match_all_with_str_and_options_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_match_all_with_str_and_options_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                let options =
                    <&CacheQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_match_all_with_str_and_options_Cache(self_, request, options)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache", feature = "Request", feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_put_with_request_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Cache as WasmDescribe>::describe();
    <&Request as WasmDescribe>::describe();
    <&Response as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache", feature = "Request", feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `put()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/put)\n\n*This API requires the following crate features to be activated: `Cache`, `Request`, `Response`*"]
    #[allow(clippy::all)]
    pub fn put_with_request(&self, request: &Request, response: &Response) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache", feature = "Request", feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_put_with_request_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                response: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_put_with_request_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            request: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            response: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(request);
            drop(response);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                let response =
                    <&Response as wasm_bindgen::convert::IntoWasmAbi>::into_abi(response);
                __widl_f_put_with_request_Cache(self_, request, response)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Cache", feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_put_with_str_Cache() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Cache as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&Response as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Cache {
    #[cfg(all(feature = "Cache", feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `put()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/put)\n\n*This API requires the following crate features to be activated: `Cache`, `Response`*"]
    #[allow(clippy::all)]
    pub fn put_with_str(&self, request: &str, response: &Response) -> ::js_sys::Promise {
        #[cfg(all(feature = "Cache", feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_put_with_str_Cache(
                self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                request: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                response: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_put_with_str_Cache(
            self_: <&Cache as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            request: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            response: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(request);
            drop(response);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Cache as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let request = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(request);
                let response =
                    <&Response as wasm_bindgen::convert::IntoWasmAbi>::into_abi(response);
                __widl_f_put_with_str_Cache(self_, request, response)
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
pub static __WASM_BINDGEN_GENERATED_9ca85e41ae47f2cc: [u8; 2002usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x90\x07\0\0\0\0\x19\0\0\x02\x05Cache\x17__widl_instanceof_Cache\0\0\0\0\x1F__widl_f_add_with_request_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x02\x05self_\x07request\x03add\0\0\0\x1B__widl_f_add_with_str_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x02\x05self_\x07request\x03add\0\0\0,__widl_f_add_all_with_request_sequence_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x02\x05self_\x08requests\x06addAll\0\0\0(__widl_f_add_all_with_str_sequence_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x02\x05self_\x08requests\x06addAll\0\0\0\"__widl_f_delete_with_request_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x02\x05self_\x07request\x06delete\0\0\0\x1E__widl_f_delete_with_str_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x02\x05self_\x07request\x06delete\0\0\0.__widl_f_delete_with_request_and_options_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x03\x05self_\x07request\x07options\x06delete\0\0\0*__widl_f_delete_with_str_and_options_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x03\x05self_\x07request\x07options\x06delete\0\0\0\x13__widl_f_keys_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x01\x05self_\x04keys\0\0\0 __widl_f_keys_with_request_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x02\x05self_\x07request\x04keys\0\0\0\x1C__widl_f_keys_with_str_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x02\x05self_\x07request\x04keys\0\0\0,__widl_f_keys_with_request_and_options_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x03\x05self_\x07request\x07options\x04keys\0\0\0(__widl_f_keys_with_str_and_options_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x03\x05self_\x07request\x07options\x04keys\0\0\0!__widl_f_match_with_request_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x02\x05self_\x07request\x05match\0\0\0\x1D__widl_f_match_with_str_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x02\x05self_\x07request\x05match\0\0\0-__widl_f_match_with_request_and_options_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x03\x05self_\x07request\x07options\x05match\0\0\0)__widl_f_match_with_str_and_options_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x03\x05self_\x07request\x07options\x05match\0\0\0\x18__widl_f_match_all_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x01\x05self_\x08matchAll\0\0\0%__widl_f_match_all_with_request_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x02\x05self_\x07request\x08matchAll\0\0\0!__widl_f_match_all_with_str_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x02\x05self_\x07request\x08matchAll\0\0\01__widl_f_match_all_with_request_and_options_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x03\x05self_\x07request\x07options\x08matchAll\0\0\0-__widl_f_match_all_with_str_and_options_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x03\x05self_\x07request\x07options\x08matchAll\0\0\0\x1F__widl_f_put_with_request_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x03\x05self_\x07request\x08response\x03put\0\0\0\x1B__widl_f_put_with_str_Cache\0\0\0\x01\x05Cache\x01\0\0\x01\x03\x05self_\x07request\x08response\x03put\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
