use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGFESpecularLightingElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement)\n\n*This API requires the following crate features to be activated: `SvgfeSpecularLightingElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgfeSpecularLightingElement {
    obj: SvgElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgfeSpecularLightingElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgfeSpecularLightingElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(28u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(70u32);
            inform(69u32);
            inform(83u32);
            inform(112u32);
            inform(101u32);
            inform(99u32);
            inform(117u32);
            inform(108u32);
            inform(97u32);
            inform(114u32);
            inform(76u32);
            inform(105u32);
            inform(103u32);
            inform(104u32);
            inform(116u32);
            inform(105u32);
            inform(110u32);
            inform(103u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgfeSpecularLightingElement {
        type Target = SvgElement;
        #[inline]
        fn deref(&self) -> &SvgElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgfeSpecularLightingElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgfeSpecularLightingElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgfeSpecularLightingElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgfeSpecularLightingElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgfeSpecularLightingElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgfeSpecularLightingElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgfeSpecularLightingElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgfeSpecularLightingElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgfeSpecularLightingElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgfeSpecularLightingElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgfeSpecularLightingElement {
        #[inline]
        fn from(obj: JsValue) -> SvgfeSpecularLightingElement {
            SvgfeSpecularLightingElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgfeSpecularLightingElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgfeSpecularLightingElement> for SvgfeSpecularLightingElement {
        #[inline]
        fn as_ref(&self) -> &SvgfeSpecularLightingElement {
            self
        }
    }
    impl From<SvgfeSpecularLightingElement> for JsValue {
        #[inline]
        fn from(obj: SvgfeSpecularLightingElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgfeSpecularLightingElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGFESpecularLightingElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGFESpecularLightingElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGFESpecularLightingElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgfeSpecularLightingElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgfeSpecularLightingElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgfeSpecularLightingElement> for SvgElement {
    #[inline]
    fn from(obj: SvgfeSpecularLightingElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgfeSpecularLightingElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeSpecularLightingElement> for Element {
    #[inline]
    fn from(obj: SvgfeSpecularLightingElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgfeSpecularLightingElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeSpecularLightingElement> for Node {
    #[inline]
    fn from(obj: SvgfeSpecularLightingElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgfeSpecularLightingElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeSpecularLightingElement> for EventTarget {
    #[inline]
    fn from(obj: SvgfeSpecularLightingElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgfeSpecularLightingElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeSpecularLightingElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgfeSpecularLightingElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgfeSpecularLightingElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(
    feature = "SvgAnimatedString",
    feature = "SvgfeSpecularLightingElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_in1_SVGFESpecularLightingElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeSpecularLightingElement as WasmDescribe>::describe();
    <SvgAnimatedString as WasmDescribe>::describe();
}
impl SvgfeSpecularLightingElement {
    #[cfg(all(
        feature = "SvgAnimatedString",
        feature = "SvgfeSpecularLightingElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `in1` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/in1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeSpecularLightingElement`*"]
    #[allow(clippy::all)]
    pub fn in1(&self) -> SvgAnimatedString {
        #[cfg(all(
            feature = "SvgAnimatedString",
            feature = "SvgfeSpecularLightingElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_in1_SVGFESpecularLightingElement(
                self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_in1_SVGFESpecularLightingElement(
            self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_in1_SVGFESpecularLightingElement(self_)
            };
            <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedNumber",
    feature = "SvgfeSpecularLightingElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_surface_scale_SVGFESpecularLightingElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeSpecularLightingElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeSpecularLightingElement {
    #[cfg(all(
        feature = "SvgAnimatedNumber",
        feature = "SvgfeSpecularLightingElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `surfaceScale` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/surfaceScale)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpecularLightingElement`*"]
    #[allow(clippy::all)]
    pub fn surface_scale(&self) -> SvgAnimatedNumber {
        #[cfg(all(
            feature = "SvgAnimatedNumber",
            feature = "SvgfeSpecularLightingElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_surface_scale_SVGFESpecularLightingElement(
                self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_surface_scale_SVGFESpecularLightingElement(
            self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_surface_scale_SVGFESpecularLightingElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedNumber",
    feature = "SvgfeSpecularLightingElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_specular_constant_SVGFESpecularLightingElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeSpecularLightingElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeSpecularLightingElement {
    #[cfg(all(
        feature = "SvgAnimatedNumber",
        feature = "SvgfeSpecularLightingElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `specularConstant` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/specularConstant)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpecularLightingElement`*"]
    #[allow(clippy::all)]
    pub fn specular_constant(&self) -> SvgAnimatedNumber {
        #[cfg(all(
            feature = "SvgAnimatedNumber",
            feature = "SvgfeSpecularLightingElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_specular_constant_SVGFESpecularLightingElement(
                self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_specular_constant_SVGFESpecularLightingElement(
            self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_specular_constant_SVGFESpecularLightingElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedNumber",
    feature = "SvgfeSpecularLightingElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_specular_exponent_SVGFESpecularLightingElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeSpecularLightingElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeSpecularLightingElement {
    #[cfg(all(
        feature = "SvgAnimatedNumber",
        feature = "SvgfeSpecularLightingElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `specularExponent` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/specularExponent)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpecularLightingElement`*"]
    #[allow(clippy::all)]
    pub fn specular_exponent(&self) -> SvgAnimatedNumber {
        #[cfg(all(
            feature = "SvgAnimatedNumber",
            feature = "SvgfeSpecularLightingElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_specular_exponent_SVGFESpecularLightingElement(
                self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_specular_exponent_SVGFESpecularLightingElement(
            self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_specular_exponent_SVGFESpecularLightingElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedNumber",
    feature = "SvgfeSpecularLightingElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_kernel_unit_length_x_SVGFESpecularLightingElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeSpecularLightingElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeSpecularLightingElement {
    #[cfg(all(
        feature = "SvgAnimatedNumber",
        feature = "SvgfeSpecularLightingElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `kernelUnitLengthX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/kernelUnitLengthX)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpecularLightingElement`*"]
    #[allow(clippy::all)]
    pub fn kernel_unit_length_x(&self) -> SvgAnimatedNumber {
        #[cfg(all(
            feature = "SvgAnimatedNumber",
            feature = "SvgfeSpecularLightingElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_kernel_unit_length_x_SVGFESpecularLightingElement(
                self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_kernel_unit_length_x_SVGFESpecularLightingElement(
            self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_kernel_unit_length_x_SVGFESpecularLightingElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedNumber",
    feature = "SvgfeSpecularLightingElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_kernel_unit_length_y_SVGFESpecularLightingElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeSpecularLightingElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeSpecularLightingElement {
    #[cfg(all(
        feature = "SvgAnimatedNumber",
        feature = "SvgfeSpecularLightingElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `kernelUnitLengthY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/kernelUnitLengthY)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpecularLightingElement`*"]
    #[allow(clippy::all)]
    pub fn kernel_unit_length_y(&self) -> SvgAnimatedNumber {
        #[cfg(all(
            feature = "SvgAnimatedNumber",
            feature = "SvgfeSpecularLightingElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_kernel_unit_length_y_SVGFESpecularLightingElement(
                self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_kernel_unit_length_y_SVGFESpecularLightingElement(
            self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_kernel_unit_length_y_SVGFESpecularLightingElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedLength",
    feature = "SvgfeSpecularLightingElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x_SVGFESpecularLightingElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeSpecularLightingElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeSpecularLightingElement {
    #[cfg(all(
        feature = "SvgAnimatedLength",
        feature = "SvgfeSpecularLightingElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `x` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeSpecularLightingElement`*"]
    #[allow(clippy::all)]
    pub fn x(&self) -> SvgAnimatedLength {
        #[cfg(all(
            feature = "SvgAnimatedLength",
            feature = "SvgfeSpecularLightingElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x_SVGFESpecularLightingElement(
                self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x_SVGFESpecularLightingElement(
            self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_x_SVGFESpecularLightingElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedLength",
    feature = "SvgfeSpecularLightingElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y_SVGFESpecularLightingElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeSpecularLightingElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeSpecularLightingElement {
    #[cfg(all(
        feature = "SvgAnimatedLength",
        feature = "SvgfeSpecularLightingElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `y` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeSpecularLightingElement`*"]
    #[allow(clippy::all)]
    pub fn y(&self) -> SvgAnimatedLength {
        #[cfg(all(
            feature = "SvgAnimatedLength",
            feature = "SvgfeSpecularLightingElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y_SVGFESpecularLightingElement(
                self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y_SVGFESpecularLightingElement(
            self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_y_SVGFESpecularLightingElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedLength",
    feature = "SvgfeSpecularLightingElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_SVGFESpecularLightingElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeSpecularLightingElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeSpecularLightingElement {
    #[cfg(all(
        feature = "SvgAnimatedLength",
        feature = "SvgfeSpecularLightingElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeSpecularLightingElement`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> SvgAnimatedLength {
        #[cfg(all(
            feature = "SvgAnimatedLength",
            feature = "SvgfeSpecularLightingElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_SVGFESpecularLightingElement(
                self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_SVGFESpecularLightingElement(
            self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_width_SVGFESpecularLightingElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedLength",
    feature = "SvgfeSpecularLightingElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_SVGFESpecularLightingElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeSpecularLightingElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeSpecularLightingElement {
    #[cfg(all(
        feature = "SvgAnimatedLength",
        feature = "SvgfeSpecularLightingElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeSpecularLightingElement`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> SvgAnimatedLength {
        #[cfg(all(
            feature = "SvgAnimatedLength",
            feature = "SvgfeSpecularLightingElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_SVGFESpecularLightingElement(
                self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_SVGFESpecularLightingElement(
            self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_height_SVGFESpecularLightingElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedString",
    feature = "SvgfeSpecularLightingElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_result_SVGFESpecularLightingElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeSpecularLightingElement as WasmDescribe>::describe();
    <SvgAnimatedString as WasmDescribe>::describe();
}
impl SvgfeSpecularLightingElement {
    #[cfg(all(
        feature = "SvgAnimatedString",
        feature = "SvgfeSpecularLightingElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `result` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeSpecularLightingElement`*"]
    #[allow(clippy::all)]
    pub fn result(&self) -> SvgAnimatedString {
        #[cfg(all(
            feature = "SvgAnimatedString",
            feature = "SvgfeSpecularLightingElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_result_SVGFESpecularLightingElement(
                self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_result_SVGFESpecularLightingElement(
            self_: <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeSpecularLightingElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_result_SVGFESpecularLightingElement(self_)
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
pub static __WASM_BINDGEN_GENERATED_fbbf8fbddbd93a71: [u8; 1462usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}t\x05\0\0\0\0\x0C\0\0\x02\x1CSVGFESpecularLightingElement.__widl_instanceof_SVGFESpecularLightingElement\0\0\0\0)__widl_f_in1_SVGFESpecularLightingElement\0\0\0\x01\x1CSVGFESpecularLightingElement\x01\0\x01\x03in1\x01\x01\x05self_\x03in1\0\0\03__widl_f_surface_scale_SVGFESpecularLightingElement\0\0\0\x01\x1CSVGFESpecularLightingElement\x01\0\x01\x0CsurfaceScale\x01\x01\x05self_\x0CsurfaceScale\0\0\07__widl_f_specular_constant_SVGFESpecularLightingElement\0\0\0\x01\x1CSVGFESpecularLightingElement\x01\0\x01\x10specularConstant\x01\x01\x05self_\x10specularConstant\0\0\07__widl_f_specular_exponent_SVGFESpecularLightingElement\0\0\0\x01\x1CSVGFESpecularLightingElement\x01\0\x01\x10specularExponent\x01\x01\x05self_\x10specularExponent\0\0\0:__widl_f_kernel_unit_length_x_SVGFESpecularLightingElement\0\0\0\x01\x1CSVGFESpecularLightingElement\x01\0\x01\x11kernelUnitLengthX\x01\x01\x05self_\x11kernelUnitLengthX\0\0\0:__widl_f_kernel_unit_length_y_SVGFESpecularLightingElement\0\0\0\x01\x1CSVGFESpecularLightingElement\x01\0\x01\x11kernelUnitLengthY\x01\x01\x05self_\x11kernelUnitLengthY\0\0\0'__widl_f_x_SVGFESpecularLightingElement\0\0\0\x01\x1CSVGFESpecularLightingElement\x01\0\x01\x01x\x01\x01\x05self_\x01x\0\0\0'__widl_f_y_SVGFESpecularLightingElement\0\0\0\x01\x1CSVGFESpecularLightingElement\x01\0\x01\x01y\x01\x01\x05self_\x01y\0\0\0+__widl_f_width_SVGFESpecularLightingElement\0\0\0\x01\x1CSVGFESpecularLightingElement\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0,__widl_f_height_SVGFESpecularLightingElement\0\0\0\x01\x1CSVGFESpecularLightingElement\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0,__widl_f_result_SVGFESpecularLightingElement\0\0\0\x01\x1CSVGFESpecularLightingElement\x01\0\x01\x06result\x01\x01\x05self_\x06result\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
