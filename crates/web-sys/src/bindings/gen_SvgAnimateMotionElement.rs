use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGAnimateMotionElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimateMotionElement)\n\n*This API requires the following crate features to be activated: `SvgAnimateMotionElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgAnimateMotionElement {
    obj: SvgAnimationElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgAnimateMotionElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgAnimateMotionElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(23u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(65u32);
            inform(110u32);
            inform(105u32);
            inform(109u32);
            inform(97u32);
            inform(116u32);
            inform(101u32);
            inform(77u32);
            inform(111u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgAnimateMotionElement {
        type Target = SvgAnimationElement;
        #[inline]
        fn deref(&self) -> &SvgAnimationElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgAnimateMotionElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgAnimateMotionElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgAnimateMotionElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgAnimateMotionElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgAnimateMotionElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgAnimateMotionElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgAnimateMotionElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgAnimateMotionElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgAnimateMotionElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgAnimateMotionElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgAnimateMotionElement {
        #[inline]
        fn from(obj: JsValue) -> SvgAnimateMotionElement {
            SvgAnimateMotionElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgAnimateMotionElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgAnimateMotionElement> for SvgAnimateMotionElement {
        #[inline]
        fn as_ref(&self) -> &SvgAnimateMotionElement {
            self
        }
    }
    impl From<SvgAnimateMotionElement> for JsValue {
        #[inline]
        fn from(obj: SvgAnimateMotionElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgAnimateMotionElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGAnimateMotionElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGAnimateMotionElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGAnimateMotionElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgAnimateMotionElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgAnimateMotionElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgAnimateMotionElement> for SvgAnimationElement {
    #[inline]
    fn from(obj: SvgAnimateMotionElement) -> SvgAnimationElement {
        use wasm_bindgen::JsCast;
        SvgAnimationElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgAnimationElement> for SvgAnimateMotionElement {
    #[inline]
    fn as_ref(&self) -> &SvgAnimationElement {
        use wasm_bindgen::JsCast;
        SvgAnimationElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgAnimateMotionElement> for SvgElement {
    #[inline]
    fn from(obj: SvgAnimateMotionElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgAnimateMotionElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgAnimateMotionElement> for Element {
    #[inline]
    fn from(obj: SvgAnimateMotionElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgAnimateMotionElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgAnimateMotionElement> for Node {
    #[inline]
    fn from(obj: SvgAnimateMotionElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgAnimateMotionElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgAnimateMotionElement> for EventTarget {
    #[inline]
    fn from(obj: SvgAnimateMotionElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgAnimateMotionElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgAnimateMotionElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgAnimateMotionElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgAnimateMotionElement {
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
pub static __WASM_BINDGEN_GENERATED_1753804a6ae51ffd: [u8; 175usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}m\0\0\0\0\0\x01\0\0\x02\x17SVGAnimateMotionElement)__widl_instanceof_SVGAnimateMotionElement\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
