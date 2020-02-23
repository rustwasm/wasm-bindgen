use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `XMLHttpRequest` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct XmlHttpRequest {
    obj: XmlHttpRequestEventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_XmlHttpRequest: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for XmlHttpRequest {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(88u32);
            inform(77u32);
            inform(76u32);
            inform(72u32);
            inform(116u32);
            inform(116u32);
            inform(112u32);
            inform(82u32);
            inform(101u32);
            inform(113u32);
            inform(117u32);
            inform(101u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for XmlHttpRequest {
        type Target = XmlHttpRequestEventTarget;
        #[inline]
        fn deref(&self) -> &XmlHttpRequestEventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for XmlHttpRequest {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for XmlHttpRequest {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a XmlHttpRequest {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for XmlHttpRequest {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            XmlHttpRequest {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for XmlHttpRequest {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a XmlHttpRequest {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for XmlHttpRequest {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<XmlHttpRequest>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(XmlHttpRequest {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for XmlHttpRequest {
        #[inline]
        fn from(obj: JsValue) -> XmlHttpRequest {
            XmlHttpRequest { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for XmlHttpRequest {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<XmlHttpRequest> for XmlHttpRequest {
        #[inline]
        fn as_ref(&self) -> &XmlHttpRequest {
            self
        }
    }
    impl From<XmlHttpRequest> for JsValue {
        #[inline]
        fn from(obj: XmlHttpRequest) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for XmlHttpRequest {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_XMLHttpRequest(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_XMLHttpRequest(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_XMLHttpRequest(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            XmlHttpRequest { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const XmlHttpRequest) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<XmlHttpRequest> for XmlHttpRequestEventTarget {
    #[inline]
    fn from(obj: XmlHttpRequest) -> XmlHttpRequestEventTarget {
        use wasm_bindgen::JsCast;
        XmlHttpRequestEventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<XmlHttpRequestEventTarget> for XmlHttpRequest {
    #[inline]
    fn as_ref(&self) -> &XmlHttpRequestEventTarget {
        use wasm_bindgen::JsCast;
        XmlHttpRequestEventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<XmlHttpRequest> for EventTarget {
    #[inline]
    fn from(obj: XmlHttpRequest) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for XmlHttpRequest {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<XmlHttpRequest> for ::js_sys::Object {
    #[inline]
    fn from(obj: XmlHttpRequest) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for XmlHttpRequest {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <XmlHttpRequest as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `new XMLHttpRequest(..)` constructor, creating a new instance of `XMLHttpRequest`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/XMLHttpRequest)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<XmlHttpRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_XMLHttpRequest(
            ) -> <XmlHttpRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_XMLHttpRequest(
        ) -> <XmlHttpRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_XMLHttpRequest() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<XmlHttpRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_ignored_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <XmlHttpRequest as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `new XMLHttpRequest(..)` constructor, creating a new instance of `XMLHttpRequest`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/XMLHttpRequest)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn new_with_ignored(ignored: &str) -> Result<XmlHttpRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_ignored_XMLHttpRequest(
                ignored: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <XmlHttpRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_ignored_XMLHttpRequest(
            ignored: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <XmlHttpRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(ignored);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let ignored = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ignored);
                __widl_f_new_with_ignored_XMLHttpRequest(ignored)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<XmlHttpRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_abort_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `abort()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/abort)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn abort(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_abort_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_abort_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_abort_XMLHttpRequest(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_all_response_headers_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `getAllResponseHeaders()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/getAllResponseHeaders)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn get_all_response_headers(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_all_response_headers_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_all_response_headers_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_all_response_headers_XMLHttpRequest(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_response_header_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `getResponseHeader()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/getResponseHeader)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn get_response_header(
        &self,
        header: &str,
    ) -> Result<Option<String>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_response_header_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                header: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_response_header_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            header: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(header);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let header = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(header);
                __widl_f_get_response_header_XMLHttpRequest(self_, header)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `open()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/open)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn open(&self, method: &str, url: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                method: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            method: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(method);
            drop(url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let method = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(method);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                __widl_f_open_XMLHttpRequest(self_, method, url)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_with_async_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `open()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/open)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn open_with_async(
        &self,
        method: &str,
        url: &str,
        r#async: bool,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_with_async_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                method: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                r#async: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_with_async_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            method: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            r#async: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(method);
            drop(url);
            drop(r#async);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let method = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(method);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                let r#async = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(r#async);
                __widl_f_open_with_async_XMLHttpRequest(self_, method, url, r#async)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_with_async_and_user_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `open()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/open)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn open_with_async_and_user(
        &self,
        method: &str,
        url: &str,
        r#async: bool,
        user: Option<&str>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_with_async_and_user_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                method: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                r#async: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                user: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_with_async_and_user_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            method: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            r#async: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            user: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(method);
            drop(url);
            drop(r#async);
            drop(user);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let method = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(method);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                let r#async = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(r#async);
                let user = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(user);
                __widl_f_open_with_async_and_user_XMLHttpRequest(self_, method, url, r#async, user)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_with_async_and_user_and_password_XMLHttpRequest(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `open()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/open)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn open_with_async_and_user_and_password(
        &self,
        method: &str,
        url: &str,
        r#async: bool,
        user: Option<&str>,
        password: Option<&str>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_with_async_and_user_and_password_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                method: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                r#async: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                user: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                password: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_with_async_and_user_and_password_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            method: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            r#async: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            user: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            password: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(method);
            drop(url);
            drop(r#async);
            drop(user);
            drop(password);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let method = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(method);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                let r#async = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(r#async);
                let user = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(user);
                let password =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(password);
                __widl_f_open_with_async_and_user_and_password_XMLHttpRequest(
                    self_, method, url, r#async, user, password,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_override_mime_type_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `overrideMimeType()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/overrideMimeType)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn override_mime_type(&self, mime: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_override_mime_type_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mime: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_override_mime_type_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mime: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(mime);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let mime = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mime);
                __widl_f_override_mime_type_XMLHttpRequest(self_, mime)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn send(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_send_XMLHttpRequest(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document", feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_opt_document_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <Option<&Document> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "Document", feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)\n\n*This API requires the following crate features to be activated: `Document`, `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn send_with_opt_document(
        &self,
        body: Option<&Document>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_opt_document_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                body: <Option<&Document> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_opt_document_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            body: <Option<&Document> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(body);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let body =
                    <Option<&Document> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(body);
                __widl_f_send_with_opt_document_XMLHttpRequest(self_, body)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Blob", feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_opt_blob_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <Option<&Blob> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "Blob", feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)\n\n*This API requires the following crate features to be activated: `Blob`, `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn send_with_opt_blob(&self, body: Option<&Blob>) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_opt_blob_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                body: <Option<&Blob> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_opt_blob_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            body: <Option<&Blob> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(body);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let body = <Option<&Blob> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(body);
                __widl_f_send_with_opt_blob_XMLHttpRequest(self_, body)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_opt_buffer_source_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <Option<&::js_sys::Object> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn send_with_opt_buffer_source(
        &self,
        body: Option<&::js_sys::Object>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_opt_buffer_source_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                body: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_opt_buffer_source_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            body: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(body);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let body =
                    <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        body,
                    );
                __widl_f_send_with_opt_buffer_source_XMLHttpRequest(self_, body)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_opt_u8_array_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <Option<&mut [u8]> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn send_with_opt_u8_array(
        &self,
        body: Option<&mut [u8]>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_opt_u8_array_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                body: <Option<&mut [u8]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_opt_u8_array_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            body: <Option<&mut [u8]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(body);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let body =
                    <Option<&mut [u8]> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(body);
                __widl_f_send_with_opt_u8_array_XMLHttpRequest(self_, body)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "FormData", feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_opt_form_data_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <Option<&FormData> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "FormData", feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)\n\n*This API requires the following crate features to be activated: `FormData`, `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn send_with_opt_form_data(
        &self,
        body: Option<&FormData>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FormData", feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_opt_form_data_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                body: <Option<&FormData> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_opt_form_data_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            body: <Option<&FormData> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(body);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let body =
                    <Option<&FormData> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(body);
                __widl_f_send_with_opt_form_data_XMLHttpRequest(self_, body)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "UrlSearchParams", feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_opt_url_search_params_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <Option<&UrlSearchParams> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "UrlSearchParams", feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`, `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn send_with_opt_url_search_params(
        &self,
        body: Option<&UrlSearchParams>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "UrlSearchParams", feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_opt_url_search_params_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                body: <Option<&UrlSearchParams> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_opt_url_search_params_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            body: <Option<&UrlSearchParams> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(body);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let body =
                    <Option<&UrlSearchParams> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        body,
                    );
                __widl_f_send_with_opt_url_search_params_XMLHttpRequest(self_, body)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_opt_str_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn send_with_opt_str(&self, body: Option<&str>) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_opt_str_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                body: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_opt_str_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            body: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(body);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let body = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(body);
                __widl_f_send_with_opt_str_XMLHttpRequest(self_, body)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "ReadableStream", feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_opt_readable_stream_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <Option<&ReadableStream> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "ReadableStream", feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)\n\n*This API requires the following crate features to be activated: `ReadableStream`, `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn send_with_opt_readable_stream(
        &self,
        body: Option<&ReadableStream>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ReadableStream", feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_opt_readable_stream_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                body: <Option<&ReadableStream> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_opt_readable_stream_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            body: <Option<&ReadableStream> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(body);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let body =
                    <Option<&ReadableStream> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(body);
                __widl_f_send_with_opt_readable_stream_XMLHttpRequest(self_, body)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_request_header_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `setRequestHeader()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/setRequestHeader)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn set_request_header(
        &self,
        header: &str,
        value: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_request_header_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                header: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_request_header_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            header: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(header);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let header = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(header);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_set_request_header_XMLHttpRequest(self_, header, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onreadystatechange_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `onreadystatechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/onreadystatechange)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn onreadystatechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onreadystatechange_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onreadystatechange_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onreadystatechange_XMLHttpRequest(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onreadystatechange_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `onreadystatechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/onreadystatechange)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn set_onreadystatechange(&self, onreadystatechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onreadystatechange_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onreadystatechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onreadystatechange_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onreadystatechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onreadystatechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onreadystatechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onreadystatechange,
                    );
                __widl_f_set_onreadystatechange_XMLHttpRequest(self_, onreadystatechange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ready_state_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `readyState` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/readyState)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn ready_state(&self) -> u16 {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ready_state_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ready_state_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ready_state_XMLHttpRequest(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_timeout_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `timeout` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/timeout)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn timeout(&self) -> u32 {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_timeout_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_timeout_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_timeout_XMLHttpRequest(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeout_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `timeout` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/timeout)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn set_timeout(&self, timeout: u32) {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeout_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeout_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(timeout);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let timeout = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                __widl_f_set_timeout_XMLHttpRequest(self_, timeout)
            };
            ()
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_with_credentials_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `withCredentials` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/withCredentials)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn with_credentials(&self) -> bool {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_with_credentials_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_with_credentials_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_with_credentials_XMLHttpRequest(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_with_credentials_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `withCredentials` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/withCredentials)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn set_with_credentials(&self, with_credentials: bool) {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_with_credentials_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                with_credentials: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_with_credentials_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            with_credentials: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(with_credentials);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let with_credentials =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(with_credentials);
                __widl_f_set_with_credentials_XMLHttpRequest(self_, with_credentials)
            };
            ()
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest", feature = "XmlHttpRequestUpload",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_upload_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <XmlHttpRequestUpload as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest", feature = "XmlHttpRequestUpload",))]
    #[allow(bad_style)]
    #[doc = "The `upload` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/upload)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`, `XmlHttpRequestUpload`*"]
    #[allow(clippy::all)]
    pub fn upload(&self) -> Result<XmlHttpRequestUpload, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlHttpRequest", feature = "XmlHttpRequestUpload",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_upload_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <XmlHttpRequestUpload as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_upload_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <XmlHttpRequestUpload as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_upload_XMLHttpRequest(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<XmlHttpRequestUpload as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_response_url_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `responseURL` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseURL)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn response_url(&self) -> String {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_response_url_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_response_url_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_response_url_XMLHttpRequest(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_status_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `status` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/status)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn status(&self) -> Result<u16, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_status_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_status_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_status_XMLHttpRequest(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_status_text_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `statusText` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/statusText)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn status_text(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_status_text_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_status_text_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_status_text_XMLHttpRequest(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest", feature = "XmlHttpRequestResponseType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_response_type_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <XmlHttpRequestResponseType as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest", feature = "XmlHttpRequestResponseType",))]
    #[allow(bad_style)]
    #[doc = "The `responseType` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseType)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`, `XmlHttpRequestResponseType`*"]
    #[allow(clippy::all)]
    pub fn response_type(&self) -> XmlHttpRequestResponseType {
        #[cfg(all(feature = "XmlHttpRequest", feature = "XmlHttpRequestResponseType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_response_type_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <XmlHttpRequestResponseType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_response_type_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <XmlHttpRequestResponseType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_response_type_XMLHttpRequest(self_)
            };
            <XmlHttpRequestResponseType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest", feature = "XmlHttpRequestResponseType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_response_type_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <XmlHttpRequestResponseType as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest", feature = "XmlHttpRequestResponseType",))]
    #[allow(bad_style)]
    #[doc = "The `responseType` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseType)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`, `XmlHttpRequestResponseType`*"]
    #[allow(clippy::all)]
    pub fn set_response_type(&self, response_type: XmlHttpRequestResponseType) {
        #[cfg(all(feature = "XmlHttpRequest", feature = "XmlHttpRequestResponseType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_response_type_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                response_type : < XmlHttpRequestResponseType as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_response_type_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            response_type: <XmlHttpRequestResponseType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(response_type);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let response_type =
                    <XmlHttpRequestResponseType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        response_type,
                    );
                __widl_f_set_response_type_XMLHttpRequest(self_, response_type)
            };
            ()
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_response_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `response` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/response)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn response(&self) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_response_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_response_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_response_XMLHttpRequest(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_response_text_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `responseText` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseText)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn response_text(&self) -> Result<Option<String>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_response_text_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_response_text_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_response_text_XMLHttpRequest(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Document", feature = "XmlHttpRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_response_xml_XMLHttpRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequest as WasmDescribe>::describe();
    <Option<Document> as WasmDescribe>::describe();
}
impl XmlHttpRequest {
    #[cfg(all(feature = "Document", feature = "XmlHttpRequest",))]
    #[allow(bad_style)]
    #[doc = "The `responseXML` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseXML)\n\n*This API requires the following crate features to be activated: `Document`, `XmlHttpRequest`*"]
    #[allow(clippy::all)]
    pub fn response_xml(&self) -> Result<Option<Document>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "XmlHttpRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_response_xml_XMLHttpRequest(
                self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Document> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_response_xml_XMLHttpRequest(
            self_: <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Document> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlHttpRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_response_xml_XMLHttpRequest(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Document> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
impl XmlHttpRequest {
    pub const UNSENT: u16 = 0i64 as u16;
}
impl XmlHttpRequest {
    pub const OPENED: u16 = 1u64 as u16;
}
impl XmlHttpRequest {
    pub const HEADERS_RECEIVED: u16 = 2u64 as u16;
}
impl XmlHttpRequest {
    pub const LOADING: u16 = 3u64 as u16;
}
impl XmlHttpRequest {
    pub const DONE: u16 = 4u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_68c0b22aff3fe4a0: [u8; 3567usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xAD\r\0\0\0\0%\0\0\x02\x0EXMLHttpRequest __widl_instanceof_XMLHttpRequest\0\0\0\0\x1B__widl_f_new_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\0\x01\0\x03new\0\0\0(__widl_f_new_with_ignored_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\0\x01\x01\x07ignored\x03new\0\0\0\x1D__widl_f_abort_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\0\x01\x01\x05self_\x05abort\0\0\00__widl_f_get_all_response_headers_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\0\x01\x01\x05self_\x15getAllResponseHeaders\0\0\0+__widl_f_get_response_header_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\0\x01\x02\x05self_\x06header\x11getResponseHeader\0\0\0\x1C__widl_f_open_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\0\x01\x03\x05self_\x06method\x03url\x04open\0\0\0'__widl_f_open_with_async_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\0\x01\x04\x05self_\x06method\x03url\x07r#async\x04open\0\0\00__widl_f_open_with_async_and_user_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\0\x01\x05\x05self_\x06method\x03url\x07r#async\x04user\x04open\0\0\0=__widl_f_open_with_async_and_user_and_password_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\0\x01\x06\x05self_\x06method\x03url\x07r#async\x04user\x08password\x04open\0\0\0*__widl_f_override_mime_type_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\0\x01\x02\x05self_\x04mime\x10overrideMimeType\0\0\0\x1C__widl_f_send_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\0\x01\x01\x05self_\x04send\0\0\0.__widl_f_send_with_opt_document_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\0\x01\x02\x05self_\x04body\x04send\0\0\0*__widl_f_send_with_opt_blob_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\0\x01\x02\x05self_\x04body\x04send\0\0\03__widl_f_send_with_opt_buffer_source_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\0\x01\x02\x05self_\x04body\x04send\0\0\0.__widl_f_send_with_opt_u8_array_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\0\x01\x02\x05self_\x04body\x04send\0\0\0/__widl_f_send_with_opt_form_data_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\0\x01\x02\x05self_\x04body\x04send\0\0\07__widl_f_send_with_opt_url_search_params_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\0\x01\x02\x05self_\x04body\x04send\0\0\0)__widl_f_send_with_opt_str_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\0\x01\x02\x05self_\x04body\x04send\0\0\05__widl_f_send_with_opt_readable_stream_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\0\x01\x02\x05self_\x04body\x04send\0\0\0*__widl_f_set_request_header_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\0\x01\x03\x05self_\x06header\x05value\x10setRequestHeader\0\0\0*__widl_f_onreadystatechange_XMLHttpRequest\0\0\0\x01\x0EXMLHttpRequest\x01\0\x01\x12onreadystatechange\x01\x01\x05self_\x12onreadystatechange\0\0\0.__widl_f_set_onreadystatechange_XMLHttpRequest\0\0\0\x01\x0EXMLHttpRequest\x01\0\x02\x12onreadystatechange\x01\x02\x05self_\x12onreadystatechange\x12onreadystatechange\0\0\0#__widl_f_ready_state_XMLHttpRequest\0\0\0\x01\x0EXMLHttpRequest\x01\0\x01\nreadyState\x01\x01\x05self_\nreadyState\0\0\0\x1F__widl_f_timeout_XMLHttpRequest\0\0\0\x01\x0EXMLHttpRequest\x01\0\x01\x07timeout\x01\x01\x05self_\x07timeout\0\0\0#__widl_f_set_timeout_XMLHttpRequest\0\0\0\x01\x0EXMLHttpRequest\x01\0\x02\x07timeout\x01\x02\x05self_\x07timeout\x07timeout\0\0\0(__widl_f_with_credentials_XMLHttpRequest\0\0\0\x01\x0EXMLHttpRequest\x01\0\x01\x0FwithCredentials\x01\x01\x05self_\x0FwithCredentials\0\0\0,__widl_f_set_with_credentials_XMLHttpRequest\0\0\0\x01\x0EXMLHttpRequest\x01\0\x02\x0FwithCredentials\x01\x02\x05self_\x10with_credentials\x0FwithCredentials\0\0\0\x1E__widl_f_upload_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\x01\x06upload\x01\x01\x05self_\x06upload\0\0\0$__widl_f_response_url_XMLHttpRequest\0\0\0\x01\x0EXMLHttpRequest\x01\0\x01\x0BresponseURL\x01\x01\x05self_\x0BresponseURL\0\0\0\x1E__widl_f_status_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\x01\x06status\x01\x01\x05self_\x06status\0\0\0#__widl_f_status_text_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\x01\nstatusText\x01\x01\x05self_\nstatusText\0\0\0%__widl_f_response_type_XMLHttpRequest\0\0\0\x01\x0EXMLHttpRequest\x01\0\x01\x0CresponseType\x01\x01\x05self_\x0CresponseType\0\0\0)__widl_f_set_response_type_XMLHttpRequest\0\0\0\x01\x0EXMLHttpRequest\x01\0\x02\x0CresponseType\x01\x02\x05self_\rresponse_type\x0CresponseType\0\0\0 __widl_f_response_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\x01\x08response\x01\x01\x05self_\x08response\0\0\0%__widl_f_response_text_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\x01\x0CresponseText\x01\x01\x05self_\x0CresponseText\0\0\0$__widl_f_response_xml_XMLHttpRequest\x01\0\0\x01\x0EXMLHttpRequest\x01\0\x01\x0BresponseXML\x01\x01\x05self_\x0BresponseXML\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
