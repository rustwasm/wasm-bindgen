use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CSSStyleSheet` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet)\n\n*This API requires the following crate features to be activated: `CssStyleSheet`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CssStyleSheet {
    obj: StyleSheet,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CssStyleSheet: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CssStyleSheet {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(67u32);
            inform(83u32);
            inform(83u32);
            inform(83u32);
            inform(116u32);
            inform(121u32);
            inform(108u32);
            inform(101u32);
            inform(83u32);
            inform(104u32);
            inform(101u32);
            inform(101u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for CssStyleSheet {
        type Target = StyleSheet;
        #[inline]
        fn deref(&self) -> &StyleSheet {
            &self.obj
        }
    }
    impl IntoWasmAbi for CssStyleSheet {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CssStyleSheet {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CssStyleSheet {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CssStyleSheet {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CssStyleSheet {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CssStyleSheet {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CssStyleSheet {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CssStyleSheet {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CssStyleSheet>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CssStyleSheet {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CssStyleSheet {
        #[inline]
        fn from(obj: JsValue) -> CssStyleSheet {
            CssStyleSheet { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CssStyleSheet {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CssStyleSheet> for CssStyleSheet {
        #[inline]
        fn as_ref(&self) -> &CssStyleSheet {
            self
        }
    }
    impl From<CssStyleSheet> for JsValue {
        #[inline]
        fn from(obj: CssStyleSheet) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CssStyleSheet {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CSSStyleSheet(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CSSStyleSheet(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CSSStyleSheet(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CssStyleSheet { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CssStyleSheet) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CssStyleSheet> for StyleSheet {
    #[inline]
    fn from(obj: CssStyleSheet) -> StyleSheet {
        use wasm_bindgen::JsCast;
        StyleSheet::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<StyleSheet> for CssStyleSheet {
    #[inline]
    fn as_ref(&self) -> &StyleSheet {
        use wasm_bindgen::JsCast;
        StyleSheet::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CssStyleSheet> for ::js_sys::Object {
    #[inline]
    fn from(obj: CssStyleSheet) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CssStyleSheet {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CssStyleSheet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_rule_CSSStyleSheet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssStyleSheet as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssStyleSheet {
    #[cfg(all(feature = "CssStyleSheet",))]
    #[allow(bad_style)]
    #[doc = "The `deleteRule()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/deleteRule)\n\n*This API requires the following crate features to be activated: `CssStyleSheet`*"]
    #[allow(clippy::all)]
    pub fn delete_rule(&self, index: u32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CssStyleSheet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_rule_CSSStyleSheet(
                self_: <&CssStyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_rule_CSSStyleSheet(
            self_: <&CssStyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CssStyleSheet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_delete_rule_CSSStyleSheet(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CssStyleSheet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_insert_rule_CSSStyleSheet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssStyleSheet as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl CssStyleSheet {
    #[cfg(all(feature = "CssStyleSheet",))]
    #[allow(bad_style)]
    #[doc = "The `insertRule()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/insertRule)\n\n*This API requires the following crate features to be activated: `CssStyleSheet`*"]
    #[allow(clippy::all)]
    pub fn insert_rule(&self, rule: &str) -> Result<u32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CssStyleSheet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_insert_rule_CSSStyleSheet(
                self_: <&CssStyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rule: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_insert_rule_CSSStyleSheet(
            self_: <&CssStyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rule: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(rule);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CssStyleSheet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rule = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rule);
                __widl_f_insert_rule_CSSStyleSheet(self_, rule)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CssStyleSheet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_insert_rule_with_index_CSSStyleSheet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CssStyleSheet as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl CssStyleSheet {
    #[cfg(all(feature = "CssStyleSheet",))]
    #[allow(bad_style)]
    #[doc = "The `insertRule()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/insertRule)\n\n*This API requires the following crate features to be activated: `CssStyleSheet`*"]
    #[allow(clippy::all)]
    pub fn insert_rule_with_index(
        &self,
        rule: &str,
        index: u32,
    ) -> Result<u32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CssStyleSheet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_insert_rule_with_index_CSSStyleSheet(
                self_: <&CssStyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rule: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_insert_rule_with_index_CSSStyleSheet(
            self_: <&CssStyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rule: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(rule);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CssStyleSheet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rule = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rule);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_insert_rule_with_index_CSSStyleSheet(self_, rule, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CssRule", feature = "CssStyleSheet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_owner_rule_CSSStyleSheet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssStyleSheet as WasmDescribe>::describe();
    <Option<CssRule> as WasmDescribe>::describe();
}
impl CssStyleSheet {
    #[cfg(all(feature = "CssRule", feature = "CssStyleSheet",))]
    #[allow(bad_style)]
    #[doc = "The `ownerRule` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/ownerRule)\n\n*This API requires the following crate features to be activated: `CssRule`, `CssStyleSheet`*"]
    #[allow(clippy::all)]
    pub fn owner_rule(&self) -> Option<CssRule> {
        #[cfg(all(feature = "CssRule", feature = "CssStyleSheet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_owner_rule_CSSStyleSheet(
                self_: <&CssStyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<CssRule> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_owner_rule_CSSStyleSheet(
            self_: <&CssStyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<CssRule> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CssStyleSheet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_owner_rule_CSSStyleSheet(self_)
            };
            <Option<CssRule> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssRuleList", feature = "CssStyleSheet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_css_rules_CSSStyleSheet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssStyleSheet as WasmDescribe>::describe();
    <CssRuleList as WasmDescribe>::describe();
}
impl CssStyleSheet {
    #[cfg(all(feature = "CssRuleList", feature = "CssStyleSheet",))]
    #[allow(bad_style)]
    #[doc = "The `cssRules` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/cssRules)\n\n*This API requires the following crate features to be activated: `CssRuleList`, `CssStyleSheet`*"]
    #[allow(clippy::all)]
    pub fn css_rules(&self) -> Result<CssRuleList, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CssRuleList", feature = "CssStyleSheet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_css_rules_CSSStyleSheet(
                self_: <&CssStyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <CssRuleList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_css_rules_CSSStyleSheet(
            self_: <&CssStyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <CssRuleList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CssStyleSheet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_css_rules_CSSStyleSheet(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<CssRuleList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_849a121e2f51c86d: [u8; 591usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\r\x02\0\0\0\0\x06\0\0\x02\rCSSStyleSheet\x1F__widl_instanceof_CSSStyleSheet\0\0\0\0\"__widl_f_delete_rule_CSSStyleSheet\x01\0\0\x01\rCSSStyleSheet\x01\0\0\x01\x02\x05self_\x05index\ndeleteRule\0\0\0\"__widl_f_insert_rule_CSSStyleSheet\x01\0\0\x01\rCSSStyleSheet\x01\0\0\x01\x02\x05self_\x04rule\ninsertRule\0\0\0-__widl_f_insert_rule_with_index_CSSStyleSheet\x01\0\0\x01\rCSSStyleSheet\x01\0\0\x01\x03\x05self_\x04rule\x05index\ninsertRule\0\0\0!__widl_f_owner_rule_CSSStyleSheet\0\0\0\x01\rCSSStyleSheet\x01\0\x01\townerRule\x01\x01\x05self_\townerRule\0\0\0 __widl_f_css_rules_CSSStyleSheet\x01\0\0\x01\rCSSStyleSheet\x01\0\x01\x08cssRules\x01\x01\x05self_\x08cssRules\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
