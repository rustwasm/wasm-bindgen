use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGPolylineElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolylineElement)\n\n*This API requires the following crate features to be activated: `SvgPolylineElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgPolylineElement {
    obj: SvgGeometryElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgPolylineElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgPolylineElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(18u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(80u32);
            inform(111u32);
            inform(108u32);
            inform(121u32);
            inform(108u32);
            inform(105u32);
            inform(110u32);
            inform(101u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgPolylineElement {
        type Target = SvgGeometryElement;
        #[inline]
        fn deref(&self) -> &SvgGeometryElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgPolylineElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgPolylineElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgPolylineElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgPolylineElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgPolylineElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgPolylineElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgPolylineElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgPolylineElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgPolylineElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgPolylineElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgPolylineElement {
        #[inline]
        fn from(obj: JsValue) -> SvgPolylineElement {
            SvgPolylineElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgPolylineElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgPolylineElement> for SvgPolylineElement {
        #[inline]
        fn as_ref(&self) -> &SvgPolylineElement {
            self
        }
    }
    impl From<SvgPolylineElement> for JsValue {
        #[inline]
        fn from(obj: SvgPolylineElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgPolylineElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGPolylineElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGPolylineElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGPolylineElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgPolylineElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgPolylineElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgPolylineElement> for SvgGeometryElement {
    #[inline]
    fn from(obj: SvgPolylineElement) -> SvgGeometryElement {
        use wasm_bindgen::JsCast;
        SvgGeometryElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgGeometryElement> for SvgPolylineElement {
    #[inline]
    fn as_ref(&self) -> &SvgGeometryElement {
        use wasm_bindgen::JsCast;
        SvgGeometryElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgPolylineElement> for SvgGraphicsElement {
    #[inline]
    fn from(obj: SvgPolylineElement) -> SvgGraphicsElement {
        use wasm_bindgen::JsCast;
        SvgGraphicsElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgGraphicsElement> for SvgPolylineElement {
    #[inline]
    fn as_ref(&self) -> &SvgGraphicsElement {
        use wasm_bindgen::JsCast;
        SvgGraphicsElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgPolylineElement> for SvgElement {
    #[inline]
    fn from(obj: SvgPolylineElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgPolylineElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgPolylineElement> for Element {
    #[inline]
    fn from(obj: SvgPolylineElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgPolylineElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgPolylineElement> for Node {
    #[inline]
    fn from(obj: SvgPolylineElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgPolylineElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgPolylineElement> for EventTarget {
    #[inline]
    fn from(obj: SvgPolylineElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgPolylineElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgPolylineElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgPolylineElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgPolylineElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgPointList", feature = "SvgPolylineElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_points_SVGPolylineElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPolylineElement as WasmDescribe>::describe();
    <SvgPointList as WasmDescribe>::describe();
}
impl SvgPolylineElement {
    #[cfg(all(feature = "SvgPointList", feature = "SvgPolylineElement",))]
    #[allow(bad_style)]
    #[doc = "The `points` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolylineElement/points)\n\n*This API requires the following crate features to be activated: `SvgPointList`, `SvgPolylineElement`*"]
    #[allow(clippy::all)]
    pub fn points(&self) -> SvgPointList {
        #[cfg(all(feature = "SvgPointList", feature = "SvgPolylineElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_points_SVGPolylineElement(
                self_: <&SvgPolylineElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgPointList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_points_SVGPolylineElement(
            self_: <&SvgPolylineElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgPointList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgPolylineElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_points_SVGPolylineElement(self_)
            };
            <SvgPointList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPointList", feature = "SvgPolylineElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_animated_points_SVGPolylineElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPolylineElement as WasmDescribe>::describe();
    <SvgPointList as WasmDescribe>::describe();
}
impl SvgPolylineElement {
    #[cfg(all(feature = "SvgPointList", feature = "SvgPolylineElement",))]
    #[allow(bad_style)]
    #[doc = "The `animatedPoints` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolylineElement/animatedPoints)\n\n*This API requires the following crate features to be activated: `SvgPointList`, `SvgPolylineElement`*"]
    #[allow(clippy::all)]
    pub fn animated_points(&self) -> SvgPointList {
        #[cfg(all(feature = "SvgPointList", feature = "SvgPolylineElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_animated_points_SVGPolylineElement(
                self_: <&SvgPolylineElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgPointList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_animated_points_SVGPolylineElement(
            self_: <&SvgPolylineElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgPointList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgPolylineElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_animated_points_SVGPolylineElement(self_)
            };
            <SvgPointList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8cf8ce929adcf394: [u8; 362usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}(\x01\0\0\0\0\x03\0\0\x02\x12SVGPolylineElement$__widl_instanceof_SVGPolylineElement\0\0\0\0\"__widl_f_points_SVGPolylineElement\0\0\0\x01\x12SVGPolylineElement\x01\0\x01\x06points\x01\x01\x05self_\x06points\0\0\0+__widl_f_animated_points_SVGPolylineElement\0\0\0\x01\x12SVGPolylineElement\x01\0\x01\x0EanimatedPoints\x01\x01\x05self_\x0EanimatedPoints\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
