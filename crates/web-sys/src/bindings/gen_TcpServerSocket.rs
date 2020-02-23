use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `TCPServerSocket` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket)\n\n*This API requires the following crate features to be activated: `TcpServerSocket`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct TcpServerSocket {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_TcpServerSocket: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for TcpServerSocket {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(84u32);
            inform(67u32);
            inform(80u32);
            inform(83u32);
            inform(101u32);
            inform(114u32);
            inform(118u32);
            inform(101u32);
            inform(114u32);
            inform(83u32);
            inform(111u32);
            inform(99u32);
            inform(107u32);
            inform(101u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for TcpServerSocket {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for TcpServerSocket {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for TcpServerSocket {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a TcpServerSocket {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for TcpServerSocket {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            TcpServerSocket {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for TcpServerSocket {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a TcpServerSocket {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for TcpServerSocket {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<TcpServerSocket>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(TcpServerSocket {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for TcpServerSocket {
        #[inline]
        fn from(obj: JsValue) -> TcpServerSocket {
            TcpServerSocket { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for TcpServerSocket {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<TcpServerSocket> for TcpServerSocket {
        #[inline]
        fn as_ref(&self) -> &TcpServerSocket {
            self
        }
    }
    impl From<TcpServerSocket> for JsValue {
        #[inline]
        fn from(obj: TcpServerSocket) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for TcpServerSocket {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_TCPServerSocket(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_TCPServerSocket(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_TCPServerSocket(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            TcpServerSocket { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const TcpServerSocket) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<TcpServerSocket> for EventTarget {
    #[inline]
    fn from(obj: TcpServerSocket) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for TcpServerSocket {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<TcpServerSocket> for ::js_sys::Object {
    #[inline]
    fn from(obj: TcpServerSocket) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for TcpServerSocket {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "TcpServerSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_TCPServerSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <u16 as WasmDescribe>::describe();
    <TcpServerSocket as WasmDescribe>::describe();
}
impl TcpServerSocket {
    #[cfg(all(feature = "TcpServerSocket",))]
    #[allow(bad_style)]
    #[doc = "The `new TCPServerSocket(..)` constructor, creating a new instance of `TCPServerSocket`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/TCPServerSocket)\n\n*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    #[allow(clippy::all)]
    pub fn new(port: u16) -> Result<TcpServerSocket, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TcpServerSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_TCPServerSocket(
                port: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TcpServerSocket as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_TCPServerSocket(
            port: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TcpServerSocket as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(port);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let port = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(port);
                __widl_f_new_TCPServerSocket(port)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TcpServerSocket as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ServerSocketOptions", feature = "TcpServerSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_options_TCPServerSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <u16 as WasmDescribe>::describe();
    <&ServerSocketOptions as WasmDescribe>::describe();
    <TcpServerSocket as WasmDescribe>::describe();
}
impl TcpServerSocket {
    #[cfg(all(feature = "ServerSocketOptions", feature = "TcpServerSocket",))]
    #[allow(bad_style)]
    #[doc = "The `new TCPServerSocket(..)` constructor, creating a new instance of `TCPServerSocket`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/TCPServerSocket)\n\n*This API requires the following crate features to be activated: `ServerSocketOptions`, `TcpServerSocket`*"]
    #[allow(clippy::all)]
    pub fn new_with_options(
        port: u16,
        options: &ServerSocketOptions,
    ) -> Result<TcpServerSocket, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ServerSocketOptions", feature = "TcpServerSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_options_TCPServerSocket(
                port: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ServerSocketOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TcpServerSocket as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_options_TCPServerSocket(
            port: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ServerSocketOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TcpServerSocket as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(port);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let port = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(port);
                let options =
                    <&ServerSocketOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_options_TCPServerSocket(port, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TcpServerSocket as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ServerSocketOptions", feature = "TcpServerSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_options_and_backlog_TCPServerSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <u16 as WasmDescribe>::describe();
    <&ServerSocketOptions as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <TcpServerSocket as WasmDescribe>::describe();
}
impl TcpServerSocket {
    #[cfg(all(feature = "ServerSocketOptions", feature = "TcpServerSocket",))]
    #[allow(bad_style)]
    #[doc = "The `new TCPServerSocket(..)` constructor, creating a new instance of `TCPServerSocket`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/TCPServerSocket)\n\n*This API requires the following crate features to be activated: `ServerSocketOptions`, `TcpServerSocket`*"]
    #[allow(clippy::all)]
    pub fn new_with_options_and_backlog(
        port: u16,
        options: &ServerSocketOptions,
        backlog: u16,
    ) -> Result<TcpServerSocket, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ServerSocketOptions", feature = "TcpServerSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_options_and_backlog_TCPServerSocket(
                port: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ServerSocketOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                backlog: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TcpServerSocket as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_options_and_backlog_TCPServerSocket(
            port: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ServerSocketOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            backlog: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TcpServerSocket as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(port);
            drop(options);
            drop(backlog);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let port = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(port);
                let options =
                    <&ServerSocketOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                let backlog = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(backlog);
                __widl_f_new_with_options_and_backlog_TCPServerSocket(port, options, backlog)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TcpServerSocket as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TcpServerSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_TCPServerSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpServerSocket as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TcpServerSocket {
    #[cfg(all(feature = "TcpServerSocket",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/close)\n\n*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    #[allow(clippy::all)]
    pub fn close(&self) {
        #[cfg(all(feature = "TcpServerSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_TCPServerSocket(
                self_: <&TcpServerSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_TCPServerSocket(
            self_: <&TcpServerSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&TcpServerSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_close_TCPServerSocket(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TcpServerSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_local_port_TCPServerSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpServerSocket as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl TcpServerSocket {
    #[cfg(all(feature = "TcpServerSocket",))]
    #[allow(bad_style)]
    #[doc = "The `localPort` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/localPort)\n\n*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    #[allow(clippy::all)]
    pub fn local_port(&self) -> u16 {
        #[cfg(all(feature = "TcpServerSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_local_port_TCPServerSocket(
                self_: <&TcpServerSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_local_port_TCPServerSocket(
            self_: <&TcpServerSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&TcpServerSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_local_port_TCPServerSocket(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TcpServerSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onconnect_TCPServerSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpServerSocket as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl TcpServerSocket {
    #[cfg(all(feature = "TcpServerSocket",))]
    #[allow(bad_style)]
    #[doc = "The `onconnect` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/onconnect)\n\n*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    #[allow(clippy::all)]
    pub fn onconnect(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "TcpServerSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onconnect_TCPServerSocket(
                self_: <&TcpServerSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onconnect_TCPServerSocket(
            self_: <&TcpServerSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&TcpServerSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onconnect_TCPServerSocket(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TcpServerSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onconnect_TCPServerSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TcpServerSocket as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TcpServerSocket {
    #[cfg(all(feature = "TcpServerSocket",))]
    #[allow(bad_style)]
    #[doc = "The `onconnect` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/onconnect)\n\n*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    #[allow(clippy::all)]
    pub fn set_onconnect(&self, onconnect: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "TcpServerSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onconnect_TCPServerSocket(
                self_: <&TcpServerSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onconnect: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onconnect_TCPServerSocket(
            self_: <&TcpServerSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onconnect: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onconnect);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&TcpServerSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onconnect =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onconnect,
                    );
                __widl_f_set_onconnect_TCPServerSocket(self_, onconnect)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TcpServerSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_TCPServerSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpServerSocket as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl TcpServerSocket {
    #[cfg(all(feature = "TcpServerSocket",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/onerror)\n\n*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "TcpServerSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_TCPServerSocket(
                self_: <&TcpServerSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_TCPServerSocket(
            self_: <&TcpServerSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&TcpServerSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_TCPServerSocket(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TcpServerSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_TCPServerSocket() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TcpServerSocket as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TcpServerSocket {
    #[cfg(all(feature = "TcpServerSocket",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/onerror)\n\n*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "TcpServerSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_TCPServerSocket(
                self_: <&TcpServerSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_TCPServerSocket(
            self_: <&TcpServerSocket as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ =
                    <&TcpServerSocket as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_TCPServerSocket(self_, onerror)
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
pub static __WASM_BINDGEN_GENERATED_75042a6f36807ee2: [u8; 944usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}n\x03\0\0\0\0\n\0\0\x02\x0FTCPServerSocket!__widl_instanceof_TCPServerSocket\0\0\0\0\x1C__widl_f_new_TCPServerSocket\x01\0\0\x01\x0FTCPServerSocket\0\x01\x01\x04port\x03new\0\0\0)__widl_f_new_with_options_TCPServerSocket\x01\0\0\x01\x0FTCPServerSocket\0\x01\x02\x04port\x07options\x03new\0\0\05__widl_f_new_with_options_and_backlog_TCPServerSocket\x01\0\0\x01\x0FTCPServerSocket\0\x01\x03\x04port\x07options\x07backlog\x03new\0\0\0\x1E__widl_f_close_TCPServerSocket\0\0\0\x01\x0FTCPServerSocket\x01\0\0\x01\x01\x05self_\x05close\0\0\0#__widl_f_local_port_TCPServerSocket\0\0\0\x01\x0FTCPServerSocket\x01\0\x01\tlocalPort\x01\x01\x05self_\tlocalPort\0\0\0\"__widl_f_onconnect_TCPServerSocket\0\0\0\x01\x0FTCPServerSocket\x01\0\x01\tonconnect\x01\x01\x05self_\tonconnect\0\0\0&__widl_f_set_onconnect_TCPServerSocket\0\0\0\x01\x0FTCPServerSocket\x01\0\x02\tonconnect\x01\x02\x05self_\tonconnect\tonconnect\0\0\0 __widl_f_onerror_TCPServerSocket\0\0\0\x01\x0FTCPServerSocket\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0$__widl_f_set_onerror_TCPServerSocket\0\0\0\x01\x0FTCPServerSocket\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
