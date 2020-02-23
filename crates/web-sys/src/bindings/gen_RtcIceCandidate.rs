use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `RTCIceCandidate` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct RtcIceCandidate {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_RtcIceCandidate: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for RtcIceCandidate {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(82u32);
            inform(84u32);
            inform(67u32);
            inform(73u32);
            inform(99u32);
            inform(101u32);
            inform(67u32);
            inform(97u32);
            inform(110u32);
            inform(100u32);
            inform(105u32);
            inform(100u32);
            inform(97u32);
            inform(116u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for RtcIceCandidate {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for RtcIceCandidate {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for RtcIceCandidate {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a RtcIceCandidate {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for RtcIceCandidate {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            RtcIceCandidate {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for RtcIceCandidate {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a RtcIceCandidate {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for RtcIceCandidate {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<RtcIceCandidate>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(RtcIceCandidate {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for RtcIceCandidate {
        #[inline]
        fn from(obj: JsValue) -> RtcIceCandidate {
            RtcIceCandidate { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for RtcIceCandidate {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<RtcIceCandidate> for RtcIceCandidate {
        #[inline]
        fn as_ref(&self) -> &RtcIceCandidate {
            self
        }
    }
    impl From<RtcIceCandidate> for JsValue {
        #[inline]
        fn from(obj: RtcIceCandidate) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for RtcIceCandidate {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_RTCIceCandidate(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_RTCIceCandidate(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_RTCIceCandidate(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            RtcIceCandidate { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const RtcIceCandidate) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<RtcIceCandidate> for ::js_sys::Object {
    #[inline]
    fn from(obj: RtcIceCandidate) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for RtcIceCandidate {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "RtcIceCandidate", feature = "RtcIceCandidateInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_RTCIceCandidate() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcIceCandidateInit as WasmDescribe>::describe();
    <RtcIceCandidate as WasmDescribe>::describe();
}
impl RtcIceCandidate {
    #[cfg(all(feature = "RtcIceCandidate", feature = "RtcIceCandidateInit",))]
    #[allow(bad_style)]
    #[doc = "The `new RTCIceCandidate(..)` constructor, creating a new instance of `RTCIceCandidate`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/RTCIceCandidate)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcIceCandidateInit`*"]
    #[allow(clippy::all)]
    pub fn new(
        candidate_init_dict: &RtcIceCandidateInit,
    ) -> Result<RtcIceCandidate, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "RtcIceCandidate", feature = "RtcIceCandidateInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_RTCIceCandidate(
                candidate_init_dict : < & RtcIceCandidateInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <RtcIceCandidate as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_RTCIceCandidate(
            candidate_init_dict: <&RtcIceCandidateInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcIceCandidate as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(candidate_init_dict);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let candidate_init_dict =
                    <&RtcIceCandidateInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        candidate_init_dict,
                    );
                __widl_f_new_RTCIceCandidate(candidate_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<RtcIceCandidate as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "RtcIceCandidate",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_json_RTCIceCandidate() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcIceCandidate as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl RtcIceCandidate {
    #[cfg(all(feature = "RtcIceCandidate",))]
    #[allow(bad_style)]
    #[doc = "The `toJSON()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/toJSON)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    #[allow(clippy::all)]
    pub fn to_json(&self) -> ::js_sys::Object {
        #[cfg(all(feature = "RtcIceCandidate",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_json_RTCIceCandidate(
                self_: <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_json_RTCIceCandidate(
            self_: <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_to_json_RTCIceCandidate(self_)
            };
            <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcIceCandidate",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_candidate_RTCIceCandidate() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcIceCandidate as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl RtcIceCandidate {
    #[cfg(all(feature = "RtcIceCandidate",))]
    #[allow(bad_style)]
    #[doc = "The `candidate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/candidate)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    #[allow(clippy::all)]
    pub fn candidate(&self) -> String {
        #[cfg(all(feature = "RtcIceCandidate",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_candidate_RTCIceCandidate(
                self_: <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_candidate_RTCIceCandidate(
            self_: <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_candidate_RTCIceCandidate(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcIceCandidate",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_candidate_RTCIceCandidate() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcIceCandidate as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcIceCandidate {
    #[cfg(all(feature = "RtcIceCandidate",))]
    #[allow(bad_style)]
    #[doc = "The `candidate` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/candidate)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    #[allow(clippy::all)]
    pub fn set_candidate(&self, candidate: &str) {
        #[cfg(all(feature = "RtcIceCandidate",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_candidate_RTCIceCandidate(
                self_: <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                candidate: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_candidate_RTCIceCandidate(
            self_: <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            candidate: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(candidate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let candidate = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(candidate);
                __widl_f_set_candidate_RTCIceCandidate(self_, candidate)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcIceCandidate",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sdp_mid_RTCIceCandidate() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcIceCandidate as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl RtcIceCandidate {
    #[cfg(all(feature = "RtcIceCandidate",))]
    #[allow(bad_style)]
    #[doc = "The `sdpMid` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMid)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    #[allow(clippy::all)]
    pub fn sdp_mid(&self) -> Option<String> {
        #[cfg(all(feature = "RtcIceCandidate",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sdp_mid_RTCIceCandidate(
                self_: <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sdp_mid_RTCIceCandidate(
            self_: <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_sdp_mid_RTCIceCandidate(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcIceCandidate",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_sdp_mid_RTCIceCandidate() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcIceCandidate as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcIceCandidate {
    #[cfg(all(feature = "RtcIceCandidate",))]
    #[allow(bad_style)]
    #[doc = "The `sdpMid` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMid)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    #[allow(clippy::all)]
    pub fn set_sdp_mid(&self, sdp_mid: Option<&str>) {
        #[cfg(all(feature = "RtcIceCandidate",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_sdp_mid_RTCIceCandidate(
                self_: <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sdp_mid: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_sdp_mid_RTCIceCandidate(
            self_: <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sdp_mid: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(sdp_mid);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let sdp_mid =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sdp_mid);
                __widl_f_set_sdp_mid_RTCIceCandidate(self_, sdp_mid)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcIceCandidate",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sdp_m_line_index_RTCIceCandidate() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcIceCandidate as WasmDescribe>::describe();
    <Option<u16> as WasmDescribe>::describe();
}
impl RtcIceCandidate {
    #[cfg(all(feature = "RtcIceCandidate",))]
    #[allow(bad_style)]
    #[doc = "The `sdpMLineIndex` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMLineIndex)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    #[allow(clippy::all)]
    pub fn sdp_m_line_index(&self) -> Option<u16> {
        #[cfg(all(feature = "RtcIceCandidate",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sdp_m_line_index_RTCIceCandidate(
                self_: <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<u16> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sdp_m_line_index_RTCIceCandidate(
            self_: <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<u16> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_sdp_m_line_index_RTCIceCandidate(self_)
            };
            <Option<u16> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcIceCandidate",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_sdp_m_line_index_RTCIceCandidate() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcIceCandidate as WasmDescribe>::describe();
    <Option<u16> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcIceCandidate {
    #[cfg(all(feature = "RtcIceCandidate",))]
    #[allow(bad_style)]
    #[doc = "The `sdpMLineIndex` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMLineIndex)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    #[allow(clippy::all)]
    pub fn set_sdp_m_line_index(&self, sdp_m_line_index: Option<u16>) {
        #[cfg(all(feature = "RtcIceCandidate",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_sdp_m_line_index_RTCIceCandidate(
                self_: <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sdp_m_line_index: <Option<u16> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_sdp_m_line_index_RTCIceCandidate(
            self_: <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sdp_m_line_index: <Option<u16> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(sdp_m_line_index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let sdp_m_line_index =
                    <Option<u16> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sdp_m_line_index);
                __widl_f_set_sdp_m_line_index_RTCIceCandidate(self_, sdp_m_line_index)
            };
            ()
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_b4dcd265670cd90a: [u8; 907usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}I\x03\0\0\0\0\t\0\0\x02\x0FRTCIceCandidate!__widl_instanceof_RTCIceCandidate\0\0\0\0\x1C__widl_f_new_RTCIceCandidate\x01\0\0\x01\x0FRTCIceCandidate\0\x01\x01\x13candidate_init_dict\x03new\0\0\0 __widl_f_to_json_RTCIceCandidate\0\0\0\x01\x0FRTCIceCandidate\x01\0\0\x01\x01\x05self_\x06toJSON\0\0\0\"__widl_f_candidate_RTCIceCandidate\0\0\0\x01\x0FRTCIceCandidate\x01\0\x01\tcandidate\x01\x01\x05self_\tcandidate\0\0\0&__widl_f_set_candidate_RTCIceCandidate\0\0\0\x01\x0FRTCIceCandidate\x01\0\x02\tcandidate\x01\x02\x05self_\tcandidate\tcandidate\0\0\0 __widl_f_sdp_mid_RTCIceCandidate\0\0\0\x01\x0FRTCIceCandidate\x01\0\x01\x06sdpMid\x01\x01\x05self_\x06sdpMid\0\0\0$__widl_f_set_sdp_mid_RTCIceCandidate\0\0\0\x01\x0FRTCIceCandidate\x01\0\x02\x06sdpMid\x01\x02\x05self_\x07sdp_mid\x06sdpMid\0\0\0)__widl_f_sdp_m_line_index_RTCIceCandidate\0\0\0\x01\x0FRTCIceCandidate\x01\0\x01\rsdpMLineIndex\x01\x01\x05self_\rsdpMLineIndex\0\0\0-__widl_f_set_sdp_m_line_index_RTCIceCandidate\0\0\0\x01\x0FRTCIceCandidate\x01\0\x02\rsdpMLineIndex\x01\x02\x05self_\x10sdp_m_line_index\rsdpMLineIndex\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
