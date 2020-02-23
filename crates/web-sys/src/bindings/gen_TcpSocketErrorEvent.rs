use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `TCPSocketErrorEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent)\n\n*This API requires the following crate features to be activated: `TcpSocketErrorEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct TcpSocketErrorEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_TcpSocketErrorEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for TcpSocketErrorEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
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
            inform(114u32);
            inform(114u32);
            inform(111u32);
            inform(114u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for TcpSocketErrorEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for TcpSocketErrorEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for TcpSocketErrorEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a TcpSocketErrorEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for TcpSocketErrorEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            TcpSocketErrorEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for TcpSocketErrorEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a TcpSocketErrorEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for TcpSocketErrorEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<TcpSocketErrorEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(TcpSocketErrorEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for TcpSocketErrorEvent {
        #[inline]
        fn from(obj: JsValue) -> TcpSocketErrorEvent {
            TcpSocketErrorEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for TcpSocketErrorEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<TcpSocketErrorEvent> for TcpSocketErrorEvent {
        #[inline]
        fn as_ref(&self) -> &TcpSocketErrorEvent {
            self
        }
    }
    impl From<TcpSocketErrorEvent> for JsValue {
        #[inline]
        fn from(obj: TcpSocketErrorEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for TcpSocketErrorEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_TCPSocketErrorEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_TCPSocketErrorEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_TCPSocketErrorEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            TcpSocketErrorEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const TcpSocketErrorEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<TcpSocketErrorEvent> for Event {
    #[inline]
    fn from(obj: TcpSocketErrorEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for TcpSocketErrorEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<TcpSocketErrorEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: TcpSocketErrorEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for TcpSocketErrorEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "TcpSocketErrorEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_TCPSocketErrorEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <TcpSocketErrorEvent as WasmDescribe>::describe();
}
impl TcpSocketErrorEvent {
    #[cfg(all(feature = "TcpSocketErrorEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new TCPSocketErrorEvent(..)` constructor, creating a new instance of `TCPSocketErrorEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent/TCPSocketErrorEvent)\n\n*This API requires the following crate features to be activated: `TcpSocketErrorEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<TcpSocketErrorEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TcpSocketErrorEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_TCPSocketErrorEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TcpSocketErrorEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_TCPSocketErrorEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TcpSocketErrorEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_TCPSocketErrorEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TcpSocketErrorEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TcpSocketErrorEvent", feature = "TcpSocketErrorEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_TCPSocketErrorEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&TcpSocketErrorEventInit as WasmDescribe>::describe();
    <TcpSocketErrorEvent as WasmDescribe>::describe();
}
impl TcpSocketErrorEvent {
    #[cfg(all(feature = "TcpSocketErrorEvent", feature = "TcpSocketErrorEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new TCPSocketErrorEvent(..)` constructor, creating a new instance of `TCPSocketErrorEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent/TCPSocketErrorEvent)\n\n*This API requires the following crate features to be activated: `TcpSocketErrorEvent`, `TcpSocketErrorEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &TcpSocketErrorEventInit,
    ) -> Result<TcpSocketErrorEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TcpSocketErrorEvent", feature = "TcpSocketErrorEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_TCPSocketErrorEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & TcpSocketErrorEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <TcpSocketErrorEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_TCPSocketErrorEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&TcpSocketErrorEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TcpSocketErrorEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&TcpSocketErrorEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_TCPSocketErrorEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TcpSocketErrorEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TcpSocketErrorEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_TCPSocketErrorEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpSocketErrorEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl TcpSocketErrorEvent {
    #[cfg(all(feature = "TcpSocketErrorEvent",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent/name)\n\n*This API requires the following crate features to be activated: `TcpSocketErrorEvent`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "TcpSocketErrorEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_TCPSocketErrorEvent(
                self_: <&TcpSocketErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_TCPSocketErrorEvent(
            self_: <&TcpSocketErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&TcpSocketErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_TCPSocketErrorEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TcpSocketErrorEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_message_TCPSocketErrorEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TcpSocketErrorEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl TcpSocketErrorEvent {
    #[cfg(all(feature = "TcpSocketErrorEvent",))]
    #[allow(bad_style)]
    #[doc = "The `message` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent/message)\n\n*This API requires the following crate features to be activated: `TcpSocketErrorEvent`*"]
    #[allow(clippy::all)]
    pub fn message(&self) -> String {
        #[cfg(all(feature = "TcpSocketErrorEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_message_TCPSocketErrorEvent(
                self_: <&TcpSocketErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_message_TCPSocketErrorEvent(
            self_: <&TcpSocketErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&TcpSocketErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_message_TCPSocketErrorEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8d5119fb1cf9b872: [u8; 523usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xC9\x01\0\0\0\0\x05\0\0\x02\x13TCPSocketErrorEvent%__widl_instanceof_TCPSocketErrorEvent\0\0\0\0 __widl_f_new_TCPSocketErrorEvent\x01\0\0\x01\x13TCPSocketErrorEvent\0\x01\x01\x05type_\x03new\0\0\05__widl_f_new_with_event_init_dict_TCPSocketErrorEvent\x01\0\0\x01\x13TCPSocketErrorEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0!__widl_f_name_TCPSocketErrorEvent\0\0\0\x01\x13TCPSocketErrorEvent\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0$__widl_f_message_TCPSocketErrorEvent\0\0\0\x01\x13TCPSocketErrorEvent\x01\0\x01\x07message\x01\x01\x05self_\x07message\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
