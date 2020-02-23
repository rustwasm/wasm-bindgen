use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CSSFontFeatureValuesRule` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule)\n\n*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CssFontFeatureValuesRule {
    obj: CssRule,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CssFontFeatureValuesRule: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CssFontFeatureValuesRule {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(24u32);
            inform(67u32);
            inform(83u32);
            inform(83u32);
            inform(70u32);
            inform(111u32);
            inform(110u32);
            inform(116u32);
            inform(70u32);
            inform(101u32);
            inform(97u32);
            inform(116u32);
            inform(117u32);
            inform(114u32);
            inform(101u32);
            inform(86u32);
            inform(97u32);
            inform(108u32);
            inform(117u32);
            inform(101u32);
            inform(115u32);
            inform(82u32);
            inform(117u32);
            inform(108u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for CssFontFeatureValuesRule {
        type Target = CssRule;
        #[inline]
        fn deref(&self) -> &CssRule {
            &self.obj
        }
    }
    impl IntoWasmAbi for CssFontFeatureValuesRule {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CssFontFeatureValuesRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CssFontFeatureValuesRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CssFontFeatureValuesRule {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CssFontFeatureValuesRule {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CssFontFeatureValuesRule {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CssFontFeatureValuesRule {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CssFontFeatureValuesRule {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CssFontFeatureValuesRule>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CssFontFeatureValuesRule {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CssFontFeatureValuesRule {
        #[inline]
        fn from(obj: JsValue) -> CssFontFeatureValuesRule {
            CssFontFeatureValuesRule { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CssFontFeatureValuesRule {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CssFontFeatureValuesRule> for CssFontFeatureValuesRule {
        #[inline]
        fn as_ref(&self) -> &CssFontFeatureValuesRule {
            self
        }
    }
    impl From<CssFontFeatureValuesRule> for JsValue {
        #[inline]
        fn from(obj: CssFontFeatureValuesRule) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CssFontFeatureValuesRule {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CSSFontFeatureValuesRule(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CSSFontFeatureValuesRule(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CSSFontFeatureValuesRule(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CssFontFeatureValuesRule { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CssFontFeatureValuesRule) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CssFontFeatureValuesRule> for CssRule {
    #[inline]
    fn from(obj: CssFontFeatureValuesRule) -> CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<CssRule> for CssFontFeatureValuesRule {
    #[inline]
    fn as_ref(&self) -> &CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CssFontFeatureValuesRule> for ::js_sys::Object {
    #[inline]
    fn from(obj: CssFontFeatureValuesRule) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CssFontFeatureValuesRule {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CssFontFeatureValuesRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_font_family_CSSFontFeatureValuesRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssFontFeatureValuesRule as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssFontFeatureValuesRule {
    #[cfg(all(feature = "CssFontFeatureValuesRule",))]
    #[allow(bad_style)]
    #[doc = "The `fontFamily` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/fontFamily)\n\n*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*"]
    #[allow(clippy::all)]
    pub fn font_family(&self) -> String {
        #[cfg(all(feature = "CssFontFeatureValuesRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_font_family_CSSFontFeatureValuesRule(
                self_: <&CssFontFeatureValuesRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_font_family_CSSFontFeatureValuesRule(
            self_: <&CssFontFeatureValuesRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CssFontFeatureValuesRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_font_family_CSSFontFeatureValuesRule(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssFontFeatureValuesRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_font_family_CSSFontFeatureValuesRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssFontFeatureValuesRule as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssFontFeatureValuesRule {
    #[cfg(all(feature = "CssFontFeatureValuesRule",))]
    #[allow(bad_style)]
    #[doc = "The `fontFamily` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/fontFamily)\n\n*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*"]
    #[allow(clippy::all)]
    pub fn set_font_family(&self, font_family: &str) {
        #[cfg(all(feature = "CssFontFeatureValuesRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_font_family_CSSFontFeatureValuesRule(
                self_: <&CssFontFeatureValuesRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                font_family: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_font_family_CSSFontFeatureValuesRule(
            self_: <&CssFontFeatureValuesRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            font_family: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(font_family);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssFontFeatureValuesRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let font_family =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(font_family);
                __widl_f_set_font_family_CSSFontFeatureValuesRule(self_, font_family)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CssFontFeatureValuesRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_text_CSSFontFeatureValuesRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssFontFeatureValuesRule as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssFontFeatureValuesRule {
    #[cfg(all(feature = "CssFontFeatureValuesRule",))]
    #[allow(bad_style)]
    #[doc = "The `valueText` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/valueText)\n\n*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*"]
    #[allow(clippy::all)]
    pub fn value_text(&self) -> String {
        #[cfg(all(feature = "CssFontFeatureValuesRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_text_CSSFontFeatureValuesRule(
                self_: <&CssFontFeatureValuesRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_text_CSSFontFeatureValuesRule(
            self_: <&CssFontFeatureValuesRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CssFontFeatureValuesRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_value_text_CSSFontFeatureValuesRule(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssFontFeatureValuesRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_text_CSSFontFeatureValuesRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssFontFeatureValuesRule as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssFontFeatureValuesRule {
    #[cfg(all(feature = "CssFontFeatureValuesRule",))]
    #[allow(bad_style)]
    #[doc = "The `valueText` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/valueText)\n\n*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*"]
    #[allow(clippy::all)]
    pub fn set_value_text(&self, value_text: &str) {
        #[cfg(all(feature = "CssFontFeatureValuesRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_text_CSSFontFeatureValuesRule(
                self_: <&CssFontFeatureValuesRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value_text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_text_CSSFontFeatureValuesRule(
            self_: <&CssFontFeatureValuesRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value_text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(value_text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssFontFeatureValuesRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let value_text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value_text);
                __widl_f_set_value_text_CSSFontFeatureValuesRule(self_, value_text)
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
pub static __WASM_BINDGEN_GENERATED_d45993bfc07eabde: [u8; 646usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}D\x02\0\0\0\0\x05\0\0\x02\x18CSSFontFeatureValuesRule*__widl_instanceof_CSSFontFeatureValuesRule\0\0\0\0-__widl_f_font_family_CSSFontFeatureValuesRule\0\0\0\x01\x18CSSFontFeatureValuesRule\x01\0\x01\nfontFamily\x01\x01\x05self_\nfontFamily\0\0\01__widl_f_set_font_family_CSSFontFeatureValuesRule\0\0\0\x01\x18CSSFontFeatureValuesRule\x01\0\x02\nfontFamily\x01\x02\x05self_\x0Bfont_family\nfontFamily\0\0\0,__widl_f_value_text_CSSFontFeatureValuesRule\0\0\0\x01\x18CSSFontFeatureValuesRule\x01\0\x01\tvalueText\x01\x01\x05self_\tvalueText\0\0\00__widl_f_set_value_text_CSSFontFeatureValuesRule\0\0\0\x01\x18CSSFontFeatureValuesRule\x01\0\x02\tvalueText\x01\x02\x05self_\nvalue_text\tvalueText\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
