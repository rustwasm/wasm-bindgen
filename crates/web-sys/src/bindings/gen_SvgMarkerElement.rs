use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGMarkerElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement)\n\n*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgMarkerElement {
    obj: SvgElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgMarkerElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgMarkerElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(77u32);
            inform(97u32);
            inform(114u32);
            inform(107u32);
            inform(101u32);
            inform(114u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgMarkerElement {
        type Target = SvgElement;
        #[inline]
        fn deref(&self) -> &SvgElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgMarkerElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgMarkerElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgMarkerElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgMarkerElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgMarkerElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgMarkerElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgMarkerElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgMarkerElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgMarkerElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgMarkerElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgMarkerElement {
        #[inline]
        fn from(obj: JsValue) -> SvgMarkerElement {
            SvgMarkerElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgMarkerElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgMarkerElement> for SvgMarkerElement {
        #[inline]
        fn as_ref(&self) -> &SvgMarkerElement {
            self
        }
    }
    impl From<SvgMarkerElement> for JsValue {
        #[inline]
        fn from(obj: SvgMarkerElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgMarkerElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGMarkerElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGMarkerElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGMarkerElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgMarkerElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgMarkerElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgMarkerElement> for SvgElement {
    #[inline]
    fn from(obj: SvgMarkerElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgMarkerElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgMarkerElement> for Element {
    #[inline]
    fn from(obj: SvgMarkerElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgMarkerElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgMarkerElement> for Node {
    #[inline]
    fn from(obj: SvgMarkerElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgMarkerElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgMarkerElement> for EventTarget {
    #[inline]
    fn from(obj: SvgMarkerElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgMarkerElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgMarkerElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgMarkerElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgMarkerElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAngle", feature = "SvgMarkerElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_orient_to_angle_SVGMarkerElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgMarkerElement as WasmDescribe>::describe();
    <&SvgAngle as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgMarkerElement {
    #[cfg(all(feature = "SvgAngle", feature = "SvgMarkerElement",))]
    #[allow(bad_style)]
    #[doc = "The `setOrientToAngle()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/setOrientToAngle)\n\n*This API requires the following crate features to be activated: `SvgAngle`, `SvgMarkerElement`*"]
    #[allow(clippy::all)]
    pub fn set_orient_to_angle(&self, angle: &SvgAngle) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgAngle", feature = "SvgMarkerElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_orient_to_angle_SVGMarkerElement(
                self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angle: <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_orient_to_angle_SVGMarkerElement(
            self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            angle: <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(angle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let angle = <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(angle);
                __widl_f_set_orient_to_angle_SVGMarkerElement(self_, angle)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SvgMarkerElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_orient_to_auto_SVGMarkerElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgMarkerElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgMarkerElement {
    #[cfg(all(feature = "SvgMarkerElement",))]
    #[allow(bad_style)]
    #[doc = "The `setOrientToAuto()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/setOrientToAuto)\n\n*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
    #[allow(clippy::all)]
    pub fn set_orient_to_auto(&self) {
        #[cfg(all(feature = "SvgMarkerElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_orient_to_auto_SVGMarkerElement(
                self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_orient_to_auto_SVGMarkerElement(
            self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_set_orient_to_auto_SVGMarkerElement(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgMarkerElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ref_x_SVGMarkerElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgMarkerElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgMarkerElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgMarkerElement",))]
    #[allow(bad_style)]
    #[doc = "The `refX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/refX)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*"]
    #[allow(clippy::all)]
    pub fn ref_x(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgMarkerElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ref_x_SVGMarkerElement(
                self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ref_x_SVGMarkerElement(
            self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ref_x_SVGMarkerElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgMarkerElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ref_y_SVGMarkerElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgMarkerElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgMarkerElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgMarkerElement",))]
    #[allow(bad_style)]
    #[doc = "The `refY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/refY)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*"]
    #[allow(clippy::all)]
    pub fn ref_y(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgMarkerElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ref_y_SVGMarkerElement(
                self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ref_y_SVGMarkerElement(
            self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ref_y_SVGMarkerElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgMarkerElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_marker_units_SVGMarkerElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgMarkerElement as WasmDescribe>::describe();
    <SvgAnimatedEnumeration as WasmDescribe>::describe();
}
impl SvgMarkerElement {
    #[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgMarkerElement",))]
    #[allow(bad_style)]
    #[doc = "The `markerUnits` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/markerUnits)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgMarkerElement`*"]
    #[allow(clippy::all)]
    pub fn marker_units(&self) -> SvgAnimatedEnumeration {
        #[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgMarkerElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_marker_units_SVGMarkerElement(
                self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_marker_units_SVGMarkerElement(
            self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_marker_units_SVGMarkerElement(self_)
            };
            <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgMarkerElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_marker_width_SVGMarkerElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgMarkerElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgMarkerElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgMarkerElement",))]
    #[allow(bad_style)]
    #[doc = "The `markerWidth` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/markerWidth)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*"]
    #[allow(clippy::all)]
    pub fn marker_width(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgMarkerElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_marker_width_SVGMarkerElement(
                self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_marker_width_SVGMarkerElement(
            self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_marker_width_SVGMarkerElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgMarkerElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_marker_height_SVGMarkerElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgMarkerElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgMarkerElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgMarkerElement",))]
    #[allow(bad_style)]
    #[doc = "The `markerHeight` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/markerHeight)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*"]
    #[allow(clippy::all)]
    pub fn marker_height(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgMarkerElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_marker_height_SVGMarkerElement(
                self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_marker_height_SVGMarkerElement(
            self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_marker_height_SVGMarkerElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgMarkerElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_orient_type_SVGMarkerElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgMarkerElement as WasmDescribe>::describe();
    <SvgAnimatedEnumeration as WasmDescribe>::describe();
}
impl SvgMarkerElement {
    #[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgMarkerElement",))]
    #[allow(bad_style)]
    #[doc = "The `orientType` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/orientType)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgMarkerElement`*"]
    #[allow(clippy::all)]
    pub fn orient_type(&self) -> SvgAnimatedEnumeration {
        #[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgMarkerElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_orient_type_SVGMarkerElement(
                self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_orient_type_SVGMarkerElement(
            self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_orient_type_SVGMarkerElement(self_)
            };
            <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedAngle", feature = "SvgMarkerElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_orient_angle_SVGMarkerElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgMarkerElement as WasmDescribe>::describe();
    <SvgAnimatedAngle as WasmDescribe>::describe();
}
impl SvgMarkerElement {
    #[cfg(all(feature = "SvgAnimatedAngle", feature = "SvgMarkerElement",))]
    #[allow(bad_style)]
    #[doc = "The `orientAngle` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/orientAngle)\n\n*This API requires the following crate features to be activated: `SvgAnimatedAngle`, `SvgMarkerElement`*"]
    #[allow(clippy::all)]
    pub fn orient_angle(&self) -> SvgAnimatedAngle {
        #[cfg(all(feature = "SvgAnimatedAngle", feature = "SvgMarkerElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_orient_angle_SVGMarkerElement(
                self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedAngle as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_orient_angle_SVGMarkerElement(
            self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedAngle as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_orient_angle_SVGMarkerElement(self_)
            };
            <SvgAnimatedAngle as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedRect", feature = "SvgMarkerElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_view_box_SVGMarkerElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgMarkerElement as WasmDescribe>::describe();
    <SvgAnimatedRect as WasmDescribe>::describe();
}
impl SvgMarkerElement {
    #[cfg(all(feature = "SvgAnimatedRect", feature = "SvgMarkerElement",))]
    #[allow(bad_style)]
    #[doc = "The `viewBox` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/viewBox)\n\n*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgMarkerElement`*"]
    #[allow(clippy::all)]
    pub fn view_box(&self) -> SvgAnimatedRect {
        #[cfg(all(feature = "SvgAnimatedRect", feature = "SvgMarkerElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_view_box_SVGMarkerElement(
                self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedRect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_view_box_SVGMarkerElement(
            self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedRect as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_view_box_SVGMarkerElement(self_)
            };
            <SvgAnimatedRect as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedPreserveAspectRatio",
    feature = "SvgMarkerElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_preserve_aspect_ratio_SVGMarkerElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgMarkerElement as WasmDescribe>::describe();
    <SvgAnimatedPreserveAspectRatio as WasmDescribe>::describe();
}
impl SvgMarkerElement {
    #[cfg(all(
        feature = "SvgAnimatedPreserveAspectRatio",
        feature = "SvgMarkerElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `preserveAspectRatio` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/preserveAspectRatio)\n\n*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgMarkerElement`*"]
    #[allow(clippy::all)]
    pub fn preserve_aspect_ratio(&self) -> SvgAnimatedPreserveAspectRatio {
        #[cfg(all(
            feature = "SvgAnimatedPreserveAspectRatio",
            feature = "SvgMarkerElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_preserve_aspect_ratio_SVGMarkerElement(
                self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedPreserveAspectRatio as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_preserve_aspect_ratio_SVGMarkerElement(
            self_: <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedPreserveAspectRatio as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgMarkerElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_preserve_aspect_ratio_SVGMarkerElement(self_)
            };
            <SvgAnimatedPreserveAspectRatio as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl SvgMarkerElement {
    pub const SVG_MARKERUNITS_UNKNOWN: u16 = 0i64 as u16;
}
impl SvgMarkerElement {
    pub const SVG_MARKERUNITS_USERSPACEONUSE: u16 = 1u64 as u16;
}
impl SvgMarkerElement {
    pub const SVG_MARKERUNITS_STROKEWIDTH: u16 = 2u64 as u16;
}
impl SvgMarkerElement {
    pub const SVG_MARKER_ORIENT_UNKNOWN: u16 = 0i64 as u16;
}
impl SvgMarkerElement {
    pub const SVG_MARKER_ORIENT_AUTO: u16 = 1u64 as u16;
}
impl SvgMarkerElement {
    pub const SVG_MARKER_ORIENT_ANGLE: u16 = 2u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_1f1760dae99b6dfe: [u8; 1214usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}|\x04\0\0\0\0\x0C\0\0\x02\x10SVGMarkerElement\"__widl_instanceof_SVGMarkerElement\0\0\0\0-__widl_f_set_orient_to_angle_SVGMarkerElement\x01\0\0\x01\x10SVGMarkerElement\x01\0\0\x01\x02\x05self_\x05angle\x10setOrientToAngle\0\0\0,__widl_f_set_orient_to_auto_SVGMarkerElement\0\0\0\x01\x10SVGMarkerElement\x01\0\0\x01\x01\x05self_\x0FsetOrientToAuto\0\0\0\x1F__widl_f_ref_x_SVGMarkerElement\0\0\0\x01\x10SVGMarkerElement\x01\0\x01\x04refX\x01\x01\x05self_\x04refX\0\0\0\x1F__widl_f_ref_y_SVGMarkerElement\0\0\0\x01\x10SVGMarkerElement\x01\0\x01\x04refY\x01\x01\x05self_\x04refY\0\0\0&__widl_f_marker_units_SVGMarkerElement\0\0\0\x01\x10SVGMarkerElement\x01\0\x01\x0BmarkerUnits\x01\x01\x05self_\x0BmarkerUnits\0\0\0&__widl_f_marker_width_SVGMarkerElement\0\0\0\x01\x10SVGMarkerElement\x01\0\x01\x0BmarkerWidth\x01\x01\x05self_\x0BmarkerWidth\0\0\0'__widl_f_marker_height_SVGMarkerElement\0\0\0\x01\x10SVGMarkerElement\x01\0\x01\x0CmarkerHeight\x01\x01\x05self_\x0CmarkerHeight\0\0\0%__widl_f_orient_type_SVGMarkerElement\0\0\0\x01\x10SVGMarkerElement\x01\0\x01\norientType\x01\x01\x05self_\norientType\0\0\0&__widl_f_orient_angle_SVGMarkerElement\0\0\0\x01\x10SVGMarkerElement\x01\0\x01\x0BorientAngle\x01\x01\x05self_\x0BorientAngle\0\0\0\"__widl_f_view_box_SVGMarkerElement\0\0\0\x01\x10SVGMarkerElement\x01\0\x01\x07viewBox\x01\x01\x05self_\x07viewBox\0\0\0/__widl_f_preserve_aspect_ratio_SVGMarkerElement\0\0\0\x01\x10SVGMarkerElement\x01\0\x01\x13preserveAspectRatio\x01\x01\x05self_\x13preserveAspectRatio\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
