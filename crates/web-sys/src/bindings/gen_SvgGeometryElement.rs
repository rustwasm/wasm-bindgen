use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGGeometryElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement)\n\n*This API requires the following crate features to be activated: `SvgGeometryElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgGeometryElement {
    obj: SvgGraphicsElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgGeometryElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgGeometryElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(18u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(71u32);
            inform(101u32);
            inform(111u32);
            inform(109u32);
            inform(101u32);
            inform(116u32);
            inform(114u32);
            inform(121u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgGeometryElement {
        type Target = SvgGraphicsElement;
        #[inline]
        fn deref(&self) -> &SvgGraphicsElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgGeometryElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgGeometryElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgGeometryElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgGeometryElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgGeometryElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgGeometryElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgGeometryElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgGeometryElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgGeometryElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgGeometryElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgGeometryElement {
        #[inline]
        fn from(obj: JsValue) -> SvgGeometryElement {
            SvgGeometryElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgGeometryElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgGeometryElement> for SvgGeometryElement {
        #[inline]
        fn as_ref(&self) -> &SvgGeometryElement {
            self
        }
    }
    impl From<SvgGeometryElement> for JsValue {
        #[inline]
        fn from(obj: SvgGeometryElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgGeometryElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGGeometryElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGGeometryElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGGeometryElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgGeometryElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgGeometryElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgGeometryElement> for SvgGraphicsElement {
    #[inline]
    fn from(obj: SvgGeometryElement) -> SvgGraphicsElement {
        use wasm_bindgen::JsCast;
        SvgGraphicsElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgGraphicsElement> for SvgGeometryElement {
    #[inline]
    fn as_ref(&self) -> &SvgGraphicsElement {
        use wasm_bindgen::JsCast;
        SvgGraphicsElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgGeometryElement> for SvgElement {
    #[inline]
    fn from(obj: SvgGeometryElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgGeometryElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgGeometryElement> for Element {
    #[inline]
    fn from(obj: SvgGeometryElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgGeometryElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgGeometryElement> for Node {
    #[inline]
    fn from(obj: SvgGeometryElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgGeometryElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgGeometryElement> for EventTarget {
    #[inline]
    fn from(obj: SvgGeometryElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgGeometryElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgGeometryElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgGeometryElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgGeometryElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgGeometryElement", feature = "SvgPoint",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_point_at_length_SVGGeometryElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgGeometryElement as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <SvgPoint as WasmDescribe>::describe();
}
impl SvgGeometryElement {
    #[cfg(all(feature = "SvgGeometryElement", feature = "SvgPoint",))]
    #[allow(bad_style)]
    #[doc = "The `getPointAtLength()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/getPointAtLength)\n\n*This API requires the following crate features to be activated: `SvgGeometryElement`, `SvgPoint`*"]
    #[allow(clippy::all)]
    pub fn get_point_at_length(&self, distance: f32) -> Result<SvgPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgGeometryElement", feature = "SvgPoint",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_point_at_length_SVGGeometryElement(
                self_: <&SvgGeometryElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                distance: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_point_at_length_SVGGeometryElement(
            self_: <&SvgGeometryElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            distance: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(distance);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgGeometryElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let distance = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(distance);
                __widl_f_get_point_at_length_SVGGeometryElement(self_, distance)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "SvgGeometryElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_total_length_SVGGeometryElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgGeometryElement as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgGeometryElement {
    #[cfg(all(feature = "SvgGeometryElement",))]
    #[allow(bad_style)]
    #[doc = "The `getTotalLength()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/getTotalLength)\n\n*This API requires the following crate features to be activated: `SvgGeometryElement`*"]
    #[allow(clippy::all)]
    pub fn get_total_length(&self) -> f32 {
        #[cfg(all(feature = "SvgGeometryElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_total_length_SVGGeometryElement(
                self_: <&SvgGeometryElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_total_length_SVGGeometryElement(
            self_: <&SvgGeometryElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgGeometryElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_total_length_SVGGeometryElement(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgGeometryElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_path_length_SVGGeometryElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgGeometryElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgGeometryElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgGeometryElement",))]
    #[allow(bad_style)]
    #[doc = "The `pathLength` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/pathLength)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgGeometryElement`*"]
    #[allow(clippy::all)]
    pub fn path_length(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgGeometryElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_path_length_SVGGeometryElement(
                self_: <&SvgGeometryElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_path_length_SVGGeometryElement(
            self_: <&SvgGeometryElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgGeometryElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_path_length_SVGGeometryElement(self_)
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
pub static __WASM_BINDGEN_GENERATED_6addc282b13a1114: [u8; 472usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x96\x01\0\0\0\0\x04\0\0\x02\x12SVGGeometryElement$__widl_instanceof_SVGGeometryElement\0\0\0\0/__widl_f_get_point_at_length_SVGGeometryElement\x01\0\0\x01\x12SVGGeometryElement\x01\0\0\x01\x02\x05self_\x08distance\x10getPointAtLength\0\0\0,__widl_f_get_total_length_SVGGeometryElement\0\0\0\x01\x12SVGGeometryElement\x01\0\0\x01\x01\x05self_\x0EgetTotalLength\0\0\0'__widl_f_path_length_SVGGeometryElement\0\0\0\x01\x12SVGGeometryElement\x01\0\x01\npathLength\x01\x01\x05self_\npathLength\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
