use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLHtmlElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHtmlElement)\n\n*This API requires the following crate features to be activated: `HtmlHtmlElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlHtmlElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlHtmlElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlHtmlElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(72u32);
            inform(116u32);
            inform(109u32);
            inform(108u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlHtmlElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlHtmlElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlHtmlElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlHtmlElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlHtmlElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlHtmlElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlHtmlElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlHtmlElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlHtmlElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlHtmlElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlHtmlElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlHtmlElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlHtmlElement {
            HtmlHtmlElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlHtmlElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlHtmlElement> for HtmlHtmlElement {
        #[inline]
        fn as_ref(&self) -> &HtmlHtmlElement {
            self
        }
    }
    impl From<HtmlHtmlElement> for JsValue {
        #[inline]
        fn from(obj: HtmlHtmlElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlHtmlElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLHtmlElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLHtmlElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLHtmlElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlHtmlElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlHtmlElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlHtmlElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlHtmlElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlHtmlElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlHtmlElement> for Element {
    #[inline]
    fn from(obj: HtmlHtmlElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlHtmlElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlHtmlElement> for Node {
    #[inline]
    fn from(obj: HtmlHtmlElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlHtmlElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlHtmlElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlHtmlElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlHtmlElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlHtmlElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlHtmlElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlHtmlElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlHtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_version_HTMLHtmlElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlHtmlElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlHtmlElement {
    #[cfg(all(feature = "HtmlHtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `version` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHtmlElement/version)\n\n*This API requires the following crate features to be activated: `HtmlHtmlElement`*"]
    #[allow(clippy::all)]
    pub fn version(&self) -> String {
        #[cfg(all(feature = "HtmlHtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_version_HTMLHtmlElement(
                self_: <&HtmlHtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_version_HTMLHtmlElement(
            self_: <&HtmlHtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlHtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_version_HTMLHtmlElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlHtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_version_HTMLHtmlElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlHtmlElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlHtmlElement {
    #[cfg(all(feature = "HtmlHtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `version` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHtmlElement/version)\n\n*This API requires the following crate features to be activated: `HtmlHtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_version(&self, version: &str) {
        #[cfg(all(feature = "HtmlHtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_version_HTMLHtmlElement(
                self_: <&HtmlHtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                version: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_version_HTMLHtmlElement(
            self_: <&HtmlHtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            version: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(version);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlHtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let version = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(version);
                __widl_f_set_version_HTMLHtmlElement(self_, version)
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
pub static __WASM_BINDGEN_GENERATED_5f0b5f4e42633894: [u8; 337usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x0F\x01\0\0\0\0\x03\0\0\x02\x0FHTMLHtmlElement!__widl_instanceof_HTMLHtmlElement\0\0\0\0 __widl_f_version_HTMLHtmlElement\0\0\0\x01\x0FHTMLHtmlElement\x01\0\x01\x07version\x01\x01\x05self_\x07version\0\0\0$__widl_f_set_version_HTMLHtmlElement\0\0\0\x01\x0FHTMLHtmlElement\x01\0\x02\x07version\x01\x02\x05self_\x07version\x07version\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
