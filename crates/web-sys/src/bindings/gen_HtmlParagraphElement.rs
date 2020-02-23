use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLParagraphElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParagraphElement)\n\n*This API requires the following crate features to be activated: `HtmlParagraphElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlParagraphElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlParagraphElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlParagraphElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(20u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(80u32);
            inform(97u32);
            inform(114u32);
            inform(97u32);
            inform(103u32);
            inform(114u32);
            inform(97u32);
            inform(112u32);
            inform(104u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlParagraphElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlParagraphElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlParagraphElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlParagraphElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlParagraphElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlParagraphElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlParagraphElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlParagraphElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlParagraphElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlParagraphElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlParagraphElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlParagraphElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlParagraphElement {
            HtmlParagraphElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlParagraphElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlParagraphElement> for HtmlParagraphElement {
        #[inline]
        fn as_ref(&self) -> &HtmlParagraphElement {
            self
        }
    }
    impl From<HtmlParagraphElement> for JsValue {
        #[inline]
        fn from(obj: HtmlParagraphElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlParagraphElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLParagraphElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLParagraphElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLParagraphElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlParagraphElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlParagraphElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlParagraphElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlParagraphElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlParagraphElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlParagraphElement> for Element {
    #[inline]
    fn from(obj: HtmlParagraphElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlParagraphElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlParagraphElement> for Node {
    #[inline]
    fn from(obj: HtmlParagraphElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlParagraphElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlParagraphElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlParagraphElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlParagraphElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlParagraphElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlParagraphElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlParagraphElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlParagraphElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_align_HTMLParagraphElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlParagraphElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlParagraphElement {
    #[cfg(all(feature = "HtmlParagraphElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParagraphElement/align)\n\n*This API requires the following crate features to be activated: `HtmlParagraphElement`*"]
    #[allow(clippy::all)]
    pub fn align(&self) -> String {
        #[cfg(all(feature = "HtmlParagraphElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_align_HTMLParagraphElement(
                self_: <&HtmlParagraphElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_align_HTMLParagraphElement(
            self_: <&HtmlParagraphElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlParagraphElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_align_HTMLParagraphElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlParagraphElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_align_HTMLParagraphElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlParagraphElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlParagraphElement {
    #[cfg(all(feature = "HtmlParagraphElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParagraphElement/align)\n\n*This API requires the following crate features to be activated: `HtmlParagraphElement`*"]
    #[allow(clippy::all)]
    pub fn set_align(&self, align: &str) {
        #[cfg(all(feature = "HtmlParagraphElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_align_HTMLParagraphElement(
                self_: <&HtmlParagraphElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_align_HTMLParagraphElement(
            self_: <&HtmlParagraphElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(align);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlParagraphElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let align = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(align);
                __widl_f_set_align_HTMLParagraphElement(self_, align)
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
pub static __WASM_BINDGEN_GENERATED_dfcbd5e68b50af6f: [u8; 353usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1F\x01\0\0\0\0\x03\0\0\x02\x14HTMLParagraphElement&__widl_instanceof_HTMLParagraphElement\0\0\0\0#__widl_f_align_HTMLParagraphElement\0\0\0\x01\x14HTMLParagraphElement\x01\0\x01\x05align\x01\x01\x05self_\x05align\0\0\0'__widl_f_set_align_HTMLParagraphElement\0\0\0\x01\x14HTMLParagraphElement\x01\0\x02\x05align\x01\x02\x05self_\x05align\x05align\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
