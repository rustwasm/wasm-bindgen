use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `TCPSocketEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketEvent)\n\n*This API requires the following crate features to be activated: `TcpSocketEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct TcpSocketEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_TcpSocketEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for TcpSocketEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(84u32);
            inform(67u32);
            inform(80u32);
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
    impl core::ops::Deref for TcpSocketEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for TcpSocketEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for TcpSocketEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a TcpSocketEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for TcpSocketEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            TcpSocketEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for TcpSocketEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a TcpSocketEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for TcpSocketEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<TcpSocketEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(TcpSocketEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for TcpSocketEvent {
        #[inline]
        fn from(obj: JsValue) -> TcpSocketEvent {
            TcpSocketEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for TcpSocketEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<TcpSocketEvent> for TcpSocketEvent {
        #[inline]
        fn as_ref(&self) -> &TcpSocketEvent {
            self
        }
    }
    impl From<TcpSocketEvent> for JsValue {
        #[inline]
        fn from(obj: TcpSocketEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for TcpSocketEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_TCPSocketEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_TCPSocketEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_TCPSocketEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            TcpSocketEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const TcpSocketEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<TcpSocketEvent> for Event {
    #[inline]
    fn from(obj: TcpSocketEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for TcpSocketEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<TcpSocketEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: TcpSocketEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for TcpSocketEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "TcpSocketEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_TCPSocketEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <TcpSocketEvent as WasmDescribe>::describe();
}
impl TcpSocketEvent {
    #[cfg(all(feature = "TcpSocketEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new TCPSocketEvent(..)` constructor, creating a new instance of `TCPSocketEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketEvent/TCPSocketEvent)\n\n*This API requires the following crate features to be activated: `TcpSocketEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<TcpSocketEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TcpSocketEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_TCPSocketEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TcpSocketEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_TCPSocketEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TcpSocketEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_TCPSocketEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TcpSocketEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TcpSocketEvent", feature = "TcpSocketEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_TCPSocketEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&TcpSocketEventInit as WasmDescribe>::describe();
    <TcpSocketEvent as WasmDescribe>::describe();
}
impl TcpSocketEvent {
    #[cfg(all(feature = "TcpSocketEvent", feature = "TcpSocketEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new TCPSocketEvent(..)` constructor, creating a new instance of `TCPSocketEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketEvent/TCPSocketEvent)\n\n*This API requires the following crate features to be activated: `TcpSocketEvent`, `TcpSocketEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &TcpSocketEventInit,
    ) -> Result<TcpSocketEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TcpSocketEvent", feature = "TcpSocketEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_TCPSocketEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&TcpSocketEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TcpSocketEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_TCPSocketEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&TcpSocketEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TcpSocketEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&TcpSocketEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_TCPSocketEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TcpSocketEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TcpSocketEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_data_TCPSocketEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpSocketEvent as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl TcpSocketEvent {
    #[cfg(all(feature = "TcpSocketEvent",))]
    #[allow(bad_style)]
    #[doc = "The `data` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketEvent/data)\n\n*This API requires the following crate features to be activated: `TcpSocketEvent`*"]
    #[allow(clippy::all)]
    pub fn data(&self) -> ::wasm_bindgen::JsValue {
        #[cfg(all(feature = "TcpSocketEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_data_TCPSocketEvent(
                self_: <&TcpSocketEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_data_TCPSocketEvent(
            self_: <&TcpSocketEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TcpSocketEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_data_TCPSocketEvent(self_)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_674f12572368beed: [u8; 392usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}F\x01\0\0\0\0\x04\0\0\x02\x0ETCPSocketEvent __widl_instanceof_TCPSocketEvent\0\0\0\0\x1B__widl_f_new_TCPSocketEvent\x01\0\0\x01\x0ETCPSocketEvent\0\x01\x01\x05type_\x03new\0\0\00__widl_f_new_with_event_init_dict_TCPSocketEvent\x01\0\0\x01\x0ETCPSocketEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0\x1C__widl_f_data_TCPSocketEvent\0\0\0\x01\x0ETCPSocketEvent\x01\0\x01\x04data\x01\x01\x05self_\x04data\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
