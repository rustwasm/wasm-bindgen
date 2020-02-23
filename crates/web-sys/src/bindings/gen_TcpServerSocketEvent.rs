use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `TCPServerSocketEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocketEvent)\n\n*This API requires the following crate features to be activated: `TcpServerSocketEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct TcpServerSocketEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_TcpServerSocketEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for TcpServerSocketEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(20u32);
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
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for TcpServerSocketEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for TcpServerSocketEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for TcpServerSocketEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a TcpServerSocketEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for TcpServerSocketEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            TcpServerSocketEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for TcpServerSocketEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a TcpServerSocketEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for TcpServerSocketEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<TcpServerSocketEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(TcpServerSocketEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for TcpServerSocketEvent {
        #[inline]
        fn from(obj: JsValue) -> TcpServerSocketEvent {
            TcpServerSocketEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for TcpServerSocketEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<TcpServerSocketEvent> for TcpServerSocketEvent {
        #[inline]
        fn as_ref(&self) -> &TcpServerSocketEvent {
            self
        }
    }
    impl From<TcpServerSocketEvent> for JsValue {
        #[inline]
        fn from(obj: TcpServerSocketEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for TcpServerSocketEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_TCPServerSocketEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_TCPServerSocketEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_TCPServerSocketEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            TcpServerSocketEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const TcpServerSocketEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<TcpServerSocketEvent> for Event {
    #[inline]
    fn from(obj: TcpServerSocketEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for TcpServerSocketEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<TcpServerSocketEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: TcpServerSocketEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for TcpServerSocketEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "TcpServerSocketEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_TCPServerSocketEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <TcpServerSocketEvent as WasmDescribe>::describe();
}
impl TcpServerSocketEvent {
    #[cfg(all(feature = "TcpServerSocketEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new TCPServerSocketEvent(..)` constructor, creating a new instance of `TCPServerSocketEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocketEvent/TCPServerSocketEvent)\n\n*This API requires the following crate features to be activated: `TcpServerSocketEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<TcpServerSocketEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TcpServerSocketEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_TCPServerSocketEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TcpServerSocketEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_TCPServerSocketEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TcpServerSocketEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_TCPServerSocketEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TcpServerSocketEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TcpServerSocketEvent", feature = "TcpServerSocketEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_TCPServerSocketEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&TcpServerSocketEventInit as WasmDescribe>::describe();
    <TcpServerSocketEvent as WasmDescribe>::describe();
}
impl TcpServerSocketEvent {
    #[cfg(all(feature = "TcpServerSocketEvent", feature = "TcpServerSocketEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new TCPServerSocketEvent(..)` constructor, creating a new instance of `TCPServerSocketEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocketEvent/TCPServerSocketEvent)\n\n*This API requires the following crate features to be activated: `TcpServerSocketEvent`, `TcpServerSocketEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &TcpServerSocketEventInit,
    ) -> Result<TcpServerSocketEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TcpServerSocketEvent", feature = "TcpServerSocketEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_TCPServerSocketEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & TcpServerSocketEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <TcpServerSocketEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_TCPServerSocketEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&TcpServerSocketEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TcpServerSocketEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            drop(event_init_dict);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let event_init_dict =
                    <&TcpServerSocketEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_TCPServerSocketEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TcpServerSocketEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TcpServerSocketEvent", feature = "TcpSocket",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_socket_TCPServerSocketEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpServerSocketEvent as WasmDescribe>::describe();
    <TcpSocket as WasmDescribe>::describe();
}
impl TcpServerSocketEvent {
    #[cfg(all(feature = "TcpServerSocketEvent", feature = "TcpSocket",))]
    #[allow(bad_style)]
    #[doc = "The `socket` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocketEvent/socket)\n\n*This API requires the following crate features to be activated: `TcpServerSocketEvent`, `TcpSocket`*"]
    #[allow(clippy::all)]
    pub fn socket(&self) -> TcpSocket {
        #[cfg(all(feature = "TcpServerSocketEvent", feature = "TcpSocket",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_socket_TCPServerSocketEvent(
                self_: <&TcpServerSocketEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TcpSocket as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_socket_TCPServerSocketEvent(
            self_: <&TcpServerSocketEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TcpSocket as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&TcpServerSocketEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_socket_TCPServerSocketEvent(self_)
            };
            <TcpSocket as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8ac53af352d3a019: [u8; 446usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}|\x01\0\0\0\0\x04\0\0\x02\x14TCPServerSocketEvent&__widl_instanceof_TCPServerSocketEvent\0\0\0\0!__widl_f_new_TCPServerSocketEvent\x01\0\0\x01\x14TCPServerSocketEvent\0\x01\x01\x05type_\x03new\0\0\06__widl_f_new_with_event_init_dict_TCPServerSocketEvent\x01\0\0\x01\x14TCPServerSocketEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0$__widl_f_socket_TCPServerSocketEvent\0\0\0\x01\x14TCPServerSocketEvent\x01\0\x01\x06socket\x01\x01\x05self_\x06socket\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
