use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `RTCIdentityProviderRegistrar` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIdentityProviderRegistrar)\n\n*This API requires the following crate features to be activated: `RtcIdentityProviderRegistrar`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct RtcIdentityProviderRegistrar {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_RtcIdentityProviderRegistrar: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for RtcIdentityProviderRegistrar {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(28u32);
            inform(82u32);
            inform(84u32);
            inform(67u32);
            inform(73u32);
            inform(100u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(105u32);
            inform(116u32);
            inform(121u32);
            inform(80u32);
            inform(114u32);
            inform(111u32);
            inform(118u32);
            inform(105u32);
            inform(100u32);
            inform(101u32);
            inform(114u32);
            inform(82u32);
            inform(101u32);
            inform(103u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
            inform(114u32);
            inform(97u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for RtcIdentityProviderRegistrar {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for RtcIdentityProviderRegistrar {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for RtcIdentityProviderRegistrar {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a RtcIdentityProviderRegistrar {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for RtcIdentityProviderRegistrar {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            RtcIdentityProviderRegistrar {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for RtcIdentityProviderRegistrar {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a RtcIdentityProviderRegistrar {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for RtcIdentityProviderRegistrar {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<RtcIdentityProviderRegistrar>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(RtcIdentityProviderRegistrar {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for RtcIdentityProviderRegistrar {
        #[inline]
        fn from(obj: JsValue) -> RtcIdentityProviderRegistrar {
            RtcIdentityProviderRegistrar { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for RtcIdentityProviderRegistrar {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<RtcIdentityProviderRegistrar> for RtcIdentityProviderRegistrar {
        #[inline]
        fn as_ref(&self) -> &RtcIdentityProviderRegistrar {
            self
        }
    }
    impl From<RtcIdentityProviderRegistrar> for JsValue {
        #[inline]
        fn from(obj: RtcIdentityProviderRegistrar) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for RtcIdentityProviderRegistrar {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_RTCIdentityProviderRegistrar(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_RTCIdentityProviderRegistrar(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_RTCIdentityProviderRegistrar(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            RtcIdentityProviderRegistrar { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const RtcIdentityProviderRegistrar) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<RtcIdentityProviderRegistrar> for ::js_sys::Object {
    #[inline]
    fn from(obj: RtcIdentityProviderRegistrar) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for RtcIdentityProviderRegistrar {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(
    feature = "RtcIdentityProvider",
    feature = "RtcIdentityProviderRegistrar",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_register_RTCIdentityProviderRegistrar() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcIdentityProviderRegistrar as WasmDescribe>::describe();
    <&RtcIdentityProvider as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcIdentityProviderRegistrar {
    #[cfg(all(
        feature = "RtcIdentityProvider",
        feature = "RtcIdentityProviderRegistrar",
    ))]
    #[allow(bad_style)]
    #[doc = "The `register()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIdentityProviderRegistrar/register)\n\n*This API requires the following crate features to be activated: `RtcIdentityProvider`, `RtcIdentityProviderRegistrar`*"]
    #[allow(clippy::all)]
    pub fn register(&self, idp: &RtcIdentityProvider) {
        #[cfg(all(
            feature = "RtcIdentityProvider",
            feature = "RtcIdentityProviderRegistrar",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_register_RTCIdentityProviderRegistrar(
                self_: <&RtcIdentityProviderRegistrar as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                idp: <&RtcIdentityProvider as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_register_RTCIdentityProviderRegistrar(
            self_: <&RtcIdentityProviderRegistrar as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            idp: <&RtcIdentityProvider as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(idp);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&RtcIdentityProviderRegistrar as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let idp =
                    <&RtcIdentityProvider as wasm_bindgen::convert::IntoWasmAbi>::into_abi(idp);
                __widl_f_register_RTCIdentityProviderRegistrar(self_, idp)
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
pub static __WASM_BINDGEN_GENERATED_214bff31d60d8032: [u8; 292usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xE2\0\0\0\0\0\x02\0\0\x02\x1CRTCIdentityProviderRegistrar.__widl_instanceof_RTCIdentityProviderRegistrar\0\0\0\0.__widl_f_register_RTCIdentityProviderRegistrar\0\0\0\x01\x1CRTCIdentityProviderRegistrar\x01\0\0\x01\x02\x05self_\x03idp\x08register\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
