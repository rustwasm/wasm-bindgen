use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `RTCDataChannelEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannelEvent)\n\n*This API requires the following crate features to be activated: `RtcDataChannelEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct RtcDataChannelEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_RtcDataChannelEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for RtcDataChannelEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(82u32);
            inform(84u32);
            inform(67u32);
            inform(68u32);
            inform(97u32);
            inform(116u32);
            inform(97u32);
            inform(67u32);
            inform(104u32);
            inform(97u32);
            inform(110u32);
            inform(110u32);
            inform(101u32);
            inform(108u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for RtcDataChannelEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for RtcDataChannelEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for RtcDataChannelEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a RtcDataChannelEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for RtcDataChannelEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            RtcDataChannelEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for RtcDataChannelEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a RtcDataChannelEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for RtcDataChannelEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<RtcDataChannelEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(RtcDataChannelEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for RtcDataChannelEvent {
        #[inline]
        fn from(obj: JsValue) -> RtcDataChannelEvent {
            RtcDataChannelEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for RtcDataChannelEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<RtcDataChannelEvent> for RtcDataChannelEvent {
        #[inline]
        fn as_ref(&self) -> &RtcDataChannelEvent {
            self
        }
    }
    impl From<RtcDataChannelEvent> for JsValue {
        #[inline]
        fn from(obj: RtcDataChannelEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for RtcDataChannelEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_RTCDataChannelEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_RTCDataChannelEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_RTCDataChannelEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            RtcDataChannelEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const RtcDataChannelEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<RtcDataChannelEvent> for Event {
    #[inline]
    fn from(obj: RtcDataChannelEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for RtcDataChannelEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<RtcDataChannelEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: RtcDataChannelEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for RtcDataChannelEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "RtcDataChannelEvent", feature = "RtcDataChannelEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_RTCDataChannelEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&RtcDataChannelEventInit as WasmDescribe>::describe();
    <RtcDataChannelEvent as WasmDescribe>::describe();
}
impl RtcDataChannelEvent {
    #[cfg(all(feature = "RtcDataChannelEvent", feature = "RtcDataChannelEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new RTCDataChannelEvent(..)` constructor, creating a new instance of `RTCDataChannelEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannelEvent/RTCDataChannelEvent)\n\n*This API requires the following crate features to be activated: `RtcDataChannelEvent`, `RtcDataChannelEventInit`*"]
    #[allow(clippy::all)]
    pub fn new(
        type_: &str,
        event_init_dict: &RtcDataChannelEventInit,
    ) -> Result<RtcDataChannelEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "RtcDataChannelEvent", feature = "RtcDataChannelEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_RTCDataChannelEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & RtcDataChannelEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <RtcDataChannelEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_RTCDataChannelEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&RtcDataChannelEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcDataChannelEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&RtcDataChannelEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_RTCDataChannelEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<RtcDataChannelEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "RtcDataChannel", feature = "RtcDataChannelEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_channel_RTCDataChannelEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcDataChannelEvent as WasmDescribe>::describe();
    <RtcDataChannel as WasmDescribe>::describe();
}
impl RtcDataChannelEvent {
    #[cfg(all(feature = "RtcDataChannel", feature = "RtcDataChannelEvent",))]
    #[allow(bad_style)]
    #[doc = "The `channel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannelEvent/channel)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelEvent`*"]
    #[allow(clippy::all)]
    pub fn channel(&self) -> RtcDataChannel {
        #[cfg(all(feature = "RtcDataChannel", feature = "RtcDataChannelEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_channel_RTCDataChannelEvent(
                self_: <&RtcDataChannelEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcDataChannel as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_channel_RTCDataChannelEvent(
            self_: <&RtcDataChannelEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcDataChannel as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcDataChannelEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_channel_RTCDataChannelEvent(self_)
            };
            <RtcDataChannel as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_259962e6020b625d: [u8; 347usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x19\x01\0\0\0\0\x03\0\0\x02\x13RTCDataChannelEvent%__widl_instanceof_RTCDataChannelEvent\0\0\0\0 __widl_f_new_RTCDataChannelEvent\x01\0\0\x01\x13RTCDataChannelEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0$__widl_f_channel_RTCDataChannelEvent\0\0\0\x01\x13RTCDataChannelEvent\x01\0\x01\x07channel\x01\x01\x05self_\x07channel\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
