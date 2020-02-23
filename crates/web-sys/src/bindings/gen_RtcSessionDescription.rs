use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `RTCSessionDescription` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription)\n\n*This API requires the following crate features to be activated: `RtcSessionDescription`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct RtcSessionDescription {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_RtcSessionDescription: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for RtcSessionDescription {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(21u32);
            inform(82u32);
            inform(84u32);
            inform(67u32);
            inform(83u32);
            inform(101u32);
            inform(115u32);
            inform(115u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(68u32);
            inform(101u32);
            inform(115u32);
            inform(99u32);
            inform(114u32);
            inform(105u32);
            inform(112u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for RtcSessionDescription {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for RtcSessionDescription {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for RtcSessionDescription {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a RtcSessionDescription {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for RtcSessionDescription {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            RtcSessionDescription {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for RtcSessionDescription {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a RtcSessionDescription {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for RtcSessionDescription {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<RtcSessionDescription>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(RtcSessionDescription {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for RtcSessionDescription {
        #[inline]
        fn from(obj: JsValue) -> RtcSessionDescription {
            RtcSessionDescription { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for RtcSessionDescription {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<RtcSessionDescription> for RtcSessionDescription {
        #[inline]
        fn as_ref(&self) -> &RtcSessionDescription {
            self
        }
    }
    impl From<RtcSessionDescription> for JsValue {
        #[inline]
        fn from(obj: RtcSessionDescription) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for RtcSessionDescription {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_RTCSessionDescription(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_RTCSessionDescription(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_RTCSessionDescription(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            RtcSessionDescription { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const RtcSessionDescription) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<RtcSessionDescription> for ::js_sys::Object {
    #[inline]
    fn from(obj: RtcSessionDescription) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for RtcSessionDescription {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "RtcSessionDescription",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_RTCSessionDescription() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <RtcSessionDescription as WasmDescribe>::describe();
}
impl RtcSessionDescription {
    #[cfg(all(feature = "RtcSessionDescription",))]
    #[allow(bad_style)]
    #[doc = "The `new RTCSessionDescription(..)` constructor, creating a new instance of `RTCSessionDescription`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/RTCSessionDescription)\n\n*This API requires the following crate features to be activated: `RtcSessionDescription`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<RtcSessionDescription, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "RtcSessionDescription",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_RTCSessionDescription(
            ) -> <RtcSessionDescription as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_RTCSessionDescription(
        ) -> <RtcSessionDescription as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_RTCSessionDescription() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<RtcSessionDescription as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "RtcSessionDescription",
    feature = "RtcSessionDescriptionInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_description_init_dict_RTCSessionDescription(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcSessionDescriptionInit as WasmDescribe>::describe();
    <RtcSessionDescription as WasmDescribe>::describe();
}
impl RtcSessionDescription {
    #[cfg(all(
        feature = "RtcSessionDescription",
        feature = "RtcSessionDescriptionInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new RTCSessionDescription(..)` constructor, creating a new instance of `RTCSessionDescription`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/RTCSessionDescription)\n\n*This API requires the following crate features to be activated: `RtcSessionDescription`, `RtcSessionDescriptionInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_description_init_dict(
        description_init_dict: &RtcSessionDescriptionInit,
    ) -> Result<RtcSessionDescription, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "RtcSessionDescription",
            feature = "RtcSessionDescriptionInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_description_init_dict_RTCSessionDescription(
                description_init_dict : < & RtcSessionDescriptionInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <RtcSessionDescription as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_description_init_dict_RTCSessionDescription(
            description_init_dict : < & RtcSessionDescriptionInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <RtcSessionDescription as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(description_init_dict);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let description_init_dict =
                    <&RtcSessionDescriptionInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        description_init_dict,
                    );
                __widl_f_new_with_description_init_dict_RTCSessionDescription(description_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<RtcSessionDescription as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "RtcSessionDescription",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_json_RTCSessionDescription() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcSessionDescription as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl RtcSessionDescription {
    #[cfg(all(feature = "RtcSessionDescription",))]
    #[allow(bad_style)]
    #[doc = "The `toJSON()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/toJSON)\n\n*This API requires the following crate features to be activated: `RtcSessionDescription`*"]
    #[allow(clippy::all)]
    pub fn to_json(&self) -> ::js_sys::Object {
        #[cfg(all(feature = "RtcSessionDescription",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_json_RTCSessionDescription(
                self_: <&RtcSessionDescription as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_json_RTCSessionDescription(
            self_: <&RtcSessionDescription as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&RtcSessionDescription as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_to_json_RTCSessionDescription(self_)
            };
            <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcSdpType", feature = "RtcSessionDescription",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_RTCSessionDescription() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcSessionDescription as WasmDescribe>::describe();
    <RtcSdpType as WasmDescribe>::describe();
}
impl RtcSessionDescription {
    #[cfg(all(feature = "RtcSdpType", feature = "RtcSessionDescription",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/type)\n\n*This API requires the following crate features to be activated: `RtcSdpType`, `RtcSessionDescription`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> RtcSdpType {
        #[cfg(all(feature = "RtcSdpType", feature = "RtcSessionDescription",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_RTCSessionDescription(
                self_: <&RtcSessionDescription as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcSdpType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_RTCSessionDescription(
            self_: <&RtcSessionDescription as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcSdpType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcSessionDescription as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_RTCSessionDescription(self_)
            };
            <RtcSdpType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcSdpType", feature = "RtcSessionDescription",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_type_RTCSessionDescription() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcSessionDescription as WasmDescribe>::describe();
    <RtcSdpType as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcSessionDescription {
    #[cfg(all(feature = "RtcSdpType", feature = "RtcSessionDescription",))]
    #[allow(bad_style)]
    #[doc = "The `type` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/type)\n\n*This API requires the following crate features to be activated: `RtcSdpType`, `RtcSessionDescription`*"]
    #[allow(clippy::all)]
    pub fn set_type(&self, type_: RtcSdpType) {
        #[cfg(all(feature = "RtcSdpType", feature = "RtcSessionDescription",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_type_RTCSessionDescription(
                self_: <&RtcSessionDescription as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <RtcSdpType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_type_RTCSessionDescription(
            self_: <&RtcSessionDescription as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <RtcSdpType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcSessionDescription as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <RtcSdpType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_set_type_RTCSessionDescription(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcSessionDescription",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sdp_RTCSessionDescription() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcSessionDescription as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl RtcSessionDescription {
    #[cfg(all(feature = "RtcSessionDescription",))]
    #[allow(bad_style)]
    #[doc = "The `sdp` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/sdp)\n\n*This API requires the following crate features to be activated: `RtcSessionDescription`*"]
    #[allow(clippy::all)]
    pub fn sdp(&self) -> String {
        #[cfg(all(feature = "RtcSessionDescription",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sdp_RTCSessionDescription(
                self_: <&RtcSessionDescription as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sdp_RTCSessionDescription(
            self_: <&RtcSessionDescription as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&RtcSessionDescription as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_sdp_RTCSessionDescription(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcSessionDescription",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_sdp_RTCSessionDescription() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcSessionDescription as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcSessionDescription {
    #[cfg(all(feature = "RtcSessionDescription",))]
    #[allow(bad_style)]
    #[doc = "The `sdp` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/sdp)\n\n*This API requires the following crate features to be activated: `RtcSessionDescription`*"]
    #[allow(clippy::all)]
    pub fn set_sdp(&self, sdp: &str) {
        #[cfg(all(feature = "RtcSessionDescription",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_sdp_RTCSessionDescription(
                self_: <&RtcSessionDescription as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sdp: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_sdp_RTCSessionDescription(
            self_: <&RtcSessionDescription as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sdp: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(sdp);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcSessionDescription as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let sdp = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sdp);
                __widl_f_set_sdp_RTCSessionDescription(self_, sdp)
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
pub static __WASM_BINDGEN_GENERATED_60db00e74a25ba79: [u8; 804usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xE2\x02\0\0\0\0\x08\0\0\x02\x15RTCSessionDescription'__widl_instanceof_RTCSessionDescription\0\0\0\0\"__widl_f_new_RTCSessionDescription\x01\0\0\x01\x15RTCSessionDescription\0\x01\0\x03new\0\0\0=__widl_f_new_with_description_init_dict_RTCSessionDescription\x01\0\0\x01\x15RTCSessionDescription\0\x01\x01\x15description_init_dict\x03new\0\0\0&__widl_f_to_json_RTCSessionDescription\0\0\0\x01\x15RTCSessionDescription\x01\0\0\x01\x01\x05self_\x06toJSON\0\0\0#__widl_f_type_RTCSessionDescription\0\0\0\x01\x15RTCSessionDescription\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0'__widl_f_set_type_RTCSessionDescription\0\0\0\x01\x15RTCSessionDescription\x01\0\x02\x04type\x01\x02\x05self_\x05type_\x04type\0\0\0\"__widl_f_sdp_RTCSessionDescription\0\0\0\x01\x15RTCSessionDescription\x01\0\x01\x03sdp\x01\x01\x05self_\x03sdp\0\0\0&__widl_f_set_sdp_RTCSessionDescription\0\0\0\x01\x15RTCSessionDescription\x01\0\x02\x03sdp\x01\x02\x05self_\x03sdp\x03sdp\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
