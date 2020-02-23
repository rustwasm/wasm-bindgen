use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CSSSupportsRule` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSSupportsRule)\n\n*This API requires the following crate features to be activated: `CssSupportsRule`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CssSupportsRule {
    obj: CssConditionRule,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CssSupportsRule: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CssSupportsRule {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(67u32);
            inform(83u32);
            inform(83u32);
            inform(83u32);
            inform(117u32);
            inform(112u32);
            inform(112u32);
            inform(111u32);
            inform(114u32);
            inform(116u32);
            inform(115u32);
            inform(82u32);
            inform(117u32);
            inform(108u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for CssSupportsRule {
        type Target = CssConditionRule;
        #[inline]
        fn deref(&self) -> &CssConditionRule {
            &self.obj
        }
    }
    impl IntoWasmAbi for CssSupportsRule {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CssSupportsRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CssSupportsRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CssSupportsRule {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CssSupportsRule {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CssSupportsRule {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CssSupportsRule {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CssSupportsRule {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CssSupportsRule>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CssSupportsRule {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CssSupportsRule {
        #[inline]
        fn from(obj: JsValue) -> CssSupportsRule {
            CssSupportsRule { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CssSupportsRule {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CssSupportsRule> for CssSupportsRule {
        #[inline]
        fn as_ref(&self) -> &CssSupportsRule {
            self
        }
    }
    impl From<CssSupportsRule> for JsValue {
        #[inline]
        fn from(obj: CssSupportsRule) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CssSupportsRule {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CSSSupportsRule(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CSSSupportsRule(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CSSSupportsRule(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CssSupportsRule { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CssSupportsRule) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CssSupportsRule> for CssConditionRule {
    #[inline]
    fn from(obj: CssSupportsRule) -> CssConditionRule {
        use wasm_bindgen::JsCast;
        CssConditionRule::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<CssConditionRule> for CssSupportsRule {
    #[inline]
    fn as_ref(&self) -> &CssConditionRule {
        use wasm_bindgen::JsCast;
        CssConditionRule::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CssSupportsRule> for CssGroupingRule {
    #[inline]
    fn from(obj: CssSupportsRule) -> CssGroupingRule {
        use wasm_bindgen::JsCast;
        CssGroupingRule::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<CssGroupingRule> for CssSupportsRule {
    #[inline]
    fn as_ref(&self) -> &CssGroupingRule {
        use wasm_bindgen::JsCast;
        CssGroupingRule::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CssSupportsRule> for CssRule {
    #[inline]
    fn from(obj: CssSupportsRule) -> CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<CssRule> for CssSupportsRule {
    #[inline]
    fn as_ref(&self) -> &CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CssSupportsRule> for ::js_sys::Object {
    #[inline]
    fn from(obj: CssSupportsRule) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CssSupportsRule {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_923ee54f0ac454c5: [u8; 159usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}]\0\0\0\0\0\x01\0\0\x02\x0FCSSSupportsRule!__widl_instanceof_CSSSupportsRule\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
