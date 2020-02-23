use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGFEColorMatrixElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement)\n\n*This API requires the following crate features to be activated: `SvgfeColorMatrixElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgfeColorMatrixElement {
    obj: SvgElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgfeColorMatrixElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgfeColorMatrixElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(23u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(70u32);
            inform(69u32);
            inform(67u32);
            inform(111u32);
            inform(108u32);
            inform(111u32);
            inform(114u32);
            inform(77u32);
            inform(97u32);
            inform(116u32);
            inform(114u32);
            inform(105u32);
            inform(120u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgfeColorMatrixElement {
        type Target = SvgElement;
        #[inline]
        fn deref(&self) -> &SvgElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgfeColorMatrixElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgfeColorMatrixElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgfeColorMatrixElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgfeColorMatrixElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgfeColorMatrixElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgfeColorMatrixElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgfeColorMatrixElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgfeColorMatrixElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgfeColorMatrixElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgfeColorMatrixElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgfeColorMatrixElement {
        #[inline]
        fn from(obj: JsValue) -> SvgfeColorMatrixElement {
            SvgfeColorMatrixElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgfeColorMatrixElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgfeColorMatrixElement> for SvgfeColorMatrixElement {
        #[inline]
        fn as_ref(&self) -> &SvgfeColorMatrixElement {
            self
        }
    }
    impl From<SvgfeColorMatrixElement> for JsValue {
        #[inline]
        fn from(obj: SvgfeColorMatrixElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgfeColorMatrixElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGFEColorMatrixElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGFEColorMatrixElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGFEColorMatrixElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgfeColorMatrixElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgfeColorMatrixElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgfeColorMatrixElement> for SvgElement {
    #[inline]
    fn from(obj: SvgfeColorMatrixElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgfeColorMatrixElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeColorMatrixElement> for Element {
    #[inline]
    fn from(obj: SvgfeColorMatrixElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgfeColorMatrixElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeColorMatrixElement> for Node {
    #[inline]
    fn from(obj: SvgfeColorMatrixElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgfeColorMatrixElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeColorMatrixElement> for EventTarget {
    #[inline]
    fn from(obj: SvgfeColorMatrixElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgfeColorMatrixElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeColorMatrixElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgfeColorMatrixElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgfeColorMatrixElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeColorMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_in1_SVGFEColorMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeColorMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedString as WasmDescribe>::describe();
}
impl SvgfeColorMatrixElement {
    #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeColorMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `in1` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/in1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeColorMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn in1(&self) -> SvgAnimatedString {
        #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeColorMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_in1_SVGFEColorMatrixElement(
                self_: <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_in1_SVGFEColorMatrixElement(
            self_: <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_in1_SVGFEColorMatrixElement(self_)
            };
            <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedEnumeration",
    feature = "SvgfeColorMatrixElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_SVGFEColorMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeColorMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedEnumeration as WasmDescribe>::describe();
}
impl SvgfeColorMatrixElement {
    #[cfg(all(
        feature = "SvgAnimatedEnumeration",
        feature = "SvgfeColorMatrixElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/type)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeColorMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> SvgAnimatedEnumeration {
        #[cfg(all(
            feature = "SvgAnimatedEnumeration",
            feature = "SvgfeColorMatrixElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_SVGFEColorMatrixElement(
                self_: <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_SVGFEColorMatrixElement(
            self_: <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_type_SVGFEColorMatrixElement(self_)
            };
            <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumberList", feature = "SvgfeColorMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_values_SVGFEColorMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeColorMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedNumberList as WasmDescribe>::describe();
}
impl SvgfeColorMatrixElement {
    #[cfg(all(feature = "SvgAnimatedNumberList", feature = "SvgfeColorMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `values` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/values)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgfeColorMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn values(&self) -> SvgAnimatedNumberList {
        #[cfg(all(feature = "SvgAnimatedNumberList", feature = "SvgfeColorMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_values_SVGFEColorMatrixElement(
                self_: <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumberList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_values_SVGFEColorMatrixElement(
            self_: <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedNumberList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_values_SVGFEColorMatrixElement(self_)
            };
            <SvgAnimatedNumberList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeColorMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x_SVGFEColorMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeColorMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeColorMatrixElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeColorMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `x` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeColorMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn x(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeColorMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x_SVGFEColorMatrixElement(
                self_: <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x_SVGFEColorMatrixElement(
            self_: <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_x_SVGFEColorMatrixElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeColorMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y_SVGFEColorMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeColorMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeColorMatrixElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeColorMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `y` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeColorMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn y(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeColorMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y_SVGFEColorMatrixElement(
                self_: <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y_SVGFEColorMatrixElement(
            self_: <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_y_SVGFEColorMatrixElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeColorMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_SVGFEColorMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeColorMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeColorMatrixElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeColorMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeColorMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeColorMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_SVGFEColorMatrixElement(
                self_: <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_SVGFEColorMatrixElement(
            self_: <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_width_SVGFEColorMatrixElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeColorMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_SVGFEColorMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeColorMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeColorMatrixElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeColorMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeColorMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeColorMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_SVGFEColorMatrixElement(
                self_: <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_SVGFEColorMatrixElement(
            self_: <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_height_SVGFEColorMatrixElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeColorMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_result_SVGFEColorMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeColorMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedString as WasmDescribe>::describe();
}
impl SvgfeColorMatrixElement {
    #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeColorMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `result` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeColorMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn result(&self) -> SvgAnimatedString {
        #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeColorMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_result_SVGFEColorMatrixElement(
                self_: <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_result_SVGFEColorMatrixElement(
            self_: <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeColorMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_result_SVGFEColorMatrixElement(self_)
            };
            <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl SvgfeColorMatrixElement {
    pub const SVG_FECOLORMATRIX_TYPE_UNKNOWN: u16 = 0i64 as u16;
}
impl SvgfeColorMatrixElement {
    pub const SVG_FECOLORMATRIX_TYPE_MATRIX: u16 = 1u64 as u16;
}
impl SvgfeColorMatrixElement {
    pub const SVG_FECOLORMATRIX_TYPE_SATURATE: u16 = 2u64 as u16;
}
impl SvgfeColorMatrixElement {
    pub const SVG_FECOLORMATRIX_TYPE_HUEROTATE: u16 = 3u64 as u16;
}
impl SvgfeColorMatrixElement {
    pub const SVG_FECOLORMATRIX_TYPE_LUMINANCETOALPHA: u16 = 4u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_d196ac1f39c6241a: [u8; 895usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}=\x03\0\0\0\0\t\0\0\x02\x17SVGFEColorMatrixElement)__widl_instanceof_SVGFEColorMatrixElement\0\0\0\0$__widl_f_in1_SVGFEColorMatrixElement\0\0\0\x01\x17SVGFEColorMatrixElement\x01\0\x01\x03in1\x01\x01\x05self_\x03in1\0\0\0%__widl_f_type_SVGFEColorMatrixElement\0\0\0\x01\x17SVGFEColorMatrixElement\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0'__widl_f_values_SVGFEColorMatrixElement\0\0\0\x01\x17SVGFEColorMatrixElement\x01\0\x01\x06values\x01\x01\x05self_\x06values\0\0\0\"__widl_f_x_SVGFEColorMatrixElement\0\0\0\x01\x17SVGFEColorMatrixElement\x01\0\x01\x01x\x01\x01\x05self_\x01x\0\0\0\"__widl_f_y_SVGFEColorMatrixElement\0\0\0\x01\x17SVGFEColorMatrixElement\x01\0\x01\x01y\x01\x01\x05self_\x01y\0\0\0&__widl_f_width_SVGFEColorMatrixElement\0\0\0\x01\x17SVGFEColorMatrixElement\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0'__widl_f_height_SVGFEColorMatrixElement\0\0\0\x01\x17SVGFEColorMatrixElement\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0'__widl_f_result_SVGFEColorMatrixElement\0\0\0\x01\x17SVGFEColorMatrixElement\x01\0\x01\x06result\x01\x01\x05self_\x06result\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
