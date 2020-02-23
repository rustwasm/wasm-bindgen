use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `TCPSocket` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct TcpSocket {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_TcpSocket: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for TcpSocket {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(84u32);
            inform(67u32);
            inform(80u32);
            inform(83u32);
            inform(111u32);
            inform(99u32);
            inform(107u32);
            inform(101u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for TcpSocket {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for TcpSocket {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for TcpSocket {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a TcpSocket {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for TcpSocket {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            TcpSocket {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for TcpSocket {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a TcpSocket {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for TcpSocket {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<TcpSocket>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(TcpSocket {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for TcpSocket {
        #[inline]
        fn from(obj: JsValue) -> TcpSocket {
            TcpSocket { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for TcpSocket {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<TcpSocket> for TcpSocket {
        #[inline]
        fn as_ref(&self) -> &TcpSocket {
            self
        }
    }
    impl From<TcpSocket> for JsValue {
        #[inline]
        fn from(obj: TcpSocket) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for TcpSocket {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_TCPSocket(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_TCPSocket(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_TCPSocket(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            TcpSocket { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const TcpSocket) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<TcpSocket> for EventTarget {
    #[inline]
    fn from(obj: TcpSocket) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for TcpSocket {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<TcpSocket> for ::js_sys::Object {
    #[inline]
    fn from(obj: TcpSocket) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for TcpSocket {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <TcpSocket as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `new TCPSocket(..)` constructor, creating a new instance of `TCPSocket`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/TCPSocket)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn new(host: &str, port: u16) -> Result<TcpSocket, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_TCPSocket(
                host: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                port: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TcpSocket as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_TCPSocket(
            host: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            port: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TcpSocket as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(host);
            drop(port);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let host = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(host);
                let port = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(port);
                __widl_f_new_TCPSocket(host, port)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TcpSocket as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "SocketOptions", feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_options_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&str as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <&SocketOptions as WasmDescribe>::describe();
    <TcpSocket as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "SocketOptions", feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `new TCPSocket(..)` constructor, creating a new instance of `TCPSocket`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/TCPSocket)\n\n*This API requires the following crate features to be activated: `SocketOptions`, `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn new_with_options(
        host: &str,
        port: u16,
        options: &SocketOptions,
    ) -> Result<TcpSocket, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SocketOptions", feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_options_TCPSocket(
                host: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                port: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&SocketOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TcpSocket as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_options_TCPSocket(
            host: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            port: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&SocketOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TcpSocket as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(host);
            drop(port);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let host = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(host);
                let port = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(port);
                let options =
                    <&SocketOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_options_TCPSocket(host, port, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TcpSocket as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpSocket as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/close)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn close(&self) {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_close_TCPSocket(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_resume_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpSocket as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `resume()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/resume)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn resume(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_resume_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_resume_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_resume_TCPSocket(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_str_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TcpSocket as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/send)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn send_with_str(&self, data: &str) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_str_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_str_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_with_str_TCPSocket(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_array_buffer_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TcpSocket as WasmDescribe>::describe();
    <&::js_sys::ArrayBuffer as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/send)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn send_with_array_buffer(
        &self,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_array_buffer_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_array_buffer_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data =
                    <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_with_array_buffer_TCPSocket(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_array_buffer_and_byte_offset_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&TcpSocket as WasmDescribe>::describe();
    <&::js_sys::ArrayBuffer as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/send)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn send_with_array_buffer_and_byte_offset(
        &self,
        data: &::js_sys::ArrayBuffer,
        byte_offset: u32,
    ) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_array_buffer_and_byte_offset_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                byte_offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_array_buffer_and_byte_offset_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            byte_offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(data);
            drop(byte_offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data =
                    <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                let byte_offset =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(byte_offset);
                __widl_f_send_with_array_buffer_and_byte_offset_TCPSocket(self_, data, byte_offset)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_array_buffer_and_byte_offset_and_byte_length_TCPSocket(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&TcpSocket as WasmDescribe>::describe();
    <&::js_sys::ArrayBuffer as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/send)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn send_with_array_buffer_and_byte_offset_and_byte_length(
        &self,
        data: &::js_sys::ArrayBuffer,
        byte_offset: u32,
        byte_length: u32,
    ) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_array_buffer_and_byte_offset_and_byte_length_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                byte_offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                byte_length: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_array_buffer_and_byte_offset_and_byte_length_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            byte_offset: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            byte_length: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(data);
            drop(byte_offset);
            drop(byte_length);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data =
                    <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                let byte_offset =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(byte_offset);
                let byte_length =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(byte_length);
                __widl_f_send_with_array_buffer_and_byte_offset_and_byte_length_TCPSocket(
                    self_,
                    data,
                    byte_offset,
                    byte_length,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_suspend_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpSocket as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `suspend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/suspend)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn suspend(&self) {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_suspend_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_suspend_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_suspend_TCPSocket(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_upgrade_to_secure_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpSocket as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `upgradeToSecure()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/upgradeToSecure)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn upgrade_to_secure(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_upgrade_to_secure_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_upgrade_to_secure_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_upgrade_to_secure_TCPSocket(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_host_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpSocket as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `host` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/host)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn host(&self) -> String {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_host_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_host_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_host_TCPSocket(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_port_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpSocket as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `port` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/port)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn port(&self) -> u16 {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_port_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_port_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_port_TCPSocket(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ssl_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpSocket as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `ssl` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ssl)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn ssl(&self) -> bool {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ssl_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ssl_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ssl_TCPSocket(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buffered_amount_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpSocket as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `bufferedAmount` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/bufferedAmount)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn buffered_amount(&self) -> f64 {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buffered_amount_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buffered_amount_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_buffered_amount_TCPSocket(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TcpReadyState", feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ready_state_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpSocket as WasmDescribe>::describe();
    <TcpReadyState as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpReadyState", feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `readyState` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/readyState)\n\n*This API requires the following crate features to be activated: `TcpReadyState`, `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn ready_state(&self) -> TcpReadyState {
        #[cfg(all(feature = "TcpReadyState", feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ready_state_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TcpReadyState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ready_state_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TcpReadyState as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ready_state_TCPSocket(self_)
            };
            <TcpReadyState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TcpSocket", feature = "TcpSocketBinaryType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_binary_type_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpSocket as WasmDescribe>::describe();
    <TcpSocketBinaryType as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket", feature = "TcpSocketBinaryType",))]
    #[allow(bad_style)]
    #[doc = "The `binaryType` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/binaryType)\n\n*This API requires the following crate features to be activated: `TcpSocket`, `TcpSocketBinaryType`*"]
    #[allow(clippy::all)]
    pub fn binary_type(&self) -> TcpSocketBinaryType {
        #[cfg(all(feature = "TcpSocket", feature = "TcpSocketBinaryType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_binary_type_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TcpSocketBinaryType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_binary_type_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TcpSocketBinaryType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_binary_type_TCPSocket(self_)
            };
            <TcpSocketBinaryType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onopen_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpSocket as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `onopen` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onopen)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn onopen(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onopen_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onopen_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onopen_TCPSocket(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onopen_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TcpSocket as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `onopen` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onopen)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn set_onopen(&self, onopen: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onopen_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onopen: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onopen_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onopen =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onopen,
                    );
                __widl_f_set_onopen_TCPSocket(self_, onopen)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondrain_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpSocket as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `ondrain` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ondrain)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn ondrain(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondrain_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondrain_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondrain_TCPSocket(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondrain_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TcpSocket as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `ondrain` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ondrain)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn set_ondrain(&self, ondrain: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondrain_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondrain: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondrain_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondrain: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondrain);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondrain =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondrain,
                    );
                __widl_f_set_ondrain_TCPSocket(self_, ondrain)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondata_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpSocket as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `ondata` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ondata)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn ondata(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondata_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondata_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondata_TCPSocket(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondata_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TcpSocket as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `ondata` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ondata)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn set_ondata(&self, ondata: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondata_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondata: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondata_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondata: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondata);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondata =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondata,
                    );
                __widl_f_set_ondata_TCPSocket(self_, ondata)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpSocket as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onerror)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_TCPSocket(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TcpSocket as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onerror)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_TCPSocket(self_, onerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onclose_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpSocket as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `onclose` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onclose)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn onclose(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onclose_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onclose_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onclose_TCPSocket(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onclose_TCPSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TcpSocket as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TcpSocket {
    #[cfg(all(feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `onclose` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onclose)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn set_onclose(&self, onclose: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onclose_TCPSocket(
                self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onclose: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onclose_TCPSocket(
            self_: <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&TcpSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onclose =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onclose,
                    );
                __widl_f_set_onclose_TCPSocket(self_, onclose)
            };
            ()
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_45390f1addbd1d8d: [u8; 2135usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x15\x08\0\0\0\0\x1B\0\0\x02\tTCPSocket\x1B__widl_instanceof_TCPSocket\0\0\0\0\x16__widl_f_new_TCPSocket\x01\0\0\x01\tTCPSocket\0\x01\x02\x04host\x04port\x03new\0\0\0#__widl_f_new_with_options_TCPSocket\x01\0\0\x01\tTCPSocket\0\x01\x03\x04host\x04port\x07options\x03new\0\0\0\x18__widl_f_close_TCPSocket\0\0\0\x01\tTCPSocket\x01\0\0\x01\x01\x05self_\x05close\0\0\0\x19__widl_f_resume_TCPSocket\x01\0\0\x01\tTCPSocket\x01\0\0\x01\x01\x05self_\x06resume\0\0\0 __widl_f_send_with_str_TCPSocket\x01\0\0\x01\tTCPSocket\x01\0\0\x01\x02\x05self_\x04data\x04send\0\0\0)__widl_f_send_with_array_buffer_TCPSocket\x01\0\0\x01\tTCPSocket\x01\0\0\x01\x02\x05self_\x04data\x04send\0\0\09__widl_f_send_with_array_buffer_and_byte_offset_TCPSocket\x01\0\0\x01\tTCPSocket\x01\0\0\x01\x03\x05self_\x04data\x0Bbyte_offset\x04send\0\0\0I__widl_f_send_with_array_buffer_and_byte_offset_and_byte_length_TCPSocket\x01\0\0\x01\tTCPSocket\x01\0\0\x01\x04\x05self_\x04data\x0Bbyte_offset\x0Bbyte_length\x04send\0\0\0\x1A__widl_f_suspend_TCPSocket\0\0\0\x01\tTCPSocket\x01\0\0\x01\x01\x05self_\x07suspend\0\0\0$__widl_f_upgrade_to_secure_TCPSocket\x01\0\0\x01\tTCPSocket\x01\0\0\x01\x01\x05self_\x0FupgradeToSecure\0\0\0\x17__widl_f_host_TCPSocket\0\0\0\x01\tTCPSocket\x01\0\x01\x04host\x01\x01\x05self_\x04host\0\0\0\x17__widl_f_port_TCPSocket\0\0\0\x01\tTCPSocket\x01\0\x01\x04port\x01\x01\x05self_\x04port\0\0\0\x16__widl_f_ssl_TCPSocket\0\0\0\x01\tTCPSocket\x01\0\x01\x03ssl\x01\x01\x05self_\x03ssl\0\0\0\"__widl_f_buffered_amount_TCPSocket\0\0\0\x01\tTCPSocket\x01\0\x01\x0EbufferedAmount\x01\x01\x05self_\x0EbufferedAmount\0\0\0\x1E__widl_f_ready_state_TCPSocket\0\0\0\x01\tTCPSocket\x01\0\x01\nreadyState\x01\x01\x05self_\nreadyState\0\0\0\x1E__widl_f_binary_type_TCPSocket\0\0\0\x01\tTCPSocket\x01\0\x01\nbinaryType\x01\x01\x05self_\nbinaryType\0\0\0\x19__widl_f_onopen_TCPSocket\0\0\0\x01\tTCPSocket\x01\0\x01\x06onopen\x01\x01\x05self_\x06onopen\0\0\0\x1D__widl_f_set_onopen_TCPSocket\0\0\0\x01\tTCPSocket\x01\0\x02\x06onopen\x01\x02\x05self_\x06onopen\x06onopen\0\0\0\x1A__widl_f_ondrain_TCPSocket\0\0\0\x01\tTCPSocket\x01\0\x01\x07ondrain\x01\x01\x05self_\x07ondrain\0\0\0\x1E__widl_f_set_ondrain_TCPSocket\0\0\0\x01\tTCPSocket\x01\0\x02\x07ondrain\x01\x02\x05self_\x07ondrain\x07ondrain\0\0\0\x19__widl_f_ondata_TCPSocket\0\0\0\x01\tTCPSocket\x01\0\x01\x06ondata\x01\x01\x05self_\x06ondata\0\0\0\x1D__widl_f_set_ondata_TCPSocket\0\0\0\x01\tTCPSocket\x01\0\x02\x06ondata\x01\x02\x05self_\x06ondata\x06ondata\0\0\0\x1A__widl_f_onerror_TCPSocket\0\0\0\x01\tTCPSocket\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0\x1E__widl_f_set_onerror_TCPSocket\0\0\0\x01\tTCPSocket\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0\x1A__widl_f_onclose_TCPSocket\0\0\0\x01\tTCPSocket\x01\0\x01\x07onclose\x01\x01\x05self_\x07onclose\0\0\0\x1E__widl_f_set_onclose_TCPSocket\0\0\0\x01\tTCPSocket\x01\0\x02\x07onclose\x01\x02\x05self_\x07onclose\x07onclose\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
