use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Request` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request)\n\n*This API requires the following crate features to be activated: `Request`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Request {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Request: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Request {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(7u32);
            inform(82u32);
            inform(101u32);
            inform(113u32);
            inform(117u32);
            inform(101u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for Request {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Request {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Request {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Request {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Request {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Request {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Request {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Request {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Request {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Request>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Request {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Request {
        #[inline]
        fn from(obj: JsValue) -> Request {
            Request { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Request {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Request> for Request {
        #[inline]
        fn as_ref(&self) -> &Request {
            self
        }
    }
    impl From<Request> for JsValue {
        #[inline]
        fn from(obj: Request) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Request {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Request(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Request(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Request(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Request { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Request) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Request> for ::js_sys::Object {
    #[inline]
    fn from(obj: Request) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Request {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_request_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Request as WasmDescribe>::describe();
    <Request as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `new Request(..)` constructor, creating a new instance of `Request`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)\n\n*This API requires the following crate features to be activated: `Request`*"]
    #[allow(clippy::all)]
    pub fn new_with_request(input: &Request) -> Result<Request, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_request_Request(
                input: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Request as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_request_Request(
            input: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Request as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(input);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let input = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(input);
                __widl_f_new_with_request_Request(input)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Request as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_str_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <Request as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `new Request(..)` constructor, creating a new instance of `Request`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)\n\n*This API requires the following crate features to be activated: `Request`*"]
    #[allow(clippy::all)]
    pub fn new_with_str(input: &str) -> Result<Request, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_str_Request(
                input: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Request as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_str_Request(
            input: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Request as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(input);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let input = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(input);
                __widl_f_new_with_str_Request(input)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Request as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Request", feature = "RequestInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_request_and_init_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Request as WasmDescribe>::describe();
    <&RequestInit as WasmDescribe>::describe();
    <Request as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "Request", feature = "RequestInit",))]
    #[allow(bad_style)]
    #[doc = "The `new Request(..)` constructor, creating a new instance of `Request`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)\n\n*This API requires the following crate features to be activated: `Request`, `RequestInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_request_and_init(
        input: &Request,
        init: &RequestInit,
    ) -> Result<Request, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Request", feature = "RequestInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_request_and_init_Request(
                input: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                init: <&RequestInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Request as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_request_and_init_Request(
            input: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            init: <&RequestInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Request as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(input);
            drop(init);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let input = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(input);
                let init = <&RequestInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(init);
                __widl_f_new_with_request_and_init_Request(input, init)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Request as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Request", feature = "RequestInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_str_and_init_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&RequestInit as WasmDescribe>::describe();
    <Request as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "Request", feature = "RequestInit",))]
    #[allow(bad_style)]
    #[doc = "The `new Request(..)` constructor, creating a new instance of `Request`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)\n\n*This API requires the following crate features to be activated: `Request`, `RequestInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_str_and_init(
        input: &str,
        init: &RequestInit,
    ) -> Result<Request, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Request", feature = "RequestInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_str_and_init_Request(
                input: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                init: <&RequestInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Request as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_str_and_init_Request(
            input: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            init: <&RequestInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Request as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(input);
            drop(init);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let input = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(input);
                let init = <&RequestInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(init);
                __widl_f_new_with_str_and_init_Request(input, init)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Request as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clone_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Request as WasmDescribe>::describe();
    <Request as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `clone()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/clone)\n\n*This API requires the following crate features to be activated: `Request`*"]
    #[allow(clippy::all)]
    pub fn clone(&self) -> Result<Request, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clone_Request(
                self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Request as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clone_Request(
            self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Request as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clone_Request(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Request as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_method_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Request as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `method` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/method)\n\n*This API requires the following crate features to be activated: `Request`*"]
    #[allow(clippy::all)]
    pub fn method(&self) -> String {
        #[cfg(all(feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_method_Request(
                self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_method_Request(
            self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_method_Request(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_url_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Request as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `url` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/url)\n\n*This API requires the following crate features to be activated: `Request`*"]
    #[allow(clippy::all)]
    pub fn url(&self) -> String {
        #[cfg(all(feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_url_Request(
                self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_url_Request(
            self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_url_Request(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Headers", feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_headers_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Request as WasmDescribe>::describe();
    <Headers as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "Headers", feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `headers` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/headers)\n\n*This API requires the following crate features to be activated: `Headers`, `Request`*"]
    #[allow(clippy::all)]
    pub fn headers(&self) -> Headers {
        #[cfg(all(feature = "Headers", feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_headers_Request(
                self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Headers as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_headers_Request(
            self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Headers as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_headers_Request(self_)
            };
            <Headers as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Request", feature = "RequestDestination",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_destination_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Request as WasmDescribe>::describe();
    <RequestDestination as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "Request", feature = "RequestDestination",))]
    #[allow(bad_style)]
    #[doc = "The `destination` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/destination)\n\n*This API requires the following crate features to be activated: `Request`, `RequestDestination`*"]
    #[allow(clippy::all)]
    pub fn destination(&self) -> RequestDestination {
        #[cfg(all(feature = "Request", feature = "RequestDestination",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_destination_Request(
                self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RequestDestination as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_destination_Request(
            self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RequestDestination as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_destination_Request(self_)
            };
            <RequestDestination as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_referrer_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Request as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `referrer` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/referrer)\n\n*This API requires the following crate features to be activated: `Request`*"]
    #[allow(clippy::all)]
    pub fn referrer(&self) -> String {
        #[cfg(all(feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_referrer_Request(
                self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_referrer_Request(
            self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_referrer_Request(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ReferrerPolicy", feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_referrer_policy_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Request as WasmDescribe>::describe();
    <ReferrerPolicy as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "ReferrerPolicy", feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `referrerPolicy` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/referrerPolicy)\n\n*This API requires the following crate features to be activated: `ReferrerPolicy`, `Request`*"]
    #[allow(clippy::all)]
    pub fn referrer_policy(&self) -> ReferrerPolicy {
        #[cfg(all(feature = "ReferrerPolicy", feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_referrer_policy_Request(
                self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ReferrerPolicy as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_referrer_policy_Request(
            self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ReferrerPolicy as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_referrer_policy_Request(self_)
            };
            <ReferrerPolicy as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Request", feature = "RequestMode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_mode_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Request as WasmDescribe>::describe();
    <RequestMode as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "Request", feature = "RequestMode",))]
    #[allow(bad_style)]
    #[doc = "The `mode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/mode)\n\n*This API requires the following crate features to be activated: `Request`, `RequestMode`*"]
    #[allow(clippy::all)]
    pub fn mode(&self) -> RequestMode {
        #[cfg(all(feature = "Request", feature = "RequestMode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_mode_Request(
                self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RequestMode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_mode_Request(
            self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RequestMode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_mode_Request(self_)
            };
            <RequestMode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Request", feature = "RequestCredentials",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_credentials_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Request as WasmDescribe>::describe();
    <RequestCredentials as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "Request", feature = "RequestCredentials",))]
    #[allow(bad_style)]
    #[doc = "The `credentials` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/credentials)\n\n*This API requires the following crate features to be activated: `Request`, `RequestCredentials`*"]
    #[allow(clippy::all)]
    pub fn credentials(&self) -> RequestCredentials {
        #[cfg(all(feature = "Request", feature = "RequestCredentials",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_credentials_Request(
                self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RequestCredentials as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_credentials_Request(
            self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RequestCredentials as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_credentials_Request(self_)
            };
            <RequestCredentials as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Request", feature = "RequestCache",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cache_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Request as WasmDescribe>::describe();
    <RequestCache as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "Request", feature = "RequestCache",))]
    #[allow(bad_style)]
    #[doc = "The `cache` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/cache)\n\n*This API requires the following crate features to be activated: `Request`, `RequestCache`*"]
    #[allow(clippy::all)]
    pub fn cache(&self) -> RequestCache {
        #[cfg(all(feature = "Request", feature = "RequestCache",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cache_Request(
                self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RequestCache as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cache_Request(
            self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RequestCache as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_cache_Request(self_)
            };
            <RequestCache as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Request", feature = "RequestRedirect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_redirect_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Request as WasmDescribe>::describe();
    <RequestRedirect as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "Request", feature = "RequestRedirect",))]
    #[allow(bad_style)]
    #[doc = "The `redirect` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/redirect)\n\n*This API requires the following crate features to be activated: `Request`, `RequestRedirect`*"]
    #[allow(clippy::all)]
    pub fn redirect(&self) -> RequestRedirect {
        #[cfg(all(feature = "Request", feature = "RequestRedirect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_redirect_Request(
                self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RequestRedirect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_redirect_Request(
            self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RequestRedirect as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_redirect_Request(self_)
            };
            <RequestRedirect as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_integrity_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Request as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `integrity` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/integrity)\n\n*This API requires the following crate features to be activated: `Request`*"]
    #[allow(clippy::all)]
    pub fn integrity(&self) -> String {
        #[cfg(all(feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_integrity_Request(
                self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_integrity_Request(
            self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_integrity_Request(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AbortSignal", feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_signal_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Request as WasmDescribe>::describe();
    <AbortSignal as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "AbortSignal", feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `signal` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/signal)\n\n*This API requires the following crate features to be activated: `AbortSignal`, `Request`*"]
    #[allow(clippy::all)]
    pub fn signal(&self) -> AbortSignal {
        #[cfg(all(feature = "AbortSignal", feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_signal_Request(
                self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AbortSignal as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_signal_Request(
            self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AbortSignal as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_signal_Request(self_)
            };
            <AbortSignal as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_array_buffer_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Request as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `arrayBuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/arrayBuffer)\n\n*This API requires the following crate features to be activated: `Request`*"]
    #[allow(clippy::all)]
    pub fn array_buffer(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_array_buffer_Request(
                self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_array_buffer_Request(
            self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_array_buffer_Request(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_blob_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Request as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `blob()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/blob)\n\n*This API requires the following crate features to be activated: `Request`*"]
    #[allow(clippy::all)]
    pub fn blob(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_blob_Request(
                self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_blob_Request(
            self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_blob_Request(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_form_data_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Request as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `formData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/formData)\n\n*This API requires the following crate features to be activated: `Request`*"]
    #[allow(clippy::all)]
    pub fn form_data(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_form_data_Request(
                self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_form_data_Request(
            self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_form_data_Request(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_json_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Request as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `json()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/json)\n\n*This API requires the following crate features to be activated: `Request`*"]
    #[allow(clippy::all)]
    pub fn json(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_json_Request(
                self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_json_Request(
            self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_json_Request(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_text_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Request as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `text()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/text)\n\n*This API requires the following crate features to be activated: `Request`*"]
    #[allow(clippy::all)]
    pub fn text(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_text_Request(
                self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_text_Request(
            self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_text_Request(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_body_used_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Request as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `bodyUsed` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/bodyUsed)\n\n*This API requires the following crate features to be activated: `Request`*"]
    #[allow(clippy::all)]
    pub fn body_used(&self) -> bool {
        #[cfg(all(feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_body_used_Request(
                self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_body_used_Request(
            self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_body_used_Request(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ReadableStream", feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_body_Request() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Request as WasmDescribe>::describe();
    <Option<ReadableStream> as WasmDescribe>::describe();
}
impl Request {
    #[cfg(all(feature = "ReadableStream", feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `body` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/body)\n\n*This API requires the following crate features to be activated: `ReadableStream`, `Request`*"]
    #[allow(clippy::all)]
    pub fn body(&self) -> Option<ReadableStream> {
        #[cfg(all(feature = "ReadableStream", feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_body_Request(
                self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<ReadableStream> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_body_Request(
            self_: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<ReadableStream> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_body_Request(self_)
            };
            <Option<ReadableStream> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_eb6e47ffb662cdb4: [u8; 1713usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}o\x06\0\0\0\0\x19\0\0\x02\x07Request\x19__widl_instanceof_Request\0\0\0\0!__widl_f_new_with_request_Request\x01\0\0\x01\x07Request\0\x01\x01\x05input\x03new\0\0\0\x1D__widl_f_new_with_str_Request\x01\0\0\x01\x07Request\0\x01\x01\x05input\x03new\0\0\0*__widl_f_new_with_request_and_init_Request\x01\0\0\x01\x07Request\0\x01\x02\x05input\x04init\x03new\0\0\0&__widl_f_new_with_str_and_init_Request\x01\0\0\x01\x07Request\0\x01\x02\x05input\x04init\x03new\0\0\0\x16__widl_f_clone_Request\x01\0\0\x01\x07Request\x01\0\0\x01\x01\x05self_\x05clone\0\0\0\x17__widl_f_method_Request\0\0\0\x01\x07Request\x01\0\x01\x06method\x01\x01\x05self_\x06method\0\0\0\x14__widl_f_url_Request\0\0\0\x01\x07Request\x01\0\x01\x03url\x01\x01\x05self_\x03url\0\0\0\x18__widl_f_headers_Request\0\0\0\x01\x07Request\x01\0\x01\x07headers\x01\x01\x05self_\x07headers\0\0\0\x1C__widl_f_destination_Request\0\0\0\x01\x07Request\x01\0\x01\x0Bdestination\x01\x01\x05self_\x0Bdestination\0\0\0\x19__widl_f_referrer_Request\0\0\0\x01\x07Request\x01\0\x01\x08referrer\x01\x01\x05self_\x08referrer\0\0\0 __widl_f_referrer_policy_Request\0\0\0\x01\x07Request\x01\0\x01\x0EreferrerPolicy\x01\x01\x05self_\x0EreferrerPolicy\0\0\0\x15__widl_f_mode_Request\0\0\0\x01\x07Request\x01\0\x01\x04mode\x01\x01\x05self_\x04mode\0\0\0\x1C__widl_f_credentials_Request\0\0\0\x01\x07Request\x01\0\x01\x0Bcredentials\x01\x01\x05self_\x0Bcredentials\0\0\0\x16__widl_f_cache_Request\0\0\0\x01\x07Request\x01\0\x01\x05cache\x01\x01\x05self_\x05cache\0\0\0\x19__widl_f_redirect_Request\0\0\0\x01\x07Request\x01\0\x01\x08redirect\x01\x01\x05self_\x08redirect\0\0\0\x1A__widl_f_integrity_Request\0\0\0\x01\x07Request\x01\0\x01\tintegrity\x01\x01\x05self_\tintegrity\0\0\0\x17__widl_f_signal_Request\0\0\0\x01\x07Request\x01\0\x01\x06signal\x01\x01\x05self_\x06signal\0\0\0\x1D__widl_f_array_buffer_Request\x01\0\0\x01\x07Request\x01\0\0\x01\x01\x05self_\x0BarrayBuffer\0\0\0\x15__widl_f_blob_Request\x01\0\0\x01\x07Request\x01\0\0\x01\x01\x05self_\x04blob\0\0\0\x1A__widl_f_form_data_Request\x01\0\0\x01\x07Request\x01\0\0\x01\x01\x05self_\x08formData\0\0\0\x15__widl_f_json_Request\x01\0\0\x01\x07Request\x01\0\0\x01\x01\x05self_\x04json\0\0\0\x15__widl_f_text_Request\x01\0\0\x01\x07Request\x01\0\0\x01\x01\x05self_\x04text\0\0\0\x1A__widl_f_body_used_Request\0\0\0\x01\x07Request\x01\0\x01\x08bodyUsed\x01\x01\x05self_\x08bodyUsed\0\0\0\x15__widl_f_body_Request\0\0\0\x01\x07Request\x01\0\x01\x04body\x01\x01\x05self_\x04body\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
