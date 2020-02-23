use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `RTCRtpSender` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender)\n\n*This API requires the following crate features to be activated: `RtcRtpSender`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct RtcRtpSender {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_RtcRtpSender: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for RtcRtpSender {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(82u32);
            inform(84u32);
            inform(67u32);
            inform(82u32);
            inform(116u32);
            inform(112u32);
            inform(83u32);
            inform(101u32);
            inform(110u32);
            inform(100u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for RtcRtpSender {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for RtcRtpSender {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for RtcRtpSender {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a RtcRtpSender {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for RtcRtpSender {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            RtcRtpSender {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for RtcRtpSender {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a RtcRtpSender {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for RtcRtpSender {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<RtcRtpSender>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(RtcRtpSender {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for RtcRtpSender {
        #[inline]
        fn from(obj: JsValue) -> RtcRtpSender {
            RtcRtpSender { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for RtcRtpSender {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<RtcRtpSender> for RtcRtpSender {
        #[inline]
        fn as_ref(&self) -> &RtcRtpSender {
            self
        }
    }
    impl From<RtcRtpSender> for JsValue {
        #[inline]
        fn from(obj: RtcRtpSender) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for RtcRtpSender {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_RTCRtpSender(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_RTCRtpSender(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_RTCRtpSender(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            RtcRtpSender { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const RtcRtpSender) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<RtcRtpSender> for ::js_sys::Object {
    #[inline]
    fn from(obj: RtcRtpSender) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for RtcRtpSender {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "RtcRtpParameters", feature = "RtcRtpSender",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_parameters_RTCRtpSender() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcRtpSender as WasmDescribe>::describe();
    <RtcRtpParameters as WasmDescribe>::describe();
}
impl RtcRtpSender {
    #[cfg(all(feature = "RtcRtpParameters", feature = "RtcRtpSender",))]
    #[allow(bad_style)]
    #[doc = "The `getParameters()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/getParameters)\n\n*This API requires the following crate features to be activated: `RtcRtpParameters`, `RtcRtpSender`*"]
    #[allow(clippy::all)]
    pub fn get_parameters(&self) -> RtcRtpParameters {
        #[cfg(all(feature = "RtcRtpParameters", feature = "RtcRtpSender",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_parameters_RTCRtpSender(
                self_: <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcRtpParameters as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_parameters_RTCRtpSender(
            self_: <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcRtpParameters as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_parameters_RTCRtpSender(self_)
            };
            <RtcRtpParameters as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcRtpSender",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_stats_RTCRtpSender() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcRtpSender as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcRtpSender {
    #[cfg(all(feature = "RtcRtpSender",))]
    #[allow(bad_style)]
    #[doc = "The `getStats()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/getStats)\n\n*This API requires the following crate features to be activated: `RtcRtpSender`*"]
    #[allow(clippy::all)]
    pub fn get_stats(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "RtcRtpSender",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_stats_RTCRtpSender(
                self_: <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_stats_RTCRtpSender(
            self_: <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_stats_RTCRtpSender(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack", feature = "RtcRtpSender",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_track_RTCRtpSender() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcRtpSender as WasmDescribe>::describe();
    <Option<&MediaStreamTrack> as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcRtpSender {
    #[cfg(all(feature = "MediaStreamTrack", feature = "RtcRtpSender",))]
    #[allow(bad_style)]
    #[doc = "The `replaceTrack()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/replaceTrack)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcRtpSender`*"]
    #[allow(clippy::all)]
    pub fn replace_track(&self, with_track: Option<&MediaStreamTrack>) -> ::js_sys::Promise {
        #[cfg(all(feature = "MediaStreamTrack", feature = "RtcRtpSender",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_track_RTCRtpSender(
                self_: <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                with_track: <Option<&MediaStreamTrack> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_track_RTCRtpSender(
            self_: <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            with_track: <Option<&MediaStreamTrack> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(with_track);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let with_track =
                    <Option<&MediaStreamTrack> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        with_track,
                    );
                __widl_f_replace_track_RTCRtpSender(self_, with_track)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcRtpSender",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_parameters_RTCRtpSender() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcRtpSender as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcRtpSender {
    #[cfg(all(feature = "RtcRtpSender",))]
    #[allow(bad_style)]
    #[doc = "The `setParameters()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/setParameters)\n\n*This API requires the following crate features to be activated: `RtcRtpSender`*"]
    #[allow(clippy::all)]
    pub fn set_parameters(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "RtcRtpSender",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_parameters_RTCRtpSender(
                self_: <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_parameters_RTCRtpSender(
            self_: <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_set_parameters_RTCRtpSender(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcRtpParameters", feature = "RtcRtpSender",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_parameters_with_parameters_RTCRtpSender() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcRtpSender as WasmDescribe>::describe();
    <&RtcRtpParameters as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcRtpSender {
    #[cfg(all(feature = "RtcRtpParameters", feature = "RtcRtpSender",))]
    #[allow(bad_style)]
    #[doc = "The `setParameters()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/setParameters)\n\n*This API requires the following crate features to be activated: `RtcRtpParameters`, `RtcRtpSender`*"]
    #[allow(clippy::all)]
    pub fn set_parameters_with_parameters(
        &self,
        parameters: &RtcRtpParameters,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "RtcRtpParameters", feature = "RtcRtpSender",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_parameters_with_parameters_RTCRtpSender(
                self_: <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                parameters: <&RtcRtpParameters as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_parameters_with_parameters_RTCRtpSender(
            self_: <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            parameters: <&RtcRtpParameters as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(parameters);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let parameters =
                    <&RtcRtpParameters as wasm_bindgen::convert::IntoWasmAbi>::into_abi(parameters);
                __widl_f_set_parameters_with_parameters_RTCRtpSender(self_, parameters)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack", feature = "RtcRtpSender",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_track_RTCRtpSender() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcRtpSender as WasmDescribe>::describe();
    <Option<MediaStreamTrack> as WasmDescribe>::describe();
}
impl RtcRtpSender {
    #[cfg(all(feature = "MediaStreamTrack", feature = "RtcRtpSender",))]
    #[allow(bad_style)]
    #[doc = "The `track` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/track)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcRtpSender`*"]
    #[allow(clippy::all)]
    pub fn track(&self) -> Option<MediaStreamTrack> {
        #[cfg(all(feature = "MediaStreamTrack", feature = "RtcRtpSender",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_track_RTCRtpSender(
                self_: <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<MediaStreamTrack> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_track_RTCRtpSender(
            self_: <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<MediaStreamTrack> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_track_RTCRtpSender(self_)
            };
            <Option<MediaStreamTrack> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcRtpSender", feature = "RtcdtmfSender",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dtmf_RTCRtpSender() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcRtpSender as WasmDescribe>::describe();
    <Option<RtcdtmfSender> as WasmDescribe>::describe();
}
impl RtcRtpSender {
    #[cfg(all(feature = "RtcRtpSender", feature = "RtcdtmfSender",))]
    #[allow(bad_style)]
    #[doc = "The `dtmf` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/dtmf)\n\n*This API requires the following crate features to be activated: `RtcRtpSender`, `RtcdtmfSender`*"]
    #[allow(clippy::all)]
    pub fn dtmf(&self) -> Option<RtcdtmfSender> {
        #[cfg(all(feature = "RtcRtpSender", feature = "RtcdtmfSender",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dtmf_RTCRtpSender(
                self_: <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<RtcdtmfSender> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dtmf_RTCRtpSender(
            self_: <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<RtcdtmfSender> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_dtmf_RTCRtpSender(self_)
            };
            <Option<RtcdtmfSender> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_476282797fe7623c: [u8; 728usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x96\x02\0\0\0\0\x08\0\0\x02\x0CRTCRtpSender\x1E__widl_instanceof_RTCRtpSender\0\0\0\0$__widl_f_get_parameters_RTCRtpSender\0\0\0\x01\x0CRTCRtpSender\x01\0\0\x01\x01\x05self_\rgetParameters\0\0\0\x1F__widl_f_get_stats_RTCRtpSender\0\0\0\x01\x0CRTCRtpSender\x01\0\0\x01\x01\x05self_\x08getStats\0\0\0#__widl_f_replace_track_RTCRtpSender\0\0\0\x01\x0CRTCRtpSender\x01\0\0\x01\x02\x05self_\nwith_track\x0CreplaceTrack\0\0\0$__widl_f_set_parameters_RTCRtpSender\0\0\0\x01\x0CRTCRtpSender\x01\0\0\x01\x01\x05self_\rsetParameters\0\0\04__widl_f_set_parameters_with_parameters_RTCRtpSender\0\0\0\x01\x0CRTCRtpSender\x01\0\0\x01\x02\x05self_\nparameters\rsetParameters\0\0\0\x1B__widl_f_track_RTCRtpSender\0\0\0\x01\x0CRTCRtpSender\x01\0\x01\x05track\x01\x01\x05self_\x05track\0\0\0\x1A__widl_f_dtmf_RTCRtpSender\0\0\0\x01\x0CRTCRtpSender\x01\0\x01\x04dtmf\x01\x01\x05self_\x04dtmf\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
