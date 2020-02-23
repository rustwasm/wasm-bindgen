use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGClipPathElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGClipPathElement)\n\n*This API requires the following crate features to be activated: `SvgClipPathElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgClipPathElement {
    obj: SvgElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgClipPathElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgClipPathElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(18u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(67u32);
            inform(108u32);
            inform(105u32);
            inform(112u32);
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
    impl core::ops::Deref for SvgClipPathElement {
        type Target = SvgElement;
        #[inline]
        fn deref(&self) -> &SvgElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgClipPathElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgClipPathElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgClipPathElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgClipPathElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgClipPathElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgClipPathElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgClipPathElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgClipPathElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgClipPathElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgClipPathElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgClipPathElement {
        #[inline]
        fn from(obj: JsValue) -> SvgClipPathElement {
            SvgClipPathElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgClipPathElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgClipPathElement> for SvgClipPathElement {
        #[inline]
        fn as_ref(&self) -> &SvgClipPathElement {
            self
        }
    }
    impl From<SvgClipPathElement> for JsValue {
        #[inline]
        fn from(obj: SvgClipPathElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgClipPathElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGClipPathElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGClipPathElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGClipPathElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgClipPathElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgClipPathElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgClipPathElement> for SvgElement {
    #[inline]
    fn from(obj: SvgClipPathElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgClipPathElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgClipPathElement> for Element {
    #[inline]
    fn from(obj: SvgClipPathElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgClipPathElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgClipPathElement> for Node {
    #[inline]
    fn from(obj: SvgClipPathElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgClipPathElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgClipPathElement> for EventTarget {
    #[inline]
    fn from(obj: SvgClipPathElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgClipPathElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgClipPathElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgClipPathElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgClipPathElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgClipPathElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clip_path_units_SVGClipPathElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgClipPathElement as WasmDescribe>::describe();
    <SvgAnimatedEnumeration as WasmDescribe>::describe();
}
impl SvgClipPathElement {
    #[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgClipPathElement",))]
    #[allow(bad_style)]
    #[doc = "The `clipPathUnits` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGClipPathElement/clipPathUnits)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgClipPathElement`*"]
    #[allow(clippy::all)]
    pub fn clip_path_units(&self) -> SvgAnimatedEnumeration {
        #[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgClipPathElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clip_path_units_SVGClipPathElement(
                self_: <&SvgClipPathElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clip_path_units_SVGClipPathElement(
            self_: <&SvgClipPathElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgClipPathElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clip_path_units_SVGClipPathElement(self_)
            };
            <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedTransformList", feature = "SvgClipPathElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transform_SVGClipPathElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgClipPathElement as WasmDescribe>::describe();
    <SvgAnimatedTransformList as WasmDescribe>::describe();
}
impl SvgClipPathElement {
    #[cfg(all(feature = "SvgAnimatedTransformList", feature = "SvgClipPathElement",))]
    #[allow(bad_style)]
    #[doc = "The `transform` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGClipPathElement/transform)\n\n*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgClipPathElement`*"]
    #[allow(clippy::all)]
    pub fn transform(&self) -> SvgAnimatedTransformList {
        #[cfg(all(feature = "SvgAnimatedTransformList", feature = "SvgClipPathElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transform_SVGClipPathElement(
                self_: <&SvgClipPathElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedTransformList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transform_SVGClipPathElement(
            self_: <&SvgClipPathElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedTransformList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgClipPathElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_transform_SVGClipPathElement(self_)
            };
            <SvgAnimatedTransformList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_9bdc33427677bbe1: [u8; 369usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}/\x01\0\0\0\0\x03\0\0\x02\x12SVGClipPathElement$__widl_instanceof_SVGClipPathElement\0\0\0\0+__widl_f_clip_path_units_SVGClipPathElement\0\0\0\x01\x12SVGClipPathElement\x01\0\x01\rclipPathUnits\x01\x01\x05self_\rclipPathUnits\0\0\0%__widl_f_transform_SVGClipPathElement\0\0\0\x01\x12SVGClipPathElement\x01\0\x01\ttransform\x01\x01\x05self_\ttransform\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
