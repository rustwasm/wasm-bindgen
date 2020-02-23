use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGViewElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement)\n\n*This API requires the following crate features to be activated: `SvgViewElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgViewElement {
    obj: SvgElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgViewElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgViewElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(86u32);
            inform(105u32);
            inform(101u32);
            inform(119u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgViewElement {
        type Target = SvgElement;
        #[inline]
        fn deref(&self) -> &SvgElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgViewElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgViewElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgViewElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgViewElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgViewElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgViewElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgViewElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgViewElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgViewElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgViewElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgViewElement {
        #[inline]
        fn from(obj: JsValue) -> SvgViewElement {
            SvgViewElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgViewElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgViewElement> for SvgViewElement {
        #[inline]
        fn as_ref(&self) -> &SvgViewElement {
            self
        }
    }
    impl From<SvgViewElement> for JsValue {
        #[inline]
        fn from(obj: SvgViewElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgViewElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGViewElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGViewElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGViewElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgViewElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgViewElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgViewElement> for SvgElement {
    #[inline]
    fn from(obj: SvgViewElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgViewElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgViewElement> for Element {
    #[inline]
    fn from(obj: SvgViewElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgViewElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgViewElement> for Node {
    #[inline]
    fn from(obj: SvgViewElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgViewElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgViewElement> for EventTarget {
    #[inline]
    fn from(obj: SvgViewElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgViewElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgViewElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgViewElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgViewElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAnimatedRect", feature = "SvgViewElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_view_box_SVGViewElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgViewElement as WasmDescribe>::describe();
    <SvgAnimatedRect as WasmDescribe>::describe();
}
impl SvgViewElement {
    #[cfg(all(feature = "SvgAnimatedRect", feature = "SvgViewElement",))]
    #[allow(bad_style)]
    #[doc = "The `viewBox` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/viewBox)\n\n*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgViewElement`*"]
    #[allow(clippy::all)]
    pub fn view_box(&self) -> SvgAnimatedRect {
        #[cfg(all(feature = "SvgAnimatedRect", feature = "SvgViewElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_view_box_SVGViewElement(
                self_: <&SvgViewElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedRect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_view_box_SVGViewElement(
            self_: <&SvgViewElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedRect as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgViewElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_view_box_SVGViewElement(self_)
            };
            <SvgAnimatedRect as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedPreserveAspectRatio", feature = "SvgViewElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_preserve_aspect_ratio_SVGViewElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgViewElement as WasmDescribe>::describe();
    <SvgAnimatedPreserveAspectRatio as WasmDescribe>::describe();
}
impl SvgViewElement {
    #[cfg(all(feature = "SvgAnimatedPreserveAspectRatio", feature = "SvgViewElement",))]
    #[allow(bad_style)]
    #[doc = "The `preserveAspectRatio` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/preserveAspectRatio)\n\n*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgViewElement`*"]
    #[allow(clippy::all)]
    pub fn preserve_aspect_ratio(&self) -> SvgAnimatedPreserveAspectRatio {
        #[cfg(all(feature = "SvgAnimatedPreserveAspectRatio", feature = "SvgViewElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_preserve_aspect_ratio_SVGViewElement(
                self_: <&SvgViewElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedPreserveAspectRatio as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_preserve_aspect_ratio_SVGViewElement(
            self_: <&SvgViewElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedPreserveAspectRatio as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgViewElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_preserve_aspect_ratio_SVGViewElement(self_)
            };
            <SvgAnimatedPreserveAspectRatio as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgViewElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_zoom_and_pan_SVGViewElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgViewElement as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl SvgViewElement {
    #[cfg(all(feature = "SvgViewElement",))]
    #[allow(bad_style)]
    #[doc = "The `zoomAndPan` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/zoomAndPan)\n\n*This API requires the following crate features to be activated: `SvgViewElement`*"]
    #[allow(clippy::all)]
    pub fn zoom_and_pan(&self) -> u16 {
        #[cfg(all(feature = "SvgViewElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_zoom_and_pan_SVGViewElement(
                self_: <&SvgViewElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_zoom_and_pan_SVGViewElement(
            self_: <&SvgViewElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgViewElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_zoom_and_pan_SVGViewElement(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgViewElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_zoom_and_pan_SVGViewElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgViewElement as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgViewElement {
    #[cfg(all(feature = "SvgViewElement",))]
    #[allow(bad_style)]
    #[doc = "The `zoomAndPan` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/zoomAndPan)\n\n*This API requires the following crate features to be activated: `SvgViewElement`*"]
    #[allow(clippy::all)]
    pub fn set_zoom_and_pan(&self, zoom_and_pan: u16) {
        #[cfg(all(feature = "SvgViewElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_zoom_and_pan_SVGViewElement(
                self_: <&SvgViewElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                zoom_and_pan: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_zoom_and_pan_SVGViewElement(
            self_: <&SvgViewElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            zoom_and_pan: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(zoom_and_pan);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgViewElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let zoom_and_pan =
                    <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(zoom_and_pan);
                __widl_f_set_zoom_and_pan_SVGViewElement(self_, zoom_and_pan)
            };
            ()
        }
    }
}
impl SvgViewElement {
    pub const SVG_ZOOMANDPAN_UNKNOWN: u16 = 0i64 as u16;
}
impl SvgViewElement {
    pub const SVG_ZOOMANDPAN_DISABLE: u16 = 1u64 as u16;
}
impl SvgViewElement {
    pub const SVG_ZOOMANDPAN_MAGNIFY: u16 = 2u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_b7b9913beb827cde: [u8; 559usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xED\x01\0\0\0\0\x05\0\0\x02\x0ESVGViewElement __widl_instanceof_SVGViewElement\0\0\0\0 __widl_f_view_box_SVGViewElement\0\0\0\x01\x0ESVGViewElement\x01\0\x01\x07viewBox\x01\x01\x05self_\x07viewBox\0\0\0-__widl_f_preserve_aspect_ratio_SVGViewElement\0\0\0\x01\x0ESVGViewElement\x01\0\x01\x13preserveAspectRatio\x01\x01\x05self_\x13preserveAspectRatio\0\0\0$__widl_f_zoom_and_pan_SVGViewElement\0\0\0\x01\x0ESVGViewElement\x01\0\x01\nzoomAndPan\x01\x01\x05self_\nzoomAndPan\0\0\0(__widl_f_set_zoom_and_pan_SVGViewElement\0\0\0\x01\x0ESVGViewElement\x01\0\x02\nzoomAndPan\x01\x02\x05self_\x0Czoom_and_pan\nzoomAndPan\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
