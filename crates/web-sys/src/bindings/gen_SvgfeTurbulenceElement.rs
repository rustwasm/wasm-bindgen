use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGFETurbulenceElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement)\n\n*This API requires the following crate features to be activated: `SvgfeTurbulenceElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgfeTurbulenceElement {
    obj: SvgElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgfeTurbulenceElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgfeTurbulenceElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(22u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(70u32);
            inform(69u32);
            inform(84u32);
            inform(117u32);
            inform(114u32);
            inform(98u32);
            inform(117u32);
            inform(108u32);
            inform(101u32);
            inform(110u32);
            inform(99u32);
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
    impl core::ops::Deref for SvgfeTurbulenceElement {
        type Target = SvgElement;
        #[inline]
        fn deref(&self) -> &SvgElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgfeTurbulenceElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgfeTurbulenceElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgfeTurbulenceElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgfeTurbulenceElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgfeTurbulenceElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgfeTurbulenceElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgfeTurbulenceElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgfeTurbulenceElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgfeTurbulenceElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgfeTurbulenceElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgfeTurbulenceElement {
        #[inline]
        fn from(obj: JsValue) -> SvgfeTurbulenceElement {
            SvgfeTurbulenceElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgfeTurbulenceElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgfeTurbulenceElement> for SvgfeTurbulenceElement {
        #[inline]
        fn as_ref(&self) -> &SvgfeTurbulenceElement {
            self
        }
    }
    impl From<SvgfeTurbulenceElement> for JsValue {
        #[inline]
        fn from(obj: SvgfeTurbulenceElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgfeTurbulenceElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGFETurbulenceElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGFETurbulenceElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGFETurbulenceElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgfeTurbulenceElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgfeTurbulenceElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgfeTurbulenceElement> for SvgElement {
    #[inline]
    fn from(obj: SvgfeTurbulenceElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgfeTurbulenceElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeTurbulenceElement> for Element {
    #[inline]
    fn from(obj: SvgfeTurbulenceElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgfeTurbulenceElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeTurbulenceElement> for Node {
    #[inline]
    fn from(obj: SvgfeTurbulenceElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgfeTurbulenceElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeTurbulenceElement> for EventTarget {
    #[inline]
    fn from(obj: SvgfeTurbulenceElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgfeTurbulenceElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeTurbulenceElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgfeTurbulenceElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgfeTurbulenceElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeTurbulenceElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_base_frequency_x_SVGFETurbulenceElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeTurbulenceElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeTurbulenceElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeTurbulenceElement",))]
    #[allow(bad_style)]
    #[doc = "The `baseFrequencyX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/baseFrequencyX)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeTurbulenceElement`*"]
    #[allow(clippy::all)]
    pub fn base_frequency_x(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeTurbulenceElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_base_frequency_x_SVGFETurbulenceElement(
                self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_base_frequency_x_SVGFETurbulenceElement(
            self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_base_frequency_x_SVGFETurbulenceElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeTurbulenceElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_base_frequency_y_SVGFETurbulenceElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeTurbulenceElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeTurbulenceElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeTurbulenceElement",))]
    #[allow(bad_style)]
    #[doc = "The `baseFrequencyY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/baseFrequencyY)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeTurbulenceElement`*"]
    #[allow(clippy::all)]
    pub fn base_frequency_y(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeTurbulenceElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_base_frequency_y_SVGFETurbulenceElement(
                self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_base_frequency_y_SVGFETurbulenceElement(
            self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_base_frequency_y_SVGFETurbulenceElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedInteger", feature = "SvgfeTurbulenceElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_num_octaves_SVGFETurbulenceElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeTurbulenceElement as WasmDescribe>::describe();
    <SvgAnimatedInteger as WasmDescribe>::describe();
}
impl SvgfeTurbulenceElement {
    #[cfg(all(feature = "SvgAnimatedInteger", feature = "SvgfeTurbulenceElement",))]
    #[allow(bad_style)]
    #[doc = "The `numOctaves` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/numOctaves)\n\n*This API requires the following crate features to be activated: `SvgAnimatedInteger`, `SvgfeTurbulenceElement`*"]
    #[allow(clippy::all)]
    pub fn num_octaves(&self) -> SvgAnimatedInteger {
        #[cfg(all(feature = "SvgAnimatedInteger", feature = "SvgfeTurbulenceElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_num_octaves_SVGFETurbulenceElement(
                self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedInteger as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_num_octaves_SVGFETurbulenceElement(
            self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedInteger as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_num_octaves_SVGFETurbulenceElement(self_)
            };
            <SvgAnimatedInteger as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeTurbulenceElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_seed_SVGFETurbulenceElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeTurbulenceElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeTurbulenceElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeTurbulenceElement",))]
    #[allow(bad_style)]
    #[doc = "The `seed` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/seed)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeTurbulenceElement`*"]
    #[allow(clippy::all)]
    pub fn seed(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeTurbulenceElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_seed_SVGFETurbulenceElement(
                self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_seed_SVGFETurbulenceElement(
            self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_seed_SVGFETurbulenceElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgfeTurbulenceElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stitch_tiles_SVGFETurbulenceElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeTurbulenceElement as WasmDescribe>::describe();
    <SvgAnimatedEnumeration as WasmDescribe>::describe();
}
impl SvgfeTurbulenceElement {
    #[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgfeTurbulenceElement",))]
    #[allow(bad_style)]
    #[doc = "The `stitchTiles` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/stitchTiles)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeTurbulenceElement`*"]
    #[allow(clippy::all)]
    pub fn stitch_tiles(&self) -> SvgAnimatedEnumeration {
        #[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgfeTurbulenceElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stitch_tiles_SVGFETurbulenceElement(
                self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stitch_tiles_SVGFETurbulenceElement(
            self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_stitch_tiles_SVGFETurbulenceElement(self_)
            };
            <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgfeTurbulenceElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_SVGFETurbulenceElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeTurbulenceElement as WasmDescribe>::describe();
    <SvgAnimatedEnumeration as WasmDescribe>::describe();
}
impl SvgfeTurbulenceElement {
    #[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgfeTurbulenceElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/type)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeTurbulenceElement`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> SvgAnimatedEnumeration {
        #[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgfeTurbulenceElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_SVGFETurbulenceElement(
                self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_SVGFETurbulenceElement(
            self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_SVGFETurbulenceElement(self_)
            };
            <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTurbulenceElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x_SVGFETurbulenceElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeTurbulenceElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeTurbulenceElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTurbulenceElement",))]
    #[allow(bad_style)]
    #[doc = "The `x` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTurbulenceElement`*"]
    #[allow(clippy::all)]
    pub fn x(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTurbulenceElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x_SVGFETurbulenceElement(
                self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x_SVGFETurbulenceElement(
            self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_x_SVGFETurbulenceElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTurbulenceElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y_SVGFETurbulenceElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeTurbulenceElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeTurbulenceElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTurbulenceElement",))]
    #[allow(bad_style)]
    #[doc = "The `y` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTurbulenceElement`*"]
    #[allow(clippy::all)]
    pub fn y(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTurbulenceElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y_SVGFETurbulenceElement(
                self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y_SVGFETurbulenceElement(
            self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_y_SVGFETurbulenceElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTurbulenceElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_SVGFETurbulenceElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeTurbulenceElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeTurbulenceElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTurbulenceElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTurbulenceElement`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTurbulenceElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_SVGFETurbulenceElement(
                self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_SVGFETurbulenceElement(
            self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_SVGFETurbulenceElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTurbulenceElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_SVGFETurbulenceElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeTurbulenceElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeTurbulenceElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTurbulenceElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTurbulenceElement`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeTurbulenceElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_SVGFETurbulenceElement(
                self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_SVGFETurbulenceElement(
            self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_SVGFETurbulenceElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeTurbulenceElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_result_SVGFETurbulenceElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeTurbulenceElement as WasmDescribe>::describe();
    <SvgAnimatedString as WasmDescribe>::describe();
}
impl SvgfeTurbulenceElement {
    #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeTurbulenceElement",))]
    #[allow(bad_style)]
    #[doc = "The `result` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeTurbulenceElement`*"]
    #[allow(clippy::all)]
    pub fn result(&self) -> SvgAnimatedString {
        #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeTurbulenceElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_result_SVGFETurbulenceElement(
                self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_result_SVGFETurbulenceElement(
            self_: <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeTurbulenceElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_result_SVGFETurbulenceElement(self_)
            };
            <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl SvgfeTurbulenceElement {
    pub const SVG_TURBULENCE_TYPE_UNKNOWN: u16 = 0i64 as u16;
}
impl SvgfeTurbulenceElement {
    pub const SVG_TURBULENCE_TYPE_FRACTALNOISE: u16 = 1u64 as u16;
}
impl SvgfeTurbulenceElement {
    pub const SVG_TURBULENCE_TYPE_TURBULENCE: u16 = 2u64 as u16;
}
impl SvgfeTurbulenceElement {
    pub const SVG_STITCHTYPE_UNKNOWN: u16 = 0i64 as u16;
}
impl SvgfeTurbulenceElement {
    pub const SVG_STITCHTYPE_STITCH: u16 = 1u64 as u16;
}
impl SvgfeTurbulenceElement {
    pub const SVG_STITCHTYPE_NOSTITCH: u16 = 2u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_d60995a7397def42: [u8; 1243usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x99\x04\0\0\0\0\x0C\0\0\x02\x16SVGFETurbulenceElement(__widl_instanceof_SVGFETurbulenceElement\0\0\0\00__widl_f_base_frequency_x_SVGFETurbulenceElement\0\0\0\x01\x16SVGFETurbulenceElement\x01\0\x01\x0EbaseFrequencyX\x01\x01\x05self_\x0EbaseFrequencyX\0\0\00__widl_f_base_frequency_y_SVGFETurbulenceElement\0\0\0\x01\x16SVGFETurbulenceElement\x01\0\x01\x0EbaseFrequencyY\x01\x01\x05self_\x0EbaseFrequencyY\0\0\0+__widl_f_num_octaves_SVGFETurbulenceElement\0\0\0\x01\x16SVGFETurbulenceElement\x01\0\x01\nnumOctaves\x01\x01\x05self_\nnumOctaves\0\0\0$__widl_f_seed_SVGFETurbulenceElement\0\0\0\x01\x16SVGFETurbulenceElement\x01\0\x01\x04seed\x01\x01\x05self_\x04seed\0\0\0,__widl_f_stitch_tiles_SVGFETurbulenceElement\0\0\0\x01\x16SVGFETurbulenceElement\x01\0\x01\x0BstitchTiles\x01\x01\x05self_\x0BstitchTiles\0\0\0$__widl_f_type_SVGFETurbulenceElement\0\0\0\x01\x16SVGFETurbulenceElement\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0!__widl_f_x_SVGFETurbulenceElement\0\0\0\x01\x16SVGFETurbulenceElement\x01\0\x01\x01x\x01\x01\x05self_\x01x\0\0\0!__widl_f_y_SVGFETurbulenceElement\0\0\0\x01\x16SVGFETurbulenceElement\x01\0\x01\x01y\x01\x01\x05self_\x01y\0\0\0%__widl_f_width_SVGFETurbulenceElement\0\0\0\x01\x16SVGFETurbulenceElement\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0&__widl_f_height_SVGFETurbulenceElement\0\0\0\x01\x16SVGFETurbulenceElement\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0&__widl_f_result_SVGFETurbulenceElement\0\0\0\x01\x16SVGFETurbulenceElement\x01\0\x01\x06result\x01\x01\x05self_\x06result\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
