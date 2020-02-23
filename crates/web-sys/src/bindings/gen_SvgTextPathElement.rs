use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGTextPathElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement)\n\n*This API requires the following crate features to be activated: `SvgTextPathElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgTextPathElement {
    obj: SvgTextContentElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgTextPathElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgTextPathElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(18u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(84u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
            inform(80u32);
            inform(97u32);
            inform(116u32);
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
    impl core::ops::Deref for SvgTextPathElement {
        type Target = SvgTextContentElement;
        #[inline]
        fn deref(&self) -> &SvgTextContentElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgTextPathElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgTextPathElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgTextPathElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgTextPathElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgTextPathElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgTextPathElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgTextPathElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgTextPathElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgTextPathElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgTextPathElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgTextPathElement {
        #[inline]
        fn from(obj: JsValue) -> SvgTextPathElement {
            SvgTextPathElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgTextPathElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgTextPathElement> for SvgTextPathElement {
        #[inline]
        fn as_ref(&self) -> &SvgTextPathElement {
            self
        }
    }
    impl From<SvgTextPathElement> for JsValue {
        #[inline]
        fn from(obj: SvgTextPathElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgTextPathElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGTextPathElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGTextPathElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGTextPathElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgTextPathElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgTextPathElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgTextPathElement> for SvgTextContentElement {
    #[inline]
    fn from(obj: SvgTextPathElement) -> SvgTextContentElement {
        use wasm_bindgen::JsCast;
        SvgTextContentElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgTextContentElement> for SvgTextPathElement {
    #[inline]
    fn as_ref(&self) -> &SvgTextContentElement {
        use wasm_bindgen::JsCast;
        SvgTextContentElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextPathElement> for SvgGraphicsElement {
    #[inline]
    fn from(obj: SvgTextPathElement) -> SvgGraphicsElement {
        use wasm_bindgen::JsCast;
        SvgGraphicsElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgGraphicsElement> for SvgTextPathElement {
    #[inline]
    fn as_ref(&self) -> &SvgGraphicsElement {
        use wasm_bindgen::JsCast;
        SvgGraphicsElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextPathElement> for SvgElement {
    #[inline]
    fn from(obj: SvgTextPathElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgTextPathElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextPathElement> for Element {
    #[inline]
    fn from(obj: SvgTextPathElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgTextPathElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextPathElement> for Node {
    #[inline]
    fn from(obj: SvgTextPathElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgTextPathElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextPathElement> for EventTarget {
    #[inline]
    fn from(obj: SvgTextPathElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgTextPathElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextPathElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgTextPathElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgTextPathElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgTextPathElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_offset_SVGTextPathElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgTextPathElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgTextPathElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgTextPathElement",))]
    #[allow(bad_style)]
    #[doc = "The `startOffset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement/startOffset)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgTextPathElement`*"]
    #[allow(clippy::all)]
    pub fn start_offset(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgTextPathElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_offset_SVGTextPathElement(
                self_: <&SvgTextPathElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_offset_SVGTextPathElement(
            self_: <&SvgTextPathElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTextPathElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_start_offset_SVGTextPathElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgTextPathElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_method_SVGTextPathElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgTextPathElement as WasmDescribe>::describe();
    <SvgAnimatedEnumeration as WasmDescribe>::describe();
}
impl SvgTextPathElement {
    #[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgTextPathElement",))]
    #[allow(bad_style)]
    #[doc = "The `method` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement/method)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgTextPathElement`*"]
    #[allow(clippy::all)]
    pub fn method(&self) -> SvgAnimatedEnumeration {
        #[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgTextPathElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_method_SVGTextPathElement(
                self_: <&SvgTextPathElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_method_SVGTextPathElement(
            self_: <&SvgTextPathElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTextPathElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_method_SVGTextPathElement(self_)
            };
            <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgTextPathElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_spacing_SVGTextPathElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgTextPathElement as WasmDescribe>::describe();
    <SvgAnimatedEnumeration as WasmDescribe>::describe();
}
impl SvgTextPathElement {
    #[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgTextPathElement",))]
    #[allow(bad_style)]
    #[doc = "The `spacing` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement/spacing)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgTextPathElement`*"]
    #[allow(clippy::all)]
    pub fn spacing(&self) -> SvgAnimatedEnumeration {
        #[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgTextPathElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_spacing_SVGTextPathElement(
                self_: <&SvgTextPathElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_spacing_SVGTextPathElement(
            self_: <&SvgTextPathElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTextPathElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_spacing_SVGTextPathElement(self_)
            };
            <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedString", feature = "SvgTextPathElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_href_SVGTextPathElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgTextPathElement as WasmDescribe>::describe();
    <SvgAnimatedString as WasmDescribe>::describe();
}
impl SvgTextPathElement {
    #[cfg(all(feature = "SvgAnimatedString", feature = "SvgTextPathElement",))]
    #[allow(bad_style)]
    #[doc = "The `href` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement/href)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgTextPathElement`*"]
    #[allow(clippy::all)]
    pub fn href(&self) -> SvgAnimatedString {
        #[cfg(all(feature = "SvgAnimatedString", feature = "SvgTextPathElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_href_SVGTextPathElement(
                self_: <&SvgTextPathElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_href_SVGTextPathElement(
            self_: <&SvgTextPathElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTextPathElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_href_SVGTextPathElement(self_)
            };
            <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl SvgTextPathElement {
    pub const TEXTPATH_METHODTYPE_UNKNOWN: u16 = 0i64 as u16;
}
impl SvgTextPathElement {
    pub const TEXTPATH_METHODTYPE_ALIGN: u16 = 1u64 as u16;
}
impl SvgTextPathElement {
    pub const TEXTPATH_METHODTYPE_STRETCH: u16 = 2u64 as u16;
}
impl SvgTextPathElement {
    pub const TEXTPATH_SPACINGTYPE_UNKNOWN: u16 = 0i64 as u16;
}
impl SvgTextPathElement {
    pub const TEXTPATH_SPACINGTYPE_AUTO: u16 = 1u64 as u16;
}
impl SvgTextPathElement {
    pub const TEXTPATH_SPACINGTYPE_EXACT: u16 = 2u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_d2ecaccb76cd3a8b: [u8; 522usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xC8\x01\0\0\0\0\x05\0\0\x02\x12SVGTextPathElement$__widl_instanceof_SVGTextPathElement\0\0\0\0(__widl_f_start_offset_SVGTextPathElement\0\0\0\x01\x12SVGTextPathElement\x01\0\x01\x0BstartOffset\x01\x01\x05self_\x0BstartOffset\0\0\0\"__widl_f_method_SVGTextPathElement\0\0\0\x01\x12SVGTextPathElement\x01\0\x01\x06method\x01\x01\x05self_\x06method\0\0\0#__widl_f_spacing_SVGTextPathElement\0\0\0\x01\x12SVGTextPathElement\x01\0\x01\x07spacing\x01\x01\x05self_\x07spacing\0\0\0 __widl_f_href_SVGTextPathElement\0\0\0\x01\x12SVGTextPathElement\x01\0\x01\x04href\x01\x01\x05self_\x04href\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
