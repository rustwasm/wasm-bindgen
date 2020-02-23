use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CSSStyleRule` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule)\n\n*This API requires the following crate features to be activated: `CssStyleRule`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CssStyleRule {
    obj: CssRule,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CssStyleRule: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CssStyleRule {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(67u32);
            inform(83u32);
            inform(83u32);
            inform(83u32);
            inform(116u32);
            inform(121u32);
            inform(108u32);
            inform(101u32);
            inform(82u32);
            inform(117u32);
            inform(108u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for CssStyleRule {
        type Target = CssRule;
        #[inline]
        fn deref(&self) -> &CssRule {
            &self.obj
        }
    }
    impl IntoWasmAbi for CssStyleRule {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CssStyleRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CssStyleRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CssStyleRule {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CssStyleRule {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CssStyleRule {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CssStyleRule {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CssStyleRule {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CssStyleRule>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CssStyleRule {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CssStyleRule {
        #[inline]
        fn from(obj: JsValue) -> CssStyleRule {
            CssStyleRule { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CssStyleRule {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CssStyleRule> for CssStyleRule {
        #[inline]
        fn as_ref(&self) -> &CssStyleRule {
            self
        }
    }
    impl From<CssStyleRule> for JsValue {
        #[inline]
        fn from(obj: CssStyleRule) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CssStyleRule {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CSSStyleRule(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CSSStyleRule(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CSSStyleRule(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CssStyleRule { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CssStyleRule) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CssStyleRule> for CssRule {
    #[inline]
    fn from(obj: CssStyleRule) -> CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<CssRule> for CssStyleRule {
    #[inline]
    fn as_ref(&self) -> &CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CssStyleRule> for ::js_sys::Object {
    #[inline]
    fn from(obj: CssStyleRule) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CssStyleRule {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CssStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_selector_text_CSSStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssStyleRule as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssStyleRule {
    #[cfg(all(feature = "CssStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `selectorText` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/selectorText)\n\n*This API requires the following crate features to be activated: `CssStyleRule`*"]
    #[allow(clippy::all)]
    pub fn selector_text(&self) -> String {
        #[cfg(all(feature = "CssStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_selector_text_CSSStyleRule(
                self_: <&CssStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_selector_text_CSSStyleRule(
            self_: <&CssStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CssStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_selector_text_CSSStyleRule(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_selector_text_CSSStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssStyleRule as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssStyleRule {
    #[cfg(all(feature = "CssStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `selectorText` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/selectorText)\n\n*This API requires the following crate features to be activated: `CssStyleRule`*"]
    #[allow(clippy::all)]
    pub fn set_selector_text(&self, selector_text: &str) {
        #[cfg(all(feature = "CssStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_selector_text_CSSStyleRule(
                self_: <&CssStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selector_text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_selector_text_CSSStyleRule(
            self_: <&CssStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            selector_text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(selector_text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CssStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selector_text =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(selector_text);
                __widl_f_set_selector_text_CSSStyleRule(self_, selector_text)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CssStyleDeclaration", feature = "CssStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_style_CSSStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssStyleRule as WasmDescribe>::describe();
    <CssStyleDeclaration as WasmDescribe>::describe();
}
impl CssStyleRule {
    #[cfg(all(feature = "CssStyleDeclaration", feature = "CssStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `style` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/style)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`, `CssStyleRule`*"]
    #[allow(clippy::all)]
    pub fn style(&self) -> CssStyleDeclaration {
        #[cfg(all(feature = "CssStyleDeclaration", feature = "CssStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_style_CSSStyleRule(
                self_: <&CssStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <CssStyleDeclaration as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_style_CSSStyleRule(
            self_: <&CssStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <CssStyleDeclaration as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CssStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_style_CSSStyleRule(self_)
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
pub static __WASM_BINDGEN_GENERATED_49a9e358a33a0f50: [u8; 428usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}j\x01\0\0\0\0\x04\0\0\x02\x0CCSSStyleRule\x1E__widl_instanceof_CSSStyleRule\0\0\0\0#__widl_f_selector_text_CSSStyleRule\0\0\0\x01\x0CCSSStyleRule\x01\0\x01\x0CselectorText\x01\x01\x05self_\x0CselectorText\0\0\0'__widl_f_set_selector_text_CSSStyleRule\0\0\0\x01\x0CCSSStyleRule\x01\0\x02\x0CselectorText\x01\x02\x05self_\rselector_text\x0CselectorText\0\0\0\x1B__widl_f_style_CSSStyleRule\0\0\0\x01\x0CCSSStyleRule\x01\0\x01\x05style\x01\x01\x05self_\x05style\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
