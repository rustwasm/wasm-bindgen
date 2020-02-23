use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CSSPseudoElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement)\n\n*This API requires the following crate features to be activated: `CssPseudoElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CssPseudoElement {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CssPseudoElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CssPseudoElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(67u32);
            inform(83u32);
            inform(83u32);
            inform(80u32);
            inform(115u32);
            inform(101u32);
            inform(117u32);
            inform(100u32);
            inform(111u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for CssPseudoElement {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for CssPseudoElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CssPseudoElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CssPseudoElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CssPseudoElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CssPseudoElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CssPseudoElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CssPseudoElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CssPseudoElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CssPseudoElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CssPseudoElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CssPseudoElement {
        #[inline]
        fn from(obj: JsValue) -> CssPseudoElement {
            CssPseudoElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CssPseudoElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CssPseudoElement> for CssPseudoElement {
        #[inline]
        fn as_ref(&self) -> &CssPseudoElement {
            self
        }
    }
    impl From<CssPseudoElement> for JsValue {
        #[inline]
        fn from(obj: CssPseudoElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CssPseudoElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CSSPseudoElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CSSPseudoElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CSSPseudoElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CssPseudoElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CssPseudoElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CssPseudoElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: CssPseudoElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CssPseudoElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CssPseudoElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_CSSPseudoElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssPseudoElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssPseudoElement {
    #[cfg(all(feature = "CssPseudoElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/type)\n\n*This API requires the following crate features to be activated: `CssPseudoElement`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "CssPseudoElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_CSSPseudoElement(
                self_: <&CssPseudoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_CSSPseudoElement(
            self_: <&CssPseudoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CssPseudoElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_CSSPseudoElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssPseudoElement", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_parent_element_CSSPseudoElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssPseudoElement as WasmDescribe>::describe();
    <Element as WasmDescribe>::describe();
}
impl CssPseudoElement {
    #[cfg(all(feature = "CssPseudoElement", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `parentElement` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/parentElement)\n\n*This API requires the following crate features to be activated: `CssPseudoElement`, `Element`*"]
    #[allow(clippy::all)]
    pub fn parent_element(&self) -> Element {
        #[cfg(all(feature = "CssPseudoElement", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_parent_element_CSSPseudoElement(
                self_: <&CssPseudoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Element as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_parent_element_CSSPseudoElement(
            self_: <&CssPseudoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Element as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssPseudoElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_parent_element_CSSPseudoElement(self_)
            };
            <Element as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_5c165975b1a64d7e: [u8; 341usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x13\x01\0\0\0\0\x03\0\0\x02\x10CSSPseudoElement\"__widl_instanceof_CSSPseudoElement\0\0\0\0\x1E__widl_f_type_CSSPseudoElement\0\0\0\x01\x10CSSPseudoElement\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0(__widl_f_parent_element_CSSPseudoElement\0\0\0\x01\x10CSSPseudoElement\x01\0\x01\rparentElement\x01\x01\x05self_\rparentElement\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
