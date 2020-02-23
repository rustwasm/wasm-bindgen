use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGRadialGradientElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement)\n\n*This API requires the following crate features to be activated: `SvgRadialGradientElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgRadialGradientElement {
    obj: SvgGradientElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgRadialGradientElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgRadialGradientElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(24u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(82u32);
            inform(97u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(108u32);
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
    impl core::ops::Deref for SvgRadialGradientElement {
        type Target = SvgGradientElement;
        #[inline]
        fn deref(&self) -> &SvgGradientElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgRadialGradientElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgRadialGradientElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgRadialGradientElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgRadialGradientElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgRadialGradientElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgRadialGradientElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgRadialGradientElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgRadialGradientElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgRadialGradientElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgRadialGradientElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgRadialGradientElement {
        #[inline]
        fn from(obj: JsValue) -> SvgRadialGradientElement {
            SvgRadialGradientElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgRadialGradientElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgRadialGradientElement> for SvgRadialGradientElement {
        #[inline]
        fn as_ref(&self) -> &SvgRadialGradientElement {
            self
        }
    }
    impl From<SvgRadialGradientElement> for JsValue {
        #[inline]
        fn from(obj: SvgRadialGradientElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgRadialGradientElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGRadialGradientElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGRadialGradientElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGRadialGradientElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgRadialGradientElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgRadialGradientElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgRadialGradientElement> for SvgGradientElement {
    #[inline]
    fn from(obj: SvgRadialGradientElement) -> SvgGradientElement {
        use wasm_bindgen::JsCast;
        SvgGradientElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgGradientElement> for SvgRadialGradientElement {
    #[inline]
    fn as_ref(&self) -> &SvgGradientElement {
        use wasm_bindgen::JsCast;
        SvgGradientElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgRadialGradientElement> for SvgElement {
    #[inline]
    fn from(obj: SvgRadialGradientElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgRadialGradientElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgRadialGradientElement> for Element {
    #[inline]
    fn from(obj: SvgRadialGradientElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgRadialGradientElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgRadialGradientElement> for Node {
    #[inline]
    fn from(obj: SvgRadialGradientElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgRadialGradientElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgRadialGradientElement> for EventTarget {
    #[inline]
    fn from(obj: SvgRadialGradientElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgRadialGradientElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgRadialGradientElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgRadialGradientElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgRadialGradientElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgRadialGradientElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cx_SVGRadialGradientElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgRadialGradientElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgRadialGradientElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgRadialGradientElement",))]
    #[allow(bad_style)]
    #[doc = "The `cx` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/cx)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRadialGradientElement`*"]
    #[allow(clippy::all)]
    pub fn cx(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgRadialGradientElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cx_SVGRadialGradientElement(
                self_: <&SvgRadialGradientElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cx_SVGRadialGradientElement(
            self_: <&SvgRadialGradientElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgRadialGradientElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_cx_SVGRadialGradientElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgRadialGradientElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cy_SVGRadialGradientElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgRadialGradientElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgRadialGradientElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgRadialGradientElement",))]
    #[allow(bad_style)]
    #[doc = "The `cy` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/cy)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRadialGradientElement`*"]
    #[allow(clippy::all)]
    pub fn cy(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgRadialGradientElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cy_SVGRadialGradientElement(
                self_: <&SvgRadialGradientElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cy_SVGRadialGradientElement(
            self_: <&SvgRadialGradientElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgRadialGradientElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_cy_SVGRadialGradientElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgRadialGradientElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_r_SVGRadialGradientElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgRadialGradientElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgRadialGradientElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgRadialGradientElement",))]
    #[allow(bad_style)]
    #[doc = "The `r` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/r)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRadialGradientElement`*"]
    #[allow(clippy::all)]
    pub fn r(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgRadialGradientElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_r_SVGRadialGradientElement(
                self_: <&SvgRadialGradientElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_r_SVGRadialGradientElement(
            self_: <&SvgRadialGradientElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgRadialGradientElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_r_SVGRadialGradientElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgRadialGradientElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fx_SVGRadialGradientElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgRadialGradientElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgRadialGradientElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgRadialGradientElement",))]
    #[allow(bad_style)]
    #[doc = "The `fx` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/fx)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRadialGradientElement`*"]
    #[allow(clippy::all)]
    pub fn fx(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgRadialGradientElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fx_SVGRadialGradientElement(
                self_: <&SvgRadialGradientElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fx_SVGRadialGradientElement(
            self_: <&SvgRadialGradientElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgRadialGradientElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_fx_SVGRadialGradientElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgRadialGradientElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fy_SVGRadialGradientElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgRadialGradientElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgRadialGradientElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgRadialGradientElement",))]
    #[allow(bad_style)]
    #[doc = "The `fy` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/fy)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRadialGradientElement`*"]
    #[allow(clippy::all)]
    pub fn fy(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgRadialGradientElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fy_SVGRadialGradientElement(
                self_: <&SvgRadialGradientElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fy_SVGRadialGradientElement(
            self_: <&SvgRadialGradientElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgRadialGradientElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_fy_SVGRadialGradientElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgRadialGradientElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fr_SVGRadialGradientElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgRadialGradientElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgRadialGradientElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgRadialGradientElement",))]
    #[allow(bad_style)]
    #[doc = "The `fr` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/fr)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRadialGradientElement`*"]
    #[allow(clippy::all)]
    pub fn fr(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgRadialGradientElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fr_SVGRadialGradientElement(
                self_: <&SvgRadialGradientElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fr_SVGRadialGradientElement(
            self_: <&SvgRadialGradientElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgRadialGradientElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_fr_SVGRadialGradientElement(self_)
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
pub static __WASM_BINDGEN_GENERATED_33bdf22a36ccc967: [u8; 690usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}p\x02\0\0\0\0\x07\0\0\x02\x18SVGRadialGradientElement*__widl_instanceof_SVGRadialGradientElement\0\0\0\0$__widl_f_cx_SVGRadialGradientElement\0\0\0\x01\x18SVGRadialGradientElement\x01\0\x01\x02cx\x01\x01\x05self_\x02cx\0\0\0$__widl_f_cy_SVGRadialGradientElement\0\0\0\x01\x18SVGRadialGradientElement\x01\0\x01\x02cy\x01\x01\x05self_\x02cy\0\0\0#__widl_f_r_SVGRadialGradientElement\0\0\0\x01\x18SVGRadialGradientElement\x01\0\x01\x01r\x01\x01\x05self_\x01r\0\0\0$__widl_f_fx_SVGRadialGradientElement\0\0\0\x01\x18SVGRadialGradientElement\x01\0\x01\x02fx\x01\x01\x05self_\x02fx\0\0\0$__widl_f_fy_SVGRadialGradientElement\0\0\0\x01\x18SVGRadialGradientElement\x01\0\x01\x02fy\x01\x01\x05self_\x02fy\0\0\0$__widl_f_fr_SVGRadialGradientElement\0\0\0\x01\x18SVGRadialGradientElement\x01\0\x01\x02fr\x01\x01\x05self_\x02fr\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
