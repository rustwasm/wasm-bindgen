use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGFEFloodElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFloodElement)\n\n*This API requires the following crate features to be activated: `SvgfeFloodElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgfeFloodElement {
    obj: SvgElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgfeFloodElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgfeFloodElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(70u32);
            inform(69u32);
            inform(70u32);
            inform(108u32);
            inform(111u32);
            inform(111u32);
            inform(100u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgfeFloodElement {
        type Target = SvgElement;
        #[inline]
        fn deref(&self) -> &SvgElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgfeFloodElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgfeFloodElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgfeFloodElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgfeFloodElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgfeFloodElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgfeFloodElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgfeFloodElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgfeFloodElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgfeFloodElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgfeFloodElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgfeFloodElement {
        #[inline]
        fn from(obj: JsValue) -> SvgfeFloodElement {
            SvgfeFloodElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgfeFloodElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgfeFloodElement> for SvgfeFloodElement {
        #[inline]
        fn as_ref(&self) -> &SvgfeFloodElement {
            self
        }
    }
    impl From<SvgfeFloodElement> for JsValue {
        #[inline]
        fn from(obj: SvgfeFloodElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgfeFloodElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGFEFloodElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGFEFloodElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGFEFloodElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgfeFloodElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgfeFloodElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgfeFloodElement> for SvgElement {
    #[inline]
    fn from(obj: SvgfeFloodElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgfeFloodElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeFloodElement> for Element {
    #[inline]
    fn from(obj: SvgfeFloodElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgfeFloodElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeFloodElement> for Node {
    #[inline]
    fn from(obj: SvgfeFloodElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgfeFloodElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeFloodElement> for EventTarget {
    #[inline]
    fn from(obj: SvgfeFloodElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgfeFloodElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeFloodElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgfeFloodElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgfeFloodElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeFloodElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x_SVGFEFloodElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeFloodElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeFloodElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeFloodElement",))]
    #[allow(bad_style)]
    #[doc = "The `x` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFloodElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeFloodElement`*"]
    #[allow(clippy::all)]
    pub fn x(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeFloodElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x_SVGFEFloodElement(
                self_: <&SvgfeFloodElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x_SVGFEFloodElement(
            self_: <&SvgfeFloodElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeFloodElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_x_SVGFEFloodElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeFloodElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y_SVGFEFloodElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeFloodElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeFloodElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeFloodElement",))]
    #[allow(bad_style)]
    #[doc = "The `y` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFloodElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeFloodElement`*"]
    #[allow(clippy::all)]
    pub fn y(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeFloodElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y_SVGFEFloodElement(
                self_: <&SvgfeFloodElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y_SVGFEFloodElement(
            self_: <&SvgfeFloodElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeFloodElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_y_SVGFEFloodElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeFloodElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_SVGFEFloodElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeFloodElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeFloodElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeFloodElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFloodElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeFloodElement`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeFloodElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_SVGFEFloodElement(
                self_: <&SvgfeFloodElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_SVGFEFloodElement(
            self_: <&SvgfeFloodElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeFloodElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_SVGFEFloodElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeFloodElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_SVGFEFloodElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeFloodElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeFloodElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeFloodElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFloodElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeFloodElement`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeFloodElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_SVGFEFloodElement(
                self_: <&SvgfeFloodElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_SVGFEFloodElement(
            self_: <&SvgfeFloodElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeFloodElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_SVGFEFloodElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeFloodElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_result_SVGFEFloodElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeFloodElement as WasmDescribe>::describe();
    <SvgAnimatedString as WasmDescribe>::describe();
}
impl SvgfeFloodElement {
    #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeFloodElement",))]
    #[allow(bad_style)]
    #[doc = "The `result` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFloodElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeFloodElement`*"]
    #[allow(clippy::all)]
    pub fn result(&self) -> SvgAnimatedString {
        #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeFloodElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_result_SVGFEFloodElement(
                self_: <&SvgfeFloodElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_result_SVGFEFloodElement(
            self_: <&SvgfeFloodElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeFloodElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_result_SVGFEFloodElement(self_)
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
pub static __WASM_BINDGEN_GENERATED_0f589a0944b75969: [u8; 550usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xE4\x01\0\0\0\0\x06\0\0\x02\x11SVGFEFloodElement#__widl_instanceof_SVGFEFloodElement\0\0\0\0\x1C__widl_f_x_SVGFEFloodElement\0\0\0\x01\x11SVGFEFloodElement\x01\0\x01\x01x\x01\x01\x05self_\x01x\0\0\0\x1C__widl_f_y_SVGFEFloodElement\0\0\0\x01\x11SVGFEFloodElement\x01\0\x01\x01y\x01\x01\x05self_\x01y\0\0\0 __widl_f_width_SVGFEFloodElement\0\0\0\x01\x11SVGFEFloodElement\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0!__widl_f_height_SVGFEFloodElement\0\0\0\x01\x11SVGFEFloodElement\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0!__widl_f_result_SVGFEFloodElement\0\0\0\x01\x11SVGFEFloodElement\x01\0\x01\x06result\x01\x01\x05self_\x06result\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
