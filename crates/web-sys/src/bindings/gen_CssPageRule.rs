use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CSSPageRule` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageRule)\n\n*This API requires the following crate features to be activated: `CssPageRule`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CssPageRule {
    obj: CssRule,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CssPageRule: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CssPageRule {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(67u32);
            inform(83u32);
            inform(83u32);
            inform(80u32);
            inform(97u32);
            inform(103u32);
            inform(101u32);
            inform(82u32);
            inform(117u32);
            inform(108u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for CssPageRule {
        type Target = CssRule;
        #[inline]
        fn deref(&self) -> &CssRule {
            &self.obj
        }
    }
    impl IntoWasmAbi for CssPageRule {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CssPageRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CssPageRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CssPageRule {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CssPageRule {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CssPageRule {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CssPageRule {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CssPageRule {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CssPageRule>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CssPageRule {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CssPageRule {
        #[inline]
        fn from(obj: JsValue) -> CssPageRule {
            CssPageRule { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CssPageRule {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CssPageRule> for CssPageRule {
        #[inline]
        fn as_ref(&self) -> &CssPageRule {
            self
        }
    }
    impl From<CssPageRule> for JsValue {
        #[inline]
        fn from(obj: CssPageRule) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CssPageRule {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CSSPageRule(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CSSPageRule(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CSSPageRule(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CssPageRule { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CssPageRule) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CssPageRule> for CssRule {
    #[inline]
    fn from(obj: CssPageRule) -> CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<CssRule> for CssPageRule {
    #[inline]
    fn as_ref(&self) -> &CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CssPageRule> for ::js_sys::Object {
    #[inline]
    fn from(obj: CssPageRule) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CssPageRule {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CssPageRule", feature = "CssStyleDeclaration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_style_CSSPageRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssPageRule as WasmDescribe>::describe();
    <CssStyleDeclaration as WasmDescribe>::describe();
}
impl CssPageRule {
    #[cfg(all(feature = "CssPageRule", feature = "CssStyleDeclaration",))]
    #[allow(bad_style)]
    #[doc = "The `style` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageRule/style)\n\n*This API requires the following crate features to be activated: `CssPageRule`, `CssStyleDeclaration`*"]
    #[allow(clippy::all)]
    pub fn style(&self) -> CssStyleDeclaration {
        #[cfg(all(feature = "CssPageRule", feature = "CssStyleDeclaration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_style_CSSPageRule(
                self_: <&CssPageRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <CssStyleDeclaration as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_style_CSSPageRule(
            self_: <&CssPageRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <CssStyleDeclaration as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CssPageRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_style_CSSPageRule(self_)
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
pub static __WASM_BINDGEN_GENERATED_51f222822cf4d3fa: [u8; 220usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x9A\0\0\0\0\0\x02\0\0\x02\x0BCSSPageRule\x1D__widl_instanceof_CSSPageRule\0\0\0\0\x1A__widl_f_style_CSSPageRule\0\0\0\x01\x0BCSSPageRule\x01\0\x01\x05style\x01\x01\x05self_\x05style\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
