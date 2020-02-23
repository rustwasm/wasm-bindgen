use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CSSRule` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule)\n\n*This API requires the following crate features to be activated: `CssRule`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CssRule {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CssRule: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CssRule {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(7u32);
            inform(67u32);
            inform(83u32);
            inform(83u32);
            inform(82u32);
            inform(117u32);
            inform(108u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for CssRule {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for CssRule {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CssRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CssRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CssRule {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CssRule {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CssRule {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CssRule {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CssRule {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CssRule>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CssRule {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CssRule {
        #[inline]
        fn from(obj: JsValue) -> CssRule {
            CssRule { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CssRule {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CssRule> for CssRule {
        #[inline]
        fn as_ref(&self) -> &CssRule {
            self
        }
    }
    impl From<CssRule> for JsValue {
        #[inline]
        fn from(obj: CssRule) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CssRule {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CSSRule(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CSSRule(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CSSRule(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CssRule { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CssRule) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CssRule> for ::js_sys::Object {
    #[inline]
    fn from(obj: CssRule) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CssRule {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CssRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_CSSRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssRule as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl CssRule {
    #[cfg(all(feature = "CssRule",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/type)\n\n*This API requires the following crate features to be activated: `CssRule`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> u16 {
        #[cfg(all(feature = "CssRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_CSSRule(
                self_: <&CssRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_CSSRule(
            self_: <&CssRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CssRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_CSSRule(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_css_text_CSSRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssRule as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssRule {
    #[cfg(all(feature = "CssRule",))]
    #[allow(bad_style)]
    #[doc = "The `cssText` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/cssText)\n\n*This API requires the following crate features to be activated: `CssRule`*"]
    #[allow(clippy::all)]
    pub fn css_text(&self) -> String {
        #[cfg(all(feature = "CssRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_css_text_CSSRule(
                self_: <&CssRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_css_text_CSSRule(
            self_: <&CssRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CssRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_css_text_CSSRule(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_css_text_CSSRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssRule as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssRule {
    #[cfg(all(feature = "CssRule",))]
    #[allow(bad_style)]
    #[doc = "The `cssText` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/cssText)\n\n*This API requires the following crate features to be activated: `CssRule`*"]
    #[allow(clippy::all)]
    pub fn set_css_text(&self, css_text: &str) {
        #[cfg(all(feature = "CssRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_css_text_CSSRule(
                self_: <&CssRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                css_text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_css_text_CSSRule(
            self_: <&CssRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            css_text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(css_text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CssRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let css_text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(css_text);
                __widl_f_set_css_text_CSSRule(self_, css_text)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CssRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_parent_rule_CSSRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssRule as WasmDescribe>::describe();
    <Option<CssRule> as WasmDescribe>::describe();
}
impl CssRule {
    #[cfg(all(feature = "CssRule",))]
    #[allow(bad_style)]
    #[doc = "The `parentRule` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/parentRule)\n\n*This API requires the following crate features to be activated: `CssRule`*"]
    #[allow(clippy::all)]
    pub fn parent_rule(&self) -> Option<CssRule> {
        #[cfg(all(feature = "CssRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_parent_rule_CSSRule(
                self_: <&CssRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<CssRule> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_parent_rule_CSSRule(
            self_: <&CssRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<CssRule> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CssRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_parent_rule_CSSRule(self_)
            };
            <Option<CssRule> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssRule", feature = "CssStyleSheet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_parent_style_sheet_CSSRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssRule as WasmDescribe>::describe();
    <Option<CssStyleSheet> as WasmDescribe>::describe();
}
impl CssRule {
    #[cfg(all(feature = "CssRule", feature = "CssStyleSheet",))]
    #[allow(bad_style)]
    #[doc = "The `parentStyleSheet` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/parentStyleSheet)\n\n*This API requires the following crate features to be activated: `CssRule`, `CssStyleSheet`*"]
    #[allow(clippy::all)]
    pub fn parent_style_sheet(&self) -> Option<CssStyleSheet> {
        #[cfg(all(feature = "CssRule", feature = "CssStyleSheet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_parent_style_sheet_CSSRule(
                self_: <&CssRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<CssStyleSheet> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_parent_style_sheet_CSSRule(
            self_: <&CssRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<CssStyleSheet> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CssRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_parent_style_sheet_CSSRule(self_)
            };
            <Option<CssStyleSheet> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl CssRule {
    pub const STYLE_RULE: u16 = 1u64 as u16;
}
impl CssRule {
    pub const CHARSET_RULE: u16 = 2u64 as u16;
}
impl CssRule {
    pub const IMPORT_RULE: u16 = 3u64 as u16;
}
impl CssRule {
    pub const MEDIA_RULE: u16 = 4u64 as u16;
}
impl CssRule {
    pub const FONT_FACE_RULE: u16 = 5u64 as u16;
}
impl CssRule {
    pub const PAGE_RULE: u16 = 6u64 as u16;
}
impl CssRule {
    pub const NAMESPACE_RULE: u16 = 10u64 as u16;
}
impl CssRule {
    pub const KEYFRAMES_RULE: u16 = 7u64 as u16;
}
impl CssRule {
    pub const KEYFRAME_RULE: u16 = 8u64 as u16;
}
impl CssRule {
    pub const COUNTER_STYLE_RULE: u16 = 11u64 as u16;
}
impl CssRule {
    pub const SUPPORTS_RULE: u16 = 12u64 as u16;
}
impl CssRule {
    pub const FONT_FEATURE_VALUES_RULE: u16 = 14u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8031b7b063d974f1: [u8; 523usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xC9\x01\0\0\0\0\x06\0\0\x02\x07CSSRule\x19__widl_instanceof_CSSRule\0\0\0\0\x15__widl_f_type_CSSRule\0\0\0\x01\x07CSSRule\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0\x19__widl_f_css_text_CSSRule\0\0\0\x01\x07CSSRule\x01\0\x01\x07cssText\x01\x01\x05self_\x07cssText\0\0\0\x1D__widl_f_set_css_text_CSSRule\0\0\0\x01\x07CSSRule\x01\0\x02\x07cssText\x01\x02\x05self_\x08css_text\x07cssText\0\0\0\x1C__widl_f_parent_rule_CSSRule\0\0\0\x01\x07CSSRule\x01\0\x01\nparentRule\x01\x01\x05self_\nparentRule\0\0\0#__widl_f_parent_style_sheet_CSSRule\0\0\0\x01\x07CSSRule\x01\0\x01\x10parentStyleSheet\x01\x01\x05self_\x10parentStyleSheet\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
