use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CSSConditionRule` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSConditionRule)\n\n*This API requires the following crate features to be activated: `CssConditionRule`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CssConditionRule {
    obj: CssGroupingRule,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CssConditionRule: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CssConditionRule {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(67u32);
            inform(83u32);
            inform(83u32);
            inform(67u32);
            inform(111u32);
            inform(110u32);
            inform(100u32);
            inform(105u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(82u32);
            inform(117u32);
            inform(108u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for CssConditionRule {
        type Target = CssGroupingRule;
        #[inline]
        fn deref(&self) -> &CssGroupingRule {
            &self.obj
        }
    }
    impl IntoWasmAbi for CssConditionRule {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CssConditionRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CssConditionRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CssConditionRule {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CssConditionRule {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CssConditionRule {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CssConditionRule {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CssConditionRule {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CssConditionRule>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CssConditionRule {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CssConditionRule {
        #[inline]
        fn from(obj: JsValue) -> CssConditionRule {
            CssConditionRule { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CssConditionRule {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CssConditionRule> for CssConditionRule {
        #[inline]
        fn as_ref(&self) -> &CssConditionRule {
            self
        }
    }
    impl From<CssConditionRule> for JsValue {
        #[inline]
        fn from(obj: CssConditionRule) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CssConditionRule {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CSSConditionRule(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CSSConditionRule(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CSSConditionRule(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CssConditionRule { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CssConditionRule) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CssConditionRule> for CssGroupingRule {
    #[inline]
    fn from(obj: CssConditionRule) -> CssGroupingRule {
        use wasm_bindgen::JsCast;
        CssGroupingRule::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<CssGroupingRule> for CssConditionRule {
    #[inline]
    fn as_ref(&self) -> &CssGroupingRule {
        use wasm_bindgen::JsCast;
        CssGroupingRule::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CssConditionRule> for CssRule {
    #[inline]
    fn from(obj: CssConditionRule) -> CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<CssRule> for CssConditionRule {
    #[inline]
    fn as_ref(&self) -> &CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CssConditionRule> for ::js_sys::Object {
    #[inline]
    fn from(obj: CssConditionRule) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CssConditionRule {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CssConditionRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_condition_text_CSSConditionRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssConditionRule as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssConditionRule {
    #[cfg(all(feature = "CssConditionRule",))]
    #[allow(bad_style)]
    #[doc = "The `conditionText` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSConditionRule/conditionText)\n\n*This API requires the following crate features to be activated: `CssConditionRule`*"]
    #[allow(clippy::all)]
    pub fn condition_text(&self) -> String {
        #[cfg(all(feature = "CssConditionRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_condition_text_CSSConditionRule(
                self_: <&CssConditionRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_condition_text_CSSConditionRule(
            self_: <&CssConditionRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CssConditionRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_condition_text_CSSConditionRule(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssConditionRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_condition_text_CSSConditionRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssConditionRule as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssConditionRule {
    #[cfg(all(feature = "CssConditionRule",))]
    #[allow(bad_style)]
    #[doc = "The `conditionText` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSConditionRule/conditionText)\n\n*This API requires the following crate features to be activated: `CssConditionRule`*"]
    #[allow(clippy::all)]
    pub fn set_condition_text(&self, condition_text: &str) {
        #[cfg(all(feature = "CssConditionRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_condition_text_CSSConditionRule(
                self_: <&CssConditionRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                condition_text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_condition_text_CSSConditionRule(
            self_: <&CssConditionRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            condition_text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(condition_text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssConditionRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let condition_text =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition_text);
                __widl_f_set_condition_text_CSSConditionRule(self_, condition_text)
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
pub static __WASM_BINDGEN_GENERATED_67a0dd97fb61bd24: [u8; 388usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}B\x01\0\0\0\0\x03\0\0\x02\x10CSSConditionRule\"__widl_instanceof_CSSConditionRule\0\0\0\0(__widl_f_condition_text_CSSConditionRule\0\0\0\x01\x10CSSConditionRule\x01\0\x01\rconditionText\x01\x01\x05self_\rconditionText\0\0\0,__widl_f_set_condition_text_CSSConditionRule\0\0\0\x01\x10CSSConditionRule\x01\0\x02\rconditionText\x01\x02\x05self_\x0Econdition_text\rconditionText\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
