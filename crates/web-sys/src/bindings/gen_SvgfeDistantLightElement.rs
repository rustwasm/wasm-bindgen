use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGFEDistantLightElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDistantLightElement)\n\n*This API requires the following crate features to be activated: `SvgfeDistantLightElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgfeDistantLightElement {
    obj: SvgElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgfeDistantLightElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgfeDistantLightElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(24u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(70u32);
            inform(69u32);
            inform(68u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
            inform(97u32);
            inform(110u32);
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
    impl core::ops::Deref for SvgfeDistantLightElement {
        type Target = SvgElement;
        #[inline]
        fn deref(&self) -> &SvgElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgfeDistantLightElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgfeDistantLightElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgfeDistantLightElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgfeDistantLightElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgfeDistantLightElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgfeDistantLightElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgfeDistantLightElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgfeDistantLightElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgfeDistantLightElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgfeDistantLightElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgfeDistantLightElement {
        #[inline]
        fn from(obj: JsValue) -> SvgfeDistantLightElement {
            SvgfeDistantLightElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgfeDistantLightElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgfeDistantLightElement> for SvgfeDistantLightElement {
        #[inline]
        fn as_ref(&self) -> &SvgfeDistantLightElement {
            self
        }
    }
    impl From<SvgfeDistantLightElement> for JsValue {
        #[inline]
        fn from(obj: SvgfeDistantLightElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgfeDistantLightElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGFEDistantLightElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGFEDistantLightElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGFEDistantLightElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgfeDistantLightElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgfeDistantLightElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgfeDistantLightElement> for SvgElement {
    #[inline]
    fn from(obj: SvgfeDistantLightElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgfeDistantLightElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeDistantLightElement> for Element {
    #[inline]
    fn from(obj: SvgfeDistantLightElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgfeDistantLightElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeDistantLightElement> for Node {
    #[inline]
    fn from(obj: SvgfeDistantLightElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgfeDistantLightElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeDistantLightElement> for EventTarget {
    #[inline]
    fn from(obj: SvgfeDistantLightElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgfeDistantLightElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeDistantLightElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgfeDistantLightElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgfeDistantLightElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeDistantLightElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_azimuth_SVGFEDistantLightElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeDistantLightElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeDistantLightElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeDistantLightElement",))]
    #[allow(bad_style)]
    #[doc = "The `azimuth` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDistantLightElement/azimuth)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDistantLightElement`*"]
    #[allow(clippy::all)]
    pub fn azimuth(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeDistantLightElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_azimuth_SVGFEDistantLightElement(
                self_: <&SvgfeDistantLightElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_azimuth_SVGFEDistantLightElement(
            self_: <&SvgfeDistantLightElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeDistantLightElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_azimuth_SVGFEDistantLightElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeDistantLightElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_elevation_SVGFEDistantLightElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeDistantLightElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeDistantLightElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeDistantLightElement",))]
    #[allow(bad_style)]
    #[doc = "The `elevation` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDistantLightElement/elevation)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDistantLightElement`*"]
    #[allow(clippy::all)]
    pub fn elevation(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeDistantLightElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_elevation_SVGFEDistantLightElement(
                self_: <&SvgfeDistantLightElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_elevation_SVGFEDistantLightElement(
            self_: <&SvgfeDistantLightElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeDistantLightElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_elevation_SVGFEDistantLightElement(self_)
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
pub static __WASM_BINDGEN_GENERATED_216c8d1a617cf238: [u8; 385usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}?\x01\0\0\0\0\x03\0\0\x02\x18SVGFEDistantLightElement*__widl_instanceof_SVGFEDistantLightElement\0\0\0\0)__widl_f_azimuth_SVGFEDistantLightElement\0\0\0\x01\x18SVGFEDistantLightElement\x01\0\x01\x07azimuth\x01\x01\x05self_\x07azimuth\0\0\0+__widl_f_elevation_SVGFEDistantLightElement\0\0\0\x01\x18SVGFEDistantLightElement\x01\0\x01\televation\x01\x01\x05self_\televation\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
