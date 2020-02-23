use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Presentation` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Presentation)\n\n*This API requires the following crate features to be activated: `Presentation`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Presentation {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Presentation: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Presentation {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(80u32);
            inform(114u32);
            inform(101u32);
            inform(115u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for Presentation {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Presentation {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Presentation {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Presentation {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Presentation {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Presentation {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Presentation {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Presentation {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Presentation {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Presentation>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Presentation {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Presentation {
        #[inline]
        fn from(obj: JsValue) -> Presentation {
            Presentation { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Presentation {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Presentation> for Presentation {
        #[inline]
        fn as_ref(&self) -> &Presentation {
            self
        }
    }
    impl From<Presentation> for JsValue {
        #[inline]
        fn from(obj: Presentation) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Presentation {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Presentation(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Presentation(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Presentation(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Presentation { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Presentation) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Presentation> for ::js_sys::Object {
    #[inline]
    fn from(obj: Presentation) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Presentation {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Presentation", feature = "PresentationRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_default_request_Presentation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Presentation as WasmDescribe>::describe();
    <Option<PresentationRequest> as WasmDescribe>::describe();
}
impl Presentation {
    #[cfg(all(feature = "Presentation", feature = "PresentationRequest",))]
    #[allow(bad_style)]
    #[doc = "The `defaultRequest` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Presentation/defaultRequest)\n\n*This API requires the following crate features to be activated: `Presentation`, `PresentationRequest`*"]
    #[allow(clippy::all)]
    pub fn default_request(&self) -> Option<PresentationRequest> {
        #[cfg(all(feature = "Presentation", feature = "PresentationRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_default_request_Presentation(
                self_: <&Presentation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<PresentationRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_default_request_Presentation(
            self_: <&Presentation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<PresentationRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Presentation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_default_request_Presentation(self_)
            };
            <Option<PresentationRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Presentation", feature = "PresentationRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_default_request_Presentation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Presentation as WasmDescribe>::describe();
    <Option<&PresentationRequest> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Presentation {
    #[cfg(all(feature = "Presentation", feature = "PresentationRequest",))]
    #[allow(bad_style)]
    #[doc = "The `defaultRequest` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Presentation/defaultRequest)\n\n*This API requires the following crate features to be activated: `Presentation`, `PresentationRequest`*"]
    #[allow(clippy::all)]
    pub fn set_default_request(&self, default_request: Option<&PresentationRequest>) {
        #[cfg(all(feature = "Presentation", feature = "PresentationRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_default_request_Presentation(
                self_: <&Presentation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                default_request : < Option < & PresentationRequest > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_default_request_Presentation(
            self_: <&Presentation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            default_request : < Option < & PresentationRequest > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(default_request);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Presentation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let default_request =
                    <Option<&PresentationRequest> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        default_request,
                    );
                __widl_f_set_default_request_Presentation(self_, default_request)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Presentation", feature = "PresentationReceiver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_receiver_Presentation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Presentation as WasmDescribe>::describe();
    <Option<PresentationReceiver> as WasmDescribe>::describe();
}
impl Presentation {
    #[cfg(all(feature = "Presentation", feature = "PresentationReceiver",))]
    #[allow(bad_style)]
    #[doc = "The `receiver` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Presentation/receiver)\n\n*This API requires the following crate features to be activated: `Presentation`, `PresentationReceiver`*"]
    #[allow(clippy::all)]
    pub fn receiver(&self) -> Option<PresentationReceiver> {
        #[cfg(all(feature = "Presentation", feature = "PresentationReceiver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_receiver_Presentation(
                self_: <&Presentation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<PresentationReceiver> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_receiver_Presentation(
            self_: <&Presentation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<PresentationReceiver> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Presentation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_receiver_Presentation(self_)
            };
            <Option<PresentationReceiver> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_6610e54367e9fc7c: [u8; 451usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x81\x01\0\0\0\0\x04\0\0\x02\x0CPresentation\x1E__widl_instanceof_Presentation\0\0\0\0%__widl_f_default_request_Presentation\0\0\0\x01\x0CPresentation\x01\0\x01\x0EdefaultRequest\x01\x01\x05self_\x0EdefaultRequest\0\0\0)__widl_f_set_default_request_Presentation\0\0\0\x01\x0CPresentation\x01\0\x02\x0EdefaultRequest\x01\x02\x05self_\x0Fdefault_request\x0EdefaultRequest\0\0\0\x1E__widl_f_receiver_Presentation\0\0\0\x01\x0CPresentation\x01\0\x01\x08receiver\x01\x01\x05self_\x08receiver\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
