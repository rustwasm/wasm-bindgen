use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CSSKeyframesRule` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule)\n\n*This API requires the following crate features to be activated: `CssKeyframesRule`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CssKeyframesRule {
    obj: CssRule,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CssKeyframesRule: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CssKeyframesRule {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
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
            inform(115u32);
            inform(82u32);
            inform(117u32);
            inform(108u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for CssKeyframesRule {
        type Target = CssRule;
        #[inline]
        fn deref(&self) -> &CssRule {
            &self.obj
        }
    }
    impl IntoWasmAbi for CssKeyframesRule {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CssKeyframesRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CssKeyframesRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CssKeyframesRule {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CssKeyframesRule {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CssKeyframesRule {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CssKeyframesRule {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CssKeyframesRule {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CssKeyframesRule>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CssKeyframesRule {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CssKeyframesRule {
        #[inline]
        fn from(obj: JsValue) -> CssKeyframesRule {
            CssKeyframesRule { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CssKeyframesRule {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CssKeyframesRule> for CssKeyframesRule {
        #[inline]
        fn as_ref(&self) -> &CssKeyframesRule {
            self
        }
    }
    impl From<CssKeyframesRule> for JsValue {
        #[inline]
        fn from(obj: CssKeyframesRule) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CssKeyframesRule {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CSSKeyframesRule(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CSSKeyframesRule(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CSSKeyframesRule(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CssKeyframesRule { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CssKeyframesRule) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CssKeyframesRule> for CssRule {
    #[inline]
    fn from(obj: CssKeyframesRule) -> CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<CssRule> for CssKeyframesRule {
    #[inline]
    fn as_ref(&self) -> &CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CssKeyframesRule> for ::js_sys::Object {
    #[inline]
    fn from(obj: CssKeyframesRule) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CssKeyframesRule {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CssKeyframesRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_rule_CSSKeyframesRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssKeyframesRule as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssKeyframesRule {
    #[cfg(all(feature = "CssKeyframesRule",))]
    #[allow(bad_style)]
    #[doc = "The `appendRule()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/appendRule)\n\n*This API requires the following crate features to be activated: `CssKeyframesRule`*"]
    #[allow(clippy::all)]
    pub fn append_rule(&self, rule: &str) {
        #[cfg(all(feature = "CssKeyframesRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_rule_CSSKeyframesRule(
                self_: <&CssKeyframesRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rule: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_rule_CSSKeyframesRule(
            self_: <&CssKeyframesRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rule: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(rule);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssKeyframesRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rule = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rule);
                __widl_f_append_rule_CSSKeyframesRule(self_, rule)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CssKeyframesRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_rule_CSSKeyframesRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssKeyframesRule as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssKeyframesRule {
    #[cfg(all(feature = "CssKeyframesRule",))]
    #[allow(bad_style)]
    #[doc = "The `deleteRule()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/deleteRule)\n\n*This API requires the following crate features to be activated: `CssKeyframesRule`*"]
    #[allow(clippy::all)]
    pub fn delete_rule(&self, select: &str) {
        #[cfg(all(feature = "CssKeyframesRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_rule_CSSKeyframesRule(
                self_: <&CssKeyframesRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                select: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_rule_CSSKeyframesRule(
            self_: <&CssKeyframesRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            select: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(select);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssKeyframesRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let select = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(select);
                __widl_f_delete_rule_CSSKeyframesRule(self_, select)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CssKeyframeRule", feature = "CssKeyframesRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_find_rule_CSSKeyframesRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssKeyframesRule as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<CssKeyframeRule> as WasmDescribe>::describe();
}
impl CssKeyframesRule {
    #[cfg(all(feature = "CssKeyframeRule", feature = "CssKeyframesRule",))]
    #[allow(bad_style)]
    #[doc = "The `findRule()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/findRule)\n\n*This API requires the following crate features to be activated: `CssKeyframeRule`, `CssKeyframesRule`*"]
    #[allow(clippy::all)]
    pub fn find_rule(&self, select: &str) -> Option<CssKeyframeRule> {
        #[cfg(all(feature = "CssKeyframeRule", feature = "CssKeyframesRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_find_rule_CSSKeyframesRule(
                self_: <&CssKeyframesRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                select: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<CssKeyframeRule> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_find_rule_CSSKeyframesRule(
            self_: <&CssKeyframesRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            select: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<CssKeyframeRule> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(select);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssKeyframesRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let select = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(select);
                __widl_f_find_rule_CSSKeyframesRule(self_, select)
            };
            <Option<CssKeyframeRule> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssKeyframesRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_CSSKeyframesRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssKeyframesRule as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssKeyframesRule {
    #[cfg(all(feature = "CssKeyframesRule",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/name)\n\n*This API requires the following crate features to be activated: `CssKeyframesRule`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "CssKeyframesRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_CSSKeyframesRule(
                self_: <&CssKeyframesRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_CSSKeyframesRule(
            self_: <&CssKeyframesRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CssKeyframesRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_CSSKeyframesRule(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssKeyframesRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_name_CSSKeyframesRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssKeyframesRule as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssKeyframesRule {
    #[cfg(all(feature = "CssKeyframesRule",))]
    #[allow(bad_style)]
    #[doc = "The `name` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/name)\n\n*This API requires the following crate features to be activated: `CssKeyframesRule`*"]
    #[allow(clippy::all)]
    pub fn set_name(&self, name: &str) {
        #[cfg(all(feature = "CssKeyframesRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_name_CSSKeyframesRule(
                self_: <&CssKeyframesRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_name_CSSKeyframesRule(
            self_: <&CssKeyframesRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssKeyframesRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_set_name_CSSKeyframesRule(self_, name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CssKeyframesRule", feature = "CssRuleList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_css_rules_CSSKeyframesRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssKeyframesRule as WasmDescribe>::describe();
    <CssRuleList as WasmDescribe>::describe();
}
impl CssKeyframesRule {
    #[cfg(all(feature = "CssKeyframesRule", feature = "CssRuleList",))]
    #[allow(bad_style)]
    #[doc = "The `cssRules` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/cssRules)\n\n*This API requires the following crate features to be activated: `CssKeyframesRule`, `CssRuleList`*"]
    #[allow(clippy::all)]
    pub fn css_rules(&self) -> CssRuleList {
        #[cfg(all(feature = "CssKeyframesRule", feature = "CssRuleList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_css_rules_CSSKeyframesRule(
                self_: <&CssKeyframesRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <CssRuleList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_css_rules_CSSKeyframesRule(
            self_: <&CssKeyframesRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <CssRuleList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssKeyframesRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_css_rules_CSSKeyframesRule(self_)
            };
            <CssRuleList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_b57a0bbd7ef04512: [u8; 678usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}d\x02\0\0\0\0\x07\0\0\x02\x10CSSKeyframesRule\"__widl_instanceof_CSSKeyframesRule\0\0\0\0%__widl_f_append_rule_CSSKeyframesRule\0\0\0\x01\x10CSSKeyframesRule\x01\0\0\x01\x02\x05self_\x04rule\nappendRule\0\0\0%__widl_f_delete_rule_CSSKeyframesRule\0\0\0\x01\x10CSSKeyframesRule\x01\0\0\x01\x02\x05self_\x06select\ndeleteRule\0\0\0#__widl_f_find_rule_CSSKeyframesRule\0\0\0\x01\x10CSSKeyframesRule\x01\0\0\x01\x02\x05self_\x06select\x08findRule\0\0\0\x1E__widl_f_name_CSSKeyframesRule\0\0\0\x01\x10CSSKeyframesRule\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0\"__widl_f_set_name_CSSKeyframesRule\0\0\0\x01\x10CSSKeyframesRule\x01\0\x02\x04name\x01\x02\x05self_\x04name\x04name\0\0\0#__widl_f_css_rules_CSSKeyframesRule\0\0\0\x01\x10CSSKeyframesRule\x01\0\x01\x08cssRules\x01\x01\x05self_\x08cssRules\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
