use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `RTCCertificate` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCCertificate)\n\n*This API requires the following crate features to be activated: `RtcCertificate`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct RtcCertificate {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_RtcCertificate: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for RtcCertificate {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(82u32);
            inform(84u32);
            inform(67u32);
            inform(67u32);
            inform(101u32);
            inform(114u32);
            inform(116u32);
            inform(105u32);
            inform(102u32);
            inform(105u32);
            inform(99u32);
            inform(97u32);
            inform(116u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for RtcCertificate {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for RtcCertificate {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for RtcCertificate {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a RtcCertificate {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for RtcCertificate {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            RtcCertificate {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for RtcCertificate {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a RtcCertificate {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for RtcCertificate {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<RtcCertificate>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(RtcCertificate {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for RtcCertificate {
        #[inline]
        fn from(obj: JsValue) -> RtcCertificate {
            RtcCertificate { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for RtcCertificate {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<RtcCertificate> for RtcCertificate {
        #[inline]
        fn as_ref(&self) -> &RtcCertificate {
            self
        }
    }
    impl From<RtcCertificate> for JsValue {
        #[inline]
        fn from(obj: RtcCertificate) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for RtcCertificate {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_RTCCertificate(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_RTCCertificate(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_RTCCertificate(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            RtcCertificate { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const RtcCertificate) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<RtcCertificate> for ::js_sys::Object {
    #[inline]
    fn from(obj: RtcCertificate) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for RtcCertificate {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "RtcCertificate",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_expires_RTCCertificate() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcCertificate as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl RtcCertificate {
    #[cfg(all(feature = "RtcCertificate",))]
    #[allow(bad_style)]
    #[doc = "The `expires` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCCertificate/expires)\n\n*This API requires the following crate features to be activated: `RtcCertificate`*"]
    #[allow(clippy::all)]
    pub fn expires(&self) -> f64 {
        #[cfg(all(feature = "RtcCertificate",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_expires_RTCCertificate(
                self_: <&RtcCertificate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_expires_RTCCertificate(
            self_: <&RtcCertificate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcCertificate as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_expires_RTCCertificate(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_abb86a508baf857d: [u8; 238usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xAC\0\0\0\0\0\x02\0\0\x02\x0ERTCCertificate __widl_instanceof_RTCCertificate\0\0\0\0\x1F__widl_f_expires_RTCCertificate\0\0\0\x01\x0ERTCCertificate\x01\0\x01\x07expires\x01\x01\x05self_\x07expires\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
