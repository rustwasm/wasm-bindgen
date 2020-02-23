use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `RTCPeerConnectionIceEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceEvent)\n\n*This API requires the following crate features to be activated: `RtcPeerConnectionIceEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct RtcPeerConnectionIceEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_RtcPeerConnectionIceEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for RtcPeerConnectionIceEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(25u32);
            inform(82u32);
            inform(84u32);
            inform(67u32);
            inform(80u32);
            inform(101u32);
            inform(101u32);
            inform(114u32);
            inform(67u32);
            inform(111u32);
            inform(110u32);
            inform(110u32);
            inform(101u32);
            inform(99u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(73u32);
            inform(99u32);
            inform(101u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for RtcPeerConnectionIceEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for RtcPeerConnectionIceEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for RtcPeerConnectionIceEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a RtcPeerConnectionIceEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for RtcPeerConnectionIceEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            RtcPeerConnectionIceEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for RtcPeerConnectionIceEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a RtcPeerConnectionIceEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for RtcPeerConnectionIceEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<RtcPeerConnectionIceEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(RtcPeerConnectionIceEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for RtcPeerConnectionIceEvent {
        #[inline]
        fn from(obj: JsValue) -> RtcPeerConnectionIceEvent {
            RtcPeerConnectionIceEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for RtcPeerConnectionIceEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<RtcPeerConnectionIceEvent> for RtcPeerConnectionIceEvent {
        #[inline]
        fn as_ref(&self) -> &RtcPeerConnectionIceEvent {
            self
        }
    }
    impl From<RtcPeerConnectionIceEvent> for JsValue {
        #[inline]
        fn from(obj: RtcPeerConnectionIceEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for RtcPeerConnectionIceEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_RTCPeerConnectionIceEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_RTCPeerConnectionIceEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_RTCPeerConnectionIceEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            RtcPeerConnectionIceEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const RtcPeerConnectionIceEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<RtcPeerConnectionIceEvent> for Event {
    #[inline]
    fn from(obj: RtcPeerConnectionIceEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for RtcPeerConnectionIceEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<RtcPeerConnectionIceEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: RtcPeerConnectionIceEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for RtcPeerConnectionIceEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "RtcPeerConnectionIceEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_RTCPeerConnectionIceEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <RtcPeerConnectionIceEvent as WasmDescribe>::describe();
}
impl RtcPeerConnectionIceEvent {
    #[cfg(all(feature = "RtcPeerConnectionIceEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new RTCPeerConnectionIceEvent(..)` constructor, creating a new instance of `RTCPeerConnectionIceEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceEvent/RTCPeerConnectionIceEvent)\n\n*This API requires the following crate features to be activated: `RtcPeerConnectionIceEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<RtcPeerConnectionIceEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "RtcPeerConnectionIceEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_RTCPeerConnectionIceEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcPeerConnectionIceEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_RTCPeerConnectionIceEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcPeerConnectionIceEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_RTCPeerConnectionIceEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<RtcPeerConnectionIceEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "RtcPeerConnectionIceEvent",
    feature = "RtcPeerConnectionIceEventInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_RTCPeerConnectionIceEvent()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&RtcPeerConnectionIceEventInit as WasmDescribe>::describe();
    <RtcPeerConnectionIceEvent as WasmDescribe>::describe();
}
impl RtcPeerConnectionIceEvent {
    #[cfg(all(
        feature = "RtcPeerConnectionIceEvent",
        feature = "RtcPeerConnectionIceEventInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new RTCPeerConnectionIceEvent(..)` constructor, creating a new instance of `RTCPeerConnectionIceEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceEvent/RTCPeerConnectionIceEvent)\n\n*This API requires the following crate features to be activated: `RtcPeerConnectionIceEvent`, `RtcPeerConnectionIceEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &RtcPeerConnectionIceEventInit,
    ) -> Result<RtcPeerConnectionIceEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "RtcPeerConnectionIceEvent",
            feature = "RtcPeerConnectionIceEventInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_RTCPeerConnectionIceEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & RtcPeerConnectionIceEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <RtcPeerConnectionIceEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_RTCPeerConnectionIceEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict : < & RtcPeerConnectionIceEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <RtcPeerConnectionIceEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                let event_init_dict = < & RtcPeerConnectionIceEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( event_init_dict ) ;
                __widl_f_new_with_event_init_dict_RTCPeerConnectionIceEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<RtcPeerConnectionIceEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "RtcIceCandidate", feature = "RtcPeerConnectionIceEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_candidate_RTCPeerConnectionIceEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnectionIceEvent as WasmDescribe>::describe();
    <Option<RtcIceCandidate> as WasmDescribe>::describe();
}
impl RtcPeerConnectionIceEvent {
    #[cfg(all(feature = "RtcIceCandidate", feature = "RtcPeerConnectionIceEvent",))]
    #[allow(bad_style)]
    #[doc = "The `candidate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceEvent/candidate)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcPeerConnectionIceEvent`*"]
    #[allow(clippy::all)]
    pub fn candidate(&self) -> Option<RtcIceCandidate> {
        #[cfg(all(feature = "RtcIceCandidate", feature = "RtcPeerConnectionIceEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_candidate_RTCPeerConnectionIceEvent(
                self_: <&RtcPeerConnectionIceEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<RtcIceCandidate> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_candidate_RTCPeerConnectionIceEvent(
            self_: <&RtcPeerConnectionIceEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<RtcIceCandidate> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnectionIceEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_candidate_RTCPeerConnectionIceEvent(self_)
            };
            <Option<RtcIceCandidate> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_b97c8391414311a9: [u8; 495usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xAD\x01\0\0\0\0\x04\0\0\x02\x19RTCPeerConnectionIceEvent+__widl_instanceof_RTCPeerConnectionIceEvent\0\0\0\0&__widl_f_new_RTCPeerConnectionIceEvent\x01\0\0\x01\x19RTCPeerConnectionIceEvent\0\x01\x01\x05type_\x03new\0\0\0;__widl_f_new_with_event_init_dict_RTCPeerConnectionIceEvent\x01\0\0\x01\x19RTCPeerConnectionIceEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0,__widl_f_candidate_RTCPeerConnectionIceEvent\0\0\0\x01\x19RTCPeerConnectionIceEvent\x01\0\x01\tcandidate\x01\x01\x05self_\tcandidate\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
