use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGFEConvolveMatrixElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement)\n\n*This API requires the following crate features to be activated: `SvgfeConvolveMatrixElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgfeConvolveMatrixElement {
    obj: SvgElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgfeConvolveMatrixElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgfeConvolveMatrixElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(26u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(70u32);
            inform(69u32);
            inform(67u32);
            inform(111u32);
            inform(110u32);
            inform(118u32);
            inform(111u32);
            inform(108u32);
            inform(118u32);
            inform(101u32);
            inform(77u32);
            inform(97u32);
            inform(116u32);
            inform(114u32);
            inform(105u32);
            inform(120u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgfeConvolveMatrixElement {
        type Target = SvgElement;
        #[inline]
        fn deref(&self) -> &SvgElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgfeConvolveMatrixElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgfeConvolveMatrixElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgfeConvolveMatrixElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgfeConvolveMatrixElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgfeConvolveMatrixElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgfeConvolveMatrixElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgfeConvolveMatrixElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgfeConvolveMatrixElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgfeConvolveMatrixElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgfeConvolveMatrixElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgfeConvolveMatrixElement {
        #[inline]
        fn from(obj: JsValue) -> SvgfeConvolveMatrixElement {
            SvgfeConvolveMatrixElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgfeConvolveMatrixElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgfeConvolveMatrixElement> for SvgfeConvolveMatrixElement {
        #[inline]
        fn as_ref(&self) -> &SvgfeConvolveMatrixElement {
            self
        }
    }
    impl From<SvgfeConvolveMatrixElement> for JsValue {
        #[inline]
        fn from(obj: SvgfeConvolveMatrixElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgfeConvolveMatrixElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGFEConvolveMatrixElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGFEConvolveMatrixElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGFEConvolveMatrixElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgfeConvolveMatrixElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgfeConvolveMatrixElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgfeConvolveMatrixElement> for SvgElement {
    #[inline]
    fn from(obj: SvgfeConvolveMatrixElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgfeConvolveMatrixElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeConvolveMatrixElement> for Element {
    #[inline]
    fn from(obj: SvgfeConvolveMatrixElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgfeConvolveMatrixElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeConvolveMatrixElement> for Node {
    #[inline]
    fn from(obj: SvgfeConvolveMatrixElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgfeConvolveMatrixElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeConvolveMatrixElement> for EventTarget {
    #[inline]
    fn from(obj: SvgfeConvolveMatrixElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgfeConvolveMatrixElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgfeConvolveMatrixElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgfeConvolveMatrixElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgfeConvolveMatrixElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeConvolveMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_in1_SVGFEConvolveMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeConvolveMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedString as WasmDescribe>::describe();
}
impl SvgfeConvolveMatrixElement {
    #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeConvolveMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `in1` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/in1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeConvolveMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn in1(&self) -> SvgAnimatedString {
        #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeConvolveMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_in1_SVGFEConvolveMatrixElement(
                self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_in1_SVGFEConvolveMatrixElement(
            self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_in1_SVGFEConvolveMatrixElement(self_)
            };
            <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedInteger", feature = "SvgfeConvolveMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_order_x_SVGFEConvolveMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeConvolveMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedInteger as WasmDescribe>::describe();
}
impl SvgfeConvolveMatrixElement {
    #[cfg(all(feature = "SvgAnimatedInteger", feature = "SvgfeConvolveMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `orderX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/orderX)\n\n*This API requires the following crate features to be activated: `SvgAnimatedInteger`, `SvgfeConvolveMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn order_x(&self) -> SvgAnimatedInteger {
        #[cfg(all(feature = "SvgAnimatedInteger", feature = "SvgfeConvolveMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_order_x_SVGFEConvolveMatrixElement(
                self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedInteger as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_order_x_SVGFEConvolveMatrixElement(
            self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedInteger as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_order_x_SVGFEConvolveMatrixElement(self_)
            };
            <SvgAnimatedInteger as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedInteger", feature = "SvgfeConvolveMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_order_y_SVGFEConvolveMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeConvolveMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedInteger as WasmDescribe>::describe();
}
impl SvgfeConvolveMatrixElement {
    #[cfg(all(feature = "SvgAnimatedInteger", feature = "SvgfeConvolveMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `orderY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/orderY)\n\n*This API requires the following crate features to be activated: `SvgAnimatedInteger`, `SvgfeConvolveMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn order_y(&self) -> SvgAnimatedInteger {
        #[cfg(all(feature = "SvgAnimatedInteger", feature = "SvgfeConvolveMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_order_y_SVGFEConvolveMatrixElement(
                self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedInteger as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_order_y_SVGFEConvolveMatrixElement(
            self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedInteger as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_order_y_SVGFEConvolveMatrixElement(self_)
            };
            <SvgAnimatedInteger as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedNumberList",
    feature = "SvgfeConvolveMatrixElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_kernel_matrix_SVGFEConvolveMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeConvolveMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedNumberList as WasmDescribe>::describe();
}
impl SvgfeConvolveMatrixElement {
    #[cfg(all(
        feature = "SvgAnimatedNumberList",
        feature = "SvgfeConvolveMatrixElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `kernelMatrix` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/kernelMatrix)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgfeConvolveMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn kernel_matrix(&self) -> SvgAnimatedNumberList {
        #[cfg(all(
            feature = "SvgAnimatedNumberList",
            feature = "SvgfeConvolveMatrixElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_kernel_matrix_SVGFEConvolveMatrixElement(
                self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumberList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_kernel_matrix_SVGFEConvolveMatrixElement(
            self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedNumberList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_kernel_matrix_SVGFEConvolveMatrixElement(self_)
            };
            <SvgAnimatedNumberList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeConvolveMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_divisor_SVGFEConvolveMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeConvolveMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeConvolveMatrixElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeConvolveMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `divisor` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/divisor)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeConvolveMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn divisor(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeConvolveMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_divisor_SVGFEConvolveMatrixElement(
                self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_divisor_SVGFEConvolveMatrixElement(
            self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_divisor_SVGFEConvolveMatrixElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeConvolveMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bias_SVGFEConvolveMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeConvolveMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeConvolveMatrixElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeConvolveMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `bias` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/bias)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeConvolveMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn bias(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeConvolveMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bias_SVGFEConvolveMatrixElement(
                self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bias_SVGFEConvolveMatrixElement(
            self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_bias_SVGFEConvolveMatrixElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedInteger", feature = "SvgfeConvolveMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_target_x_SVGFEConvolveMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeConvolveMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedInteger as WasmDescribe>::describe();
}
impl SvgfeConvolveMatrixElement {
    #[cfg(all(feature = "SvgAnimatedInteger", feature = "SvgfeConvolveMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `targetX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/targetX)\n\n*This API requires the following crate features to be activated: `SvgAnimatedInteger`, `SvgfeConvolveMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn target_x(&self) -> SvgAnimatedInteger {
        #[cfg(all(feature = "SvgAnimatedInteger", feature = "SvgfeConvolveMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_target_x_SVGFEConvolveMatrixElement(
                self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedInteger as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_target_x_SVGFEConvolveMatrixElement(
            self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedInteger as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_target_x_SVGFEConvolveMatrixElement(self_)
            };
            <SvgAnimatedInteger as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedInteger", feature = "SvgfeConvolveMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_target_y_SVGFEConvolveMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeConvolveMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedInteger as WasmDescribe>::describe();
}
impl SvgfeConvolveMatrixElement {
    #[cfg(all(feature = "SvgAnimatedInteger", feature = "SvgfeConvolveMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `targetY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/targetY)\n\n*This API requires the following crate features to be activated: `SvgAnimatedInteger`, `SvgfeConvolveMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn target_y(&self) -> SvgAnimatedInteger {
        #[cfg(all(feature = "SvgAnimatedInteger", feature = "SvgfeConvolveMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_target_y_SVGFEConvolveMatrixElement(
                self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedInteger as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_target_y_SVGFEConvolveMatrixElement(
            self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedInteger as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_target_y_SVGFEConvolveMatrixElement(self_)
            };
            <SvgAnimatedInteger as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedEnumeration",
    feature = "SvgfeConvolveMatrixElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_edge_mode_SVGFEConvolveMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeConvolveMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedEnumeration as WasmDescribe>::describe();
}
impl SvgfeConvolveMatrixElement {
    #[cfg(all(
        feature = "SvgAnimatedEnumeration",
        feature = "SvgfeConvolveMatrixElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `edgeMode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/edgeMode)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeConvolveMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn edge_mode(&self) -> SvgAnimatedEnumeration {
        #[cfg(all(
            feature = "SvgAnimatedEnumeration",
            feature = "SvgfeConvolveMatrixElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_edge_mode_SVGFEConvolveMatrixElement(
                self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_edge_mode_SVGFEConvolveMatrixElement(
            self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_edge_mode_SVGFEConvolveMatrixElement(self_)
            };
            <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeConvolveMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_kernel_unit_length_x_SVGFEConvolveMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeConvolveMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeConvolveMatrixElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeConvolveMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `kernelUnitLengthX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/kernelUnitLengthX)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeConvolveMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn kernel_unit_length_x(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeConvolveMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_kernel_unit_length_x_SVGFEConvolveMatrixElement(
                self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_kernel_unit_length_x_SVGFEConvolveMatrixElement(
            self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_kernel_unit_length_x_SVGFEConvolveMatrixElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeConvolveMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_kernel_unit_length_y_SVGFEConvolveMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeConvolveMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgfeConvolveMatrixElement {
    #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeConvolveMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `kernelUnitLengthY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/kernelUnitLengthY)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeConvolveMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn kernel_unit_length_y(&self) -> SvgAnimatedNumber {
        #[cfg(all(feature = "SvgAnimatedNumber", feature = "SvgfeConvolveMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_kernel_unit_length_y_SVGFEConvolveMatrixElement(
                self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_kernel_unit_length_y_SVGFEConvolveMatrixElement(
            self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_kernel_unit_length_y_SVGFEConvolveMatrixElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedBoolean", feature = "SvgfeConvolveMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_preserve_alpha_SVGFEConvolveMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeConvolveMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedBoolean as WasmDescribe>::describe();
}
impl SvgfeConvolveMatrixElement {
    #[cfg(all(feature = "SvgAnimatedBoolean", feature = "SvgfeConvolveMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `preserveAlpha` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/preserveAlpha)\n\n*This API requires the following crate features to be activated: `SvgAnimatedBoolean`, `SvgfeConvolveMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn preserve_alpha(&self) -> SvgAnimatedBoolean {
        #[cfg(all(feature = "SvgAnimatedBoolean", feature = "SvgfeConvolveMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_preserve_alpha_SVGFEConvolveMatrixElement(
                self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedBoolean as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_preserve_alpha_SVGFEConvolveMatrixElement(
            self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedBoolean as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_preserve_alpha_SVGFEConvolveMatrixElement(self_)
            };
            <SvgAnimatedBoolean as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeConvolveMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x_SVGFEConvolveMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeConvolveMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeConvolveMatrixElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeConvolveMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `x` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeConvolveMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn x(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeConvolveMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x_SVGFEConvolveMatrixElement(
                self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x_SVGFEConvolveMatrixElement(
            self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_x_SVGFEConvolveMatrixElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeConvolveMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y_SVGFEConvolveMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeConvolveMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeConvolveMatrixElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeConvolveMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `y` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeConvolveMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn y(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeConvolveMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y_SVGFEConvolveMatrixElement(
                self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y_SVGFEConvolveMatrixElement(
            self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_y_SVGFEConvolveMatrixElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeConvolveMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_SVGFEConvolveMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeConvolveMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeConvolveMatrixElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeConvolveMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeConvolveMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeConvolveMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_SVGFEConvolveMatrixElement(
                self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_SVGFEConvolveMatrixElement(
            self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_width_SVGFEConvolveMatrixElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeConvolveMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_SVGFEConvolveMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeConvolveMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgfeConvolveMatrixElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeConvolveMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeConvolveMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgfeConvolveMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_SVGFEConvolveMatrixElement(
                self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_SVGFEConvolveMatrixElement(
            self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_height_SVGFEConvolveMatrixElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeConvolveMatrixElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_result_SVGFEConvolveMatrixElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgfeConvolveMatrixElement as WasmDescribe>::describe();
    <SvgAnimatedString as WasmDescribe>::describe();
}
impl SvgfeConvolveMatrixElement {
    #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeConvolveMatrixElement",))]
    #[allow(bad_style)]
    #[doc = "The `result` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeConvolveMatrixElement`*"]
    #[allow(clippy::all)]
    pub fn result(&self) -> SvgAnimatedString {
        #[cfg(all(feature = "SvgAnimatedString", feature = "SvgfeConvolveMatrixElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_result_SVGFEConvolveMatrixElement(
                self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_result_SVGFEConvolveMatrixElement(
            self_: <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgfeConvolveMatrixElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_result_SVGFEConvolveMatrixElement(self_)
            };
            <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl SvgfeConvolveMatrixElement {
    pub const SVG_EDGEMODE_UNKNOWN: u16 = 0i64 as u16;
}
impl SvgfeConvolveMatrixElement {
    pub const SVG_EDGEMODE_DUPLICATE: u16 = 1u64 as u16;
}
impl SvgfeConvolveMatrixElement {
    pub const SVG_EDGEMODE_WRAP: u16 = 2u64 as u16;
}
impl SvgfeConvolveMatrixElement {
    pub const SVG_EDGEMODE_NONE: u16 = 3u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_23649434c6149d15: [u8; 2000usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x8E\x07\0\0\0\0\x12\0\0\x02\x1ASVGFEConvolveMatrixElement,__widl_instanceof_SVGFEConvolveMatrixElement\0\0\0\0'__widl_f_in1_SVGFEConvolveMatrixElement\0\0\0\x01\x1ASVGFEConvolveMatrixElement\x01\0\x01\x03in1\x01\x01\x05self_\x03in1\0\0\0+__widl_f_order_x_SVGFEConvolveMatrixElement\0\0\0\x01\x1ASVGFEConvolveMatrixElement\x01\0\x01\x06orderX\x01\x01\x05self_\x06orderX\0\0\0+__widl_f_order_y_SVGFEConvolveMatrixElement\0\0\0\x01\x1ASVGFEConvolveMatrixElement\x01\0\x01\x06orderY\x01\x01\x05self_\x06orderY\0\0\01__widl_f_kernel_matrix_SVGFEConvolveMatrixElement\0\0\0\x01\x1ASVGFEConvolveMatrixElement\x01\0\x01\x0CkernelMatrix\x01\x01\x05self_\x0CkernelMatrix\0\0\0+__widl_f_divisor_SVGFEConvolveMatrixElement\0\0\0\x01\x1ASVGFEConvolveMatrixElement\x01\0\x01\x07divisor\x01\x01\x05self_\x07divisor\0\0\0(__widl_f_bias_SVGFEConvolveMatrixElement\0\0\0\x01\x1ASVGFEConvolveMatrixElement\x01\0\x01\x04bias\x01\x01\x05self_\x04bias\0\0\0,__widl_f_target_x_SVGFEConvolveMatrixElement\0\0\0\x01\x1ASVGFEConvolveMatrixElement\x01\0\x01\x07targetX\x01\x01\x05self_\x07targetX\0\0\0,__widl_f_target_y_SVGFEConvolveMatrixElement\0\0\0\x01\x1ASVGFEConvolveMatrixElement\x01\0\x01\x07targetY\x01\x01\x05self_\x07targetY\0\0\0-__widl_f_edge_mode_SVGFEConvolveMatrixElement\0\0\0\x01\x1ASVGFEConvolveMatrixElement\x01\0\x01\x08edgeMode\x01\x01\x05self_\x08edgeMode\0\0\08__widl_f_kernel_unit_length_x_SVGFEConvolveMatrixElement\0\0\0\x01\x1ASVGFEConvolveMatrixElement\x01\0\x01\x11kernelUnitLengthX\x01\x01\x05self_\x11kernelUnitLengthX\0\0\08__widl_f_kernel_unit_length_y_SVGFEConvolveMatrixElement\0\0\0\x01\x1ASVGFEConvolveMatrixElement\x01\0\x01\x11kernelUnitLengthY\x01\x01\x05self_\x11kernelUnitLengthY\0\0\02__widl_f_preserve_alpha_SVGFEConvolveMatrixElement\0\0\0\x01\x1ASVGFEConvolveMatrixElement\x01\0\x01\rpreserveAlpha\x01\x01\x05self_\rpreserveAlpha\0\0\0%__widl_f_x_SVGFEConvolveMatrixElement\0\0\0\x01\x1ASVGFEConvolveMatrixElement\x01\0\x01\x01x\x01\x01\x05self_\x01x\0\0\0%__widl_f_y_SVGFEConvolveMatrixElement\0\0\0\x01\x1ASVGFEConvolveMatrixElement\x01\0\x01\x01y\x01\x01\x05self_\x01y\0\0\0)__widl_f_width_SVGFEConvolveMatrixElement\0\0\0\x01\x1ASVGFEConvolveMatrixElement\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0*__widl_f_height_SVGFEConvolveMatrixElement\0\0\0\x01\x1ASVGFEConvolveMatrixElement\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0*__widl_f_result_SVGFEConvolveMatrixElement\0\0\0\x01\x1ASVGFEConvolveMatrixElement\x01\0\x01\x06result\x01\x01\x05self_\x06result\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
