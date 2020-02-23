use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `RTCPeerConnection` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct RtcPeerConnection {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_RtcPeerConnection: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for RtcPeerConnection {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
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
        }
    }
    impl core::ops::Deref for RtcPeerConnection {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for RtcPeerConnection {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for RtcPeerConnection {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a RtcPeerConnection {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for RtcPeerConnection {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            RtcPeerConnection {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for RtcPeerConnection {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a RtcPeerConnection {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for RtcPeerConnection {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<RtcPeerConnection>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(RtcPeerConnection {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for RtcPeerConnection {
        #[inline]
        fn from(obj: JsValue) -> RtcPeerConnection {
            RtcPeerConnection { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for RtcPeerConnection {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<RtcPeerConnection> for RtcPeerConnection {
        #[inline]
        fn as_ref(&self) -> &RtcPeerConnection {
            self
        }
    }
    impl From<RtcPeerConnection> for JsValue {
        #[inline]
        fn from(obj: RtcPeerConnection) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for RtcPeerConnection {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_RTCPeerConnection(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_RTCPeerConnection(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_RTCPeerConnection(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            RtcPeerConnection { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const RtcPeerConnection) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<RtcPeerConnection> for EventTarget {
    #[inline]
    fn from(obj: RtcPeerConnection) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for RtcPeerConnection {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<RtcPeerConnection> for ::js_sys::Object {
    #[inline]
    fn from(obj: RtcPeerConnection) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for RtcPeerConnection {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <RtcPeerConnection as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `new RTCPeerConnection(..)` constructor, creating a new instance of `RTCPeerConnection`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/RTCPeerConnection)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<RtcPeerConnection, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_RTCPeerConnection(
            ) -> <RtcPeerConnection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_RTCPeerConnection(
        ) -> <RtcPeerConnection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_RTCPeerConnection() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<RtcPeerConnection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "RtcConfiguration", feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_configuration_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcConfiguration as WasmDescribe>::describe();
    <RtcPeerConnection as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcConfiguration", feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `new RTCPeerConnection(..)` constructor, creating a new instance of `RTCPeerConnection`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/RTCPeerConnection)\n\n*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn new_with_configuration(
        configuration: &RtcConfiguration,
    ) -> Result<RtcPeerConnection, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "RtcConfiguration", feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_configuration_RTCPeerConnection(
                configuration: <&RtcConfiguration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcPeerConnection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_configuration_RTCPeerConnection(
            configuration: <&RtcConfiguration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcPeerConnection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(configuration);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let configuration =
                    <&RtcConfiguration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        configuration,
                    );
                __widl_f_new_with_configuration_RTCPeerConnection(configuration)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<RtcPeerConnection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "RtcConfiguration", feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_configuration_and_constraints_RTCPeerConnection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcConfiguration as WasmDescribe>::describe();
    <Option<&::js_sys::Object> as WasmDescribe>::describe();
    <RtcPeerConnection as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcConfiguration", feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `new RTCPeerConnection(..)` constructor, creating a new instance of `RTCPeerConnection`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/RTCPeerConnection)\n\n*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn new_with_configuration_and_constraints(
        configuration: &RtcConfiguration,
        constraints: Option<&::js_sys::Object>,
    ) -> Result<RtcPeerConnection, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "RtcConfiguration", feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_configuration_and_constraints_RTCPeerConnection(
                configuration: <&RtcConfiguration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                constraints: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcPeerConnection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_configuration_and_constraints_RTCPeerConnection(
            configuration: <&RtcConfiguration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            constraints: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcPeerConnection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(configuration);
            drop(constraints);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let configuration =
                    <&RtcConfiguration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        configuration,
                    );
                let constraints =
                    <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        constraints,
                    );
                __widl_f_new_with_configuration_and_constraints_RTCPeerConnection(
                    configuration,
                    constraints,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<RtcPeerConnection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "RtcIceCandidateInit", feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_ice_candidate_with_opt_rtc_ice_candidate_init_RTCPeerConnection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<&RtcIceCandidateInit> as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcIceCandidateInit", feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `addIceCandidate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addIceCandidate)\n\n*This API requires the following crate features to be activated: `RtcIceCandidateInit`, `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn add_ice_candidate_with_opt_rtc_ice_candidate_init(
        &self,
        candidate: Option<&RtcIceCandidateInit>,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "RtcIceCandidateInit", feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_ice_candidate_with_opt_rtc_ice_candidate_init_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                candidate : < Option < & RtcIceCandidateInit > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_ice_candidate_with_opt_rtc_ice_candidate_init_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            candidate: <Option<&RtcIceCandidateInit> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let candidate =
                    <Option<&RtcIceCandidateInit> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        candidate,
                    );
                __widl_f_add_ice_candidate_with_opt_rtc_ice_candidate_init_RTCPeerConnection(
                    self_, candidate,
                )
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcIceCandidate", feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_ice_candidate_with_opt_rtc_ice_candidate_RTCPeerConnection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<&RtcIceCandidate> as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcIceCandidate", feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `addIceCandidate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addIceCandidate)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn add_ice_candidate_with_opt_rtc_ice_candidate(
        &self,
        candidate: Option<&RtcIceCandidate>,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "RtcIceCandidate", feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_ice_candidate_with_opt_rtc_ice_candidate_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                candidate: <Option<&RtcIceCandidate> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_ice_candidate_with_opt_rtc_ice_candidate_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            candidate: <Option<&RtcIceCandidate> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let candidate =
                    <Option<&RtcIceCandidate> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        candidate,
                    );
                __widl_f_add_ice_candidate_with_opt_rtc_ice_candidate_RTCPeerConnection(
                    self_, candidate,
                )
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcIceCandidate", feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_ice_candidate_with_rtc_ice_candidate_and_success_callback_and_failure_callback_RTCPeerConnection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&RtcIceCandidate as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcIceCandidate", feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `addIceCandidate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addIceCandidate)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn add_ice_candidate_with_rtc_ice_candidate_and_success_callback_and_failure_callback(
        &self,
        candidate: &RtcIceCandidate,
        success_callback: &::js_sys::Function,
        failure_callback: &::js_sys::Function,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "RtcIceCandidate", feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_ice_candidate_with_rtc_ice_candidate_and_success_callback_and_failure_callback_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                candidate: <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                failure_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_ice_candidate_with_rtc_ice_candidate_and_success_callback_and_failure_callback_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            candidate: <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            failure_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(candidate);
            drop(success_callback);
            drop(failure_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let candidate =
                    <&RtcIceCandidate as wasm_bindgen::convert::IntoWasmAbi>::into_abi(candidate);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let failure_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        failure_callback,
                    );
                __widl_f_add_ice_candidate_with_rtc_ice_candidate_and_success_callback_and_failure_callback_RTCPeerConnection ( self_ , candidate , success_callback , failure_callback )
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStream", feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_stream_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "MediaStream", feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `addStream()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addStream)\n\n*This API requires the following crate features to be activated: `MediaStream`, `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn add_stream(&self, stream: &MediaStream) {
        #[cfg(all(feature = "MediaStream", feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_stream_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_stream_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(stream);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let stream = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(stream);
                __widl_f_add_stream_RTCPeerConnection(self_, stream)
            };
            ()
        }
    }
}
#[cfg(all(
    feature = "MediaStream",
    feature = "MediaStreamTrack",
    feature = "RtcPeerConnection",
    feature = "RtcRtpSender",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_track_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&MediaStreamTrack as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <RtcRtpSender as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcPeerConnection",
        feature = "RtcRtpSender",
    ))]
    #[allow(bad_style)]
    #[doc = "The `addTrack()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    #[allow(clippy::all)]
    pub fn add_track(
        &self,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams: &::js_sys::Array,
    ) -> RtcRtpSender {
        #[cfg(all(
            feature = "MediaStream",
            feature = "MediaStreamTrack",
            feature = "RtcPeerConnection",
            feature = "RtcRtpSender",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_track_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_track_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(track);
            drop(stream);
            drop(more_streams);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let track =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(track);
                let stream = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(stream);
                let more_streams =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        more_streams,
                    );
                __widl_f_add_track_RTCPeerConnection(self_, track, stream, more_streams)
            };
            <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "MediaStream",
    feature = "MediaStreamTrack",
    feature = "RtcPeerConnection",
    feature = "RtcRtpSender",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_track_0_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&MediaStreamTrack as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <RtcRtpSender as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcPeerConnection",
        feature = "RtcRtpSender",
    ))]
    #[allow(bad_style)]
    #[doc = "The `addTrack()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    #[allow(clippy::all)]
    pub fn add_track_0(&self, track: &MediaStreamTrack, stream: &MediaStream) -> RtcRtpSender {
        #[cfg(all(
            feature = "MediaStream",
            feature = "MediaStreamTrack",
            feature = "RtcPeerConnection",
            feature = "RtcRtpSender",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_track_0_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_track_0_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(track);
            drop(stream);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let track =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(track);
                let stream = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(stream);
                __widl_f_add_track_0_RTCPeerConnection(self_, track, stream)
            };
            <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "MediaStream",
    feature = "MediaStreamTrack",
    feature = "RtcPeerConnection",
    feature = "RtcRtpSender",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_track_1_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&MediaStreamTrack as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <RtcRtpSender as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcPeerConnection",
        feature = "RtcRtpSender",
    ))]
    #[allow(bad_style)]
    #[doc = "The `addTrack()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    #[allow(clippy::all)]
    pub fn add_track_1(
        &self,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams_1: &MediaStream,
    ) -> RtcRtpSender {
        #[cfg(all(
            feature = "MediaStream",
            feature = "MediaStreamTrack",
            feature = "RtcPeerConnection",
            feature = "RtcRtpSender",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_track_1_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_1: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_track_1_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_1: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(track);
            drop(stream);
            drop(more_streams_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let track =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(track);
                let stream = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(stream);
                let more_streams_1 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_1);
                __widl_f_add_track_1_RTCPeerConnection(self_, track, stream, more_streams_1)
            };
            <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "MediaStream",
    feature = "MediaStreamTrack",
    feature = "RtcPeerConnection",
    feature = "RtcRtpSender",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_track_2_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&MediaStreamTrack as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <RtcRtpSender as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcPeerConnection",
        feature = "RtcRtpSender",
    ))]
    #[allow(bad_style)]
    #[doc = "The `addTrack()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    #[allow(clippy::all)]
    pub fn add_track_2(
        &self,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams_1: &MediaStream,
        more_streams_2: &MediaStream,
    ) -> RtcRtpSender {
        #[cfg(all(
            feature = "MediaStream",
            feature = "MediaStreamTrack",
            feature = "RtcPeerConnection",
            feature = "RtcRtpSender",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_track_2_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_1: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_2: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_track_2_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_1: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_2: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(track);
            drop(stream);
            drop(more_streams_1);
            drop(more_streams_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let track =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(track);
                let stream = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(stream);
                let more_streams_1 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_1);
                let more_streams_2 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_2);
                __widl_f_add_track_2_RTCPeerConnection(
                    self_,
                    track,
                    stream,
                    more_streams_1,
                    more_streams_2,
                )
            };
            <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "MediaStream",
    feature = "MediaStreamTrack",
    feature = "RtcPeerConnection",
    feature = "RtcRtpSender",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_track_3_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&MediaStreamTrack as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <RtcRtpSender as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcPeerConnection",
        feature = "RtcRtpSender",
    ))]
    #[allow(bad_style)]
    #[doc = "The `addTrack()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    #[allow(clippy::all)]
    pub fn add_track_3(
        &self,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams_1: &MediaStream,
        more_streams_2: &MediaStream,
        more_streams_3: &MediaStream,
    ) -> RtcRtpSender {
        #[cfg(all(
            feature = "MediaStream",
            feature = "MediaStreamTrack",
            feature = "RtcPeerConnection",
            feature = "RtcRtpSender",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_track_3_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_1: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_2: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_3: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_track_3_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_1: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_2: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_3: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(track);
            drop(stream);
            drop(more_streams_1);
            drop(more_streams_2);
            drop(more_streams_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let track =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(track);
                let stream = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(stream);
                let more_streams_1 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_1);
                let more_streams_2 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_2);
                let more_streams_3 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_3);
                __widl_f_add_track_3_RTCPeerConnection(
                    self_,
                    track,
                    stream,
                    more_streams_1,
                    more_streams_2,
                    more_streams_3,
                )
            };
            <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "MediaStream",
    feature = "MediaStreamTrack",
    feature = "RtcPeerConnection",
    feature = "RtcRtpSender",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_track_4_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&MediaStreamTrack as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <RtcRtpSender as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcPeerConnection",
        feature = "RtcRtpSender",
    ))]
    #[allow(bad_style)]
    #[doc = "The `addTrack()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    #[allow(clippy::all)]
    pub fn add_track_4(
        &self,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams_1: &MediaStream,
        more_streams_2: &MediaStream,
        more_streams_3: &MediaStream,
        more_streams_4: &MediaStream,
    ) -> RtcRtpSender {
        #[cfg(all(
            feature = "MediaStream",
            feature = "MediaStreamTrack",
            feature = "RtcPeerConnection",
            feature = "RtcRtpSender",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_track_4_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_1: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_2: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_3: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_4: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_track_4_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_1: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_2: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_3: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_4: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(track);
            drop(stream);
            drop(more_streams_1);
            drop(more_streams_2);
            drop(more_streams_3);
            drop(more_streams_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let track =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(track);
                let stream = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(stream);
                let more_streams_1 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_1);
                let more_streams_2 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_2);
                let more_streams_3 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_3);
                let more_streams_4 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_4);
                __widl_f_add_track_4_RTCPeerConnection(
                    self_,
                    track,
                    stream,
                    more_streams_1,
                    more_streams_2,
                    more_streams_3,
                    more_streams_4,
                )
            };
            <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "MediaStream",
    feature = "MediaStreamTrack",
    feature = "RtcPeerConnection",
    feature = "RtcRtpSender",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_track_5_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&MediaStreamTrack as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <RtcRtpSender as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcPeerConnection",
        feature = "RtcRtpSender",
    ))]
    #[allow(bad_style)]
    #[doc = "The `addTrack()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    #[allow(clippy::all)]
    pub fn add_track_5(
        &self,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams_1: &MediaStream,
        more_streams_2: &MediaStream,
        more_streams_3: &MediaStream,
        more_streams_4: &MediaStream,
        more_streams_5: &MediaStream,
    ) -> RtcRtpSender {
        #[cfg(all(
            feature = "MediaStream",
            feature = "MediaStreamTrack",
            feature = "RtcPeerConnection",
            feature = "RtcRtpSender",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_track_5_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_1: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_2: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_3: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_4: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_5: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_track_5_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_1: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_2: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_3: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_4: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_5: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(track);
            drop(stream);
            drop(more_streams_1);
            drop(more_streams_2);
            drop(more_streams_3);
            drop(more_streams_4);
            drop(more_streams_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let track =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(track);
                let stream = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(stream);
                let more_streams_1 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_1);
                let more_streams_2 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_2);
                let more_streams_3 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_3);
                let more_streams_4 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_4);
                let more_streams_5 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_5);
                __widl_f_add_track_5_RTCPeerConnection(
                    self_,
                    track,
                    stream,
                    more_streams_1,
                    more_streams_2,
                    more_streams_3,
                    more_streams_4,
                    more_streams_5,
                )
            };
            <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "MediaStream",
    feature = "MediaStreamTrack",
    feature = "RtcPeerConnection",
    feature = "RtcRtpSender",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_track_6_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&MediaStreamTrack as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <RtcRtpSender as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcPeerConnection",
        feature = "RtcRtpSender",
    ))]
    #[allow(bad_style)]
    #[doc = "The `addTrack()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    #[allow(clippy::all)]
    pub fn add_track_6(
        &self,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams_1: &MediaStream,
        more_streams_2: &MediaStream,
        more_streams_3: &MediaStream,
        more_streams_4: &MediaStream,
        more_streams_5: &MediaStream,
        more_streams_6: &MediaStream,
    ) -> RtcRtpSender {
        #[cfg(all(
            feature = "MediaStream",
            feature = "MediaStreamTrack",
            feature = "RtcPeerConnection",
            feature = "RtcRtpSender",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_track_6_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_1: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_2: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_3: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_4: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_5: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_6: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_track_6_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_1: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_2: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_3: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_4: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_5: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_6: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(track);
            drop(stream);
            drop(more_streams_1);
            drop(more_streams_2);
            drop(more_streams_3);
            drop(more_streams_4);
            drop(more_streams_5);
            drop(more_streams_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let track =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(track);
                let stream = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(stream);
                let more_streams_1 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_1);
                let more_streams_2 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_2);
                let more_streams_3 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_3);
                let more_streams_4 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_4);
                let more_streams_5 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_5);
                let more_streams_6 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_6);
                __widl_f_add_track_6_RTCPeerConnection(
                    self_,
                    track,
                    stream,
                    more_streams_1,
                    more_streams_2,
                    more_streams_3,
                    more_streams_4,
                    more_streams_5,
                    more_streams_6,
                )
            };
            <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "MediaStream",
    feature = "MediaStreamTrack",
    feature = "RtcPeerConnection",
    feature = "RtcRtpSender",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_track_7_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(10u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&MediaStreamTrack as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <RtcRtpSender as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcPeerConnection",
        feature = "RtcRtpSender",
    ))]
    #[allow(bad_style)]
    #[doc = "The `addTrack()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    #[allow(clippy::all)]
    pub fn add_track_7(
        &self,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams_1: &MediaStream,
        more_streams_2: &MediaStream,
        more_streams_3: &MediaStream,
        more_streams_4: &MediaStream,
        more_streams_5: &MediaStream,
        more_streams_6: &MediaStream,
        more_streams_7: &MediaStream,
    ) -> RtcRtpSender {
        #[cfg(all(
            feature = "MediaStream",
            feature = "MediaStreamTrack",
            feature = "RtcPeerConnection",
            feature = "RtcRtpSender",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_track_7_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_1: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_2: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_3: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_4: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_5: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_6: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                more_streams_7: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_track_7_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_1: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_2: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_3: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_4: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_5: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_6: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            more_streams_7: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(track);
            drop(stream);
            drop(more_streams_1);
            drop(more_streams_2);
            drop(more_streams_3);
            drop(more_streams_4);
            drop(more_streams_5);
            drop(more_streams_6);
            drop(more_streams_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let track =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(track);
                let stream = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(stream);
                let more_streams_1 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_1);
                let more_streams_2 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_2);
                let more_streams_3 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_3);
                let more_streams_4 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_4);
                let more_streams_5 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_5);
                let more_streams_6 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_6);
                let more_streams_7 =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(more_streams_7);
                __widl_f_add_track_7_RTCPeerConnection(
                    self_,
                    track,
                    stream,
                    more_streams_1,
                    more_streams_2,
                    more_streams_3,
                    more_streams_4,
                    more_streams_5,
                    more_streams_6,
                    more_streams_7,
                )
            };
            <RtcRtpSender as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "MediaStreamTrack",
    feature = "RtcPeerConnection",
    feature = "RtcRtpTransceiver",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_transceiver_with_media_stream_track_RTCPeerConnection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&MediaStreamTrack as WasmDescribe>::describe();
    <RtcRtpTransceiver as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(
        feature = "MediaStreamTrack",
        feature = "RtcPeerConnection",
        feature = "RtcRtpTransceiver",
    ))]
    #[allow(bad_style)]
    #[doc = "The `addTransceiver()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTransceiver)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpTransceiver`*"]
    #[allow(clippy::all)]
    pub fn add_transceiver_with_media_stream_track(
        &self,
        track_or_kind: &MediaStreamTrack,
    ) -> RtcRtpTransceiver {
        #[cfg(all(
            feature = "MediaStreamTrack",
            feature = "RtcPeerConnection",
            feature = "RtcRtpTransceiver",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_transceiver_with_media_stream_track_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                track_or_kind: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcRtpTransceiver as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_transceiver_with_media_stream_track_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            track_or_kind: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcRtpTransceiver as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(track_or_kind);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let track_or_kind =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        track_or_kind,
                    );
                __widl_f_add_transceiver_with_media_stream_track_RTCPeerConnection(
                    self_,
                    track_or_kind,
                )
            };
            <RtcRtpTransceiver as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection", feature = "RtcRtpTransceiver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_transceiver_with_str_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <RtcRtpTransceiver as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection", feature = "RtcRtpTransceiver",))]
    #[allow(bad_style)]
    #[doc = "The `addTransceiver()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTransceiver)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcRtpTransceiver`*"]
    #[allow(clippy::all)]
    pub fn add_transceiver_with_str(&self, track_or_kind: &str) -> RtcRtpTransceiver {
        #[cfg(all(feature = "RtcPeerConnection", feature = "RtcRtpTransceiver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_transceiver_with_str_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                track_or_kind: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcRtpTransceiver as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_transceiver_with_str_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            track_or_kind: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcRtpTransceiver as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(track_or_kind);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let track_or_kind =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(track_or_kind);
                __widl_f_add_transceiver_with_str_RTCPeerConnection(self_, track_or_kind)
            };
            <RtcRtpTransceiver as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "MediaStreamTrack",
    feature = "RtcPeerConnection",
    feature = "RtcRtpTransceiver",
    feature = "RtcRtpTransceiverInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_transceiver_with_media_stream_track_and_init_RTCPeerConnection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&MediaStreamTrack as WasmDescribe>::describe();
    <&RtcRtpTransceiverInit as WasmDescribe>::describe();
    <RtcRtpTransceiver as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(
        feature = "MediaStreamTrack",
        feature = "RtcPeerConnection",
        feature = "RtcRtpTransceiver",
        feature = "RtcRtpTransceiverInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `addTransceiver()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTransceiver)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpTransceiver`, `RtcRtpTransceiverInit`*"]
    #[allow(clippy::all)]
    pub fn add_transceiver_with_media_stream_track_and_init(
        &self,
        track_or_kind: &MediaStreamTrack,
        init: &RtcRtpTransceiverInit,
    ) -> RtcRtpTransceiver {
        #[cfg(all(
            feature = "MediaStreamTrack",
            feature = "RtcPeerConnection",
            feature = "RtcRtpTransceiver",
            feature = "RtcRtpTransceiverInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_transceiver_with_media_stream_track_and_init_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                track_or_kind: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                init: <&RtcRtpTransceiverInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcRtpTransceiver as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_transceiver_with_media_stream_track_and_init_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            track_or_kind: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            init: <&RtcRtpTransceiverInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcRtpTransceiver as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(track_or_kind);
            drop(init);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let track_or_kind =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        track_or_kind,
                    );
                let init =
                    <&RtcRtpTransceiverInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(init);
                __widl_f_add_transceiver_with_media_stream_track_and_init_RTCPeerConnection(
                    self_,
                    track_or_kind,
                    init,
                )
            };
            <RtcRtpTransceiver as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "RtcPeerConnection",
    feature = "RtcRtpTransceiver",
    feature = "RtcRtpTransceiverInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_transceiver_with_str_and_init_RTCPeerConnection()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&RtcRtpTransceiverInit as WasmDescribe>::describe();
    <RtcRtpTransceiver as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(
        feature = "RtcPeerConnection",
        feature = "RtcRtpTransceiver",
        feature = "RtcRtpTransceiverInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `addTransceiver()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTransceiver)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcRtpTransceiver`, `RtcRtpTransceiverInit`*"]
    #[allow(clippy::all)]
    pub fn add_transceiver_with_str_and_init(
        &self,
        track_or_kind: &str,
        init: &RtcRtpTransceiverInit,
    ) -> RtcRtpTransceiver {
        #[cfg(all(
            feature = "RtcPeerConnection",
            feature = "RtcRtpTransceiver",
            feature = "RtcRtpTransceiverInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_transceiver_with_str_and_init_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                track_or_kind: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                init: <&RtcRtpTransceiverInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcRtpTransceiver as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_transceiver_with_str_and_init_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            track_or_kind: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            init: <&RtcRtpTransceiverInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcRtpTransceiver as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(track_or_kind);
            drop(init);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let track_or_kind =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(track_or_kind);
                let init =
                    <&RtcRtpTransceiverInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(init);
                __widl_f_add_transceiver_with_str_and_init_RTCPeerConnection(
                    self_,
                    track_or_kind,
                    init,
                )
            };
            <RtcRtpTransceiver as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/close)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn close(&self) {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_close_RTCPeerConnection(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_answer_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `createAnswer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createAnswer)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn create_answer(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_answer_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_answer_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_answer_RTCPeerConnection(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcAnswerOptions", feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_answer_with_rtc_answer_options_RTCPeerConnection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&RtcAnswerOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcAnswerOptions", feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `createAnswer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createAnswer)\n\n*This API requires the following crate features to be activated: `RtcAnswerOptions`, `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn create_answer_with_rtc_answer_options(
        &self,
        options: &RtcAnswerOptions,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "RtcAnswerOptions", feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_answer_with_rtc_answer_options_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&RtcAnswerOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_answer_with_rtc_answer_options_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&RtcAnswerOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let options =
                    <&RtcAnswerOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_create_answer_with_rtc_answer_options_RTCPeerConnection(self_, options)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_answer_with_success_callback_and_failure_callback_RTCPeerConnection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `createAnswer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createAnswer)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn create_answer_with_success_callback_and_failure_callback(
        &self,
        success_callback: &::js_sys::Function,
        failure_callback: &::js_sys::Function,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_answer_with_success_callback_and_failure_callback_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                failure_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_answer_with_success_callback_and_failure_callback_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            failure_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(success_callback);
            drop(failure_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let failure_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        failure_callback,
                    );
                __widl_f_create_answer_with_success_callback_and_failure_callback_RTCPeerConnection(
                    self_,
                    success_callback,
                    failure_callback,
                )
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcDataChannel", feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_data_channel_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <RtcDataChannel as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcDataChannel", feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `createDataChannel()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createDataChannel)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn create_data_channel(&self, label: &str) -> RtcDataChannel {
        #[cfg(all(feature = "RtcDataChannel", feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_data_channel_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcDataChannel as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_data_channel_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcDataChannel as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(label);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_create_data_channel_RTCPeerConnection(self_, label)
            };
            <RtcDataChannel as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "RtcDataChannel",
    feature = "RtcDataChannelInit",
    feature = "RtcPeerConnection",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_data_channel_with_data_channel_dict_RTCPeerConnection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&RtcDataChannelInit as WasmDescribe>::describe();
    <RtcDataChannel as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(
        feature = "RtcDataChannel",
        feature = "RtcDataChannelInit",
        feature = "RtcPeerConnection",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createDataChannel()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createDataChannel)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelInit`, `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn create_data_channel_with_data_channel_dict(
        &self,
        label: &str,
        data_channel_dict: &RtcDataChannelInit,
    ) -> RtcDataChannel {
        #[cfg(all(
            feature = "RtcDataChannel",
            feature = "RtcDataChannelInit",
            feature = "RtcPeerConnection",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_data_channel_with_data_channel_dict_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_channel_dict: <&RtcDataChannelInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcDataChannel as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_data_channel_with_data_channel_dict_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_channel_dict: <&RtcDataChannelInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcDataChannel as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(label);
            drop(data_channel_dict);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_channel_dict =
                    <&RtcDataChannelInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_channel_dict,
                    );
                __widl_f_create_data_channel_with_data_channel_dict_RTCPeerConnection(
                    self_,
                    label,
                    data_channel_dict,
                )
            };
            <RtcDataChannel as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_offer_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `createOffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn create_offer(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_offer_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_offer_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_offer_RTCPeerConnection(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcOfferOptions", feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_offer_with_rtc_offer_options_RTCPeerConnection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&RtcOfferOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcOfferOptions", feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `createOffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)\n\n*This API requires the following crate features to be activated: `RtcOfferOptions`, `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn create_offer_with_rtc_offer_options(
        &self,
        options: &RtcOfferOptions,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "RtcOfferOptions", feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_offer_with_rtc_offer_options_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&RtcOfferOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_offer_with_rtc_offer_options_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&RtcOfferOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let options =
                    <&RtcOfferOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_create_offer_with_rtc_offer_options_RTCPeerConnection(self_, options)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_offer_with_callback_and_failure_callback_RTCPeerConnection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `createOffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn create_offer_with_callback_and_failure_callback(
        &self,
        success_callback: &::js_sys::Function,
        failure_callback: &::js_sys::Function,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_offer_with_callback_and_failure_callback_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                failure_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_offer_with_callback_and_failure_callback_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            failure_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(success_callback);
            drop(failure_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let failure_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        failure_callback,
                    );
                __widl_f_create_offer_with_callback_and_failure_callback_RTCPeerConnection(
                    self_,
                    success_callback,
                    failure_callback,
                )
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcOfferOptions", feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_offer_with_callback_and_failure_callback_and_options_RTCPeerConnection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&RtcOfferOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcOfferOptions", feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `createOffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)\n\n*This API requires the following crate features to be activated: `RtcOfferOptions`, `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn create_offer_with_callback_and_failure_callback_and_options(
        &self,
        success_callback: &::js_sys::Function,
        failure_callback: &::js_sys::Function,
        options: &RtcOfferOptions,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "RtcOfferOptions", feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_offer_with_callback_and_failure_callback_and_options_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                failure_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&RtcOfferOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_offer_with_callback_and_failure_callback_and_options_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            failure_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&RtcOfferOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(success_callback);
            drop(failure_callback);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let failure_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        failure_callback,
                    );
                let options =
                    <&RtcOfferOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_create_offer_with_callback_and_failure_callback_and_options_RTCPeerConnection ( self_ , success_callback , failure_callback , options )
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_generate_certificate_with_object_RTCPeerConnection()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `generateCertificate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/generateCertificate)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn generate_certificate_with_object(
        keygen_algorithm: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_generate_certificate_with_object_RTCPeerConnection(
                keygen_algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_generate_certificate_with_object_RTCPeerConnection(
            keygen_algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(keygen_algorithm);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let keygen_algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        keygen_algorithm,
                    );
                __widl_f_generate_certificate_with_object_RTCPeerConnection(keygen_algorithm)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_generate_certificate_with_str_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `generateCertificate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/generateCertificate)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn generate_certificate_with_str(
        keygen_algorithm: &str,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_generate_certificate_with_str_RTCPeerConnection(
                keygen_algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_generate_certificate_with_str_RTCPeerConnection(
            keygen_algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(keygen_algorithm);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let keygen_algorithm =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(keygen_algorithm);
                __widl_f_generate_certificate_with_str_RTCPeerConnection(keygen_algorithm)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "RtcConfiguration", feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_configuration_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <RtcConfiguration as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcConfiguration", feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `getConfiguration()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getConfiguration)\n\n*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn get_configuration(&self) -> RtcConfiguration {
        #[cfg(all(feature = "RtcConfiguration", feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_configuration_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcConfiguration as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_configuration_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcConfiguration as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_configuration_RTCPeerConnection(self_)
            };
            <RtcConfiguration as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_identity_assertion_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `getIdentityAssertion()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getIdentityAssertion)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn get_identity_assertion(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_identity_assertion_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_identity_assertion_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_identity_assertion_RTCPeerConnection(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_local_streams_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `getLocalStreams()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getLocalStreams)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn get_local_streams(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_local_streams_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_local_streams_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_local_streams_RTCPeerConnection(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_receivers_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `getReceivers()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getReceivers)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn get_receivers(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_receivers_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_receivers_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_receivers_RTCPeerConnection(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_remote_streams_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `getRemoteStreams()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getRemoteStreams)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn get_remote_streams(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_remote_streams_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_remote_streams_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_remote_streams_RTCPeerConnection(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_senders_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `getSenders()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getSenders)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn get_senders(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_senders_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_senders_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_senders_RTCPeerConnection(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_stats_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `getStats()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getStats)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn get_stats(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_stats_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_stats_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_stats_RTCPeerConnection(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack", feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_stats_with_selector_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<&MediaStreamTrack> as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "MediaStreamTrack", feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `getStats()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getStats)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn get_stats_with_selector(
        &self,
        selector: Option<&MediaStreamTrack>,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "MediaStreamTrack", feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_stats_with_selector_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selector: <Option<&MediaStreamTrack> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_stats_with_selector_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            selector: <Option<&MediaStreamTrack> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(selector);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selector =
                    <Option<&MediaStreamTrack> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        selector,
                    );
                __widl_f_get_stats_with_selector_RTCPeerConnection(self_, selector)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack", feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_stats_with_selector_and_success_callback_and_failure_callback_RTCPeerConnection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<&MediaStreamTrack> as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "MediaStreamTrack", feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `getStats()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getStats)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn get_stats_with_selector_and_success_callback_and_failure_callback(
        &self,
        selector: Option<&MediaStreamTrack>,
        success_callback: &::js_sys::Function,
        failure_callback: &::js_sys::Function,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "MediaStreamTrack", feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_stats_with_selector_and_success_callback_and_failure_callback_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selector: <Option<&MediaStreamTrack> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                failure_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_stats_with_selector_and_success_callback_and_failure_callback_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            selector: <Option<&MediaStreamTrack> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            failure_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(selector);
            drop(success_callback);
            drop(failure_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selector =
                    <Option<&MediaStreamTrack> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        selector,
                    );
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let failure_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        failure_callback,
                    );
                __widl_f_get_stats_with_selector_and_success_callback_and_failure_callback_RTCPeerConnection ( self_ , selector , success_callback , failure_callback )
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_transceivers_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `getTransceivers()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getTransceivers)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn get_transceivers(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_transceivers_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_transceivers_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_transceivers_RTCPeerConnection(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection", feature = "RtcRtpSender",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_track_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&RtcRtpSender as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection", feature = "RtcRtpSender",))]
    #[allow(bad_style)]
    #[doc = "The `removeTrack()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/removeTrack)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcRtpSender`*"]
    #[allow(clippy::all)]
    pub fn remove_track(&self, sender: &RtcRtpSender) {
        #[cfg(all(feature = "RtcPeerConnection", feature = "RtcRtpSender",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_track_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sender: <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_track_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sender: <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(sender);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let sender =
                    <&RtcRtpSender as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sender);
                __widl_f_remove_track_RTCPeerConnection(self_, sender)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_identity_provider_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `setIdentityProvider()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setIdentityProvider)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn set_identity_provider(&self, provider: &str) {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_identity_provider_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                provider: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_identity_provider_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            provider: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(provider);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let provider = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(provider);
                __widl_f_set_identity_provider_RTCPeerConnection(self_, provider)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcIdentityProviderOptions", feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_identity_provider_with_options_RTCPeerConnection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&RtcIdentityProviderOptions as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcIdentityProviderOptions", feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `setIdentityProvider()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setIdentityProvider)\n\n*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`, `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn set_identity_provider_with_options(
        &self,
        provider: &str,
        options: &RtcIdentityProviderOptions,
    ) {
        #[cfg(all(feature = "RtcIdentityProviderOptions", feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_identity_provider_with_options_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                provider: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&RtcIdentityProviderOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_identity_provider_with_options_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            provider: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&RtcIdentityProviderOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(provider);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let provider = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(provider);
                let options =
                    <&RtcIdentityProviderOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_set_identity_provider_with_options_RTCPeerConnection(
                    self_, provider, options,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescriptionInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_local_description_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&RtcSessionDescriptionInit as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescriptionInit",))]
    #[allow(bad_style)]
    #[doc = "The `setLocalDescription()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setLocalDescription)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescriptionInit`*"]
    #[allow(clippy::all)]
    pub fn set_local_description(
        &self,
        description: &RtcSessionDescriptionInit,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescriptionInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_local_description_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                description : < & RtcSessionDescriptionInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_local_description_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            description: <&RtcSessionDescriptionInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(description);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let description =
                    <&RtcSessionDescriptionInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        description,
                    );
                __widl_f_set_local_description_RTCPeerConnection(self_, description)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescriptionInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_local_description_with_success_callback_and_failure_callback_RTCPeerConnection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&RtcSessionDescriptionInit as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescriptionInit",))]
    #[allow(bad_style)]
    #[doc = "The `setLocalDescription()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setLocalDescription)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescriptionInit`*"]
    #[allow(clippy::all)]
    pub fn set_local_description_with_success_callback_and_failure_callback(
        &self,
        description: &RtcSessionDescriptionInit,
        success_callback: &::js_sys::Function,
        failure_callback: &::js_sys::Function,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescriptionInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_local_description_with_success_callback_and_failure_callback_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                description : < & RtcSessionDescriptionInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                failure_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_local_description_with_success_callback_and_failure_callback_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            description: <&RtcSessionDescriptionInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            failure_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(description);
            drop(success_callback);
            drop(failure_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let description =
                    <&RtcSessionDescriptionInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        description,
                    );
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let failure_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        failure_callback,
                    );
                __widl_f_set_local_description_with_success_callback_and_failure_callback_RTCPeerConnection ( self_ , description , success_callback , failure_callback )
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescriptionInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_remote_description_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&RtcSessionDescriptionInit as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescriptionInit",))]
    #[allow(bad_style)]
    #[doc = "The `setRemoteDescription()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setRemoteDescription)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescriptionInit`*"]
    #[allow(clippy::all)]
    pub fn set_remote_description(
        &self,
        description: &RtcSessionDescriptionInit,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescriptionInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_remote_description_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                description : < & RtcSessionDescriptionInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_remote_description_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            description: <&RtcSessionDescriptionInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(description);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let description =
                    <&RtcSessionDescriptionInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        description,
                    );
                __widl_f_set_remote_description_RTCPeerConnection(self_, description)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescriptionInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_remote_description_with_success_callback_and_failure_callback_RTCPeerConnection(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <&RtcSessionDescriptionInit as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescriptionInit",))]
    #[allow(bad_style)]
    #[doc = "The `setRemoteDescription()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setRemoteDescription)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescriptionInit`*"]
    #[allow(clippy::all)]
    pub fn set_remote_description_with_success_callback_and_failure_callback(
        &self,
        description: &RtcSessionDescriptionInit,
        success_callback: &::js_sys::Function,
        failure_callback: &::js_sys::Function,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescriptionInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_remote_description_with_success_callback_and_failure_callback_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                description : < & RtcSessionDescriptionInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                failure_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_remote_description_with_success_callback_and_failure_callback_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            description: <&RtcSessionDescriptionInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            failure_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(description);
            drop(success_callback);
            drop(failure_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let description =
                    <&RtcSessionDescriptionInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        description,
                    );
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let failure_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        failure_callback,
                    );
                __widl_f_set_remote_description_with_success_callback_and_failure_callback_RTCPeerConnection ( self_ , description , success_callback , failure_callback )
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescription",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_local_description_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<RtcSessionDescription> as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescription",))]
    #[allow(bad_style)]
    #[doc = "The `localDescription` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/localDescription)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
    #[allow(clippy::all)]
    pub fn local_description(&self) -> Option<RtcSessionDescription> {
        #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescription",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_local_description_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<RtcSessionDescription> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_local_description_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<RtcSessionDescription> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_local_description_RTCPeerConnection(self_)
            };
            <Option<RtcSessionDescription> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescription",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_current_local_description_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<RtcSessionDescription> as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescription",))]
    #[allow(bad_style)]
    #[doc = "The `currentLocalDescription` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/currentLocalDescription)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
    #[allow(clippy::all)]
    pub fn current_local_description(&self) -> Option<RtcSessionDescription> {
        #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescription",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_current_local_description_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<RtcSessionDescription> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_current_local_description_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<RtcSessionDescription> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_current_local_description_RTCPeerConnection(self_)
            };
            <Option<RtcSessionDescription> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescription",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pending_local_description_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<RtcSessionDescription> as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescription",))]
    #[allow(bad_style)]
    #[doc = "The `pendingLocalDescription` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/pendingLocalDescription)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
    #[allow(clippy::all)]
    pub fn pending_local_description(&self) -> Option<RtcSessionDescription> {
        #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescription",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pending_local_description_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<RtcSessionDescription> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pending_local_description_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<RtcSessionDescription> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pending_local_description_RTCPeerConnection(self_)
            };
            <Option<RtcSessionDescription> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescription",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remote_description_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<RtcSessionDescription> as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescription",))]
    #[allow(bad_style)]
    #[doc = "The `remoteDescription` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/remoteDescription)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
    #[allow(clippy::all)]
    pub fn remote_description(&self) -> Option<RtcSessionDescription> {
        #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescription",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remote_description_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<RtcSessionDescription> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remote_description_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<RtcSessionDescription> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_remote_description_RTCPeerConnection(self_)
            };
            <Option<RtcSessionDescription> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescription",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_current_remote_description_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<RtcSessionDescription> as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescription",))]
    #[allow(bad_style)]
    #[doc = "The `currentRemoteDescription` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/currentRemoteDescription)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
    #[allow(clippy::all)]
    pub fn current_remote_description(&self) -> Option<RtcSessionDescription> {
        #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescription",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_current_remote_description_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<RtcSessionDescription> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_current_remote_description_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<RtcSessionDescription> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_current_remote_description_RTCPeerConnection(self_)
            };
            <Option<RtcSessionDescription> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescription",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pending_remote_description_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<RtcSessionDescription> as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescription",))]
    #[allow(bad_style)]
    #[doc = "The `pendingRemoteDescription` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/pendingRemoteDescription)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
    #[allow(clippy::all)]
    pub fn pending_remote_description(&self) -> Option<RtcSessionDescription> {
        #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSessionDescription",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pending_remote_description_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<RtcSessionDescription> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pending_remote_description_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<RtcSessionDescription> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pending_remote_description_RTCPeerConnection(self_)
            };
            <Option<RtcSessionDescription> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection", feature = "RtcSignalingState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_signaling_state_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <RtcSignalingState as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSignalingState",))]
    #[allow(bad_style)]
    #[doc = "The `signalingState` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/signalingState)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSignalingState`*"]
    #[allow(clippy::all)]
    pub fn signaling_state(&self) -> RtcSignalingState {
        #[cfg(all(feature = "RtcPeerConnection", feature = "RtcSignalingState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_signaling_state_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcSignalingState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_signaling_state_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcSignalingState as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_signaling_state_RTCPeerConnection(self_)
            };
            <RtcSignalingState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_can_trickle_ice_candidates_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<bool> as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `canTrickleIceCandidates` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/canTrickleIceCandidates)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn can_trickle_ice_candidates(&self) -> Option<bool> {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_can_trickle_ice_candidates_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<bool> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_can_trickle_ice_candidates_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<bool> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_can_trickle_ice_candidates_RTCPeerConnection(self_)
            };
            <Option<bool> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcIceGatheringState", feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ice_gathering_state_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <RtcIceGatheringState as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcIceGatheringState", feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `iceGatheringState` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/iceGatheringState)\n\n*This API requires the following crate features to be activated: `RtcIceGatheringState`, `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn ice_gathering_state(&self) -> RtcIceGatheringState {
        #[cfg(all(feature = "RtcIceGatheringState", feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ice_gathering_state_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcIceGatheringState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ice_gathering_state_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcIceGatheringState as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ice_gathering_state_RTCPeerConnection(self_)
            };
            <RtcIceGatheringState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcIceConnectionState", feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ice_connection_state_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <RtcIceConnectionState as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcIceConnectionState", feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `iceConnectionState` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/iceConnectionState)\n\n*This API requires the following crate features to be activated: `RtcIceConnectionState`, `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn ice_connection_state(&self) -> RtcIceConnectionState {
        #[cfg(all(feature = "RtcIceConnectionState", feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ice_connection_state_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcIceConnectionState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ice_connection_state_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcIceConnectionState as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ice_connection_state_RTCPeerConnection(self_)
            };
            <RtcIceConnectionState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_peer_identity_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `peerIdentity` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/peerIdentity)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn peer_identity(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_peer_identity_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_peer_identity_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_peer_identity_RTCPeerConnection(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_idp_login_url_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `idpLoginUrl` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/idpLoginUrl)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn idp_login_url(&self) -> Option<String> {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_idp_login_url_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_idp_login_url_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_idp_login_url_RTCPeerConnection(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onnegotiationneeded_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onnegotiationneeded` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onnegotiationneeded)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn onnegotiationneeded(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onnegotiationneeded_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onnegotiationneeded_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onnegotiationneeded_RTCPeerConnection(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onnegotiationneeded_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onnegotiationneeded` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onnegotiationneeded)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn set_onnegotiationneeded(&self, onnegotiationneeded: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onnegotiationneeded_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onnegotiationneeded : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onnegotiationneeded_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onnegotiationneeded : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onnegotiationneeded);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onnegotiationneeded =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onnegotiationneeded,
                    );
                __widl_f_set_onnegotiationneeded_RTCPeerConnection(self_, onnegotiationneeded)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onicecandidate_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onicecandidate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicecandidate)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn onicecandidate(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onicecandidate_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onicecandidate_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onicecandidate_RTCPeerConnection(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onicecandidate_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onicecandidate` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicecandidate)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn set_onicecandidate(&self, onicecandidate: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onicecandidate_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onicecandidate : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onicecandidate_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onicecandidate : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onicecandidate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onicecandidate =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onicecandidate,
                    );
                __widl_f_set_onicecandidate_RTCPeerConnection(self_, onicecandidate)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onsignalingstatechange_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onsignalingstatechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onsignalingstatechange)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn onsignalingstatechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onsignalingstatechange_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onsignalingstatechange_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onsignalingstatechange_RTCPeerConnection(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onsignalingstatechange_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onsignalingstatechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onsignalingstatechange)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn set_onsignalingstatechange(&self, onsignalingstatechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onsignalingstatechange_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onsignalingstatechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onsignalingstatechange_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onsignalingstatechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onsignalingstatechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onsignalingstatechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onsignalingstatechange,
                    );
                __widl_f_set_onsignalingstatechange_RTCPeerConnection(self_, onsignalingstatechange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onaddstream_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onaddstream` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onaddstream)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn onaddstream(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onaddstream_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onaddstream_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onaddstream_RTCPeerConnection(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onaddstream_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onaddstream` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onaddstream)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn set_onaddstream(&self, onaddstream: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onaddstream_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onaddstream : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onaddstream_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onaddstream: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onaddstream);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onaddstream =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onaddstream,
                    );
                __widl_f_set_onaddstream_RTCPeerConnection(self_, onaddstream)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onaddtrack_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onaddtrack` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onaddtrack)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn onaddtrack(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onaddtrack_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onaddtrack_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onaddtrack_RTCPeerConnection(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onaddtrack_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onaddtrack` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onaddtrack)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn set_onaddtrack(&self, onaddtrack: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onaddtrack_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onaddtrack : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onaddtrack_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onaddtrack: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onaddtrack);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onaddtrack =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onaddtrack,
                    );
                __widl_f_set_onaddtrack_RTCPeerConnection(self_, onaddtrack)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontrack_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `ontrack` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ontrack)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn ontrack(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontrack_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontrack_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontrack_RTCPeerConnection(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontrack_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `ontrack` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ontrack)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn set_ontrack(&self, ontrack: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontrack_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontrack: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontrack_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontrack: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontrack);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontrack =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontrack,
                    );
                __widl_f_set_ontrack_RTCPeerConnection(self_, ontrack)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onremovestream_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onremovestream` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onremovestream)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn onremovestream(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onremovestream_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onremovestream_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onremovestream_RTCPeerConnection(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onremovestream_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onremovestream` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onremovestream)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn set_onremovestream(&self, onremovestream: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onremovestream_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onremovestream : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onremovestream_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onremovestream : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onremovestream);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onremovestream =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onremovestream,
                    );
                __widl_f_set_onremovestream_RTCPeerConnection(self_, onremovestream)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oniceconnectionstatechange_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `oniceconnectionstatechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/oniceconnectionstatechange)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn oniceconnectionstatechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oniceconnectionstatechange_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oniceconnectionstatechange_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oniceconnectionstatechange_RTCPeerConnection(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oniceconnectionstatechange_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `oniceconnectionstatechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/oniceconnectionstatechange)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn set_oniceconnectionstatechange(
        &self,
        oniceconnectionstatechange: Option<&::js_sys::Function>,
    ) {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oniceconnectionstatechange_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oniceconnectionstatechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oniceconnectionstatechange_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oniceconnectionstatechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(oniceconnectionstatechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oniceconnectionstatechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oniceconnectionstatechange,
                    );
                __widl_f_set_oniceconnectionstatechange_RTCPeerConnection(
                    self_,
                    oniceconnectionstatechange,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onicegatheringstatechange_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onicegatheringstatechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicegatheringstatechange)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn onicegatheringstatechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onicegatheringstatechange_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onicegatheringstatechange_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onicegatheringstatechange_RTCPeerConnection(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onicegatheringstatechange_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onicegatheringstatechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicegatheringstatechange)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn set_onicegatheringstatechange(
        &self,
        onicegatheringstatechange: Option<&::js_sys::Function>,
    ) {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onicegatheringstatechange_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onicegatheringstatechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onicegatheringstatechange_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onicegatheringstatechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onicegatheringstatechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onicegatheringstatechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onicegatheringstatechange,
                    );
                __widl_f_set_onicegatheringstatechange_RTCPeerConnection(
                    self_,
                    onicegatheringstatechange,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondatachannel_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `ondatachannel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ondatachannel)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn ondatachannel(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondatachannel_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondatachannel_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondatachannel_RTCPeerConnection(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcPeerConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondatachannel_RTCPeerConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcPeerConnection as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcPeerConnection {
    #[cfg(all(feature = "RtcPeerConnection",))]
    #[allow(bad_style)]
    #[doc = "The `ondatachannel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ondatachannel)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    #[allow(clippy::all)]
    pub fn set_ondatachannel(&self, ondatachannel: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "RtcPeerConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondatachannel_RTCPeerConnection(
                self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondatachannel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondatachannel_RTCPeerConnection(
            self_: <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondatachannel: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondatachannel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcPeerConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondatachannel =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondatachannel,
                    );
                __widl_f_set_ondatachannel_RTCPeerConnection(self_, ondatachannel)
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
pub static __WASM_BINDGEN_GENERATED_790208af9bd45cbc: [u8; 10334usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1C(\0\0\0\0R\0\0\x02\x11RTCPeerConnection#__widl_instanceof_RTCPeerConnection\0\0\0\0\x1E__widl_f_new_RTCPeerConnection\x01\0\0\x01\x11RTCPeerConnection\0\x01\0\x03new\0\0\01__widl_f_new_with_configuration_RTCPeerConnection\x01\0\0\x01\x11RTCPeerConnection\0\x01\x01\rconfiguration\x03new\0\0\0A__widl_f_new_with_configuration_and_constraints_RTCPeerConnection\x01\0\0\x01\x11RTCPeerConnection\0\x01\x02\rconfiguration\x0Bconstraints\x03new\0\0\0L__widl_f_add_ice_candidate_with_opt_rtc_ice_candidate_init_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x02\x05self_\tcandidate\x0FaddIceCandidate\0\0\0G__widl_f_add_ice_candidate_with_opt_rtc_ice_candidate_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x02\x05self_\tcandidate\x0FaddIceCandidate\0\0\0m__widl_f_add_ice_candidate_with_rtc_ice_candidate_and_success_callback_and_failure_callback_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x04\x05self_\tcandidate\x10success_callback\x10failure_callback\x0FaddIceCandidate\0\0\0%__widl_f_add_stream_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x02\x05self_\x06stream\taddStream\0\0\0$__widl_f_add_track_RTCPeerConnection\0\x01\0\x01\x11RTCPeerConnection\x01\0\0\x01\x04\x05self_\x05track\x06stream\x0Cmore_streams\x08addTrack\0\0\0&__widl_f_add_track_0_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x03\x05self_\x05track\x06stream\x08addTrack\0\0\0&__widl_f_add_track_1_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x04\x05self_\x05track\x06stream\x0Emore_streams_1\x08addTrack\0\0\0&__widl_f_add_track_2_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x05\x05self_\x05track\x06stream\x0Emore_streams_1\x0Emore_streams_2\x08addTrack\0\0\0&__widl_f_add_track_3_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x06\x05self_\x05track\x06stream\x0Emore_streams_1\x0Emore_streams_2\x0Emore_streams_3\x08addTrack\0\0\0&__widl_f_add_track_4_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x07\x05self_\x05track\x06stream\x0Emore_streams_1\x0Emore_streams_2\x0Emore_streams_3\x0Emore_streams_4\x08addTrack\0\0\0&__widl_f_add_track_5_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x08\x05self_\x05track\x06stream\x0Emore_streams_1\x0Emore_streams_2\x0Emore_streams_3\x0Emore_streams_4\x0Emore_streams_5\x08addTrack\0\0\0&__widl_f_add_track_6_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\t\x05self_\x05track\x06stream\x0Emore_streams_1\x0Emore_streams_2\x0Emore_streams_3\x0Emore_streams_4\x0Emore_streams_5\x0Emore_streams_6\x08addTrack\0\0\0&__widl_f_add_track_7_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\n\x05self_\x05track\x06stream\x0Emore_streams_1\x0Emore_streams_2\x0Emore_streams_3\x0Emore_streams_4\x0Emore_streams_5\x0Emore_streams_6\x0Emore_streams_7\x08addTrack\0\0\0B__widl_f_add_transceiver_with_media_stream_track_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x02\x05self_\rtrack_or_kind\x0EaddTransceiver\0\0\03__widl_f_add_transceiver_with_str_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x02\x05self_\rtrack_or_kind\x0EaddTransceiver\0\0\0K__widl_f_add_transceiver_with_media_stream_track_and_init_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x03\x05self_\rtrack_or_kind\x04init\x0EaddTransceiver\0\0\0<__widl_f_add_transceiver_with_str_and_init_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x03\x05self_\rtrack_or_kind\x04init\x0EaddTransceiver\0\0\0 __widl_f_close_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x01\x05self_\x05close\0\0\0(__widl_f_create_answer_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x01\x05self_\x0CcreateAnswer\0\0\0@__widl_f_create_answer_with_rtc_answer_options_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x02\x05self_\x07options\x0CcreateAnswer\0\0\0S__widl_f_create_answer_with_success_callback_and_failure_callback_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x03\x05self_\x10success_callback\x10failure_callback\x0CcreateAnswer\0\0\0.__widl_f_create_data_channel_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x02\x05self_\x05label\x11createDataChannel\0\0\0E__widl_f_create_data_channel_with_data_channel_dict_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x03\x05self_\x05label\x11data_channel_dict\x11createDataChannel\0\0\0'__widl_f_create_offer_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x01\x05self_\x0BcreateOffer\0\0\0>__widl_f_create_offer_with_rtc_offer_options_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x02\x05self_\x07options\x0BcreateOffer\0\0\0J__widl_f_create_offer_with_callback_and_failure_callback_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x03\x05self_\x10success_callback\x10failure_callback\x0BcreateOffer\0\0\0V__widl_f_create_offer_with_callback_and_failure_callback_and_options_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x04\x05self_\x10success_callback\x10failure_callback\x07options\x0BcreateOffer\0\0\0;__widl_f_generate_certificate_with_object_RTCPeerConnection\x01\0\0\x01\x11RTCPeerConnection\x01\x01\0\x01\x01\x10keygen_algorithm\x13generateCertificate\0\0\08__widl_f_generate_certificate_with_str_RTCPeerConnection\x01\0\0\x01\x11RTCPeerConnection\x01\x01\0\x01\x01\x10keygen_algorithm\x13generateCertificate\0\0\0,__widl_f_get_configuration_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x01\x05self_\x10getConfiguration\0\0\01__widl_f_get_identity_assertion_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x01\x05self_\x14getIdentityAssertion\0\0\0,__widl_f_get_local_streams_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x01\x05self_\x0FgetLocalStreams\0\0\0(__widl_f_get_receivers_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x01\x05self_\x0CgetReceivers\0\0\0-__widl_f_get_remote_streams_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x01\x05self_\x10getRemoteStreams\0\0\0&__widl_f_get_senders_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x01\x05self_\ngetSenders\0\0\0$__widl_f_get_stats_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x01\x05self_\x08getStats\0\0\02__widl_f_get_stats_with_selector_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x02\x05self_\x08selector\x08getStats\0\0\0\\__widl_f_get_stats_with_selector_and_success_callback_and_failure_callback_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x04\x05self_\x08selector\x10success_callback\x10failure_callback\x08getStats\0\0\0+__widl_f_get_transceivers_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x01\x05self_\x0FgetTransceivers\0\0\0'__widl_f_remove_track_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x02\x05self_\x06sender\x0BremoveTrack\0\0\00__widl_f_set_identity_provider_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x02\x05self_\x08provider\x13setIdentityProvider\0\0\0=__widl_f_set_identity_provider_with_options_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x03\x05self_\x08provider\x07options\x13setIdentityProvider\0\0\00__widl_f_set_local_description_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x02\x05self_\x0Bdescription\x13setLocalDescription\0\0\0[__widl_f_set_local_description_with_success_callback_and_failure_callback_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x04\x05self_\x0Bdescription\x10success_callback\x10failure_callback\x13setLocalDescription\0\0\01__widl_f_set_remote_description_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x02\x05self_\x0Bdescription\x14setRemoteDescription\0\0\0\\__widl_f_set_remote_description_with_success_callback_and_failure_callback_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\0\x01\x04\x05self_\x0Bdescription\x10success_callback\x10failure_callback\x14setRemoteDescription\0\0\0,__widl_f_local_description_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\x10localDescription\x01\x01\x05self_\x10localDescription\0\0\04__widl_f_current_local_description_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\x17currentLocalDescription\x01\x01\x05self_\x17currentLocalDescription\0\0\04__widl_f_pending_local_description_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\x17pendingLocalDescription\x01\x01\x05self_\x17pendingLocalDescription\0\0\0-__widl_f_remote_description_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\x11remoteDescription\x01\x01\x05self_\x11remoteDescription\0\0\05__widl_f_current_remote_description_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\x18currentRemoteDescription\x01\x01\x05self_\x18currentRemoteDescription\0\0\05__widl_f_pending_remote_description_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\x18pendingRemoteDescription\x01\x01\x05self_\x18pendingRemoteDescription\0\0\0*__widl_f_signaling_state_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\x0EsignalingState\x01\x01\x05self_\x0EsignalingState\0\0\05__widl_f_can_trickle_ice_candidates_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\x17canTrickleIceCandidates\x01\x01\x05self_\x17canTrickleIceCandidates\0\0\0.__widl_f_ice_gathering_state_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\x11iceGatheringState\x01\x01\x05self_\x11iceGatheringState\0\0\0/__widl_f_ice_connection_state_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\x12iceConnectionState\x01\x01\x05self_\x12iceConnectionState\0\0\0(__widl_f_peer_identity_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\x0CpeerIdentity\x01\x01\x05self_\x0CpeerIdentity\0\0\0(__widl_f_idp_login_url_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\x0BidpLoginUrl\x01\x01\x05self_\x0BidpLoginUrl\0\0\0.__widl_f_onnegotiationneeded_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\x13onnegotiationneeded\x01\x01\x05self_\x13onnegotiationneeded\0\0\02__widl_f_set_onnegotiationneeded_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x02\x13onnegotiationneeded\x01\x02\x05self_\x13onnegotiationneeded\x13onnegotiationneeded\0\0\0)__widl_f_onicecandidate_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\x0Eonicecandidate\x01\x01\x05self_\x0Eonicecandidate\0\0\0-__widl_f_set_onicecandidate_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x02\x0Eonicecandidate\x01\x02\x05self_\x0Eonicecandidate\x0Eonicecandidate\0\0\01__widl_f_onsignalingstatechange_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\x16onsignalingstatechange\x01\x01\x05self_\x16onsignalingstatechange\0\0\05__widl_f_set_onsignalingstatechange_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x02\x16onsignalingstatechange\x01\x02\x05self_\x16onsignalingstatechange\x16onsignalingstatechange\0\0\0&__widl_f_onaddstream_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\x0Bonaddstream\x01\x01\x05self_\x0Bonaddstream\0\0\0*__widl_f_set_onaddstream_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x02\x0Bonaddstream\x01\x02\x05self_\x0Bonaddstream\x0Bonaddstream\0\0\0%__widl_f_onaddtrack_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\nonaddtrack\x01\x01\x05self_\nonaddtrack\0\0\0)__widl_f_set_onaddtrack_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x02\nonaddtrack\x01\x02\x05self_\nonaddtrack\nonaddtrack\0\0\0\"__widl_f_ontrack_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\x07ontrack\x01\x01\x05self_\x07ontrack\0\0\0&__widl_f_set_ontrack_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x02\x07ontrack\x01\x02\x05self_\x07ontrack\x07ontrack\0\0\0)__widl_f_onremovestream_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\x0Eonremovestream\x01\x01\x05self_\x0Eonremovestream\0\0\0-__widl_f_set_onremovestream_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x02\x0Eonremovestream\x01\x02\x05self_\x0Eonremovestream\x0Eonremovestream\0\0\05__widl_f_oniceconnectionstatechange_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\x1Aoniceconnectionstatechange\x01\x01\x05self_\x1Aoniceconnectionstatechange\0\0\09__widl_f_set_oniceconnectionstatechange_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x02\x1Aoniceconnectionstatechange\x01\x02\x05self_\x1Aoniceconnectionstatechange\x1Aoniceconnectionstatechange\0\0\04__widl_f_onicegatheringstatechange_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\x19onicegatheringstatechange\x01\x01\x05self_\x19onicegatheringstatechange\0\0\08__widl_f_set_onicegatheringstatechange_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x02\x19onicegatheringstatechange\x01\x02\x05self_\x19onicegatheringstatechange\x19onicegatheringstatechange\0\0\0(__widl_f_ondatachannel_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x01\rondatachannel\x01\x01\x05self_\rondatachannel\0\0\0,__widl_f_set_ondatachannel_RTCPeerConnection\0\0\0\x01\x11RTCPeerConnection\x01\0\x02\rondatachannel\x01\x02\x05self_\rondatachannel\rondatachannel\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
