use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGSVGElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgsvgElement {
    obj: SvgGraphicsElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgsvgElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgsvgElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgsvgElement {
        type Target = SvgGraphicsElement;
        #[inline]
        fn deref(&self) -> &SvgGraphicsElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgsvgElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgsvgElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgsvgElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgsvgElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgsvgElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgsvgElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgsvgElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgsvgElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgsvgElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgsvgElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgsvgElement {
        #[inline]
        fn from(obj: JsValue) -> SvgsvgElement {
            SvgsvgElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgsvgElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgsvgElement> for SvgsvgElement {
        #[inline]
        fn as_ref(&self) -> &SvgsvgElement {
            self
        }
    }
    impl From<SvgsvgElement> for JsValue {
        #[inline]
        fn from(obj: SvgsvgElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgsvgElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGSVGElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGSVGElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGSVGElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgsvgElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgsvgElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgsvgElement> for SvgGraphicsElement {
    #[inline]
    fn from(obj: SvgsvgElement) -> SvgGraphicsElement {
        use wasm_bindgen::JsCast;
        SvgGraphicsElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgGraphicsElement> for SvgsvgElement {
    #[inline]
    fn as_ref(&self) -> &SvgGraphicsElement {
        use wasm_bindgen::JsCast;
        SvgGraphicsElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgsvgElement> for SvgElement {
    #[inline]
    fn from(obj: SvgsvgElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgsvgElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgsvgElement> for Element {
    #[inline]
    fn from(obj: SvgsvgElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgsvgElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgsvgElement> for Node {
    #[inline]
    fn from(obj: SvgsvgElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgsvgElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgsvgElement> for EventTarget {
    #[inline]
    fn from(obj: SvgsvgElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgsvgElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgsvgElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgsvgElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgsvgElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_animations_paused_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `animationsPaused()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/animationsPaused)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn animations_paused(&self) -> bool {
        #[cfg(all(feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_animations_paused_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_animations_paused_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_animations_paused_SVGSVGElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAngle", feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_svg_angle_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <SvgAngle as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgAngle", feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `createSVGAngle()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGAngle)\n\n*This API requires the following crate features to be activated: `SvgAngle`, `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn create_svg_angle(&self) -> SvgAngle {
        #[cfg(all(feature = "SvgAngle", feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_svg_angle_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAngle as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_svg_angle_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAngle as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_svg_angle_SVGSVGElement(self_)
            };
            <SvgAngle as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgLength", feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_svg_length_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <SvgLength as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgLength", feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `createSVGLength()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGLength)\n\n*This API requires the following crate features to be activated: `SvgLength`, `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn create_svg_length(&self) -> SvgLength {
        #[cfg(all(feature = "SvgLength", feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_svg_length_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_svg_length_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgLength as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_svg_length_SVGSVGElement(self_)
            };
            <SvgLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgMatrix", feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_svg_matrix_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <SvgMatrix as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgMatrix", feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `createSVGMatrix()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGMatrix)\n\n*This API requires the following crate features to be activated: `SvgMatrix`, `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn create_svg_matrix(&self) -> SvgMatrix {
        #[cfg(all(feature = "SvgMatrix", feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_svg_matrix_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_svg_matrix_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_svg_matrix_SVGSVGElement(self_)
            };
            <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgNumber", feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_svg_number_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <SvgNumber as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgNumber", feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `createSVGNumber()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGNumber)\n\n*This API requires the following crate features to be activated: `SvgNumber`, `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn create_svg_number(&self) -> SvgNumber {
        #[cfg(all(feature = "SvgNumber", feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_svg_number_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_svg_number_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgNumber as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_svg_number_SVGSVGElement(self_)
            };
            <SvgNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPoint", feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_svg_point_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <SvgPoint as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgPoint", feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `createSVGPoint()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGPoint)\n\n*This API requires the following crate features to be activated: `SvgPoint`, `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn create_svg_point(&self) -> SvgPoint {
        #[cfg(all(feature = "SvgPoint", feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_svg_point_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_svg_point_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_svg_point_SVGSVGElement(self_)
            };
            <SvgPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgRect", feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_svg_rect_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <SvgRect as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgRect", feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `createSVGRect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGRect)\n\n*This API requires the following crate features to be activated: `SvgRect`, `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn create_svg_rect(&self) -> SvgRect {
        #[cfg(all(feature = "SvgRect", feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_svg_rect_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgRect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_svg_rect_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgRect as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_svg_rect_SVGSVGElement(self_)
            };
            <SvgRect as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgTransform", feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_svg_transform_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <SvgTransform as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgTransform", feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `createSVGTransform()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGTransform)\n\n*This API requires the following crate features to be activated: `SvgTransform`, `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn create_svg_transform(&self) -> SvgTransform {
        #[cfg(all(feature = "SvgTransform", feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_svg_transform_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_svg_transform_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_svg_transform_SVGSVGElement(self_)
            };
            <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgMatrix",
    feature = "SvgTransform",
    feature = "SvgsvgElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_svg_transform_from_matrix_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <&SvgMatrix as WasmDescribe>::describe();
    <SvgTransform as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(
        feature = "SvgMatrix",
        feature = "SvgTransform",
        feature = "SvgsvgElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createSVGTransformFromMatrix()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGTransformFromMatrix)\n\n*This API requires the following crate features to be activated: `SvgMatrix`, `SvgTransform`, `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn create_svg_transform_from_matrix(&self, matrix: &SvgMatrix) -> SvgTransform {
        #[cfg(all(
            feature = "SvgMatrix",
            feature = "SvgTransform",
            feature = "SvgsvgElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_svg_transform_from_matrix_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                matrix: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_svg_transform_from_matrix_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            matrix: <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(matrix);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let matrix = <&SvgMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(matrix);
                __widl_f_create_svg_transform_from_matrix_SVGSVGElement(self_, matrix)
            };
            <SvgTransform as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_deselect_all_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `deselectAll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/deselectAll)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn deselect_all(&self) {
        #[cfg(all(feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_deselect_all_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_deselect_all_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_deselect_all_SVGSVGElement(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_force_redraw_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `forceRedraw()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/forceRedraw)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn force_redraw(&self) {
        #[cfg(all(feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_force_redraw_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_force_redraw_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_force_redraw_SVGSVGElement(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_current_time_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `getCurrentTime()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/getCurrentTime)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn get_current_time(&self) -> f32 {
        #[cfg(all(feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_current_time_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_current_time_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_current_time_SVGSVGElement(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element", feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_element_by_id_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "Element", feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `getElementById()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/getElementById)\n\n*This API requires the following crate features to be activated: `Element`, `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn get_element_by_id(&self, element_id: &str) -> Option<Element> {
        #[cfg(all(feature = "Element", feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_element_by_id_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                element_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_element_by_id_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            element_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(element_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let element_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(element_id);
                __widl_f_get_element_by_id_SVGSVGElement(self_, element_id)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pause_animations_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `pauseAnimations()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/pauseAnimations)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn pause_animations(&self) {
        #[cfg(all(feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pause_animations_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pause_animations_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pause_animations_SVGSVGElement(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_current_time_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `setCurrentTime()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/setCurrentTime)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_current_time(&self, seconds: f32) {
        #[cfg(all(feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_current_time_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                seconds: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_current_time_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            seconds: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(seconds);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let seconds = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(seconds);
                __widl_f_set_current_time_SVGSVGElement(self_, seconds)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_suspend_redraw_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `suspendRedraw()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/suspendRedraw)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn suspend_redraw(&self, max_wait_milliseconds: u32) -> u32 {
        #[cfg(all(feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_suspend_redraw_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                max_wait_milliseconds: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_suspend_redraw_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            max_wait_milliseconds: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(max_wait_milliseconds);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let max_wait_milliseconds =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(max_wait_milliseconds);
                __widl_f_suspend_redraw_SVGSVGElement(self_, max_wait_milliseconds)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unpause_animations_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `unpauseAnimations()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/unpauseAnimations)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn unpause_animations(&self) {
        #[cfg(all(feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unpause_animations_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unpause_animations_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_unpause_animations_SVGSVGElement(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unsuspend_redraw_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `unsuspendRedraw()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/unsuspendRedraw)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn unsuspend_redraw(&self, suspend_handle_id: u32) {
        #[cfg(all(feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unsuspend_redraw_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                suspend_handle_id: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unsuspend_redraw_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            suspend_handle_id: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(suspend_handle_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let suspend_handle_id =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(suspend_handle_id);
                __widl_f_unsuspend_redraw_SVGSVGElement(self_, suspend_handle_id)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unsuspend_redraw_all_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `unsuspendRedrawAll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/unsuspendRedrawAll)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn unsuspend_redraw_all(&self) {
        #[cfg(all(feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unsuspend_redraw_all_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unsuspend_redraw_all_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_unsuspend_redraw_all_SVGSVGElement(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `x` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn x(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_x_SVGSVGElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `y` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn y(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_y_SVGSVGElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_SVGSVGElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_SVGSVGElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_use_current_view_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `useCurrentView` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/useCurrentView)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn use_current_view(&self) -> bool {
        #[cfg(all(feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_use_current_view_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_use_current_view_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_use_current_view_SVGSVGElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_current_scale_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `currentScale` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/currentScale)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn current_scale(&self) -> f32 {
        #[cfg(all(feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_current_scale_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_current_scale_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_current_scale_SVGSVGElement(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_current_scale_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `currentScale` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/currentScale)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_current_scale(&self, current_scale: f32) {
        #[cfg(all(feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_current_scale_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                current_scale: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_current_scale_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            current_scale: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(current_scale);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let current_scale =
                    <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(current_scale);
                __widl_f_set_current_scale_SVGSVGElement(self_, current_scale)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgPoint", feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_current_translate_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <SvgPoint as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgPoint", feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `currentTranslate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/currentTranslate)\n\n*This API requires the following crate features to be activated: `SvgPoint`, `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn current_translate(&self) -> SvgPoint {
        #[cfg(all(feature = "SvgPoint", feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_current_translate_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_current_translate_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_current_translate_SVGSVGElement(self_)
            };
            <SvgPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedRect", feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_view_box_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <SvgAnimatedRect as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgAnimatedRect", feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `viewBox` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/viewBox)\n\n*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn view_box(&self) -> SvgAnimatedRect {
        #[cfg(all(feature = "SvgAnimatedRect", feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_view_box_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedRect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_view_box_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedRect as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_view_box_SVGSVGElement(self_)
            };
            <SvgAnimatedRect as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedPreserveAspectRatio", feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_preserve_aspect_ratio_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <SvgAnimatedPreserveAspectRatio as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgAnimatedPreserveAspectRatio", feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `preserveAspectRatio` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/preserveAspectRatio)\n\n*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn preserve_aspect_ratio(&self) -> SvgAnimatedPreserveAspectRatio {
        #[cfg(all(feature = "SvgAnimatedPreserveAspectRatio", feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_preserve_aspect_ratio_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedPreserveAspectRatio as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_preserve_aspect_ratio_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedPreserveAspectRatio as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_preserve_aspect_ratio_SVGSVGElement(self_)
            };
            <SvgAnimatedPreserveAspectRatio as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_zoom_and_pan_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `zoomAndPan` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/zoomAndPan)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn zoom_and_pan(&self) -> u16 {
        #[cfg(all(feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_zoom_and_pan_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_zoom_and_pan_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_zoom_and_pan_SVGSVGElement(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_zoom_and_pan_SVGSVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgsvgElement as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgsvgElement {
    #[cfg(all(feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `zoomAndPan` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/zoomAndPan)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_zoom_and_pan(&self, zoom_and_pan: u16) {
        #[cfg(all(feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_zoom_and_pan_SVGSVGElement(
                self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                zoom_and_pan: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_zoom_and_pan_SVGSVGElement(
            self_: <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&SvgsvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let zoom_and_pan =
                    <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(zoom_and_pan);
                __widl_f_set_zoom_and_pan_SVGSVGElement(self_, zoom_and_pan)
            };
            ()
        }
    }
}
impl SvgsvgElement {
    pub const SVG_ZOOMANDPAN_UNKNOWN: u16 = 0i64 as u16;
}
impl SvgsvgElement {
    pub const SVG_ZOOMANDPAN_DISABLE: u16 = 1u64 as u16;
}
impl SvgsvgElement {
    pub const SVG_ZOOMANDPAN_MAGNIFY: u16 = 2u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_e4fb167fdff3c4ee: [u8; 3000usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}v\x0B\0\0\0\0 \0\0\x02\rSVGSVGElement\x1F__widl_instanceof_SVGSVGElement\0\0\0\0(__widl_f_animations_paused_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\0\x01\x01\x05self_\x10animationsPaused\0\0\0'__widl_f_create_svg_angle_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\0\x01\x01\x05self_\x0EcreateSVGAngle\0\0\0(__widl_f_create_svg_length_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\0\x01\x01\x05self_\x0FcreateSVGLength\0\0\0(__widl_f_create_svg_matrix_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\0\x01\x01\x05self_\x0FcreateSVGMatrix\0\0\0(__widl_f_create_svg_number_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\0\x01\x01\x05self_\x0FcreateSVGNumber\0\0\0'__widl_f_create_svg_point_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\0\x01\x01\x05self_\x0EcreateSVGPoint\0\0\0&__widl_f_create_svg_rect_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\0\x01\x01\x05self_\rcreateSVGRect\0\0\0+__widl_f_create_svg_transform_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\0\x01\x01\x05self_\x12createSVGTransform\0\0\07__widl_f_create_svg_transform_from_matrix_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\0\x01\x02\x05self_\x06matrix\x1CcreateSVGTransformFromMatrix\0\0\0#__widl_f_deselect_all_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\0\x01\x01\x05self_\x0BdeselectAll\0\0\0#__widl_f_force_redraw_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\0\x01\x01\x05self_\x0BforceRedraw\0\0\0'__widl_f_get_current_time_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\0\x01\x01\x05self_\x0EgetCurrentTime\0\0\0(__widl_f_get_element_by_id_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\0\x01\x02\x05self_\nelement_id\x0EgetElementById\0\0\0'__widl_f_pause_animations_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\0\x01\x01\x05self_\x0FpauseAnimations\0\0\0'__widl_f_set_current_time_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\0\x01\x02\x05self_\x07seconds\x0EsetCurrentTime\0\0\0%__widl_f_suspend_redraw_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\0\x01\x02\x05self_\x15max_wait_milliseconds\rsuspendRedraw\0\0\0)__widl_f_unpause_animations_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\0\x01\x01\x05self_\x11unpauseAnimations\0\0\0'__widl_f_unsuspend_redraw_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\0\x01\x02\x05self_\x11suspend_handle_id\x0FunsuspendRedraw\0\0\0+__widl_f_unsuspend_redraw_all_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\0\x01\x01\x05self_\x12unsuspendRedrawAll\0\0\0\x18__widl_f_x_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\x01\x01x\x01\x01\x05self_\x01x\0\0\0\x18__widl_f_y_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\x01\x01y\x01\x01\x05self_\x01y\0\0\0\x1C__widl_f_width_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0\x1D__widl_f_height_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0'__widl_f_use_current_view_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\x01\x0EuseCurrentView\x01\x01\x05self_\x0EuseCurrentView\0\0\0$__widl_f_current_scale_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\x01\x0CcurrentScale\x01\x01\x05self_\x0CcurrentScale\0\0\0(__widl_f_set_current_scale_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\x02\x0CcurrentScale\x01\x02\x05self_\rcurrent_scale\x0CcurrentScale\0\0\0(__widl_f_current_translate_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\x01\x10currentTranslate\x01\x01\x05self_\x10currentTranslate\0\0\0\x1F__widl_f_view_box_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\x01\x07viewBox\x01\x01\x05self_\x07viewBox\0\0\0,__widl_f_preserve_aspect_ratio_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\x01\x13preserveAspectRatio\x01\x01\x05self_\x13preserveAspectRatio\0\0\0#__widl_f_zoom_and_pan_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\x01\nzoomAndPan\x01\x01\x05self_\nzoomAndPan\0\0\0'__widl_f_set_zoom_and_pan_SVGSVGElement\0\0\0\x01\rSVGSVGElement\x01\0\x02\nzoomAndPan\x01\x02\x05self_\x0Czoom_and_pan\nzoomAndPan\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
