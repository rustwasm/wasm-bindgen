use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGLinearGradientElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLinearGradientElement)\n\n*This API requires the following crate features to be activated: `SvgLinearGradientElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgLinearGradientElement {
    obj: SvgGradientElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgLinearGradientElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgLinearGradientElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(24u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(76u32);
            inform(105u32);
            inform(110u32);
            inform(101u32);
            inform(97u32);
            inform(114u32);
            inform(71u32);
            inform(114u32);
            inform(97u32);
            inform(100u32);
            inform(105u32);
            inform(101u32);
            inform(110u32);
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
    impl core::ops::Deref for SvgLinearGradientElement {
        type Target = SvgGradientElement;
        #[inline]
        fn deref(&self) -> &SvgGradientElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgLinearGradientElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgLinearGradientElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgLinearGradientElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgLinearGradientElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgLinearGradientElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgLinearGradientElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgLinearGradientElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgLinearGradientElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgLinearGradientElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgLinearGradientElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgLinearGradientElement {
        #[inline]
        fn from(obj: JsValue) -> SvgLinearGradientElement {
            SvgLinearGradientElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgLinearGradientElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgLinearGradientElement> for SvgLinearGradientElement {
        #[inline]
        fn as_ref(&self) -> &SvgLinearGradientElement {
            self
        }
    }
    impl From<SvgLinearGradientElement> for JsValue {
        #[inline]
        fn from(obj: SvgLinearGradientElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgLinearGradientElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGLinearGradientElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGLinearGradientElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGLinearGradientElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgLinearGradientElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgLinearGradientElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgLinearGradientElement> for SvgGradientElement {
    #[inline]
    fn from(obj: SvgLinearGradientElement) -> SvgGradientElement {
        use wasm_bindgen::JsCast;
        SvgGradientElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgGradientElement> for SvgLinearGradientElement {
    #[inline]
    fn as_ref(&self) -> &SvgGradientElement {
        use wasm_bindgen::JsCast;
        SvgGradientElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgLinearGradientElement> for SvgElement {
    #[inline]
    fn from(obj: SvgLinearGradientElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgLinearGradientElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgLinearGradientElement> for Element {
    #[inline]
    fn from(obj: SvgLinearGradientElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgLinearGradientElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgLinearGradientElement> for Node {
    #[inline]
    fn from(obj: SvgLinearGradientElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgLinearGradientElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgLinearGradientElement> for EventTarget {
    #[inline]
    fn from(obj: SvgLinearGradientElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgLinearGradientElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgLinearGradientElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgLinearGradientElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgLinearGradientElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgLinearGradientElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x1_SVGLinearGradientElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgLinearGradientElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgLinearGradientElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgLinearGradientElement",))]
    #[allow(bad_style)]
    #[doc = "The `x1` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLinearGradientElement/x1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLinearGradientElement`*"]
    #[allow(clippy::all)]
    pub fn x1(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgLinearGradientElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x1_SVGLinearGradientElement(
                self_: <&SvgLinearGradientElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x1_SVGLinearGradientElement(
            self_: <&SvgLinearGradientElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgLinearGradientElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_x1_SVGLinearGradientElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgLinearGradientElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y1_SVGLinearGradientElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgLinearGradientElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgLinearGradientElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgLinearGradientElement",))]
    #[allow(bad_style)]
    #[doc = "The `y1` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLinearGradientElement/y1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLinearGradientElement`*"]
    #[allow(clippy::all)]
    pub fn y1(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgLinearGradientElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y1_SVGLinearGradientElement(
                self_: <&SvgLinearGradientElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y1_SVGLinearGradientElement(
            self_: <&SvgLinearGradientElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgLinearGradientElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_y1_SVGLinearGradientElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgLinearGradientElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x2_SVGLinearGradientElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgLinearGradientElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgLinearGradientElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgLinearGradientElement",))]
    #[allow(bad_style)]
    #[doc = "The `x2` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLinearGradientElement/x2)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLinearGradientElement`*"]
    #[allow(clippy::all)]
    pub fn x2(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgLinearGradientElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x2_SVGLinearGradientElement(
                self_: <&SvgLinearGradientElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x2_SVGLinearGradientElement(
            self_: <&SvgLinearGradientElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgLinearGradientElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_x2_SVGLinearGradientElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgLinearGradientElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y2_SVGLinearGradientElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgLinearGradientElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgLinearGradientElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgLinearGradientElement",))]
    #[allow(bad_style)]
    #[doc = "The `y2` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLinearGradientElement/y2)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLinearGradientElement`*"]
    #[allow(clippy::all)]
    pub fn y2(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgLinearGradientElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y2_SVGLinearGradientElement(
                self_: <&SvgLinearGradientElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y2_SVGLinearGradientElement(
            self_: <&SvgLinearGradientElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgLinearGradientElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_y2_SVGLinearGradientElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_4a49e3def79599fb: [u8; 521usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xC7\x01\0\0\0\0\x05\0\0\x02\x18SVGLinearGradientElement*__widl_instanceof_SVGLinearGradientElement\0\0\0\0$__widl_f_x1_SVGLinearGradientElement\0\0\0\x01\x18SVGLinearGradientElement\x01\0\x01\x02x1\x01\x01\x05self_\x02x1\0\0\0$__widl_f_y1_SVGLinearGradientElement\0\0\0\x01\x18SVGLinearGradientElement\x01\0\x01\x02y1\x01\x01\x05self_\x02y1\0\0\0$__widl_f_x2_SVGLinearGradientElement\0\0\0\x01\x18SVGLinearGradientElement\x01\0\x01\x02x2\x01\x01\x05self_\x02x2\0\0\0$__widl_f_y2_SVGLinearGradientElement\0\0\0\x01\x18SVGLinearGradientElement\x01\0\x01\x02y2\x01\x01\x05self_\x02y2\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
