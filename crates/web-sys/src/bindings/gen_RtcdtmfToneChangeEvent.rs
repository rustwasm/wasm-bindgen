use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `RTCDTMFToneChangeEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent)\n\n*This API requires the following crate features to be activated: `RtcdtmfToneChangeEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct RtcdtmfToneChangeEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_RtcdtmfToneChangeEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for RtcdtmfToneChangeEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(22u32);
            inform(82u32);
            inform(84u32);
            inform(67u32);
            inform(68u32);
            inform(84u32);
            inform(77u32);
            inform(70u32);
            inform(84u32);
            inform(111u32);
            inform(110u32);
            inform(101u32);
            inform(67u32);
            inform(104u32);
            inform(97u32);
            inform(110u32);
            inform(103u32);
            inform(101u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for RtcdtmfToneChangeEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for RtcdtmfToneChangeEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for RtcdtmfToneChangeEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a RtcdtmfToneChangeEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for RtcdtmfToneChangeEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            RtcdtmfToneChangeEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for RtcdtmfToneChangeEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a RtcdtmfToneChangeEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for RtcdtmfToneChangeEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<RtcdtmfToneChangeEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(RtcdtmfToneChangeEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for RtcdtmfToneChangeEvent {
        #[inline]
        fn from(obj: JsValue) -> RtcdtmfToneChangeEvent {
            RtcdtmfToneChangeEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for RtcdtmfToneChangeEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<RtcdtmfToneChangeEvent> for RtcdtmfToneChangeEvent {
        #[inline]
        fn as_ref(&self) -> &RtcdtmfToneChangeEvent {
            self
        }
    }
    impl From<RtcdtmfToneChangeEvent> for JsValue {
        #[inline]
        fn from(obj: RtcdtmfToneChangeEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for RtcdtmfToneChangeEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_RTCDTMFToneChangeEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_RTCDTMFToneChangeEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_RTCDTMFToneChangeEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            RtcdtmfToneChangeEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const RtcdtmfToneChangeEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<RtcdtmfToneChangeEvent> for Event {
    #[inline]
    fn from(obj: RtcdtmfToneChangeEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for RtcdtmfToneChangeEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<RtcdtmfToneChangeEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: RtcdtmfToneChangeEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for RtcdtmfToneChangeEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "RtcdtmfToneChangeEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_RTCDTMFToneChangeEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <RtcdtmfToneChangeEvent as WasmDescribe>::describe();
}
impl RtcdtmfToneChangeEvent {
    #[cfg(all(feature = "RtcdtmfToneChangeEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new RTCDTMFToneChangeEvent(..)` constructor, creating a new instance of `RTCDTMFToneChangeEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent/RTCDTMFToneChangeEvent)\n\n*This API requires the following crate features to be activated: `RtcdtmfToneChangeEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<RtcdtmfToneChangeEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "RtcdtmfToneChangeEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_RTCDTMFToneChangeEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcdtmfToneChangeEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_RTCDTMFToneChangeEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcdtmfToneChangeEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_RTCDTMFToneChangeEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<RtcdtmfToneChangeEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "RtcdtmfToneChangeEvent",
    feature = "RtcdtmfToneChangeEventInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_RTCDTMFToneChangeEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&RtcdtmfToneChangeEventInit as WasmDescribe>::describe();
    <RtcdtmfToneChangeEvent as WasmDescribe>::describe();
}
impl RtcdtmfToneChangeEvent {
    #[cfg(all(
        feature = "RtcdtmfToneChangeEvent",
        feature = "RtcdtmfToneChangeEventInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new RTCDTMFToneChangeEvent(..)` constructor, creating a new instance of `RTCDTMFToneChangeEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent/RTCDTMFToneChangeEvent)\n\n*This API requires the following crate features to be activated: `RtcdtmfToneChangeEvent`, `RtcdtmfToneChangeEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &RtcdtmfToneChangeEventInit,
    ) -> Result<RtcdtmfToneChangeEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "RtcdtmfToneChangeEvent",
            feature = "RtcdtmfToneChangeEventInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_RTCDTMFToneChangeEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & RtcdtmfToneChangeEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <RtcdtmfToneChangeEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_RTCDTMFToneChangeEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict : < & RtcdtmfToneChangeEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <RtcdtmfToneChangeEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&RtcdtmfToneChangeEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_RTCDTMFToneChangeEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<RtcdtmfToneChangeEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "RtcdtmfToneChangeEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tone_RTCDTMFToneChangeEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcdtmfToneChangeEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl RtcdtmfToneChangeEvent {
    #[cfg(all(feature = "RtcdtmfToneChangeEvent",))]
    #[allow(bad_style)]
    #[doc = "The `tone` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent/tone)\n\n*This API requires the following crate features to be activated: `RtcdtmfToneChangeEvent`*"]
    #[allow(clippy::all)]
    pub fn tone(&self) -> String {
        #[cfg(all(feature = "RtcdtmfToneChangeEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tone_RTCDTMFToneChangeEvent(
                self_: <&RtcdtmfToneChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tone_RTCDTMFToneChangeEvent(
            self_: <&RtcdtmfToneChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&RtcdtmfToneChangeEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_tone_RTCDTMFToneChangeEvent(self_)
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
pub static __WASM_BINDGEN_GENERATED_1299f32c9a4da1d3: [u8; 456usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x86\x01\0\0\0\0\x04\0\0\x02\x16RTCDTMFToneChangeEvent(__widl_instanceof_RTCDTMFToneChangeEvent\0\0\0\0#__widl_f_new_RTCDTMFToneChangeEvent\x01\0\0\x01\x16RTCDTMFToneChangeEvent\0\x01\x01\x05type_\x03new\0\0\08__widl_f_new_with_event_init_dict_RTCDTMFToneChangeEvent\x01\0\0\x01\x16RTCDTMFToneChangeEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0$__widl_f_tone_RTCDTMFToneChangeEvent\0\0\0\x01\x16RTCDTMFToneChangeEvent\x01\0\x01\x04tone\x01\x01\x05self_\x04tone\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
