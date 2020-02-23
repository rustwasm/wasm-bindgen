use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGFEDiffuseLightingElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement)\n\n*This API requires the following crate features to be activated: `SvgfeDiffuseLightingElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgfeDiffuseLightingElement {
    obj: SvgElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgfeDiffuseLightingElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgfeDiffuseLightingElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(27u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(70u32);
            inform(69u32);
            inform(68u32);
            inform(105u32);
            inform(102u32);
            inform(102u32);
            inform(117u32);
            inform(115u32);
            inform(101u32);
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
    impl core::ops::Deref for SvgfeDiffuseLightingElement {
        type Target = SvgElement;
        #[inline]
        fn deref(&self) -> &SvgElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgfeDiffuseLightingElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgfeDiffuseLightingElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgfeDiffuseLightingElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgfeDiffuseLightingElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgfeDiffuseLightingElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgfeDiffuseLightingElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgfeDiffuseLightingElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgfeDiffuseLightingElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgfeDiffuseLightingElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgfeDiffuseLightingElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgfeDiffuseLightingElement {
        #[inline]
        fn from(obj: JsValue) -> SvgfeDiffuseLightingElement {
            SvgfeDiffuseLightingElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgfeDiffuseLightingElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgfeDiffuseLightingElement> for SvgfeDiffuseLightingElement {
        #[inline]
        fn as_ref(&self) -> &SvgfeDiffuseLightingElement {
            self
        }
    }
    impl From<SvgfeDiffuseLightingElement> for JsValue {
        #[inline]
        fn from(obj: SvgfeDiffuseLightingElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgfeDiffuseLightingElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGFEDiffuseLightingElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGFEDiffuseLightingElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGFEDiffuseLightingElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgfeDiffuseLightingElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgfeDiffuseLightingElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgfeDiffuseLightingElement> for SvgElement {
    #[inline]
    fn from(obj: SvgfeDiffuseLightingElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgfeDiffuseLightingElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeDiffuseLightingElement> for Element {
    #[inline]
    fn from(obj: SvgfeDiffuseLightingElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgfeDiffuseLightingElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeDiffuseLightingElement> for Node {
    #[inline]
    fn from(obj: SvgfeDiffuseLightingElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgfeDiffuseLightingElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeDiffuseLightingElement> for EventTarget {
    #[inline]
    fn from(obj: SvgfeDiffuseLightingElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgfeDiffuseLightingElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeDiffuseLightingElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgfeDiffuseLightingElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgfeDiffuseLightingElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeDiffuseLightingElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_in1_SVGFEDiffuseLightingElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeDiffuseLightingElement as WasmDescribe>::describe();
    <SvgAnimatedString as WasmDescribe>::describe();
}
impl SvgfeDiffuseLightingElement {
    #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeDiffuseLightingElement",))]
    #[allow(bad_style)]
    #[doc = "The `in1` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/in1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDiffuseLightingElement`*"]
    #[allow(clippy::all)]
    pub fn in1(&self) -> SvgAnimatedString {
        #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeDiffuseLightingElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_in1_SVGFEDiffuseLightingElement(
                self_: <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_in1_SVGFEDiffuseLightingElement(
            self_: <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_in1_SVGFEDiffuseLightingElement(self_)
            };
            <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeDiffuseLightingElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_surface_scale_SVGFEDiffuseLightingElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeDiffuseLightingElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeDiffuseLightingElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeDiffuseLightingElement",))]
    #[allow(bad_style)]
    #[doc = "The `surfaceScale` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/surfaceScale)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDiffuseLightingElement`*"]
    #[allow(clippy::all)]
    pub fn surface_scale(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeDiffuseLightingElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_surface_scale_SVGFEDiffuseLightingElement(
                self_: <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_surface_scale_SVGFEDiffuseLightingElement(
            self_: <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_surface_scale_SVGFEDiffuseLightingElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeDiffuseLightingElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_diffuse_constant_SVGFEDiffuseLightingElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeDiffuseLightingElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeDiffuseLightingElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeDiffuseLightingElement",))]
    #[allow(bad_style)]
    #[doc = "The `diffuseConstant` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/diffuseConstant)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDiffuseLightingElement`*"]
    #[allow(clippy::all)]
    pub fn diffuse_constant(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeDiffuseLightingElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_diffuse_constant_SVGFEDiffuseLightingElement(
                self_: <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_diffuse_constant_SVGFEDiffuseLightingElement(
            self_: <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_diffuse_constant_SVGFEDiffuseLightingElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeDiffuseLightingElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_kernel_unit_length_x_SVGFEDiffuseLightingElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeDiffuseLightingElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeDiffuseLightingElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeDiffuseLightingElement",))]
    #[allow(bad_style)]
    #[doc = "The `kernelUnitLengthX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/kernelUnitLengthX)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDiffuseLightingElement`*"]
    #[allow(clippy::all)]
    pub fn kernel_unit_length_x(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeDiffuseLightingElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_kernel_unit_length_x_SVGFEDiffuseLightingElement(
                self_: <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_kernel_unit_length_x_SVGFEDiffuseLightingElement(
            self_: <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_kernel_unit_length_x_SVGFEDiffuseLightingElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeDiffuseLightingElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_kernel_unit_length_y_SVGFEDiffuseLightingElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeDiffuseLightingElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeDiffuseLightingElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeDiffuseLightingElement",))]
    #[allow(bad_style)]
    #[doc = "The `kernelUnitLengthY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/kernelUnitLengthY)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDiffuseLightingElement`*"]
    #[allow(clippy::all)]
    pub fn kernel_unit_length_y(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeDiffuseLightingElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_kernel_unit_length_y_SVGFEDiffuseLightingElement(
                self_: <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_kernel_unit_length_y_SVGFEDiffuseLightingElement(
            self_: <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_kernel_unit_length_y_SVGFEDiffuseLightingElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeDiffuseLightingElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x_SVGFEDiffuseLightingElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeDiffuseLightingElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeDiffuseLightingElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeDiffuseLightingElement",))]
    #[allow(bad_style)]
    #[doc = "The `x` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDiffuseLightingElement`*"]
    #[allow(clippy::all)]
    pub fn x(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeDiffuseLightingElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x_SVGFEDiffuseLightingElement(
                self_: <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x_SVGFEDiffuseLightingElement(
            self_: <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_x_SVGFEDiffuseLightingElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeDiffuseLightingElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y_SVGFEDiffuseLightingElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeDiffuseLightingElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeDiffuseLightingElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeDiffuseLightingElement",))]
    #[allow(bad_style)]
    #[doc = "The `y` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDiffuseLightingElement`*"]
    #[allow(clippy::all)]
    pub fn y(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeDiffuseLightingElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y_SVGFEDiffuseLightingElement(
                self_: <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y_SVGFEDiffuseLightingElement(
            self_: <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_y_SVGFEDiffuseLightingElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeDiffuseLightingElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_SVGFEDiffuseLightingElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeDiffuseLightingElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeDiffuseLightingElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeDiffuseLightingElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDiffuseLightingElement`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeDiffuseLightingElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_SVGFEDiffuseLightingElement(
                self_: <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_SVGFEDiffuseLightingElement(
            self_: <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_width_SVGFEDiffuseLightingElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeDiffuseLightingElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_SVGFEDiffuseLightingElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeDiffuseLightingElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeDiffuseLightingElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeDiffuseLightingElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDiffuseLightingElement`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeDiffuseLightingElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_SVGFEDiffuseLightingElement(
                self_: <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_SVGFEDiffuseLightingElement(
            self_: <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_height_SVGFEDiffuseLightingElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeDiffuseLightingElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_result_SVGFEDiffuseLightingElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeDiffuseLightingElement as WasmDescribe>::describe();
    <SvgAnimatedString as WasmDescribe>::describe();
}
impl SvgfeDiffuseLightingElement {
    #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeDiffuseLightingElement",))]
    #[allow(bad_style)]
    #[doc = "The `result` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDiffuseLightingElement`*"]
    #[allow(clippy::all)]
    pub fn result(&self) -> SvgAnimatedString {
        #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeDiffuseLightingElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_result_SVGFEDiffuseLightingElement(
                self_: <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_result_SVGFEDiffuseLightingElement(
            self_: <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeDiffuseLightingElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_result_SVGFEDiffuseLightingElement(self_)
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
pub static __WASM_BINDGEN_GENERATED_a79ac9bf1f4206df: [u8; 1300usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xD2\x04\0\0\0\0\x0B\0\0\x02\x1BSVGFEDiffuseLightingElement-__widl_instanceof_SVGFEDiffuseLightingElement\0\0\0\0(__widl_f_in1_SVGFEDiffuseLightingElement\0\0\0\x01\x1BSVGFEDiffuseLightingElement\x01\0\x01\x03in1\x01\x01\x05self_\x03in1\0\0\02__widl_f_surface_scale_SVGFEDiffuseLightingElement\0\0\0\x01\x1BSVGFEDiffuseLightingElement\x01\0\x01\x0CsurfaceScale\x01\x01\x05self_\x0CsurfaceScale\0\0\05__widl_f_diffuse_constant_SVGFEDiffuseLightingElement\0\0\0\x01\x1BSVGFEDiffuseLightingElement\x01\0\x01\x0FdiffuseConstant\x01\x01\x05self_\x0FdiffuseConstant\0\0\09__widl_f_kernel_unit_length_x_SVGFEDiffuseLightingElement\0\0\0\x01\x1BSVGFEDiffuseLightingElement\x01\0\x01\x11kernelUnitLengthX\x01\x01\x05self_\x11kernelUnitLengthX\0\0\09__widl_f_kernel_unit_length_y_SVGFEDiffuseLightingElement\0\0\0\x01\x1BSVGFEDiffuseLightingElement\x01\0\x01\x11kernelUnitLengthY\x01\x01\x05self_\x11kernelUnitLengthY\0\0\0&__widl_f_x_SVGFEDiffuseLightingElement\0\0\0\x01\x1BSVGFEDiffuseLightingElement\x01\0\x01\x01x\x01\x01\x05self_\x01x\0\0\0&__widl_f_y_SVGFEDiffuseLightingElement\0\0\0\x01\x1BSVGFEDiffuseLightingElement\x01\0\x01\x01y\x01\x01\x05self_\x01y\0\0\0*__widl_f_width_SVGFEDiffuseLightingElement\0\0\0\x01\x1BSVGFEDiffuseLightingElement\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0+__widl_f_height_SVGFEDiffuseLightingElement\0\0\0\x01\x1BSVGFEDiffuseLightingElement\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0+__widl_f_result_SVGFEDiffuseLightingElement\0\0\0\x01\x1BSVGFEDiffuseLightingElement\x01\0\x01\x06result\x01\x01\x05self_\x06result\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
