use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGFETileElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement)\n\n*This API requires the following crate features to be activated: `SvgfeTileElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgfeTileElement {
    obj: SvgElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgfeTileElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgfeTileElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(70u32);
            inform(69u32);
            inform(84u32);
            inform(105u32);
            inform(108u32);
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
    impl core::ops::Deref for SvgfeTileElement {
        type Target = SvgElement;
        #[inline]
        fn deref(&self) -> &SvgElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgfeTileElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgfeTileElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgfeTileElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgfeTileElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgfeTileElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgfeTileElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgfeTileElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgfeTileElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgfeTileElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgfeTileElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgfeTileElement {
        #[inline]
        fn from(obj: JsValue) -> SvgfeTileElement {
            SvgfeTileElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgfeTileElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgfeTileElement> for SvgfeTileElement {
        #[inline]
        fn as_ref(&self) -> &SvgfeTileElement {
            self
        }
    }
    impl From<SvgfeTileElement> for JsValue {
        #[inline]
        fn from(obj: SvgfeTileElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgfeTileElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGFETileElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGFETileElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGFETileElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgfeTileElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgfeTileElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgfeTileElement> for SvgElement {
    #[inline]
    fn from(obj: SvgfeTileElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgfeTileElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeTileElement> for Element {
    #[inline]
    fn from(obj: SvgfeTileElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgfeTileElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeTileElement> for Node {
    #[inline]
    fn from(obj: SvgfeTileElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgfeTileElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeTileElement> for EventTarget {
    #[inline]
    fn from(obj: SvgfeTileElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgfeTileElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeTileElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgfeTileElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgfeTileElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeTileElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_in1_SVGFETileElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeTileElement as WasmDescribe>::describe();
    <SvgAnimatedString as WasmDescribe>::describe();
}
impl SvgfeTileElement {
    #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeTileElement",))]
    #[allow(bad_style)]
    #[doc = "The `in1` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/in1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeTileElement`*"]
    #[allow(clippy::all)]
    pub fn in1(&self) -> SvgAnimatedString {
        #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeTileElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_in1_SVGFETileElement(
                self_: <&SvgfeTileElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_in1_SVGFETileElement(
            self_: <&SvgfeTileElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeTileElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_in1_SVGFETileElement(self_)
            };
            <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTileElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x_SVGFETileElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeTileElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeTileElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTileElement",))]
    #[allow(bad_style)]
    #[doc = "The `x` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTileElement`*"]
    #[allow(clippy::all)]
    pub fn x(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTileElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x_SVGFETileElement(
                self_: <&SvgfeTileElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x_SVGFETileElement(
            self_: <&SvgfeTileElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeTileElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_x_SVGFETileElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTileElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y_SVGFETileElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeTileElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeTileElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTileElement",))]
    #[allow(bad_style)]
    #[doc = "The `y` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTileElement`*"]
    #[allow(clippy::all)]
    pub fn y(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTileElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y_SVGFETileElement(
                self_: <&SvgfeTileElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y_SVGFETileElement(
            self_: <&SvgfeTileElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeTileElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_y_SVGFETileElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTileElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_SVGFETileElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeTileElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeTileElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTileElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTileElement`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTileElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_SVGFETileElement(
                self_: <&SvgfeTileElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_SVGFETileElement(
            self_: <&SvgfeTileElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeTileElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_SVGFETileElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTileElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_SVGFETileElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeTileElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeTileElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTileElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTileElement`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTileElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_SVGFETileElement(
                self_: <&SvgfeTileElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_SVGFETileElement(
            self_: <&SvgfeTileElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeTileElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_SVGFETileElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeTileElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_result_SVGFETileElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeTileElement as WasmDescribe>::describe();
    <SvgAnimatedString as WasmDescribe>::describe();
}
impl SvgfeTileElement {
    #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeTileElement",))]
    #[allow(bad_style)]
    #[doc = "The `result` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeTileElement`*"]
    #[allow(clippy::all)]
    pub fn result(&self) -> SvgAnimatedString {
        #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeTileElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_result_SVGFETileElement(
                self_: <&SvgfeTileElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_result_SVGFETileElement(
            self_: <&SvgfeTileElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeTileElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_result_SVGFETileElement(self_)
            };
            <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_551919e7bf7827e7: [u8; 611usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}!\x02\0\0\0\0\x07\0\0\x02\x10SVGFETileElement\"__widl_instanceof_SVGFETileElement\0\0\0\0\x1D__widl_f_in1_SVGFETileElement\0\0\0\x01\x10SVGFETileElement\x01\0\x01\x03in1\x01\x01\x05self_\x03in1\0\0\0\x1B__widl_f_x_SVGFETileElement\0\0\0\x01\x10SVGFETileElement\x01\0\x01\x01x\x01\x01\x05self_\x01x\0\0\0\x1B__widl_f_y_SVGFETileElement\0\0\0\x01\x10SVGFETileElement\x01\0\x01\x01y\x01\x01\x05self_\x01y\0\0\0\x1F__widl_f_width_SVGFETileElement\0\0\0\x01\x10SVGFETileElement\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0 __widl_f_height_SVGFETileElement\0\0\0\x01\x10SVGFETileElement\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0 __widl_f_result_SVGFETileElement\0\0\0\x01\x10SVGFETileElement\x01\0\x01\x06result\x01\x01\x05self_\x06result\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
