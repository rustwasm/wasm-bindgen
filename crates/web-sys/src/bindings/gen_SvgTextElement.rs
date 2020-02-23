use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGTextElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextElement)\n\n*This API requires the following crate features to be activated: `SvgTextElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgTextElement {
    obj: SvgTextPositioningElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgTextElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgTextElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(84u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgTextElement {
        type Target = SvgTextPositioningElement;
        #[inline]
        fn deref(&self) -> &SvgTextPositioningElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgTextElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgTextElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgTextElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgTextElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgTextElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgTextElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgTextElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgTextElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgTextElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgTextElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgTextElement {
        #[inline]
        fn from(obj: JsValue) -> SvgTextElement {
            SvgTextElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgTextElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgTextElement> for SvgTextElement {
        #[inline]
        fn as_ref(&self) -> &SvgTextElement {
            self
        }
    }
    impl From<SvgTextElement> for JsValue {
        #[inline]
        fn from(obj: SvgTextElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgTextElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGTextElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGTextElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGTextElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgTextElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgTextElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgTextElement> for SvgTextPositioningElement {
    #[inline]
    fn from(obj: SvgTextElement) -> SvgTextPositioningElement {
        use wasm_bindgen::JsCast;
        SvgTextPositioningElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgTextPositioningElement> for SvgTextElement {
    #[inline]
    fn as_ref(&self) -> &SvgTextPositioningElement {
        use wasm_bindgen::JsCast;
        SvgTextPositioningElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextElement> for SvgTextContentElement {
    #[inline]
    fn from(obj: SvgTextElement) -> SvgTextContentElement {
        use wasm_bindgen::JsCast;
        SvgTextContentElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgTextContentElement> for SvgTextElement {
    #[inline]
    fn as_ref(&self) -> &SvgTextContentElement {
        use wasm_bindgen::JsCast;
        SvgTextContentElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextElement> for SvgGraphicsElement {
    #[inline]
    fn from(obj: SvgTextElement) -> SvgGraphicsElement {
        use wasm_bindgen::JsCast;
        SvgGraphicsElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgGraphicsElement> for SvgTextElement {
    #[inline]
    fn as_ref(&self) -> &SvgGraphicsElement {
        use wasm_bindgen::JsCast;
        SvgGraphicsElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextElement> for SvgElement {
    #[inline]
    fn from(obj: SvgTextElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgTextElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextElement> for Element {
    #[inline]
    fn from(obj: SvgTextElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgTextElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextElement> for Node {
    #[inline]
    fn from(obj: SvgTextElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgTextElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextElement> for EventTarget {
    #[inline]
    fn from(obj: SvgTextElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgTextElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgTextElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgTextElement {
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
pub static __WASM_BINDGEN_GENERATED_e07f015a202ccfc4: [u8; 157usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}[\0\0\0\0\0\x01\0\0\x02\x0ESVGTextElement __widl_instanceof_SVGTextElement\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
