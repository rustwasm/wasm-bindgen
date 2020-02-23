use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `RTCDTMFSender` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender)\n\n*This API requires the following crate features to be activated: `RtcdtmfSender`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct RtcdtmfSender {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_RtcdtmfSender: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for RtcdtmfSender {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(82u32);
            inform(84u32);
            inform(67u32);
            inform(68u32);
            inform(84u32);
            inform(77u32);
            inform(70u32);
            inform(83u32);
            inform(101u32);
            inform(110u32);
            inform(100u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for RtcdtmfSender {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for RtcdtmfSender {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for RtcdtmfSender {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a RtcdtmfSender {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for RtcdtmfSender {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            RtcdtmfSender {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for RtcdtmfSender {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a RtcdtmfSender {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for RtcdtmfSender {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<RtcdtmfSender>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(RtcdtmfSender {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for RtcdtmfSender {
        #[inline]
        fn from(obj: JsValue) -> RtcdtmfSender {
            RtcdtmfSender { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for RtcdtmfSender {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<RtcdtmfSender> for RtcdtmfSender {
        #[inline]
        fn as_ref(&self) -> &RtcdtmfSender {
            self
        }
    }
    impl From<RtcdtmfSender> for JsValue {
        #[inline]
        fn from(obj: RtcdtmfSender) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for RtcdtmfSender {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_RTCDTMFSender(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_RTCDTMFSender(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_RTCDTMFSender(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            RtcdtmfSender { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const RtcdtmfSender) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<RtcdtmfSender> for EventTarget {
    #[inline]
    fn from(obj: RtcdtmfSender) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for RtcdtmfSender {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<RtcdtmfSender> for ::js_sys::Object {
    #[inline]
    fn from(obj: RtcdtmfSender) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for RtcdtmfSender {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "RtcdtmfSender",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_insert_dtmf_RTCDTMFSender() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcdtmfSender as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcdtmfSender {
    #[cfg(all(feature = "RtcdtmfSender",))]
    #[allow(bad_style)]
    #[doc = "The `insertDTMF()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/insertDTMF)\n\n*This API requires the following crate features to be activated: `RtcdtmfSender`*"]
    #[allow(clippy::all)]
    pub fn insert_dtmf(&self, tones: &str) {
        #[cfg(all(feature = "RtcdtmfSender",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_insert_dtmf_RTCDTMFSender(
                self_: <&RtcdtmfSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tones: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_insert_dtmf_RTCDTMFSender(
            self_: <&RtcdtmfSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tones: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tones);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcdtmfSender as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tones = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tones);
                __widl_f_insert_dtmf_RTCDTMFSender(self_, tones)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcdtmfSender",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_insert_dtmf_with_duration_RTCDTMFSender() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&RtcdtmfSender as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcdtmfSender {
    #[cfg(all(feature = "RtcdtmfSender",))]
    #[allow(bad_style)]
    #[doc = "The `insertDTMF()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/insertDTMF)\n\n*This API requires the following crate features to be activated: `RtcdtmfSender`*"]
    #[allow(clippy::all)]
    pub fn insert_dtmf_with_duration(&self, tones: &str, duration: u32) {
        #[cfg(all(feature = "RtcdtmfSender",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_insert_dtmf_with_duration_RTCDTMFSender(
                self_: <&RtcdtmfSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tones: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                duration: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_insert_dtmf_with_duration_RTCDTMFSender(
            self_: <&RtcdtmfSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tones: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            duration: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tones);
            drop(duration);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcdtmfSender as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tones = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tones);
                let duration = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(duration);
                __widl_f_insert_dtmf_with_duration_RTCDTMFSender(self_, tones, duration)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcdtmfSender",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_insert_dtmf_with_duration_and_inter_tone_gap_RTCDTMFSender(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&RtcdtmfSender as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcdtmfSender {
    #[cfg(all(feature = "RtcdtmfSender",))]
    #[allow(bad_style)]
    #[doc = "The `insertDTMF()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/insertDTMF)\n\n*This API requires the following crate features to be activated: `RtcdtmfSender`*"]
    #[allow(clippy::all)]
    pub fn insert_dtmf_with_duration_and_inter_tone_gap(
        &self,
        tones: &str,
        duration: u32,
        inter_tone_gap: u32,
    ) {
        #[cfg(all(feature = "RtcdtmfSender",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_insert_dtmf_with_duration_and_inter_tone_gap_RTCDTMFSender(
                self_: <&RtcdtmfSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tones: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                duration: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                inter_tone_gap: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_insert_dtmf_with_duration_and_inter_tone_gap_RTCDTMFSender(
            self_: <&RtcdtmfSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tones: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            duration: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            inter_tone_gap: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tones);
            drop(duration);
            drop(inter_tone_gap);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcdtmfSender as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tones = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tones);
                let duration = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(duration);
                let inter_tone_gap =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(inter_tone_gap);
                __widl_f_insert_dtmf_with_duration_and_inter_tone_gap_RTCDTMFSender(
                    self_,
                    tones,
                    duration,
                    inter_tone_gap,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcdtmfSender",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontonechange_RTCDTMFSender() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcdtmfSender as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl RtcdtmfSender {
    #[cfg(all(feature = "RtcdtmfSender",))]
    #[allow(bad_style)]
    #[doc = "The `ontonechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/ontonechange)\n\n*This API requires the following crate features to be activated: `RtcdtmfSender`*"]
    #[allow(clippy::all)]
    pub fn ontonechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "RtcdtmfSender",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontonechange_RTCDTMFSender(
                self_: <&RtcdtmfSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontonechange_RTCDTMFSender(
            self_: <&RtcdtmfSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcdtmfSender as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontonechange_RTCDTMFSender(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcdtmfSender",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontonechange_RTCDTMFSender() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcdtmfSender as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcdtmfSender {
    #[cfg(all(feature = "RtcdtmfSender",))]
    #[allow(bad_style)]
    #[doc = "The `ontonechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/ontonechange)\n\n*This API requires the following crate features to be activated: `RtcdtmfSender`*"]
    #[allow(clippy::all)]
    pub fn set_ontonechange(&self, ontonechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "RtcdtmfSender",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontonechange_RTCDTMFSender(
                self_: <&RtcdtmfSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontonechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontonechange_RTCDTMFSender(
            self_: <&RtcdtmfSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontonechange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontonechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcdtmfSender as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontonechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontonechange,
                    );
                __widl_f_set_ontonechange_RTCDTMFSender(self_, ontonechange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcdtmfSender",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tone_buffer_RTCDTMFSender() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcdtmfSender as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl RtcdtmfSender {
    #[cfg(all(feature = "RtcdtmfSender",))]
    #[allow(bad_style)]
    #[doc = "The `toneBuffer` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/toneBuffer)\n\n*This API requires the following crate features to be activated: `RtcdtmfSender`*"]
    #[allow(clippy::all)]
    pub fn tone_buffer(&self) -> String {
        #[cfg(all(feature = "RtcdtmfSender",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tone_buffer_RTCDTMFSender(
                self_: <&RtcdtmfSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tone_buffer_RTCDTMFSender(
            self_: <&RtcdtmfSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcdtmfSender as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_tone_buffer_RTCDTMFSender(self_)
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
pub static __WASM_BINDGEN_GENERATED_18e15bd659558b4b: [u8; 781usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xCB\x02\0\0\0\0\x07\0\0\x02\rRTCDTMFSender\x1F__widl_instanceof_RTCDTMFSender\0\0\0\0\"__widl_f_insert_dtmf_RTCDTMFSender\0\0\0\x01\rRTCDTMFSender\x01\0\0\x01\x02\x05self_\x05tones\ninsertDTMF\0\0\00__widl_f_insert_dtmf_with_duration_RTCDTMFSender\0\0\0\x01\rRTCDTMFSender\x01\0\0\x01\x03\x05self_\x05tones\x08duration\ninsertDTMF\0\0\0C__widl_f_insert_dtmf_with_duration_and_inter_tone_gap_RTCDTMFSender\0\0\0\x01\rRTCDTMFSender\x01\0\0\x01\x04\x05self_\x05tones\x08duration\x0Einter_tone_gap\ninsertDTMF\0\0\0#__widl_f_ontonechange_RTCDTMFSender\0\0\0\x01\rRTCDTMFSender\x01\0\x01\x0Contonechange\x01\x01\x05self_\x0Contonechange\0\0\0'__widl_f_set_ontonechange_RTCDTMFSender\0\0\0\x01\rRTCDTMFSender\x01\0\x02\x0Contonechange\x01\x02\x05self_\x0Contonechange\x0Contonechange\0\0\0\"__widl_f_tone_buffer_RTCDTMFSender\0\0\0\x01\rRTCDTMFSender\x01\0\x01\ntoneBuffer\x01\x01\x05self_\ntoneBuffer\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
