use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CSSFontFaceRule` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceRule)\n\n*This API requires the following crate features to be activated: `CssFontFaceRule`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CssFontFaceRule {
    obj: CssRule,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CssFontFaceRule: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CssFontFaceRule {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(67u32);
            inform(83u32);
            inform(83u32);
            inform(70u32);
            inform(111u32);
            inform(110u32);
            inform(116u32);
            inform(70u32);
            inform(97u32);
            inform(99u32);
            inform(101u32);
            inform(82u32);
            inform(117u32);
            inform(108u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for CssFontFaceRule {
        type Target = CssRule;
        #[inline]
        fn deref(&self) -> &CssRule {
            &self.obj
        }
    }
    impl IntoWasmAbi for CssFontFaceRule {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CssFontFaceRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CssFontFaceRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CssFontFaceRule {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CssFontFaceRule {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CssFontFaceRule {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CssFontFaceRule {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CssFontFaceRule {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CssFontFaceRule>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CssFontFaceRule {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CssFontFaceRule {
        #[inline]
        fn from(obj: JsValue) -> CssFontFaceRule {
            CssFontFaceRule { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CssFontFaceRule {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CssFontFaceRule> for CssFontFaceRule {
        #[inline]
        fn as_ref(&self) -> &CssFontFaceRule {
            self
        }
    }
    impl From<CssFontFaceRule> for JsValue {
        #[inline]
        fn from(obj: CssFontFaceRule) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CssFontFaceRule {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CSSFontFaceRule(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CSSFontFaceRule(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CSSFontFaceRule(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CssFontFaceRule { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CssFontFaceRule) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CssFontFaceRule> for CssRule {
    #[inline]
    fn from(obj: CssFontFaceRule) -> CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<CssRule> for CssFontFaceRule {
    #[inline]
    fn as_ref(&self) -> &CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CssFontFaceRule> for ::js_sys::Object {
    #[inline]
    fn from(obj: CssFontFaceRule) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CssFontFaceRule {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CssFontFaceRule", feature = "CssStyleDeclaration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_style_CSSFontFaceRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssFontFaceRule as WasmDescribe>::describe();
    <CssStyleDeclaration as WasmDescribe>::describe();
}
impl CssFontFaceRule {
    #[cfg(all(feature = "CssFontFaceRule", feature = "CssStyleDeclaration",))]
    #[allow(bad_style)]
    #[doc = "The `style` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceRule/style)\n\n*This API requires the following crate features to be activated: `CssFontFaceRule`, `CssStyleDeclaration`*"]
    #[allow(clippy::all)]
    pub fn style(&self) -> CssStyleDeclaration {
        #[cfg(all(feature = "CssFontFaceRule", feature = "CssStyleDeclaration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_style_CSSFontFaceRule(
                self_: <&CssFontFaceRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <CssStyleDeclaration as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_style_CSSFontFaceRule(
            self_: <&CssFontFaceRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CssFontFaceRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_style_CSSFontFaceRule(self_)
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
pub static __WASM_BINDGEN_GENERATED_a25775ada3dd656d: [u8; 236usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xAA\0\0\0\0\0\x02\0\0\x02\x0FCSSFontFaceRule!__widl_instanceof_CSSFontFaceRule\0\0\0\0\x1E__widl_f_style_CSSFontFaceRule\0\0\0\x01\x0FCSSFontFaceRule\x01\0\x01\x05style\x01\x01\x05self_\x05style\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
