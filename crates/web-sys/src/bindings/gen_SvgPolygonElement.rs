use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGPolygonElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolygonElement)\n\n*This API requires the following crate features to be activated: `SvgPolygonElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgPolygonElement {
    obj: SvgGeometryElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgPolygonElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgPolygonElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(80u32);
            inform(111u32);
            inform(108u32);
            inform(121u32);
            inform(103u32);
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
    impl core::ops::Deref for SvgPolygonElement {
        type Target = SvgGeometryElement;
        #[inline]
        fn deref(&self) -> &SvgGeometryElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgPolygonElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgPolygonElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgPolygonElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgPolygonElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgPolygonElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgPolygonElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgPolygonElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgPolygonElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgPolygonElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgPolygonElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgPolygonElement {
        #[inline]
        fn from(obj: JsValue) -> SvgPolygonElement {
            SvgPolygonElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgPolygonElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgPolygonElement> for SvgPolygonElement {
        #[inline]
        fn as_ref(&self) -> &SvgPolygonElement {
            self
        }
    }
    impl From<SvgPolygonElement> for JsValue {
        #[inline]
        fn from(obj: SvgPolygonElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgPolygonElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGPolygonElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGPolygonElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGPolygonElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgPolygonElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgPolygonElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgPolygonElement> for SvgGeometryElement {
    #[inline]
    fn from(obj: SvgPolygonElement) -> SvgGeometryElement {
        use wasm_bindgen::JsCast;
        SvgGeometryElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgGeometryElement> for SvgPolygonElement {
    #[inline]
    fn as_ref(&self) -> &SvgGeometryElement {
        use wasm_bindgen::JsCast;
        SvgGeometryElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgPolygonElement> for SvgGraphicsElement {
    #[inline]
    fn from(obj: SvgPolygonElement) -> SvgGraphicsElement {
        use wasm_bindgen::JsCast;
        SvgGraphicsElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgGraphicsElement> for SvgPolygonElement {
    #[inline]
    fn as_ref(&self) -> &SvgGraphicsElement {
        use wasm_bindgen::JsCast;
        SvgGraphicsElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgPolygonElement> for SvgElement {
    #[inline]
    fn from(obj: SvgPolygonElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgPolygonElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgPolygonElement> for Element {
    #[inline]
    fn from(obj: SvgPolygonElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgPolygonElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgPolygonElement> for Node {
    #[inline]
    fn from(obj: SvgPolygonElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgPolygonElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgPolygonElement> for EventTarget {
    #[inline]
    fn from(obj: SvgPolygonElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgPolygonElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgPolygonElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgPolygonElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgPolygonElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgPointList", feature = "SvgPolygonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_points_SVGPolygonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPolygonElement as WasmDescribe>::describe();
    <SvgPointList as WasmDescribe>::describe();
}
impl SvgPolygonElement {
    #[cfg(all(feature = "SvgPointList", feature = "SvgPolygonElement",))]
    #[allow(bad_style)]
    #[doc = "The `points` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolygonElement/points)\n\n*This API requires the following crate features to be activated: `SvgPointList`, `SvgPolygonElement`*"]
    #[allow(clippy::all)]
    pub fn points(&self) -> SvgPointList {
        #[cfg(all(feature = "SvgPointList", feature = "SvgPolygonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_points_SVGPolygonElement(
                self_: <&SvgPolygonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgPointList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_points_SVGPolygonElement(
            self_: <&SvgPolygonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgPolygonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_points_SVGPolygonElement(self_)
            };
            <SvgPointList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPointList", feature = "SvgPolygonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_animated_points_SVGPolygonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPolygonElement as WasmDescribe>::describe();
    <SvgPointList as WasmDescribe>::describe();
}
impl SvgPolygonElement {
    #[cfg(all(feature = "SvgPointList", feature = "SvgPolygonElement",))]
    #[allow(bad_style)]
    #[doc = "The `animatedPoints` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolygonElement/animatedPoints)\n\n*This API requires the following crate features to be activated: `SvgPointList`, `SvgPolygonElement`*"]
    #[allow(clippy::all)]
    pub fn animated_points(&self) -> SvgPointList {
        #[cfg(all(feature = "SvgPointList", feature = "SvgPolygonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_animated_points_SVGPolygonElement(
                self_: <&SvgPolygonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgPointList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_animated_points_SVGPolygonElement(
            self_: <&SvgPolygonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgPolygonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_animated_points_SVGPolygonElement(self_)
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
pub static __WASM_BINDGEN_GENERATED_92dad2cd0feabb46: [u8; 356usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\"\x01\0\0\0\0\x03\0\0\x02\x11SVGPolygonElement#__widl_instanceof_SVGPolygonElement\0\0\0\0!__widl_f_points_SVGPolygonElement\0\0\0\x01\x11SVGPolygonElement\x01\0\x01\x06points\x01\x01\x05self_\x06points\0\0\0*__widl_f_animated_points_SVGPolygonElement\0\0\0\x01\x11SVGPolygonElement\x01\0\x01\x0EanimatedPoints\x01\x01\x05self_\x0EanimatedPoints\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
