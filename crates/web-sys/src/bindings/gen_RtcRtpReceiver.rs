use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `RTCRtpReceiver` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver)\n\n*This API requires the following crate features to be activated: `RtcRtpReceiver`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct RtcRtpReceiver {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_RtcRtpReceiver: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for RtcRtpReceiver {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(82u32);
            inform(84u32);
            inform(67u32);
            inform(82u32);
            inform(116u32);
            inform(112u32);
            inform(82u32);
            inform(101u32);
            inform(99u32);
            inform(101u32);
            inform(105u32);
            inform(118u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for RtcRtpReceiver {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for RtcRtpReceiver {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for RtcRtpReceiver {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a RtcRtpReceiver {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for RtcRtpReceiver {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            RtcRtpReceiver {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for RtcRtpReceiver {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a RtcRtpReceiver {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for RtcRtpReceiver {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<RtcRtpReceiver>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(RtcRtpReceiver {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for RtcRtpReceiver {
        #[inline]
        fn from(obj: JsValue) -> RtcRtpReceiver {
            RtcRtpReceiver { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for RtcRtpReceiver {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<RtcRtpReceiver> for RtcRtpReceiver {
        #[inline]
        fn as_ref(&self) -> &RtcRtpReceiver {
            self
        }
    }
    impl From<RtcRtpReceiver> for JsValue {
        #[inline]
        fn from(obj: RtcRtpReceiver) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for RtcRtpReceiver {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_RTCRtpReceiver(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_RTCRtpReceiver(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_RTCRtpReceiver(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            RtcRtpReceiver { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const RtcRtpReceiver) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<RtcRtpReceiver> for ::js_sys::Object {
    #[inline]
    fn from(obj: RtcRtpReceiver) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for RtcRtpReceiver {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "RtcRtpReceiver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_contributing_sources_RTCRtpReceiver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcRtpReceiver as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl RtcRtpReceiver {
    #[cfg(all(feature = "RtcRtpReceiver",))]
    #[allow(bad_style)]
    #[doc = "The `getContributingSources()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/getContributingSources)\n\n*This API requires the following crate features to be activated: `RtcRtpReceiver`*"]
    #[allow(clippy::all)]
    pub fn get_contributing_sources(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "RtcRtpReceiver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_contributing_sources_RTCRtpReceiver(
                self_: <&RtcRtpReceiver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_contributing_sources_RTCRtpReceiver(
            self_: <&RtcRtpReceiver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcRtpReceiver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_contributing_sources_RTCRtpReceiver(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcRtpReceiver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_stats_RTCRtpReceiver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcRtpReceiver as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcRtpReceiver {
    #[cfg(all(feature = "RtcRtpReceiver",))]
    #[allow(bad_style)]
    #[doc = "The `getStats()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/getStats)\n\n*This API requires the following crate features to be activated: `RtcRtpReceiver`*"]
    #[allow(clippy::all)]
    pub fn get_stats(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "RtcRtpReceiver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_stats_RTCRtpReceiver(
                self_: <&RtcRtpReceiver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_stats_RTCRtpReceiver(
            self_: <&RtcRtpReceiver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcRtpReceiver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_stats_RTCRtpReceiver(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcRtpReceiver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_synchronization_sources_RTCRtpReceiver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcRtpReceiver as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl RtcRtpReceiver {
    #[cfg(all(feature = "RtcRtpReceiver",))]
    #[allow(bad_style)]
    #[doc = "The `getSynchronizationSources()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/getSynchronizationSources)\n\n*This API requires the following crate features to be activated: `RtcRtpReceiver`*"]
    #[allow(clippy::all)]
    pub fn get_synchronization_sources(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "RtcRtpReceiver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_synchronization_sources_RTCRtpReceiver(
                self_: <&RtcRtpReceiver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_synchronization_sources_RTCRtpReceiver(
            self_: <&RtcRtpReceiver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcRtpReceiver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_synchronization_sources_RTCRtpReceiver(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack", feature = "RtcRtpReceiver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_track_RTCRtpReceiver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcRtpReceiver as WasmDescribe>::describe();
    <MediaStreamTrack as WasmDescribe>::describe();
}
impl RtcRtpReceiver {
    #[cfg(all(feature = "MediaStreamTrack", feature = "RtcRtpReceiver",))]
    #[allow(bad_style)]
    #[doc = "The `track` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/track)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcRtpReceiver`*"]
    #[allow(clippy::all)]
    pub fn track(&self) -> MediaStreamTrack {
        #[cfg(all(feature = "MediaStreamTrack", feature = "RtcRtpReceiver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_track_RTCRtpReceiver(
                self_: <&RtcRtpReceiver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaStreamTrack as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_track_RTCRtpReceiver(
            self_: <&RtcRtpReceiver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaStreamTrack as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcRtpReceiver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_track_RTCRtpReceiver(self_)
            };
            <MediaStreamTrack as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_c616d2ec348a9f8e: [u8; 524usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xCA\x01\0\0\0\0\x05\0\0\x02\x0ERTCRtpReceiver __widl_instanceof_RTCRtpReceiver\0\0\0\00__widl_f_get_contributing_sources_RTCRtpReceiver\0\0\0\x01\x0ERTCRtpReceiver\x01\0\0\x01\x01\x05self_\x16getContributingSources\0\0\0!__widl_f_get_stats_RTCRtpReceiver\0\0\0\x01\x0ERTCRtpReceiver\x01\0\0\x01\x01\x05self_\x08getStats\0\0\03__widl_f_get_synchronization_sources_RTCRtpReceiver\0\0\0\x01\x0ERTCRtpReceiver\x01\0\0\x01\x01\x05self_\x19getSynchronizationSources\0\0\0\x1D__widl_f_track_RTCRtpReceiver\0\0\0\x01\x0ERTCRtpReceiver\x01\0\x01\x05track\x01\x01\x05self_\x05track\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
