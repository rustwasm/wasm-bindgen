use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CSSAnimation` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSAnimation)\n\n*This API requires the following crate features to be activated: `CssAnimation`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CssAnimation {
    obj: Animation,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CssAnimation: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CssAnimation {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(67u32);
            inform(83u32);
            inform(83u32);
            inform(65u32);
            inform(110u32);
            inform(105u32);
            inform(109u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for CssAnimation {
        type Target = Animation;
        #[inline]
        fn deref(&self) -> &Animation {
            &self.obj
        }
    }
    impl IntoWasmAbi for CssAnimation {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CssAnimation {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CssAnimation {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CssAnimation {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CssAnimation {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CssAnimation {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CssAnimation {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CssAnimation {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CssAnimation>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CssAnimation {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CssAnimation {
        #[inline]
        fn from(obj: JsValue) -> CssAnimation {
            CssAnimation { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CssAnimation {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CssAnimation> for CssAnimation {
        #[inline]
        fn as_ref(&self) -> &CssAnimation {
            self
        }
    }
    impl From<CssAnimation> for JsValue {
        #[inline]
        fn from(obj: CssAnimation) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CssAnimation {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CSSAnimation(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CSSAnimation(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CSSAnimation(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CssAnimation { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CssAnimation) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CssAnimation> for Animation {
    #[inline]
    fn from(obj: CssAnimation) -> Animation {
        use wasm_bindgen::JsCast;
        Animation::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Animation> for CssAnimation {
    #[inline]
    fn as_ref(&self) -> &Animation {
        use wasm_bindgen::JsCast;
        Animation::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CssAnimation> for EventTarget {
    #[inline]
    fn from(obj: CssAnimation) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for CssAnimation {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CssAnimation> for ::js_sys::Object {
    #[inline]
    fn from(obj: CssAnimation) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CssAnimation {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CssAnimation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_animation_name_CSSAnimation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssAnimation as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssAnimation {
    #[cfg(all(feature = "CssAnimation",))]
    #[allow(bad_style)]
    #[doc = "The `animationName` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSAnimation/animationName)\n\n*This API requires the following crate features to be activated: `CssAnimation`*"]
    #[allow(clippy::all)]
    pub fn animation_name(&self) -> String {
        #[cfg(all(feature = "CssAnimation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_animation_name_CSSAnimation(
                self_: <&CssAnimation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_animation_name_CSSAnimation(
            self_: <&CssAnimation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CssAnimation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_animation_name_CSSAnimation(self_)
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
pub static __WASM_BINDGEN_GENERATED_50cf9da8cb889c0d: [u8; 249usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xB7\0\0\0\0\0\x02\0\0\x02\x0CCSSAnimation\x1E__widl_instanceof_CSSAnimation\0\0\0\0$__widl_f_animation_name_CSSAnimation\0\0\0\x01\x0CCSSAnimation\x01\0\x01\ranimationName\x01\x01\x05self_\ranimationName\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
