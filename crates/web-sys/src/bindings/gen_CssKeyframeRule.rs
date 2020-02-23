use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CSSKeyframeRule` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule)\n\n*This API requires the following crate features to be activated: `CssKeyframeRule`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CssKeyframeRule {
    obj: CssRule,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CssKeyframeRule: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CssKeyframeRule {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(67u32);
            inform(83u32);
            inform(83u32);
            inform(75u32);
            inform(101u32);
            inform(121u32);
            inform(102u32);
            inform(114u32);
            inform(97u32);
            inform(109u32);
            inform(101u32);
            inform(82u32);
            inform(117u32);
            inform(108u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for CssKeyframeRule {
        type Target = CssRule;
        #[inline]
        fn deref(&self) -> &CssRule {
            &self.obj
        }
    }
    impl IntoWasmAbi for CssKeyframeRule {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CssKeyframeRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CssKeyframeRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CssKeyframeRule {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CssKeyframeRule {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CssKeyframeRule {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CssKeyframeRule {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CssKeyframeRule {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CssKeyframeRule>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CssKeyframeRule {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CssKeyframeRule {
        #[inline]
        fn from(obj: JsValue) -> CssKeyframeRule {
            CssKeyframeRule { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CssKeyframeRule {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CssKeyframeRule> for CssKeyframeRule {
        #[inline]
        fn as_ref(&self) -> &CssKeyframeRule {
            self
        }
    }
    impl From<CssKeyframeRule> for JsValue {
        #[inline]
        fn from(obj: CssKeyframeRule) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CssKeyframeRule {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CSSKeyframeRule(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CSSKeyframeRule(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CSSKeyframeRule(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CssKeyframeRule { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CssKeyframeRule) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CssKeyframeRule> for CssRule {
    #[inline]
    fn from(obj: CssKeyframeRule) -> CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<CssRule> for CssKeyframeRule {
    #[inline]
    fn as_ref(&self) -> &CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CssKeyframeRule> for ::js_sys::Object {
    #[inline]
    fn from(obj: CssKeyframeRule) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CssKeyframeRule {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CssKeyframeRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_key_text_CSSKeyframeRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssKeyframeRule as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssKeyframeRule {
    #[cfg(all(feature = "CssKeyframeRule",))]
    #[allow(bad_style)]
    #[doc = "The `keyText` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule/keyText)\n\n*This API requires the following crate features to be activated: `CssKeyframeRule`*"]
    #[allow(clippy::all)]
    pub fn key_text(&self) -> String {
        #[cfg(all(feature = "CssKeyframeRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_key_text_CSSKeyframeRule(
                self_: <&CssKeyframeRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_key_text_CSSKeyframeRule(
            self_: <&CssKeyframeRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CssKeyframeRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_key_text_CSSKeyframeRule(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssKeyframeRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_key_text_CSSKeyframeRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssKeyframeRule as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssKeyframeRule {
    #[cfg(all(feature = "CssKeyframeRule",))]
    #[allow(bad_style)]
    #[doc = "The `keyText` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule/keyText)\n\n*This API requires the following crate features to be activated: `CssKeyframeRule`*"]
    #[allow(clippy::all)]
    pub fn set_key_text(&self, key_text: &str) {
        #[cfg(all(feature = "CssKeyframeRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_key_text_CSSKeyframeRule(
                self_: <&CssKeyframeRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_key_text_CSSKeyframeRule(
            self_: <&CssKeyframeRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(key_text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssKeyframeRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key_text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key_text);
                __widl_f_set_key_text_CSSKeyframeRule(self_, key_text)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CssKeyframeRule", feature = "CssStyleDeclaration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_style_CSSKeyframeRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssKeyframeRule as WasmDescribe>::describe();
    <CssStyleDeclaration as WasmDescribe>::describe();
}
impl CssKeyframeRule {
    #[cfg(all(feature = "CssKeyframeRule", feature = "CssStyleDeclaration",))]
    #[allow(bad_style)]
    #[doc = "The `style` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule/style)\n\n*This API requires the following crate features to be activated: `CssKeyframeRule`, `CssStyleDeclaration`*"]
    #[allow(clippy::all)]
    pub fn style(&self) -> CssStyleDeclaration {
        #[cfg(all(feature = "CssKeyframeRule", feature = "CssStyleDeclaration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_style_CSSKeyframeRule(
                self_: <&CssKeyframeRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <CssStyleDeclaration as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_style_CSSKeyframeRule(
            self_: <&CssKeyframeRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <CssStyleDeclaration as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssKeyframeRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_style_CSSKeyframeRule(self_)
            };
            <CssStyleDeclaration as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_91bea498a341a674: [u8; 417usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}_\x01\0\0\0\0\x04\0\0\x02\x0FCSSKeyframeRule!__widl_instanceof_CSSKeyframeRule\0\0\0\0!__widl_f_key_text_CSSKeyframeRule\0\0\0\x01\x0FCSSKeyframeRule\x01\0\x01\x07keyText\x01\x01\x05self_\x07keyText\0\0\0%__widl_f_set_key_text_CSSKeyframeRule\0\0\0\x01\x0FCSSKeyframeRule\x01\0\x02\x07keyText\x01\x02\x05self_\x08key_text\x07keyText\0\0\0\x1E__widl_f_style_CSSKeyframeRule\0\0\0\x01\x0FCSSKeyframeRule\x01\0\x01\x05style\x01\x01\x05self_\x05style\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
