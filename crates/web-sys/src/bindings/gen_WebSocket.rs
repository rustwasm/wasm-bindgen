use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WebSocket` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WebSocket {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WebSocket: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WebSocket {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(87u32);
            inform(101u32);
            inform(98u32);
            inform(83u32);
            inform(111u32);
            inform(99u32);
            inform(107u32);
            inform(101u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for WebSocket {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for WebSocket {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WebSocket {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WebSocket {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WebSocket {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WebSocket {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WebSocket {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WebSocket {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WebSocket {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WebSocket>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WebSocket {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WebSocket {
        #[inline]
        fn from(obj: JsValue) -> WebSocket {
            WebSocket { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WebSocket {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WebSocket> for WebSocket {
        #[inline]
        fn as_ref(&self) -> &WebSocket {
            self
        }
    }
    impl From<WebSocket> for JsValue {
        #[inline]
        fn from(obj: WebSocket) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WebSocket {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WebSocket(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WebSocket(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WebSocket(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WebSocket { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WebSocket) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WebSocket> for EventTarget {
    #[inline]
    fn from(obj: WebSocket) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for WebSocket {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<WebSocket> for ::js_sys::Object {
    #[inline]
    fn from(obj: WebSocket) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WebSocket {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <WebSocket as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `new WebSocket(..)` constructor, creating a new instance of `WebSocket`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn new(url: &str) -> Result<WebSocket, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_WebSocket(
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebSocket as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_WebSocket(
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebSocket as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                __widl_f_new_WebSocket(url)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<WebSocket as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_str_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <WebSocket as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `new WebSocket(..)` constructor, creating a new instance of `WebSocket`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn new_with_str(url: &str, protocols: &str) -> Result<WebSocket, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_str_WebSocket(
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                protocols: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebSocket as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_str_WebSocket(
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            protocols: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebSocket as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(url);
            drop(protocols);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                let protocols = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(protocols);
                __widl_f_new_with_str_WebSocket(url, protocols)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<WebSocket as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_str_sequence_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <WebSocket as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `new WebSocket(..)` constructor, creating a new instance of `WebSocket`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn new_with_str_sequence(
        url: &str,
        protocols: &::wasm_bindgen::JsValue,
    ) -> Result<WebSocket, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_str_sequence_WebSocket(
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                protocols: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebSocket as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_str_sequence_WebSocket(
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            protocols: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebSocket as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(url);
            drop(protocols);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                let protocols =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        protocols,
                    );
                __widl_f_new_with_str_sequence_WebSocket(url, protocols)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<WebSocket as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebSocket as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/close)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn close(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_close_WebSocket(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_with_code_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebSocket as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/close)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn close_with_code(&self, code: u16) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_with_code_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                code: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_with_code_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            code: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(code);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let code = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(code);
                __widl_f_close_with_code_WebSocket(self_, code)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_with_code_and_reason_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebSocket as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/close)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn close_with_code_and_reason(
        &self,
        code: u16,
        reason: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_with_code_and_reason_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                code: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                reason: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_with_code_and_reason_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            code: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            reason: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(code);
            drop(reason);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let code = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(code);
                let reason = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(reason);
                __widl_f_close_with_code_and_reason_WebSocket(self_, code, reason)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_str_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebSocket as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn send_with_str(&self, data: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_str_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_str_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_with_str_WebSocket(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Blob", feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_blob_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebSocket as WasmDescribe>::describe();
    <&Blob as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "Blob", feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)\n\n*This API requires the following crate features to be activated: `Blob`, `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn send_with_blob(&self, data: &Blob) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_blob_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_blob_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_with_blob_WebSocket(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_array_buffer_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebSocket as WasmDescribe>::describe();
    <&::js_sys::ArrayBuffer as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn send_with_array_buffer(
        &self,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_array_buffer_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_array_buffer_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data =
                    <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_with_array_buffer_WebSocket(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_array_buffer_view_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebSocket as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn send_with_array_buffer_view(
        &self,
        data: &::js_sys::Object,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_array_buffer_view_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_array_buffer_view_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_with_array_buffer_view_WebSocket(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_u8_array_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebSocket as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn send_with_u8_array(&self, data: &mut [u8]) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_u8_array_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_u8_array_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_with_u8_array_WebSocket(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_url_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebSocket as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `url` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/url)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn url(&self) -> String {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_url_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_url_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_url_WebSocket(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ready_state_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebSocket as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `readyState` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/readyState)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn ready_state(&self) -> u16 {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ready_state_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ready_state_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ready_state_WebSocket(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buffered_amount_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebSocket as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `bufferedAmount` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/bufferedAmount)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn buffered_amount(&self) -> u32 {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buffered_amount_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buffered_amount_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_buffered_amount_WebSocket(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onopen_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebSocket as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `onopen` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onopen)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn onopen(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onopen_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onopen_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onopen_WebSocket(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onopen_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebSocket as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `onopen` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onopen)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn set_onopen(&self, onopen: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onopen_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onopen: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onopen_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onopen: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onopen);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onopen =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onopen,
                    );
                __widl_f_set_onopen_WebSocket(self_, onopen)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebSocket as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onerror)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_WebSocket(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebSocket as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onerror)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onerror);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_WebSocket(self_, onerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onclose_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebSocket as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `onclose` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onclose)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn onclose(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onclose_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onclose_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onclose_WebSocket(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onclose_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebSocket as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `onclose` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onclose)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn set_onclose(&self, onclose: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onclose_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onclose: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onclose_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onclose: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onclose);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onclose =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onclose,
                    );
                __widl_f_set_onclose_WebSocket(self_, onclose)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_extensions_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebSocket as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `extensions` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/extensions)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn extensions(&self) -> String {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_extensions_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_extensions_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_extensions_WebSocket(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_protocol_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebSocket as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `protocol` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/protocol)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn protocol(&self) -> String {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_protocol_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_protocol_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_protocol_WebSocket(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmessage_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebSocket as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onmessage)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn onmessage(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmessage_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmessage_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmessage_WebSocket(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmessage_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebSocket as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onmessage)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn set_onmessage(&self, onmessage: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmessage_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmessage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmessage_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmessage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmessage);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmessage =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmessage,
                    );
                __widl_f_set_onmessage_WebSocket(self_, onmessage)
            };
            ()
        }
    }
}
#[cfg(all(feature = "BinaryType", feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_binary_type_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebSocket as WasmDescribe>::describe();
    <BinaryType as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "BinaryType", feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `binaryType` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/binaryType)\n\n*This API requires the following crate features to be activated: `BinaryType`, `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn binary_type(&self) -> BinaryType {
        #[cfg(all(feature = "BinaryType", feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_binary_type_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <BinaryType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_binary_type_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <BinaryType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_binary_type_WebSocket(self_)
            };
            <BinaryType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "BinaryType", feature = "WebSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_binary_type_WebSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebSocket as WasmDescribe>::describe();
    <BinaryType as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebSocket {
    #[cfg(all(feature = "BinaryType", feature = "WebSocket",))]
    #[allow(bad_style)]
    #[doc = "The `binaryType` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/binaryType)\n\n*This API requires the following crate features to be activated: `BinaryType`, `WebSocket`*"]
    #[allow(clippy::all)]
    pub fn set_binary_type(&self, binary_type: BinaryType) {
        #[cfg(all(feature = "BinaryType", feature = "WebSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_binary_type_WebSocket(
                self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                binary_type: <BinaryType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_binary_type_WebSocket(
            self_: <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            binary_type: <BinaryType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(binary_type);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WebSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let binary_type =
                    <BinaryType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(binary_type);
                __widl_f_set_binary_type_WebSocket(self_, binary_type)
            };
            ()
        }
    }
}
impl WebSocket {
    pub const CONNECTING: u16 = 0i64 as u16;
}
impl WebSocket {
    pub const OPEN: u16 = 1u64 as u16;
}
impl WebSocket {
    pub const CLOSING: u16 = 2u64 as u16;
}
impl WebSocket {
    pub const CLOSED: u16 = 3u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_ed6bfa8bfea8a902: [u8; 2144usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1E\x08\0\0\0\0\x1B\0\0\x02\tWebSocket\x1B__widl_instanceof_WebSocket\0\0\0\0\x16__widl_f_new_WebSocket\x01\0\0\x01\tWebSocket\0\x01\x01\x03url\x03new\0\0\0\x1F__widl_f_new_with_str_WebSocket\x01\0\0\x01\tWebSocket\0\x01\x02\x03url\tprotocols\x03new\0\0\0(__widl_f_new_with_str_sequence_WebSocket\x01\0\0\x01\tWebSocket\0\x01\x02\x03url\tprotocols\x03new\0\0\0\x18__widl_f_close_WebSocket\x01\0\0\x01\tWebSocket\x01\0\0\x01\x01\x05self_\x05close\0\0\0\"__widl_f_close_with_code_WebSocket\x01\0\0\x01\tWebSocket\x01\0\0\x01\x02\x05self_\x04code\x05close\0\0\0-__widl_f_close_with_code_and_reason_WebSocket\x01\0\0\x01\tWebSocket\x01\0\0\x01\x03\x05self_\x04code\x06reason\x05close\0\0\0 __widl_f_send_with_str_WebSocket\x01\0\0\x01\tWebSocket\x01\0\0\x01\x02\x05self_\x04data\x04send\0\0\0!__widl_f_send_with_blob_WebSocket\x01\0\0\x01\tWebSocket\x01\0\0\x01\x02\x05self_\x04data\x04send\0\0\0)__widl_f_send_with_array_buffer_WebSocket\x01\0\0\x01\tWebSocket\x01\0\0\x01\x02\x05self_\x04data\x04send\0\0\0.__widl_f_send_with_array_buffer_view_WebSocket\x01\0\0\x01\tWebSocket\x01\0\0\x01\x02\x05self_\x04data\x04send\0\0\0%__widl_f_send_with_u8_array_WebSocket\x01\0\0\x01\tWebSocket\x01\0\0\x01\x02\x05self_\x04data\x04send\0\0\0\x16__widl_f_url_WebSocket\0\0\0\x01\tWebSocket\x01\0\x01\x03url\x01\x01\x05self_\x03url\0\0\0\x1E__widl_f_ready_state_WebSocket\0\0\0\x01\tWebSocket\x01\0\x01\nreadyState\x01\x01\x05self_\nreadyState\0\0\0\"__widl_f_buffered_amount_WebSocket\0\0\0\x01\tWebSocket\x01\0\x01\x0EbufferedAmount\x01\x01\x05self_\x0EbufferedAmount\0\0\0\x19__widl_f_onopen_WebSocket\0\0\0\x01\tWebSocket\x01\0\x01\x06onopen\x01\x01\x05self_\x06onopen\0\0\0\x1D__widl_f_set_onopen_WebSocket\0\0\0\x01\tWebSocket\x01\0\x02\x06onopen\x01\x02\x05self_\x06onopen\x06onopen\0\0\0\x1A__widl_f_onerror_WebSocket\0\0\0\x01\tWebSocket\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0\x1E__widl_f_set_onerror_WebSocket\0\0\0\x01\tWebSocket\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0\x1A__widl_f_onclose_WebSocket\0\0\0\x01\tWebSocket\x01\0\x01\x07onclose\x01\x01\x05self_\x07onclose\0\0\0\x1E__widl_f_set_onclose_WebSocket\0\0\0\x01\tWebSocket\x01\0\x02\x07onclose\x01\x02\x05self_\x07onclose\x07onclose\0\0\0\x1D__widl_f_extensions_WebSocket\0\0\0\x01\tWebSocket\x01\0\x01\nextensions\x01\x01\x05self_\nextensions\0\0\0\x1B__widl_f_protocol_WebSocket\0\0\0\x01\tWebSocket\x01\0\x01\x08protocol\x01\x01\x05self_\x08protocol\0\0\0\x1C__widl_f_onmessage_WebSocket\0\0\0\x01\tWebSocket\x01\0\x01\tonmessage\x01\x01\x05self_\tonmessage\0\0\0 __widl_f_set_onmessage_WebSocket\0\0\0\x01\tWebSocket\x01\0\x02\tonmessage\x01\x02\x05self_\tonmessage\tonmessage\0\0\0\x1E__widl_f_binary_type_WebSocket\0\0\0\x01\tWebSocket\x01\0\x01\nbinaryType\x01\x01\x05self_\nbinaryType\0\0\0\"__widl_f_set_binary_type_WebSocket\0\0\0\x01\tWebSocket\x01\0\x02\nbinaryType\x01\x02\x05self_\x0Bbinary_type\nbinaryType\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
