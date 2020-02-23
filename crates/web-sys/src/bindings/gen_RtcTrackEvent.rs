use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `RTCTrackEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent)\n\n*This API requires the following crate features to be activated: `RtcTrackEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct RtcTrackEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_RtcTrackEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for RtcTrackEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(82u32);
            inform(84u32);
            inform(67u32);
            inform(84u32);
            inform(114u32);
            inform(97u32);
            inform(99u32);
            inform(107u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for RtcTrackEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for RtcTrackEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for RtcTrackEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a RtcTrackEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for RtcTrackEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            RtcTrackEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for RtcTrackEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a RtcTrackEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for RtcTrackEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<RtcTrackEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(RtcTrackEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for RtcTrackEvent {
        #[inline]
        fn from(obj: JsValue) -> RtcTrackEvent {
            RtcTrackEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for RtcTrackEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<RtcTrackEvent> for RtcTrackEvent {
        #[inline]
        fn as_ref(&self) -> &RtcTrackEvent {
            self
        }
    }
    impl From<RtcTrackEvent> for JsValue {
        #[inline]
        fn from(obj: RtcTrackEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for RtcTrackEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_RTCTrackEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_RTCTrackEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_RTCTrackEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            RtcTrackEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const RtcTrackEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<RtcTrackEvent> for Event {
    #[inline]
    fn from(obj: RtcTrackEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for RtcTrackEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<RtcTrackEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: RtcTrackEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for RtcTrackEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "RtcTrackEvent", feature = "RtcTrackEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_RTCTrackEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&RtcTrackEventInit as WasmDescribe>::describe();
    <RtcTrackEvent as WasmDescribe>::describe();
}
impl RtcTrackEvent {
    #[cfg(all(feature = "RtcTrackEvent", feature = "RtcTrackEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new RTCTrackEvent(..)` constructor, creating a new instance of `RTCTrackEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/RTCTrackEvent)\n\n*This API requires the following crate features to be activated: `RtcTrackEvent`, `RtcTrackEventInit`*"]
    #[allow(clippy::all)]
    pub fn new(
        type_: &str,
        event_init_dict: &RtcTrackEventInit,
    ) -> Result<RtcTrackEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "RtcTrackEvent", feature = "RtcTrackEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_RTCTrackEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&RtcTrackEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcTrackEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_RTCTrackEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&RtcTrackEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcTrackEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&RtcTrackEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_RTCTrackEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<RtcTrackEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "RtcRtpReceiver", feature = "RtcTrackEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_receiver_RTCTrackEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcTrackEvent as WasmDescribe>::describe();
    <RtcRtpReceiver as WasmDescribe>::describe();
}
impl RtcTrackEvent {
    #[cfg(all(feature = "RtcRtpReceiver", feature = "RtcTrackEvent",))]
    #[allow(bad_style)]
    #[doc = "The `receiver` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/receiver)\n\n*This API requires the following crate features to be activated: `RtcRtpReceiver`, `RtcTrackEvent`*"]
    #[allow(clippy::all)]
    pub fn receiver(&self) -> RtcRtpReceiver {
        #[cfg(all(feature = "RtcRtpReceiver", feature = "RtcTrackEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_receiver_RTCTrackEvent(
                self_: <&RtcTrackEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcRtpReceiver as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_receiver_RTCTrackEvent(
            self_: <&RtcTrackEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcRtpReceiver as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcTrackEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_receiver_RTCTrackEvent(self_)
            };
            <RtcRtpReceiver as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack", feature = "RtcTrackEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_track_RTCTrackEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcTrackEvent as WasmDescribe>::describe();
    <MediaStreamTrack as WasmDescribe>::describe();
}
impl RtcTrackEvent {
    #[cfg(all(feature = "MediaStreamTrack", feature = "RtcTrackEvent",))]
    #[allow(bad_style)]
    #[doc = "The `track` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/track)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcTrackEvent`*"]
    #[allow(clippy::all)]
    pub fn track(&self) -> MediaStreamTrack {
        #[cfg(all(feature = "MediaStreamTrack", feature = "RtcTrackEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_track_RTCTrackEvent(
                self_: <&RtcTrackEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaStreamTrack as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_track_RTCTrackEvent(
            self_: <&RtcTrackEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaStreamTrack as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcTrackEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_track_RTCTrackEvent(self_)
            };
            <MediaStreamTrack as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcTrackEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_streams_RTCTrackEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcTrackEvent as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl RtcTrackEvent {
    #[cfg(all(feature = "RtcTrackEvent",))]
    #[allow(bad_style)]
    #[doc = "The `streams` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/streams)\n\n*This API requires the following crate features to be activated: `RtcTrackEvent`*"]
    #[allow(clippy::all)]
    pub fn streams(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "RtcTrackEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_streams_RTCTrackEvent(
                self_: <&RtcTrackEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_streams_RTCTrackEvent(
            self_: <&RtcTrackEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcTrackEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_streams_RTCTrackEvent(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcRtpTransceiver", feature = "RtcTrackEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transceiver_RTCTrackEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcTrackEvent as WasmDescribe>::describe();
    <RtcRtpTransceiver as WasmDescribe>::describe();
}
impl RtcTrackEvent {
    #[cfg(all(feature = "RtcRtpTransceiver", feature = "RtcTrackEvent",))]
    #[allow(bad_style)]
    #[doc = "The `transceiver` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/transceiver)\n\n*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcTrackEvent`*"]
    #[allow(clippy::all)]
    pub fn transceiver(&self) -> RtcRtpTransceiver {
        #[cfg(all(feature = "RtcRtpTransceiver", feature = "RtcTrackEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transceiver_RTCTrackEvent(
                self_: <&RtcTrackEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcRtpTransceiver as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transceiver_RTCTrackEvent(
            self_: <&RtcTrackEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcRtpTransceiver as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcTrackEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_transceiver_RTCTrackEvent(self_)
            };
            <RtcRtpTransceiver as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_9fd252fadd8fe9a7: [u8; 557usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xEB\x01\0\0\0\0\x06\0\0\x02\rRTCTrackEvent\x1F__widl_instanceof_RTCTrackEvent\0\0\0\0\x1A__widl_f_new_RTCTrackEvent\x01\0\0\x01\rRTCTrackEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0\x1F__widl_f_receiver_RTCTrackEvent\0\0\0\x01\rRTCTrackEvent\x01\0\x01\x08receiver\x01\x01\x05self_\x08receiver\0\0\0\x1C__widl_f_track_RTCTrackEvent\0\0\0\x01\rRTCTrackEvent\x01\0\x01\x05track\x01\x01\x05self_\x05track\0\0\0\x1E__widl_f_streams_RTCTrackEvent\0\0\0\x01\rRTCTrackEvent\x01\0\x01\x07streams\x01\x01\x05self_\x07streams\0\0\0\"__widl_f_transceiver_RTCTrackEvent\0\0\0\x01\rRTCTrackEvent\x01\0\x01\x0Btransceiver\x01\x01\x05self_\x0Btransceiver\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
