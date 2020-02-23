use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGFESpotLightElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement)\n\n*This API requires the following crate features to be activated: `SvgfeSpotLightElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgfeSpotLightElement {
    obj: SvgElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgfeSpotLightElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgfeSpotLightElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(21u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(70u32);
            inform(69u32);
            inform(83u32);
            inform(112u32);
            inform(111u32);
            inform(116u32);
            inform(76u32);
            inform(105u32);
            inform(103u32);
            inform(104u32);
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
    impl core::ops::Deref for SvgfeSpotLightElement {
        type Target = SvgElement;
        #[inline]
        fn deref(&self) -> &SvgElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgfeSpotLightElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgfeSpotLightElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgfeSpotLightElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgfeSpotLightElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgfeSpotLightElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgfeSpotLightElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgfeSpotLightElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgfeSpotLightElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgfeSpotLightElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgfeSpotLightElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgfeSpotLightElement {
        #[inline]
        fn from(obj: JsValue) -> SvgfeSpotLightElement {
            SvgfeSpotLightElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgfeSpotLightElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgfeSpotLightElement> for SvgfeSpotLightElement {
        #[inline]
        fn as_ref(&self) -> &SvgfeSpotLightElement {
            self
        }
    }
    impl From<SvgfeSpotLightElement> for JsValue {
        #[inline]
        fn from(obj: SvgfeSpotLightElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgfeSpotLightElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGFESpotLightElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGFESpotLightElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGFESpotLightElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgfeSpotLightElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgfeSpotLightElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgfeSpotLightElement> for SvgElement {
    #[inline]
    fn from(obj: SvgfeSpotLightElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgfeSpotLightElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeSpotLightElement> for Element {
    #[inline]
    fn from(obj: SvgfeSpotLightElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgfeSpotLightElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeSpotLightElement> for Node {
    #[inline]
    fn from(obj: SvgfeSpotLightElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgfeSpotLightElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeSpotLightElement> for EventTarget {
    #[inline]
    fn from(obj: SvgfeSpotLightElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgfeSpotLightElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeSpotLightElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgfeSpotLightElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgfeSpotLightElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x_SVGFESpotLightElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeSpotLightElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeSpotLightElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
    #[allow(bad_style)]
    #[doc = "The `x` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    #[allow(clippy::all)]
    pub fn x(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x_SVGFESpotLightElement(
                self_: <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x_SVGFESpotLightElement(
            self_: <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_x_SVGFESpotLightElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y_SVGFESpotLightElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeSpotLightElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeSpotLightElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
    #[allow(bad_style)]
    #[doc = "The `y` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    #[allow(clippy::all)]
    pub fn y(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y_SVGFESpotLightElement(
                self_: <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y_SVGFESpotLightElement(
            self_: <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_y_SVGFESpotLightElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_z_SVGFESpotLightElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeSpotLightElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeSpotLightElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
    #[allow(bad_style)]
    #[doc = "The `z` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/z)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    #[allow(clippy::all)]
    pub fn z(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_z_SVGFESpotLightElement(
                self_: <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_z_SVGFESpotLightElement(
            self_: <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_z_SVGFESpotLightElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_points_at_x_SVGFESpotLightElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeSpotLightElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeSpotLightElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
    #[allow(bad_style)]
    #[doc = "The `pointsAtX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/pointsAtX)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    #[allow(clippy::all)]
    pub fn points_at_x(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_points_at_x_SVGFESpotLightElement(
                self_: <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_points_at_x_SVGFESpotLightElement(
            self_: <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_points_at_x_SVGFESpotLightElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_points_at_y_SVGFESpotLightElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeSpotLightElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeSpotLightElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
    #[allow(bad_style)]
    #[doc = "The `pointsAtY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/pointsAtY)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    #[allow(clippy::all)]
    pub fn points_at_y(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_points_at_y_SVGFESpotLightElement(
                self_: <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_points_at_y_SVGFESpotLightElement(
            self_: <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_points_at_y_SVGFESpotLightElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_points_at_z_SVGFESpotLightElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeSpotLightElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeSpotLightElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
    #[allow(bad_style)]
    #[doc = "The `pointsAtZ` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/pointsAtZ)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    #[allow(clippy::all)]
    pub fn points_at_z(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_points_at_z_SVGFESpotLightElement(
                self_: <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_points_at_z_SVGFESpotLightElement(
            self_: <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_points_at_z_SVGFESpotLightElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_specular_exponent_SVGFESpotLightElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeSpotLightElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeSpotLightElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
    #[allow(bad_style)]
    #[doc = "The `specularExponent` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/specularExponent)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    #[allow(clippy::all)]
    pub fn specular_exponent(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_specular_exponent_SVGFESpotLightElement(
                self_: <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_specular_exponent_SVGFESpotLightElement(
            self_: <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_specular_exponent_SVGFESpotLightElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_limiting_cone_angle_SVGFESpotLightElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeSpotLightElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeSpotLightElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
    #[allow(bad_style)]
    #[doc = "The `limitingConeAngle` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/limitingConeAngle)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    #[allow(clippy::all)]
    pub fn limiting_cone_angle(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeSpotLightElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_limiting_cone_angle_SVGFESpotLightElement(
                self_: <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_limiting_cone_angle_SVGFESpotLightElement(
            self_: <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeSpotLightElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_limiting_cone_angle_SVGFESpotLightElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_2e5304b79388d63a: [u8; 961usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x7F\x03\0\0\0\0\t\0\0\x02\x15SVGFESpotLightElement'__widl_instanceof_SVGFESpotLightElement\0\0\0\0 __widl_f_x_SVGFESpotLightElement\0\0\0\x01\x15SVGFESpotLightElement\x01\0\x01\x01x\x01\x01\x05self_\x01x\0\0\0 __widl_f_y_SVGFESpotLightElement\0\0\0\x01\x15SVGFESpotLightElement\x01\0\x01\x01y\x01\x01\x05self_\x01y\0\0\0 __widl_f_z_SVGFESpotLightElement\0\0\0\x01\x15SVGFESpotLightElement\x01\0\x01\x01z\x01\x01\x05self_\x01z\0\0\0*__widl_f_points_at_x_SVGFESpotLightElement\0\0\0\x01\x15SVGFESpotLightElement\x01\0\x01\tpointsAtX\x01\x01\x05self_\tpointsAtX\0\0\0*__widl_f_points_at_y_SVGFESpotLightElement\0\0\0\x01\x15SVGFESpotLightElement\x01\0\x01\tpointsAtY\x01\x01\x05self_\tpointsAtY\0\0\0*__widl_f_points_at_z_SVGFESpotLightElement\0\0\0\x01\x15SVGFESpotLightElement\x01\0\x01\tpointsAtZ\x01\x01\x05self_\tpointsAtZ\0\0\00__widl_f_specular_exponent_SVGFESpotLightElement\0\0\0\x01\x15SVGFESpotLightElement\x01\0\x01\x10specularExponent\x01\x01\x05self_\x10specularExponent\0\0\02__widl_f_limiting_cone_angle_SVGFESpotLightElement\0\0\0\x01\x15SVGFESpotLightElement\x01\0\x01\x11limitingConeAngle\x01\x01\x05self_\x11limitingConeAngle\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
