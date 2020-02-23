use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGFEFuncRElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFuncRElement)\n\n*This API requires the following crate features to be activated: `SvgfeFuncRElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgfeFuncRElement {
    obj: SvgComponentTransferFunctionElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgfeFuncRElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgfeFuncRElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(70u32);
            inform(69u32);
            inform(70u32);
            inform(117u32);
            inform(110u32);
            inform(99u32);
            inform(82u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgfeFuncRElement {
        type Target = SvgComponentTransferFunctionElement;
        #[inline]
        fn deref(&self) -> &SvgComponentTransferFunctionElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgfeFuncRElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgfeFuncRElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgfeFuncRElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgfeFuncRElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgfeFuncRElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgfeFuncRElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgfeFuncRElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgfeFuncRElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgfeFuncRElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgfeFuncRElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgfeFuncRElement {
        #[inline]
        fn from(obj: JsValue) -> SvgfeFuncRElement {
            SvgfeFuncRElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgfeFuncRElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgfeFuncRElement> for SvgfeFuncRElement {
        #[inline]
        fn as_ref(&self) -> &SvgfeFuncRElement {
            self
        }
    }
    impl From<SvgfeFuncRElement> for JsValue {
        #[inline]
        fn from(obj: SvgfeFuncRElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgfeFuncRElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGFEFuncRElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGFEFuncRElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGFEFuncRElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgfeFuncRElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgfeFuncRElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgfeFuncRElement> for SvgComponentTransferFunctionElement {
    #[inline]
    fn from(obj: SvgfeFuncRElement) -> SvgComponentTransferFunctionElement {
        use wasm_bindgen::JsCast;
        SvgComponentTransferFunctionElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgComponentTransferFunctionElement> for SvgfeFuncRElement {
    #[inline]
    fn as_ref(&self) -> &SvgComponentTransferFunctionElement {
        use wasm_bindgen::JsCast;
        SvgComponentTransferFunctionElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeFuncRElement> for SvgElement {
    #[inline]
    fn from(obj: SvgfeFuncRElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgfeFuncRElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeFuncRElement> for Element {
    #[inline]
    fn from(obj: SvgfeFuncRElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgfeFuncRElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeFuncRElement> for Node {
    #[inline]
    fn from(obj: SvgfeFuncRElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgfeFuncRElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeFuncRElement> for EventTarget {
    #[inline]
    fn from(obj: SvgfeFuncRElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgfeFuncRElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeFuncRElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgfeFuncRElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgfeFuncRElement {
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
pub static __WASM_BINDGEN_GENERATED_0f0c492b3b66332c: [u8; 163usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}a\0\0\0\0\0\x01\0\0\x02\x11SVGFEFuncRElement#__widl_instanceof_SVGFEFuncRElement\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
