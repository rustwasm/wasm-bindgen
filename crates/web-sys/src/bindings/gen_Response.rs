use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Response` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response)\n\n*This API requires the following crate features to be activated: `Response`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Response {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Response: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Response {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(8u32);
            inform(82u32);
            inform(101u32);
            inform(115u32);
            inform(112u32);
            inform(111u32);
            inform(110u32);
            inform(115u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for Response {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Response {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Response {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Response {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Response {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Response {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Response {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Response {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Response {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Response>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Response {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Response {
        #[inline]
        fn from(obj: JsValue) -> Response {
            Response { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Response {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Response> for Response {
        #[inline]
        fn as_ref(&self) -> &Response {
            self
        }
    }
    impl From<Response> for JsValue {
        #[inline]
        fn from(obj: Response) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Response {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Response(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Response(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Response(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Response { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Response) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Response> for ::js_sys::Object {
    #[inline]
    fn from(obj: Response) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Response {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <Response as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Response`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<Response, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_Response() -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_Response() -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_Response() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Response as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Blob", feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_opt_blob_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <Option<&Blob> as WasmDescribe>::describe();
    <Response as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Blob", feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Blob`, `Response`*"]
    #[allow(clippy::all)]
    pub fn new_with_opt_blob(body: Option<&Blob>) -> Result<Response, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_opt_blob_Response(
                body: <Option<&Blob> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_opt_blob_Response(
            body: <Option<&Blob> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(body);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let body = <Option<&Blob> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(body);
                __widl_f_new_with_opt_blob_Response(body)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Response as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_opt_buffer_source_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <Option<&::js_sys::Object> as WasmDescribe>::describe();
    <Response as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Response`*"]
    #[allow(clippy::all)]
    pub fn new_with_opt_buffer_source(
        body: Option<&::js_sys::Object>,
    ) -> Result<Response, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_opt_buffer_source_Response(
                body: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_opt_buffer_source_Response(
            body: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(body);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let body =
                    <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        body,
                    );
                __widl_f_new_with_opt_buffer_source_Response(body)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Response as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_opt_u8_array_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <Option<&mut [u8]> as WasmDescribe>::describe();
    <Response as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Response`*"]
    #[allow(clippy::all)]
    pub fn new_with_opt_u8_array(
        body: Option<&mut [u8]>,
    ) -> Result<Response, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_opt_u8_array_Response(
                body: <Option<&mut [u8]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_opt_u8_array_Response(
            body: <Option<&mut [u8]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(body);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let body =
                    <Option<&mut [u8]> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(body);
                __widl_f_new_with_opt_u8_array_Response(body)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Response as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "FormData", feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_opt_form_data_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <Option<&FormData> as WasmDescribe>::describe();
    <Response as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "FormData", feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `FormData`, `Response`*"]
    #[allow(clippy::all)]
    pub fn new_with_opt_form_data(
        body: Option<&FormData>,
    ) -> Result<Response, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FormData", feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_opt_form_data_Response(
                body: <Option<&FormData> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_opt_form_data_Response(
            body: <Option<&FormData> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(body);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let body =
                    <Option<&FormData> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(body);
                __widl_f_new_with_opt_form_data_Response(body)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Response as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Response", feature = "UrlSearchParams",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_opt_url_search_params_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <Option<&UrlSearchParams> as WasmDescribe>::describe();
    <Response as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response", feature = "UrlSearchParams",))]
    #[allow(bad_style)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Response`, `UrlSearchParams`*"]
    #[allow(clippy::all)]
    pub fn new_with_opt_url_search_params(
        body: Option<&UrlSearchParams>,
    ) -> Result<Response, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Response", feature = "UrlSearchParams",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_opt_url_search_params_Response(
                body: <Option<&UrlSearchParams> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_opt_url_search_params_Response(
            body: <Option<&UrlSearchParams> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(body);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let body =
                    <Option<&UrlSearchParams> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        body,
                    );
                __widl_f_new_with_opt_url_search_params_Response(body)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Response as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_opt_str_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <Option<&str> as WasmDescribe>::describe();
    <Response as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Response`*"]
    #[allow(clippy::all)]
    pub fn new_with_opt_str(body: Option<&str>) -> Result<Response, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_opt_str_Response(
                body: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_opt_str_Response(
            body: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(body);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let body = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(body);
                __widl_f_new_with_opt_str_Response(body)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Response as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "ReadableStream", feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_opt_readable_stream_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <Option<&ReadableStream> as WasmDescribe>::describe();
    <Response as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "ReadableStream", feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `ReadableStream`, `Response`*"]
    #[allow(clippy::all)]
    pub fn new_with_opt_readable_stream(
        body: Option<&ReadableStream>,
    ) -> Result<Response, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ReadableStream", feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_opt_readable_stream_Response(
                body: <Option<&ReadableStream> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_opt_readable_stream_Response(
            body: <Option<&ReadableStream> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(body);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let body =
                    <Option<&ReadableStream> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(body);
                __widl_f_new_with_opt_readable_stream_Response(body)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Response as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Blob", feature = "Response", feature = "ResponseInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_opt_blob_and_init_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <Option<&Blob> as WasmDescribe>::describe();
    <&ResponseInit as WasmDescribe>::describe();
    <Response as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Blob", feature = "Response", feature = "ResponseInit",))]
    #[allow(bad_style)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Blob`, `Response`, `ResponseInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_opt_blob_and_init(
        body: Option<&Blob>,
        init: &ResponseInit,
    ) -> Result<Response, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "Response", feature = "ResponseInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_opt_blob_and_init_Response(
                body: <Option<&Blob> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                init: <&ResponseInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_opt_blob_and_init_Response(
            body: <Option<&Blob> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            init: <&ResponseInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(body);
            drop(init);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let body = <Option<&Blob> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(body);
                let init = <&ResponseInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(init);
                __widl_f_new_with_opt_blob_and_init_Response(body, init)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Response as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Response", feature = "ResponseInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_opt_buffer_source_and_init_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <Option<&::js_sys::Object> as WasmDescribe>::describe();
    <&ResponseInit as WasmDescribe>::describe();
    <Response as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response", feature = "ResponseInit",))]
    #[allow(bad_style)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Response`, `ResponseInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_opt_buffer_source_and_init(
        body: Option<&::js_sys::Object>,
        init: &ResponseInit,
    ) -> Result<Response, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Response", feature = "ResponseInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_opt_buffer_source_and_init_Response(
                body: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                init: <&ResponseInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_opt_buffer_source_and_init_Response(
            body: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            init: <&ResponseInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(body);
            drop(init);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let body =
                    <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        body,
                    );
                let init = <&ResponseInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(init);
                __widl_f_new_with_opt_buffer_source_and_init_Response(body, init)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Response as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Response", feature = "ResponseInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_opt_u8_array_and_init_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <Option<&mut [u8]> as WasmDescribe>::describe();
    <&ResponseInit as WasmDescribe>::describe();
    <Response as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response", feature = "ResponseInit",))]
    #[allow(bad_style)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Response`, `ResponseInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_opt_u8_array_and_init(
        body: Option<&mut [u8]>,
        init: &ResponseInit,
    ) -> Result<Response, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Response", feature = "ResponseInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_opt_u8_array_and_init_Response(
                body: <Option<&mut [u8]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                init: <&ResponseInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_opt_u8_array_and_init_Response(
            body: <Option<&mut [u8]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            init: <&ResponseInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(body);
            drop(init);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let body =
                    <Option<&mut [u8]> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(body);
                let init = <&ResponseInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(init);
                __widl_f_new_with_opt_u8_array_and_init_Response(body, init)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Response as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "FormData", feature = "Response", feature = "ResponseInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_opt_form_data_and_init_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <Option<&FormData> as WasmDescribe>::describe();
    <&ResponseInit as WasmDescribe>::describe();
    <Response as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "FormData", feature = "Response", feature = "ResponseInit",))]
    #[allow(bad_style)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `FormData`, `Response`, `ResponseInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_opt_form_data_and_init(
        body: Option<&FormData>,
        init: &ResponseInit,
    ) -> Result<Response, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FormData", feature = "Response", feature = "ResponseInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_opt_form_data_and_init_Response(
                body: <Option<&FormData> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                init: <&ResponseInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_opt_form_data_and_init_Response(
            body: <Option<&FormData> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            init: <&ResponseInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(body);
            drop(init);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let body =
                    <Option<&FormData> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(body);
                let init = <&ResponseInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(init);
                __widl_f_new_with_opt_form_data_and_init_Response(body, init)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Response as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "Response",
    feature = "ResponseInit",
    feature = "UrlSearchParams",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_opt_url_search_params_and_init_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <Option<&UrlSearchParams> as WasmDescribe>::describe();
    <&ResponseInit as WasmDescribe>::describe();
    <Response as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(
        feature = "Response",
        feature = "ResponseInit",
        feature = "UrlSearchParams",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Response`, `ResponseInit`, `UrlSearchParams`*"]
    #[allow(clippy::all)]
    pub fn new_with_opt_url_search_params_and_init(
        body: Option<&UrlSearchParams>,
        init: &ResponseInit,
    ) -> Result<Response, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Response",
            feature = "ResponseInit",
            feature = "UrlSearchParams",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_opt_url_search_params_and_init_Response(
                body: <Option<&UrlSearchParams> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                init: <&ResponseInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_opt_url_search_params_and_init_Response(
            body: <Option<&UrlSearchParams> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            init: <&ResponseInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(body);
            drop(init);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let body =
                    <Option<&UrlSearchParams> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        body,
                    );
                let init = <&ResponseInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(init);
                __widl_f_new_with_opt_url_search_params_and_init_Response(body, init)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Response as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Response", feature = "ResponseInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_opt_str_and_init_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <Option<&str> as WasmDescribe>::describe();
    <&ResponseInit as WasmDescribe>::describe();
    <Response as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response", feature = "ResponseInit",))]
    #[allow(bad_style)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Response`, `ResponseInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_opt_str_and_init(
        body: Option<&str>,
        init: &ResponseInit,
    ) -> Result<Response, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Response", feature = "ResponseInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_opt_str_and_init_Response(
                body: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                init: <&ResponseInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_opt_str_and_init_Response(
            body: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            init: <&ResponseInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(body);
            drop(init);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let body = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(body);
                let init = <&ResponseInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(init);
                __widl_f_new_with_opt_str_and_init_Response(body, init)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Response as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "ReadableStream",
    feature = "Response",
    feature = "ResponseInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_opt_readable_stream_and_init_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <Option<&ReadableStream> as WasmDescribe>::describe();
    <&ResponseInit as WasmDescribe>::describe();
    <Response as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(
        feature = "ReadableStream",
        feature = "Response",
        feature = "ResponseInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `ReadableStream`, `Response`, `ResponseInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_opt_readable_stream_and_init(
        body: Option<&ReadableStream>,
        init: &ResponseInit,
    ) -> Result<Response, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ReadableStream",
            feature = "Response",
            feature = "ResponseInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_opt_readable_stream_and_init_Response(
                body: <Option<&ReadableStream> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                init: <&ResponseInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_opt_readable_stream_and_init_Response(
            body: <Option<&ReadableStream> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            init: <&ResponseInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(body);
            drop(init);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let body =
                    <Option<&ReadableStream> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(body);
                let init = <&ResponseInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(init);
                __widl_f_new_with_opt_readable_stream_and_init_Response(body, init)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Response as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clone_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Response as WasmDescribe>::describe();
    <Response as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `clone()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/clone)\n\n*This API requires the following crate features to be activated: `Response`*"]
    #[allow(clippy::all)]
    pub fn clone(&self) -> Result<Response, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clone_Response(
                self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clone_Response(
            self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Response as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clone_Response(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Response as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_error_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <Response as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `error()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/error)\n\n*This API requires the following crate features to be activated: `Response`*"]
    #[allow(clippy::all)]
    pub fn error() -> Response {
        #[cfg(all(feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_Response() -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_Response() -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi
        {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_error_Response() };
            <Response as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_redirect_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <Response as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `redirect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect)\n\n*This API requires the following crate features to be activated: `Response`*"]
    #[allow(clippy::all)]
    pub fn redirect(url: &str) -> Result<Response, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_redirect_Response(
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_redirect_Response(
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                __widl_f_redirect_Response(url)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Response as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_redirect_with_status_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <Response as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `redirect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect)\n\n*This API requires the following crate features to be activated: `Response`*"]
    #[allow(clippy::all)]
    pub fn redirect_with_status(
        url: &str,
        status: u16,
    ) -> Result<Response, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_redirect_with_status_Response(
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                status: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_redirect_with_status_Response(
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            status: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Response as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(url);
            drop(status);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                let status = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(status);
                __widl_f_redirect_with_status_Response(url, status)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Response as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Response", feature = "ResponseType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Response as WasmDescribe>::describe();
    <ResponseType as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response", feature = "ResponseType",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/type)\n\n*This API requires the following crate features to be activated: `Response`, `ResponseType`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> ResponseType {
        #[cfg(all(feature = "Response", feature = "ResponseType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_Response(
                self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ResponseType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_Response(
            self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ResponseType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Response as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_Response(self_)
            };
            <ResponseType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_url_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Response as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `url` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/url)\n\n*This API requires the following crate features to be activated: `Response`*"]
    #[allow(clippy::all)]
    pub fn url(&self) -> String {
        #[cfg(all(feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_url_Response(
                self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_url_Response(
            self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Response as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_url_Response(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_redirected_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Response as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `redirected` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirected)\n\n*This API requires the following crate features to be activated: `Response`*"]
    #[allow(clippy::all)]
    pub fn redirected(&self) -> bool {
        #[cfg(all(feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_redirected_Response(
                self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_redirected_Response(
            self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Response as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_redirected_Response(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_status_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Response as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `status` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/status)\n\n*This API requires the following crate features to be activated: `Response`*"]
    #[allow(clippy::all)]
    pub fn status(&self) -> u16 {
        #[cfg(all(feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_status_Response(
                self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_status_Response(
            self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Response as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_status_Response(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ok_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Response as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `ok` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/ok)\n\n*This API requires the following crate features to be activated: `Response`*"]
    #[allow(clippy::all)]
    pub fn ok(&self) -> bool {
        #[cfg(all(feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ok_Response(
                self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ok_Response(
            self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Response as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ok_Response(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_status_text_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Response as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `statusText` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/statusText)\n\n*This API requires the following crate features to be activated: `Response`*"]
    #[allow(clippy::all)]
    pub fn status_text(&self) -> String {
        #[cfg(all(feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_status_text_Response(
                self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_status_text_Response(
            self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Response as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_status_text_Response(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Headers", feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_headers_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Response as WasmDescribe>::describe();
    <Headers as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Headers", feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `headers` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/headers)\n\n*This API requires the following crate features to be activated: `Headers`, `Response`*"]
    #[allow(clippy::all)]
    pub fn headers(&self) -> Headers {
        #[cfg(all(feature = "Headers", feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_headers_Response(
                self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Headers as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_headers_Response(
            self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Headers as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Response as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_headers_Response(self_)
            };
            <Headers as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_array_buffer_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Response as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `arrayBuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/arrayBuffer)\n\n*This API requires the following crate features to be activated: `Response`*"]
    #[allow(clippy::all)]
    pub fn array_buffer(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_array_buffer_Response(
                self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_array_buffer_Response(
            self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Response as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_array_buffer_Response(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_blob_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Response as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `blob()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/blob)\n\n*This API requires the following crate features to be activated: `Response`*"]
    #[allow(clippy::all)]
    pub fn blob(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_blob_Response(
                self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_blob_Response(
            self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Response as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_blob_Response(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_form_data_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Response as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `formData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/formData)\n\n*This API requires the following crate features to be activated: `Response`*"]
    #[allow(clippy::all)]
    pub fn form_data(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_form_data_Response(
                self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_form_data_Response(
            self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Response as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_form_data_Response(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_json_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Response as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `json()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/json)\n\n*This API requires the following crate features to be activated: `Response`*"]
    #[allow(clippy::all)]
    pub fn json(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_json_Response(
                self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_json_Response(
            self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Response as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_json_Response(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_text_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Response as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `text()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/text)\n\n*This API requires the following crate features to be activated: `Response`*"]
    #[allow(clippy::all)]
    pub fn text(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_text_Response(
                self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_text_Response(
            self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Response as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_text_Response(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_body_used_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Response as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `bodyUsed` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/bodyUsed)\n\n*This API requires the following crate features to be activated: `Response`*"]
    #[allow(clippy::all)]
    pub fn body_used(&self) -> bool {
        #[cfg(all(feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_body_used_Response(
                self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_body_used_Response(
            self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Response as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_body_used_Response(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ReadableStream", feature = "Response",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_body_Response() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Response as WasmDescribe>::describe();
    <Option<ReadableStream> as WasmDescribe>::describe();
}
impl Response {
    #[cfg(all(feature = "ReadableStream", feature = "Response",))]
    #[allow(bad_style)]
    #[doc = "The `body` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/body)\n\n*This API requires the following crate features to be activated: `ReadableStream`, `Response`*"]
    #[allow(clippy::all)]
    pub fn body(&self) -> Option<ReadableStream> {
        #[cfg(all(feature = "ReadableStream", feature = "Response",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_body_Response(
                self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<ReadableStream> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_body_Response(
            self_: <&Response as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<ReadableStream> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Response as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_body_Response(self_)
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
pub static __WASM_BINDGEN_GENERATED_a5db0b3a706aa1e4: [u8; 2410usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}(\t\0\0\0\0\"\0\0\x02\x08Response\x1A__widl_instanceof_Response\0\0\0\0\x15__widl_f_new_Response\x01\0\0\x01\x08Response\0\x01\0\x03new\0\0\0#__widl_f_new_with_opt_blob_Response\x01\0\0\x01\x08Response\0\x01\x01\x04body\x03new\0\0\0,__widl_f_new_with_opt_buffer_source_Response\x01\0\0\x01\x08Response\0\x01\x01\x04body\x03new\0\0\0'__widl_f_new_with_opt_u8_array_Response\x01\0\0\x01\x08Response\0\x01\x01\x04body\x03new\0\0\0(__widl_f_new_with_opt_form_data_Response\x01\0\0\x01\x08Response\0\x01\x01\x04body\x03new\0\0\00__widl_f_new_with_opt_url_search_params_Response\x01\0\0\x01\x08Response\0\x01\x01\x04body\x03new\0\0\0\"__widl_f_new_with_opt_str_Response\x01\0\0\x01\x08Response\0\x01\x01\x04body\x03new\0\0\0.__widl_f_new_with_opt_readable_stream_Response\x01\0\0\x01\x08Response\0\x01\x01\x04body\x03new\0\0\0,__widl_f_new_with_opt_blob_and_init_Response\x01\0\0\x01\x08Response\0\x01\x02\x04body\x04init\x03new\0\0\05__widl_f_new_with_opt_buffer_source_and_init_Response\x01\0\0\x01\x08Response\0\x01\x02\x04body\x04init\x03new\0\0\00__widl_f_new_with_opt_u8_array_and_init_Response\x01\0\0\x01\x08Response\0\x01\x02\x04body\x04init\x03new\0\0\01__widl_f_new_with_opt_form_data_and_init_Response\x01\0\0\x01\x08Response\0\x01\x02\x04body\x04init\x03new\0\0\09__widl_f_new_with_opt_url_search_params_and_init_Response\x01\0\0\x01\x08Response\0\x01\x02\x04body\x04init\x03new\0\0\0+__widl_f_new_with_opt_str_and_init_Response\x01\0\0\x01\x08Response\0\x01\x02\x04body\x04init\x03new\0\0\07__widl_f_new_with_opt_readable_stream_and_init_Response\x01\0\0\x01\x08Response\0\x01\x02\x04body\x04init\x03new\0\0\0\x17__widl_f_clone_Response\x01\0\0\x01\x08Response\x01\0\0\x01\x01\x05self_\x05clone\0\0\0\x17__widl_f_error_Response\0\0\0\x01\x08Response\x01\x01\0\x01\0\x05error\0\0\0\x1A__widl_f_redirect_Response\x01\0\0\x01\x08Response\x01\x01\0\x01\x01\x03url\x08redirect\0\0\0&__widl_f_redirect_with_status_Response\x01\0\0\x01\x08Response\x01\x01\0\x01\x02\x03url\x06status\x08redirect\0\0\0\x16__widl_f_type_Response\0\0\0\x01\x08Response\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0\x15__widl_f_url_Response\0\0\0\x01\x08Response\x01\0\x01\x03url\x01\x01\x05self_\x03url\0\0\0\x1C__widl_f_redirected_Response\0\0\0\x01\x08Response\x01\0\x01\nredirected\x01\x01\x05self_\nredirected\0\0\0\x18__widl_f_status_Response\0\0\0\x01\x08Response\x01\0\x01\x06status\x01\x01\x05self_\x06status\0\0\0\x14__widl_f_ok_Response\0\0\0\x01\x08Response\x01\0\x01\x02ok\x01\x01\x05self_\x02ok\0\0\0\x1D__widl_f_status_text_Response\0\0\0\x01\x08Response\x01\0\x01\nstatusText\x01\x01\x05self_\nstatusText\0\0\0\x19__widl_f_headers_Response\0\0\0\x01\x08Response\x01\0\x01\x07headers\x01\x01\x05self_\x07headers\0\0\0\x1E__widl_f_array_buffer_Response\x01\0\0\x01\x08Response\x01\0\0\x01\x01\x05self_\x0BarrayBuffer\0\0\0\x16__widl_f_blob_Response\x01\0\0\x01\x08Response\x01\0\0\x01\x01\x05self_\x04blob\0\0\0\x1B__widl_f_form_data_Response\x01\0\0\x01\x08Response\x01\0\0\x01\x01\x05self_\x08formData\0\0\0\x16__widl_f_json_Response\x01\0\0\x01\x08Response\x01\0\0\x01\x01\x05self_\x04json\0\0\0\x16__widl_f_text_Response\x01\0\0\x01\x08Response\x01\0\0\x01\x01\x05self_\x04text\0\0\0\x1B__widl_f_body_used_Response\0\0\0\x01\x08Response\x01\0\x01\x08bodyUsed\x01\x01\x05self_\x08bodyUsed\0\0\0\x16__widl_f_body_Response\0\0\0\x01\x08Response\x01\0\x01\x04body\x01\x01\x05self_\x04body\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
