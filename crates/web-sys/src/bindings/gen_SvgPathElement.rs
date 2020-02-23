use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGPathElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement)\n\n*This API requires the following crate features to be activated: `SvgPathElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgPathElement {
    obj: SvgGeometryElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgPathElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgPathElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(80u32);
            inform(97u32);
            inform(116u32);
            inform(104u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgPathElement {
        type Target = SvgGeometryElement;
        #[inline]
        fn deref(&self) -> &SvgGeometryElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgPathElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgPathElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgPathElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgPathElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgPathElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgPathElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgPathElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgPathElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgPathElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgPathElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgPathElement {
        #[inline]
        fn from(obj: JsValue) -> SvgPathElement {
            SvgPathElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgPathElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgPathElement> for SvgPathElement {
        #[inline]
        fn as_ref(&self) -> &SvgPathElement {
            self
        }
    }
    impl From<SvgPathElement> for JsValue {
        #[inline]
        fn from(obj: SvgPathElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgPathElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGPathElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGPathElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGPathElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgPathElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgPathElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgPathElement> for SvgGeometryElement {
    #[inline]
    fn from(obj: SvgPathElement) -> SvgGeometryElement {
        use wasm_bindgen::JsCast;
        SvgGeometryElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgGeometryElement> for SvgPathElement {
    #[inline]
    fn as_ref(&self) -> &SvgGeometryElement {
        use wasm_bindgen::JsCast;
        SvgGeometryElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgPathElement> for SvgGraphicsElement {
    #[inline]
    fn from(obj: SvgPathElement) -> SvgGraphicsElement {
        use wasm_bindgen::JsCast;
        SvgGraphicsElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgGraphicsElement> for SvgPathElement {
    #[inline]
    fn as_ref(&self) -> &SvgGraphicsElement {
        use wasm_bindgen::JsCast;
        SvgGraphicsElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgPathElement> for SvgElement {
    #[inline]
    fn from(obj: SvgPathElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgPathElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgPathElement> for Element {
    #[inline]
    fn from(obj: SvgPathElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgPathElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgPathElement> for Node {
    #[inline]
    fn from(obj: SvgPathElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgPathElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgPathElement> for EventTarget {
    #[inline]
    fn from(obj: SvgPathElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgPathElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgPathElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgPathElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgPathElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgPathElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_path_seg_at_length_SVGPathElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgPathElement as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl SvgPathElement {
    #[cfg(all(feature = "SvgPathElement",))]
    #[allow(bad_style)]
    #[doc = "The `getPathSegAtLength()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement/getPathSegAtLength)\n\n*This API requires the following crate features to be activated: `SvgPathElement`*"]
    #[allow(clippy::all)]
    pub fn get_path_seg_at_length(&self, distance: f32) -> u32 {
        #[cfg(all(feature = "SvgPathElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_path_seg_at_length_SVGPathElement(
                self_: <&SvgPathElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                distance: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_path_seg_at_length_SVGPathElement(
            self_: <&SvgPathElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            distance: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(distance);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgPathElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let distance = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(distance);
                __widl_f_get_path_seg_at_length_SVGPathElement(self_, distance)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPathElement", feature = "SvgPathSegList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_path_seg_list_SVGPathElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPathElement as WasmDescribe>::describe();
    <SvgPathSegList as WasmDescribe>::describe();
}
impl SvgPathElement {
    #[cfg(all(feature = "SvgPathElement", feature = "SvgPathSegList",))]
    #[allow(bad_style)]
    #[doc = "The `pathSegList` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement/pathSegList)\n\n*This API requires the following crate features to be activated: `SvgPathElement`, `SvgPathSegList`*"]
    #[allow(clippy::all)]
    pub fn path_seg_list(&self) -> SvgPathSegList {
        #[cfg(all(feature = "SvgPathElement", feature = "SvgPathSegList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_path_seg_list_SVGPathElement(
                self_: <&SvgPathElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgPathSegList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_path_seg_list_SVGPathElement(
            self_: <&SvgPathElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgPathSegList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgPathElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_path_seg_list_SVGPathElement(self_)
            };
            <SvgPathSegList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPathElement", feature = "SvgPathSegList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_animated_path_seg_list_SVGPathElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgPathElement as WasmDescribe>::describe();
    <SvgPathSegList as WasmDescribe>::describe();
}
impl SvgPathElement {
    #[cfg(all(feature = "SvgPathElement", feature = "SvgPathSegList",))]
    #[allow(bad_style)]
    #[doc = "The `animatedPathSegList` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement/animatedPathSegList)\n\n*This API requires the following crate features to be activated: `SvgPathElement`, `SvgPathSegList`*"]
    #[allow(clippy::all)]
    pub fn animated_path_seg_list(&self) -> SvgPathSegList {
        #[cfg(all(feature = "SvgPathElement", feature = "SvgPathSegList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_animated_path_seg_list_SVGPathElement(
                self_: <&SvgPathElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgPathSegList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_animated_path_seg_list_SVGPathElement(
            self_: <&SvgPathElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgPathSegList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgPathElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_animated_path_seg_list_SVGPathElement(self_)
            };
            <SvgPathSegList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_b7c809fdab290032: [u8; 480usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x9E\x01\0\0\0\0\x04\0\0\x02\x0ESVGPathElement __widl_instanceof_SVGPathElement\0\0\0\0.__widl_f_get_path_seg_at_length_SVGPathElement\0\0\0\x01\x0ESVGPathElement\x01\0\0\x01\x02\x05self_\x08distance\x12getPathSegAtLength\0\0\0%__widl_f_path_seg_list_SVGPathElement\0\0\0\x01\x0ESVGPathElement\x01\0\x01\x0BpathSegList\x01\x01\x05self_\x0BpathSegList\0\0\0.__widl_f_animated_path_seg_list_SVGPathElement\0\0\0\x01\x0ESVGPathElement\x01\0\x01\x13animatedPathSegList\x01\x01\x05self_\x13animatedPathSegList\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
