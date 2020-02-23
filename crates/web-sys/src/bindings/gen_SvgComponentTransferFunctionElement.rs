use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGComponentTransferFunctionElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement)\n\n*This API requires the following crate features to be activated: `SvgComponentTransferFunctionElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgComponentTransferFunctionElement {
    obj: SvgElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgComponentTransferFunctionElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgComponentTransferFunctionElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(35u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(67u32);
            inform(111u32);
            inform(109u32);
            inform(112u32);
            inform(111u32);
            inform(110u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(84u32);
            inform(114u32);
            inform(97u32);
            inform(110u32);
            inform(115u32);
            inform(102u32);
            inform(101u32);
            inform(114u32);
            inform(70u32);
            inform(117u32);
            inform(110u32);
            inform(99u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgComponentTransferFunctionElement {
        type Target = SvgElement;
        #[inline]
        fn deref(&self) -> &SvgElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgComponentTransferFunctionElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgComponentTransferFunctionElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgComponentTransferFunctionElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgComponentTransferFunctionElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgComponentTransferFunctionElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgComponentTransferFunctionElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgComponentTransferFunctionElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgComponentTransferFunctionElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgComponentTransferFunctionElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgComponentTransferFunctionElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgComponentTransferFunctionElement {
        #[inline]
        fn from(obj: JsValue) -> SvgComponentTransferFunctionElement {
            SvgComponentTransferFunctionElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgComponentTransferFunctionElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgComponentTransferFunctionElement> for SvgComponentTransferFunctionElement {
        #[inline]
        fn as_ref(&self) -> &SvgComponentTransferFunctionElement {
            self
        }
    }
    impl From<SvgComponentTransferFunctionElement> for JsValue {
        #[inline]
        fn from(obj: SvgComponentTransferFunctionElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgComponentTransferFunctionElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGComponentTransferFunctionElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGComponentTransferFunctionElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGComponentTransferFunctionElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgComponentTransferFunctionElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgComponentTransferFunctionElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgComponentTransferFunctionElement> for SvgElement {
    #[inline]
    fn from(obj: SvgComponentTransferFunctionElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgComponentTransferFunctionElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgComponentTransferFunctionElement> for Element {
    #[inline]
    fn from(obj: SvgComponentTransferFunctionElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgComponentTransferFunctionElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgComponentTransferFunctionElement> for Node {
    #[inline]
    fn from(obj: SvgComponentTransferFunctionElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgComponentTransferFunctionElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgComponentTransferFunctionElement> for EventTarget {
    #[inline]
    fn from(obj: SvgComponentTransferFunctionElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgComponentTransferFunctionElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgComponentTransferFunctionElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgComponentTransferFunctionElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgComponentTransferFunctionElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(
    feature = "SvgAnimatedEnumeration",
    feature = "SvgComponentTransferFunctionElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_SVGComponentTransferFunctionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgComponentTransferFunctionElement as WasmDescribe>::describe();
    <SvgAnimatedEnumeration as WasmDescribe>::describe();
}
impl SvgComponentTransferFunctionElement {
    #[cfg(all(
        feature = "SvgAnimatedEnumeration",
        feature = "SvgComponentTransferFunctionElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/type)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgComponentTransferFunctionElement`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> SvgAnimatedEnumeration {
        #[cfg(all(
            feature = "SvgAnimatedEnumeration",
            feature = "SvgComponentTransferFunctionElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_SVGComponentTransferFunctionElement(
                self_ : < & SvgComponentTransferFunctionElement as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_SVGComponentTransferFunctionElement(
            self_ : < & SvgComponentTransferFunctionElement as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = < & SvgComponentTransferFunctionElement as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( self ) ;
                __widl_f_type_SVGComponentTransferFunctionElement(self_)
            };
            <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedNumberList",
    feature = "SvgComponentTransferFunctionElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_table_values_SVGComponentTransferFunctionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgComponentTransferFunctionElement as WasmDescribe>::describe();
    <SvgAnimatedNumberList as WasmDescribe>::describe();
}
impl SvgComponentTransferFunctionElement {
    #[cfg(all(
        feature = "SvgAnimatedNumberList",
        feature = "SvgComponentTransferFunctionElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `tableValues` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/tableValues)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgComponentTransferFunctionElement`*"]
    #[allow(clippy::all)]
    pub fn table_values(&self) -> SvgAnimatedNumberList {
        #[cfg(all(
            feature = "SvgAnimatedNumberList",
            feature = "SvgComponentTransferFunctionElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_values_SVGComponentTransferFunctionElement(
                self_ : < & SvgComponentTransferFunctionElement as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <SvgAnimatedNumberList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_values_SVGComponentTransferFunctionElement(
            self_ : < & SvgComponentTransferFunctionElement as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <SvgAnimatedNumberList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = < & SvgComponentTransferFunctionElement as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( self ) ;
                __widl_f_table_values_SVGComponentTransferFunctionElement(self_)
            };
            <SvgAnimatedNumberList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedNumber",
    feature = "SvgComponentTransferFunctionElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_slope_SVGComponentTransferFunctionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgComponentTransferFunctionElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgComponentTransferFunctionElement {
    #[cfg(all(
        feature = "SvgAnimatedNumber",
        feature = "SvgComponentTransferFunctionElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `slope` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/slope)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgComponentTransferFunctionElement`*"]
    #[allow(clippy::all)]
    pub fn slope(&self) -> SvgAnimatedNumber {
        #[cfg(all(
            feature = "SvgAnimatedNumber",
            feature = "SvgComponentTransferFunctionElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_slope_SVGComponentTransferFunctionElement(
                self_ : < & SvgComponentTransferFunctionElement as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_slope_SVGComponentTransferFunctionElement(
            self_ : < & SvgComponentTransferFunctionElement as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = < & SvgComponentTransferFunctionElement as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( self ) ;
                __widl_f_slope_SVGComponentTransferFunctionElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedNumber",
    feature = "SvgComponentTransferFunctionElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_intercept_SVGComponentTransferFunctionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgComponentTransferFunctionElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgComponentTransferFunctionElement {
    #[cfg(all(
        feature = "SvgAnimatedNumber",
        feature = "SvgComponentTransferFunctionElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `intercept` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/intercept)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgComponentTransferFunctionElement`*"]
    #[allow(clippy::all)]
    pub fn intercept(&self) -> SvgAnimatedNumber {
        #[cfg(all(
            feature = "SvgAnimatedNumber",
            feature = "SvgComponentTransferFunctionElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_intercept_SVGComponentTransferFunctionElement(
                self_ : < & SvgComponentTransferFunctionElement as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_intercept_SVGComponentTransferFunctionElement(
            self_ : < & SvgComponentTransferFunctionElement as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = < & SvgComponentTransferFunctionElement as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( self ) ;
                __widl_f_intercept_SVGComponentTransferFunctionElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedNumber",
    feature = "SvgComponentTransferFunctionElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_amplitude_SVGComponentTransferFunctionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgComponentTransferFunctionElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgComponentTransferFunctionElement {
    #[cfg(all(
        feature = "SvgAnimatedNumber",
        feature = "SvgComponentTransferFunctionElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `amplitude` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/amplitude)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgComponentTransferFunctionElement`*"]
    #[allow(clippy::all)]
    pub fn amplitude(&self) -> SvgAnimatedNumber {
        #[cfg(all(
            feature = "SvgAnimatedNumber",
            feature = "SvgComponentTransferFunctionElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_amplitude_SVGComponentTransferFunctionElement(
                self_ : < & SvgComponentTransferFunctionElement as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_amplitude_SVGComponentTransferFunctionElement(
            self_ : < & SvgComponentTransferFunctionElement as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = < & SvgComponentTransferFunctionElement as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( self ) ;
                __widl_f_amplitude_SVGComponentTransferFunctionElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedNumber",
    feature = "SvgComponentTransferFunctionElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_exponent_SVGComponentTransferFunctionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgComponentTransferFunctionElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgComponentTransferFunctionElement {
    #[cfg(all(
        feature = "SvgAnimatedNumber",
        feature = "SvgComponentTransferFunctionElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `exponent` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/exponent)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgComponentTransferFunctionElement`*"]
    #[allow(clippy::all)]
    pub fn exponent(&self) -> SvgAnimatedNumber {
        #[cfg(all(
            feature = "SvgAnimatedNumber",
            feature = "SvgComponentTransferFunctionElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exponent_SVGComponentTransferFunctionElement(
                self_ : < & SvgComponentTransferFunctionElement as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exponent_SVGComponentTransferFunctionElement(
            self_ : < & SvgComponentTransferFunctionElement as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = < & SvgComponentTransferFunctionElement as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( self ) ;
                __widl_f_exponent_SVGComponentTransferFunctionElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedNumber",
    feature = "SvgComponentTransferFunctionElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_offset_SVGComponentTransferFunctionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgComponentTransferFunctionElement as WasmDescribe>::describe();
    <SvgAnimatedNumber as WasmDescribe>::describe();
}
impl SvgComponentTransferFunctionElement {
    #[cfg(all(
        feature = "SvgAnimatedNumber",
        feature = "SvgComponentTransferFunctionElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `offset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/offset)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgComponentTransferFunctionElement`*"]
    #[allow(clippy::all)]
    pub fn offset(&self) -> SvgAnimatedNumber {
        #[cfg(all(
            feature = "SvgAnimatedNumber",
            feature = "SvgComponentTransferFunctionElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_offset_SVGComponentTransferFunctionElement(
                self_ : < & SvgComponentTransferFunctionElement as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_offset_SVGComponentTransferFunctionElement(
            self_ : < & SvgComponentTransferFunctionElement as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = < & SvgComponentTransferFunctionElement as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( self ) ;
                __widl_f_offset_SVGComponentTransferFunctionElement(self_)
            };
            <SvgAnimatedNumber as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl SvgComponentTransferFunctionElement {
    pub const SVG_FECOMPONENTTRANSFER_TYPE_UNKNOWN: u16 = 0i64 as u16;
}
impl SvgComponentTransferFunctionElement {
    pub const SVG_FECOMPONENTTRANSFER_TYPE_IDENTITY: u16 = 1u64 as u16;
}
impl SvgComponentTransferFunctionElement {
    pub const SVG_FECOMPONENTTRANSFER_TYPE_TABLE: u16 = 2u64 as u16;
}
impl SvgComponentTransferFunctionElement {
    pub const SVG_FECOMPONENTTRANSFER_TYPE_DISCRETE: u16 = 3u64 as u16;
}
impl SvgComponentTransferFunctionElement {
    pub const SVG_FECOMPONENTTRANSFER_TYPE_LINEAR: u16 = 4u64 as u16;
}
impl SvgComponentTransferFunctionElement {
    pub const SVG_FECOMPONENTTRANSFER_TYPE_GAMMA: u16 = 5u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_88487f64adf1d3f4: [u8; 1070usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xEC\x03\0\0\0\0\x08\0\0\x02#SVGComponentTransferFunctionElement5__widl_instanceof_SVGComponentTransferFunctionElement\0\0\0\01__widl_f_type_SVGComponentTransferFunctionElement\0\0\0\x01#SVGComponentTransferFunctionElement\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\09__widl_f_table_values_SVGComponentTransferFunctionElement\0\0\0\x01#SVGComponentTransferFunctionElement\x01\0\x01\x0BtableValues\x01\x01\x05self_\x0BtableValues\0\0\02__widl_f_slope_SVGComponentTransferFunctionElement\0\0\0\x01#SVGComponentTransferFunctionElement\x01\0\x01\x05slope\x01\x01\x05self_\x05slope\0\0\06__widl_f_intercept_SVGComponentTransferFunctionElement\0\0\0\x01#SVGComponentTransferFunctionElement\x01\0\x01\tintercept\x01\x01\x05self_\tintercept\0\0\06__widl_f_amplitude_SVGComponentTransferFunctionElement\0\0\0\x01#SVGComponentTransferFunctionElement\x01\0\x01\tamplitude\x01\x01\x05self_\tamplitude\0\0\05__widl_f_exponent_SVGComponentTransferFunctionElement\0\0\0\x01#SVGComponentTransferFunctionElement\x01\0\x01\x08exponent\x01\x01\x05self_\x08exponent\0\0\03__widl_f_offset_SVGComponentTransferFunctionElement\0\0\0\x01#SVGComponentTransferFunctionElement\x01\0\x01\x06offset\x01\x01\x05self_\x06offset\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
