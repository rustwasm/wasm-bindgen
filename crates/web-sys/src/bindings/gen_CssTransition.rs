use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CSSTransition` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSTransition)\n\n*This API requires the following crate features to be activated: `CssTransition`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CssTransition {
    obj: Animation,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CssTransition: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CssTransition {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(67u32);
            inform(83u32);
            inform(83u32);
            inform(84u32);
            inform(114u32);
            inform(97u32);
            inform(110u32);
            inform(115u32);
            inform(105u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for CssTransition {
        type Target = Animation;
        #[inline]
        fn deref(&self) -> &Animation {
            &self.obj
        }
    }
    impl IntoWasmAbi for CssTransition {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CssTransition {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CssTransition {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CssTransition {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CssTransition {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CssTransition {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CssTransition {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CssTransition {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CssTransition>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CssTransition {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CssTransition {
        #[inline]
        fn from(obj: JsValue) -> CssTransition {
            CssTransition { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CssTransition {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CssTransition> for CssTransition {
        #[inline]
        fn as_ref(&self) -> &CssTransition {
            self
        }
    }
    impl From<CssTransition> for JsValue {
        #[inline]
        fn from(obj: CssTransition) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CssTransition {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CSSTransition(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CSSTransition(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CSSTransition(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CssTransition { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CssTransition) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CssTransition> for Animation {
    #[inline]
    fn from(obj: CssTransition) -> Animation {
        use wasm_bindgen::JsCast;
        Animation::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Animation> for CssTransition {
    #[inline]
    fn as_ref(&self) -> &Animation {
        use wasm_bindgen::JsCast;
        Animation::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CssTransition> for EventTarget {
    #[inline]
    fn from(obj: CssTransition) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for CssTransition {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CssTransition> for ::js_sys::Object {
    #[inline]
    fn from(obj: CssTransition) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CssTransition {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CssTransition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transition_property_CSSTransition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssTransition as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssTransition {
    #[cfg(all(feature = "CssTransition",))]
    #[allow(bad_style)]
    #[doc = "The `transitionProperty` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSTransition/transitionProperty)\n\n*This API requires the following crate features to be activated: `CssTransition`*"]
    #[allow(clippy::all)]
    pub fn transition_property(&self) -> String {
        #[cfg(all(feature = "CssTransition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transition_property_CSSTransition(
                self_: <&CssTransition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transition_property_CSSTransition(
            self_: <&CssTransition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CssTransition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_transition_property_CSSTransition(self_)
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
pub static __WASM_BINDGEN_GENERATED_8a4f457117fe92f9: [u8; 268usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xCA\0\0\0\0\0\x02\0\0\x02\rCSSTransition\x1F__widl_instanceof_CSSTransition\0\0\0\0*__widl_f_transition_property_CSSTransition\0\0\0\x01\rCSSTransition\x01\0\x01\x12transitionProperty\x01\x01\x05self_\x12transitionProperty\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
