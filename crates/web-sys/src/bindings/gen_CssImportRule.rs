use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CSSImportRule` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule)\n\n*This API requires the following crate features to be activated: `CssImportRule`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CssImportRule {
    obj: CssRule,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CssImportRule: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CssImportRule {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(67u32);
            inform(83u32);
            inform(83u32);
            inform(73u32);
            inform(109u32);
            inform(112u32);
            inform(111u32);
            inform(114u32);
            inform(116u32);
            inform(82u32);
            inform(117u32);
            inform(108u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for CssImportRule {
        type Target = CssRule;
        #[inline]
        fn deref(&self) -> &CssRule {
            &self.obj
        }
    }
    impl IntoWasmAbi for CssImportRule {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CssImportRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CssImportRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CssImportRule {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CssImportRule {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CssImportRule {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CssImportRule {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CssImportRule {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CssImportRule>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CssImportRule {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CssImportRule {
        #[inline]
        fn from(obj: JsValue) -> CssImportRule {
            CssImportRule { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CssImportRule {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CssImportRule> for CssImportRule {
        #[inline]
        fn as_ref(&self) -> &CssImportRule {
            self
        }
    }
    impl From<CssImportRule> for JsValue {
        #[inline]
        fn from(obj: CssImportRule) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CssImportRule {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CSSImportRule(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CSSImportRule(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CSSImportRule(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CssImportRule { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CssImportRule) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CssImportRule> for CssRule {
    #[inline]
    fn from(obj: CssImportRule) -> CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<CssRule> for CssImportRule {
    #[inline]
    fn as_ref(&self) -> &CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CssImportRule> for ::js_sys::Object {
    #[inline]
    fn from(obj: CssImportRule) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CssImportRule {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CssImportRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_href_CSSImportRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssImportRule as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssImportRule {
    #[cfg(all(feature = "CssImportRule",))]
    #[allow(bad_style)]
    #[doc = "The `href` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/href)\n\n*This API requires the following crate features to be activated: `CssImportRule`*"]
    #[allow(clippy::all)]
    pub fn href(&self) -> String {
        #[cfg(all(feature = "CssImportRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_href_CSSImportRule(
                self_: <&CssImportRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_href_CSSImportRule(
            self_: <&CssImportRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CssImportRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_href_CSSImportRule(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssImportRule", feature = "MediaList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_media_CSSImportRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssImportRule as WasmDescribe>::describe();
    <Option<MediaList> as WasmDescribe>::describe();
}
impl CssImportRule {
    #[cfg(all(feature = "CssImportRule", feature = "MediaList",))]
    #[allow(bad_style)]
    #[doc = "The `media` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/media)\n\n*This API requires the following crate features to be activated: `CssImportRule`, `MediaList`*"]
    #[allow(clippy::all)]
    pub fn media(&self) -> Option<MediaList> {
        #[cfg(all(feature = "CssImportRule", feature = "MediaList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_media_CSSImportRule(
                self_: <&CssImportRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<MediaList> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_media_CSSImportRule(
            self_: <&CssImportRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<MediaList> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CssImportRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_media_CSSImportRule(self_)
            };
            <Option<MediaList> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssImportRule", feature = "CssStyleSheet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_style_sheet_CSSImportRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssImportRule as WasmDescribe>::describe();
    <Option<CssStyleSheet> as WasmDescribe>::describe();
}
impl CssImportRule {
    #[cfg(all(feature = "CssImportRule", feature = "CssStyleSheet",))]
    #[allow(bad_style)]
    #[doc = "The `styleSheet` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/styleSheet)\n\n*This API requires the following crate features to be activated: `CssImportRule`, `CssStyleSheet`*"]
    #[allow(clippy::all)]
    pub fn style_sheet(&self) -> Option<CssStyleSheet> {
        #[cfg(all(feature = "CssImportRule", feature = "CssStyleSheet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_style_sheet_CSSImportRule(
                self_: <&CssImportRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<CssStyleSheet> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_style_sheet_CSSImportRule(
            self_: <&CssImportRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<CssStyleSheet> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CssImportRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_style_sheet_CSSImportRule(self_)
            };
            <Option<CssStyleSheet> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_7bd26afe067a2890: [u8; 387usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}A\x01\0\0\0\0\x04\0\0\x02\rCSSImportRule\x1F__widl_instanceof_CSSImportRule\0\0\0\0\x1B__widl_f_href_CSSImportRule\0\0\0\x01\rCSSImportRule\x01\0\x01\x04href\x01\x01\x05self_\x04href\0\0\0\x1C__widl_f_media_CSSImportRule\0\0\0\x01\rCSSImportRule\x01\0\x01\x05media\x01\x01\x05self_\x05media\0\0\0\"__widl_f_style_sheet_CSSImportRule\0\0\0\x01\rCSSImportRule\x01\0\x01\nstyleSheet\x01\x01\x05self_\nstyleSheet\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
