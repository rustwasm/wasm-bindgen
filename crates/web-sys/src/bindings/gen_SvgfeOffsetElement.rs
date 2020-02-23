use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGFEOffsetElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement)\n\n*This API requires the following crate features to be activated: `SvgfeOffsetElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgfeOffsetElement {
    obj: SvgElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgfeOffsetElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgfeOffsetElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(18u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(70u32);
            inform(69u32);
            inform(79u32);
            inform(102u32);
            inform(102u32);
            inform(115u32);
            inform(101u32);
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
    impl core::ops::Deref for SvgfeOffsetElement {
        type Target = SvgElement;
        #[inline]
        fn deref(&self) -> &SvgElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgfeOffsetElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgfeOffsetElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgfeOffsetElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgfeOffsetElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgfeOffsetElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgfeOffsetElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgfeOffsetElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgfeOffsetElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgfeOffsetElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgfeOffsetElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgfeOffsetElement {
        #[inline]
        fn from(obj: JsValue) -> SvgfeOffsetElement {
            SvgfeOffsetElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgfeOffsetElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgfeOffsetElement> for SvgfeOffsetElement {
        #[inline]
        fn as_ref(&self) -> &SvgfeOffsetElement {
            self
        }
    }
    impl From<SvgfeOffsetElement> for JsValue {
        #[inline]
        fn from(obj: SvgfeOffsetElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgfeOffsetElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGFEOffsetElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGFEOffsetElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGFEOffsetElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgfeOffsetElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgfeOffsetElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgfeOffsetElement> for SvgElement {
    #[inline]
    fn from(obj: SvgfeOffsetElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgfeOffsetElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeOffsetElement> for Element {
    #[inline]
    fn from(obj: SvgfeOffsetElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgfeOffsetElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeOffsetElement> for Node {
    #[inline]
    fn from(obj: SvgfeOffsetElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgfeOffsetElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeOffsetElement> for EventTarget {
    #[inline]
    fn from(obj: SvgfeOffsetElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgfeOffsetElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeOffsetElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgfeOffsetElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgfeOffsetElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeOffsetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_in1_SVGFEOffsetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeOffsetElement as WasmDescribe>::describe();
    <SvgAnimatedString as WasmDescribe>::describe();
}
impl SvgfeOffsetElement {
    #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeOffsetElement",))]
    #[allow(bad_style)]
    #[doc = "The `in1` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/in1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeOffsetElement`*"]
    #[allow(clippy::all)]
    pub fn in1(&self) -> SvgAnimatedString {
        #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeOffsetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_in1_SVGFEOffsetElement(
                self_: <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_in1_SVGFEOffsetElement(
            self_: <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_in1_SVGFEOffsetElement(self_)
            };
            <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeOffsetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dx_SVGFEOffsetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeOffsetElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeOffsetElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeOffsetElement",))]
    #[allow(bad_style)]
    #[doc = "The `dx` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/dx)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeOffsetElement`*"]
    #[allow(clippy::all)]
    pub fn dx(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeOffsetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dx_SVGFEOffsetElement(
                self_: <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dx_SVGFEOffsetElement(
            self_: <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_dx_SVGFEOffsetElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeOffsetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dy_SVGFEOffsetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeOffsetElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeOffsetElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeOffsetElement",))]
    #[allow(bad_style)]
    #[doc = "The `dy` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/dy)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeOffsetElement`*"]
    #[allow(clippy::all)]
    pub fn dy(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeOffsetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dy_SVGFEOffsetElement(
                self_: <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dy_SVGFEOffsetElement(
            self_: <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_dy_SVGFEOffsetElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeOffsetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x_SVGFEOffsetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeOffsetElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeOffsetElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeOffsetElement",))]
    #[allow(bad_style)]
    #[doc = "The `x` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeOffsetElement`*"]
    #[allow(clippy::all)]
    pub fn x(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeOffsetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x_SVGFEOffsetElement(
                self_: <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x_SVGFEOffsetElement(
            self_: <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_x_SVGFEOffsetElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeOffsetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y_SVGFEOffsetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeOffsetElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeOffsetElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeOffsetElement",))]
    #[allow(bad_style)]
    #[doc = "The `y` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeOffsetElement`*"]
    #[allow(clippy::all)]
    pub fn y(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeOffsetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y_SVGFEOffsetElement(
                self_: <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y_SVGFEOffsetElement(
            self_: <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_y_SVGFEOffsetElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeOffsetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_SVGFEOffsetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeOffsetElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeOffsetElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeOffsetElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeOffsetElement`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeOffsetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_SVGFEOffsetElement(
                self_: <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_SVGFEOffsetElement(
            self_: <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_SVGFEOffsetElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeOffsetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_SVGFEOffsetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeOffsetElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeOffsetElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeOffsetElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeOffsetElement`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeOffsetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_SVGFEOffsetElement(
                self_: <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_SVGFEOffsetElement(
            self_: <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_SVGFEOffsetElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeOffsetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_result_SVGFEOffsetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeOffsetElement as WasmDescribe>::describe();
    <SvgAnimatedString as WasmDescribe>::describe();
}
impl SvgfeOffsetElement {
    #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeOffsetElement",))]
    #[allow(bad_style)]
    #[doc = "The `result` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeOffsetElement`*"]
    #[allow(clippy::all)]
    pub fn result(&self) -> SvgAnimatedString {
        #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeOffsetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_result_SVGFEOffsetElement(
                self_: <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_result_SVGFEOffsetElement(
            self_: <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeOffsetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_result_SVGFEOffsetElement(self_)
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
pub static __WASM_BINDGEN_GENERATED_4c6b8ef9d76cd6e2: [u8; 787usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xD1\x02\0\0\0\0\t\0\0\x02\x12SVGFEOffsetElement$__widl_instanceof_SVGFEOffsetElement\0\0\0\0\x1F__widl_f_in1_SVGFEOffsetElement\0\0\0\x01\x12SVGFEOffsetElement\x01\0\x01\x03in1\x01\x01\x05self_\x03in1\0\0\0\x1E__widl_f_dx_SVGFEOffsetElement\0\0\0\x01\x12SVGFEOffsetElement\x01\0\x01\x02dx\x01\x01\x05self_\x02dx\0\0\0\x1E__widl_f_dy_SVGFEOffsetElement\0\0\0\x01\x12SVGFEOffsetElement\x01\0\x01\x02dy\x01\x01\x05self_\x02dy\0\0\0\x1D__widl_f_x_SVGFEOffsetElement\0\0\0\x01\x12SVGFEOffsetElement\x01\0\x01\x01x\x01\x01\x05self_\x01x\0\0\0\x1D__widl_f_y_SVGFEOffsetElement\0\0\0\x01\x12SVGFEOffsetElement\x01\0\x01\x01y\x01\x01\x05self_\x01y\0\0\0!__widl_f_width_SVGFEOffsetElement\0\0\0\x01\x12SVGFEOffsetElement\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0\"__widl_f_height_SVGFEOffsetElement\0\0\0\x01\x12SVGFEOffsetElement\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0\"__widl_f_result_SVGFEOffsetElement\0\0\0\x01\x12SVGFEOffsetElement\x01\0\x01\x06result\x01\x01\x05self_\x06result\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
