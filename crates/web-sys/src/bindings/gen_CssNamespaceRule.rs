use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CSSNamespaceRule` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSNamespaceRule)\n\n*This API requires the following crate features to be activated: `CssNamespaceRule`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CssNamespaceRule {
    obj: CssRule,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CssNamespaceRule: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CssNamespaceRule {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(67u32);
            inform(83u32);
            inform(83u32);
            inform(78u32);
            inform(97u32);
            inform(109u32);
            inform(101u32);
            inform(115u32);
            inform(112u32);
            inform(97u32);
            inform(99u32);
            inform(101u32);
            inform(82u32);
            inform(117u32);
            inform(108u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for CssNamespaceRule {
        type Target = CssRule;
        #[inline]
        fn deref(&self) -> &CssRule {
            &self.obj
        }
    }
    impl IntoWasmAbi for CssNamespaceRule {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CssNamespaceRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CssNamespaceRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CssNamespaceRule {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CssNamespaceRule {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CssNamespaceRule {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CssNamespaceRule {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CssNamespaceRule {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CssNamespaceRule>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CssNamespaceRule {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CssNamespaceRule {
        #[inline]
        fn from(obj: JsValue) -> CssNamespaceRule {
            CssNamespaceRule { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CssNamespaceRule {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CssNamespaceRule> for CssNamespaceRule {
        #[inline]
        fn as_ref(&self) -> &CssNamespaceRule {
            self
        }
    }
    impl From<CssNamespaceRule> for JsValue {
        #[inline]
        fn from(obj: CssNamespaceRule) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CssNamespaceRule {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CSSNamespaceRule(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CSSNamespaceRule(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CSSNamespaceRule(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CssNamespaceRule { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CssNamespaceRule) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CssNamespaceRule> for CssRule {
    #[inline]
    fn from(obj: CssNamespaceRule) -> CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<CssRule> for CssNamespaceRule {
    #[inline]
    fn as_ref(&self) -> &CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CssNamespaceRule> for ::js_sys::Object {
    #[inline]
    fn from(obj: CssNamespaceRule) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CssNamespaceRule {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CssNamespaceRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_namespace_uri_CSSNamespaceRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssNamespaceRule as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssNamespaceRule {
    #[cfg(all(feature = "CssNamespaceRule",))]
    #[allow(bad_style)]
    #[doc = "The `namespaceURI` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSNamespaceRule/namespaceURI)\n\n*This API requires the following crate features to be activated: `CssNamespaceRule`*"]
    #[allow(clippy::all)]
    pub fn namespace_uri(&self) -> String {
        #[cfg(all(feature = "CssNamespaceRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_namespace_uri_CSSNamespaceRule(
                self_: <&CssNamespaceRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_namespace_uri_CSSNamespaceRule(
            self_: <&CssNamespaceRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CssNamespaceRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_namespace_uri_CSSNamespaceRule(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssNamespaceRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prefix_CSSNamespaceRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssNamespaceRule as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssNamespaceRule {
    #[cfg(all(feature = "CssNamespaceRule",))]
    #[allow(bad_style)]
    #[doc = "The `prefix` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSNamespaceRule/prefix)\n\n*This API requires the following crate features to be activated: `CssNamespaceRule`*"]
    #[allow(clippy::all)]
    pub fn prefix(&self) -> String {
        #[cfg(all(feature = "CssNamespaceRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prefix_CSSNamespaceRule(
                self_: <&CssNamespaceRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prefix_CSSNamespaceRule(
            self_: <&CssNamespaceRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CssNamespaceRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_prefix_CSSNamespaceRule(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_4a9d1b71177bbde0: [u8; 344usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x16\x01\0\0\0\0\x03\0\0\x02\x10CSSNamespaceRule\"__widl_instanceof_CSSNamespaceRule\0\0\0\0'__widl_f_namespace_uri_CSSNamespaceRule\0\0\0\x01\x10CSSNamespaceRule\x01\0\x01\x0CnamespaceURI\x01\x01\x05self_\x0CnamespaceURI\0\0\0 __widl_f_prefix_CSSNamespaceRule\0\0\0\x01\x10CSSNamespaceRule\x01\0\x01\x06prefix\x01\x01\x05self_\x06prefix\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
