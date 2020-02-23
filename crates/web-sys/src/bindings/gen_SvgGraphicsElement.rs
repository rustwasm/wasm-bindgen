use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGGraphicsElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement)\n\n*This API requires the following crate features to be activated: `SvgGraphicsElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgGraphicsElement {
    obj: SvgElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgGraphicsElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgGraphicsElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(18u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(71u32);
            inform(114u32);
            inform(97u32);
            inform(112u32);
            inform(104u32);
            inform(105u32);
            inform(99u32);
            inform(115u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgGraphicsElement {
        type Target = SvgElement;
        #[inline]
        fn deref(&self) -> &SvgElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgGraphicsElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgGraphicsElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgGraphicsElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgGraphicsElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgGraphicsElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgGraphicsElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgGraphicsElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgGraphicsElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgGraphicsElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgGraphicsElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgGraphicsElement {
        #[inline]
        fn from(obj: JsValue) -> SvgGraphicsElement {
            SvgGraphicsElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgGraphicsElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgGraphicsElement> for SvgGraphicsElement {
        #[inline]
        fn as_ref(&self) -> &SvgGraphicsElement {
            self
        }
    }
    impl From<SvgGraphicsElement> for JsValue {
        #[inline]
        fn from(obj: SvgGraphicsElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgGraphicsElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGGraphicsElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGGraphicsElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGGraphicsElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgGraphicsElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgGraphicsElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgGraphicsElement> for SvgElement {
    #[inline]
    fn from(obj: SvgGraphicsElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgGraphicsElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgGraphicsElement> for Element {
    #[inline]
    fn from(obj: SvgGraphicsElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgGraphicsElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgGraphicsElement> for Node {
    #[inline]
    fn from(obj: SvgGraphicsElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgGraphicsElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgGraphicsElement> for EventTarget {
    #[inline]
    fn from(obj: SvgGraphicsElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgGraphicsElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgGraphicsElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgGraphicsElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgGraphicsElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgGraphicsElement", feature = "SvgRect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_b_box_SVGGraphicsElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgGraphicsElement as WasmDescribe>::describe();
    <SvgRect as WasmDescribe>::describe();
}
impl SvgGraphicsElement {
    #[cfg(all(feature = "SvgGraphicsElement", feature = "SvgRect",))]
    #[allow(bad_style)]
    #[doc = "The `getBBox()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getBBox)\n\n*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgRect`*"]
    #[allow(clippy::all)]
    pub fn get_b_box(&self) -> Result<SvgRect, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgGraphicsElement", feature = "SvgRect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_b_box_SVGGraphicsElement(
                self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgRect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_b_box_SVGGraphicsElement(
            self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgRect as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_b_box_SVGGraphicsElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgRect as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "SvgBoundingBoxOptions",
    feature = "SvgGraphicsElement",
    feature = "SvgRect",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_b_box_with_a_options_SVGGraphicsElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgGraphicsElement as WasmDescribe>::describe();
    <&SvgBoundingBoxOptions as WasmDescribe>::describe();
    <SvgRect as WasmDescribe>::describe();
}
impl SvgGraphicsElement {
    #[cfg(all(
        feature = "SvgBoundingBoxOptions",
        feature = "SvgGraphicsElement",
        feature = "SvgRect",
    ))]
    #[allow(bad_style)]
    #[doc = "The `getBBox()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getBBox)\n\n*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`, `SvgGraphicsElement`, `SvgRect`*"]
    #[allow(clippy::all)]
    pub fn get_b_box_with_a_options(
        &self,
        a_options: &SvgBoundingBoxOptions,
    ) -> Result<SvgRect, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "SvgBoundingBoxOptions",
            feature = "SvgGraphicsElement",
            feature = "SvgRect",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_b_box_with_a_options_SVGGraphicsElement(
                self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_options: <&SvgBoundingBoxOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgRect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_b_box_with_a_options_SVGGraphicsElement(
            self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_options: <&SvgBoundingBoxOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgRect as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_options =
                    <&SvgBoundingBoxOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        a_options,
                    );
                __widl_f_get_b_box_with_a_options_SVGGraphicsElement(self_, a_options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgRect as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "SvgGraphicsElement", feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_ctm_SVGGraphicsElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgGraphicsElement as WasmDescribe>::describe();
    <Option<SvgMatrix> as WasmDescribe>::describe();
}
impl SvgGraphicsElement {
    #[cfg(all(feature = "SvgGraphicsElement", feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `getCTM()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getCTM)\n\n*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn get_ctm(&self) -> Option<SvgMatrix> {
        #[cfg(all(feature = "SvgGraphicsElement", feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_ctm_SVGGraphicsElement(
                self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<SvgMatrix> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_ctm_SVGGraphicsElement(
            self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<SvgMatrix> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_ctm_SVGGraphicsElement(self_)
            };
            <Option<SvgMatrix> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgGraphicsElement", feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_screen_ctm_SVGGraphicsElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgGraphicsElement as WasmDescribe>::describe();
    <Option<SvgMatrix> as WasmDescribe>::describe();
}
impl SvgGraphicsElement {
    #[cfg(all(feature = "SvgGraphicsElement", feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `getScreenCTM()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getScreenCTM)\n\n*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn get_screen_ctm(&self) -> Option<SvgMatrix> {
        #[cfg(all(feature = "SvgGraphicsElement", feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_screen_ctm_SVGGraphicsElement(
                self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<SvgMatrix> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_screen_ctm_SVGGraphicsElement(
            self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<SvgMatrix> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_screen_ctm_SVGGraphicsElement(self_)
            };
            <Option<SvgMatrix> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgGraphicsElement", feature = "SvgMatrix",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_transform_to_element_SVGGraphicsElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgGraphicsElement as WasmDescribe>::describe();
    <&SvgGraphicsElement as WasmDescribe>::describe();
    <SvgMatrix as WasmDescribe>::describe();
}
impl SvgGraphicsElement {
    #[cfg(all(feature = "SvgGraphicsElement", feature = "SvgMatrix",))]
    #[allow(bad_style)]
    #[doc = "The `getTransformToElement()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getTransformToElement)\n\n*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgMatrix`*"]
    #[allow(clippy::all)]
    pub fn get_transform_to_element(
        &self,
        element: &SvgGraphicsElement,
    ) -> Result<SvgMatrix, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgGraphicsElement", feature = "SvgMatrix",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_transform_to_element_SVGGraphicsElement(
                self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                element: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_transform_to_element_SVGGraphicsElement(
            self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            element: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(element);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let element =
                    <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(element);
                __widl_f_get_transform_to_element_SVGGraphicsElement(self_, element)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "SvgAnimatedTransformList", feature = "SvgGraphicsElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transform_SVGGraphicsElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgGraphicsElement as WasmDescribe>::describe();
    <SvgAnimatedTransformList as WasmDescribe>::describe();
}
impl SvgGraphicsElement {
    #[cfg(all(feature = "SvgAnimatedTransformList", feature = "SvgGraphicsElement",))]
    #[allow(bad_style)]
    #[doc = "The `transform` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/transform)\n\n*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgGraphicsElement`*"]
    #[allow(clippy::all)]
    pub fn transform(&self) -> SvgAnimatedTransformList {
        #[cfg(all(feature = "SvgAnimatedTransformList", feature = "SvgGraphicsElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transform_SVGGraphicsElement(
                self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedTransformList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transform_SVGGraphicsElement(
            self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_transform_SVGGraphicsElement(self_)
            };
            <SvgAnimatedTransformList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement", feature = "SvgGraphicsElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_nearest_viewport_element_SVGGraphicsElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgGraphicsElement as WasmDescribe>::describe();
    <Option<SvgElement> as WasmDescribe>::describe();
}
impl SvgGraphicsElement {
    #[cfg(all(feature = "SvgElement", feature = "SvgGraphicsElement",))]
    #[allow(bad_style)]
    #[doc = "The `nearestViewportElement` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/nearestViewportElement)\n\n*This API requires the following crate features to be activated: `SvgElement`, `SvgGraphicsElement`*"]
    #[allow(clippy::all)]
    pub fn nearest_viewport_element(&self) -> Option<SvgElement> {
        #[cfg(all(feature = "SvgElement", feature = "SvgGraphicsElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_nearest_viewport_element_SVGGraphicsElement(
                self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<SvgElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_nearest_viewport_element_SVGGraphicsElement(
            self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<SvgElement> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_nearest_viewport_element_SVGGraphicsElement(self_)
            };
            <Option<SvgElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement", feature = "SvgGraphicsElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_farthest_viewport_element_SVGGraphicsElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgGraphicsElement as WasmDescribe>::describe();
    <Option<SvgElement> as WasmDescribe>::describe();
}
impl SvgGraphicsElement {
    #[cfg(all(feature = "SvgElement", feature = "SvgGraphicsElement",))]
    #[allow(bad_style)]
    #[doc = "The `farthestViewportElement` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/farthestViewportElement)\n\n*This API requires the following crate features to be activated: `SvgElement`, `SvgGraphicsElement`*"]
    #[allow(clippy::all)]
    pub fn farthest_viewport_element(&self) -> Option<SvgElement> {
        #[cfg(all(feature = "SvgElement", feature = "SvgGraphicsElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_farthest_viewport_element_SVGGraphicsElement(
                self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<SvgElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_farthest_viewport_element_SVGGraphicsElement(
            self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<SvgElement> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_farthest_viewport_element_SVGGraphicsElement(self_)
            };
            <Option<SvgElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgGraphicsElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_extension_SVGGraphicsElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgGraphicsElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl SvgGraphicsElement {
    #[cfg(all(feature = "SvgGraphicsElement",))]
    #[allow(bad_style)]
    #[doc = "The `hasExtension()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/hasExtension)\n\n*This API requires the following crate features to be activated: `SvgGraphicsElement`*"]
    #[allow(clippy::all)]
    pub fn has_extension(&self, extension: &str) -> bool {
        #[cfg(all(feature = "SvgGraphicsElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_extension_SVGGraphicsElement(
                self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                extension: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_extension_SVGGraphicsElement(
            self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            extension: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(extension);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let extension = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(extension);
                __widl_f_has_extension_SVGGraphicsElement(self_, extension)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgGraphicsElement", feature = "SvgStringList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_required_features_SVGGraphicsElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgGraphicsElement as WasmDescribe>::describe();
    <SvgStringList as WasmDescribe>::describe();
}
impl SvgGraphicsElement {
    #[cfg(all(feature = "SvgGraphicsElement", feature = "SvgStringList",))]
    #[allow(bad_style)]
    #[doc = "The `requiredFeatures` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/requiredFeatures)\n\n*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgStringList`*"]
    #[allow(clippy::all)]
    pub fn required_features(&self) -> SvgStringList {
        #[cfg(all(feature = "SvgGraphicsElement", feature = "SvgStringList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_required_features_SVGGraphicsElement(
                self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_required_features_SVGGraphicsElement(
            self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_required_features_SVGGraphicsElement(self_)
            };
            <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgGraphicsElement", feature = "SvgStringList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_required_extensions_SVGGraphicsElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgGraphicsElement as WasmDescribe>::describe();
    <SvgStringList as WasmDescribe>::describe();
}
impl SvgGraphicsElement {
    #[cfg(all(feature = "SvgGraphicsElement", feature = "SvgStringList",))]
    #[allow(bad_style)]
    #[doc = "The `requiredExtensions` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/requiredExtensions)\n\n*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgStringList`*"]
    #[allow(clippy::all)]
    pub fn required_extensions(&self) -> SvgStringList {
        #[cfg(all(feature = "SvgGraphicsElement", feature = "SvgStringList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_required_extensions_SVGGraphicsElement(
                self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_required_extensions_SVGGraphicsElement(
            self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_required_extensions_SVGGraphicsElement(self_)
            };
            <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgGraphicsElement", feature = "SvgStringList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_system_language_SVGGraphicsElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgGraphicsElement as WasmDescribe>::describe();
    <SvgStringList as WasmDescribe>::describe();
}
impl SvgGraphicsElement {
    #[cfg(all(feature = "SvgGraphicsElement", feature = "SvgStringList",))]
    #[allow(bad_style)]
    #[doc = "The `systemLanguage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/systemLanguage)\n\n*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgStringList`*"]
    #[allow(clippy::all)]
    pub fn system_language(&self) -> SvgStringList {
        #[cfg(all(feature = "SvgGraphicsElement", feature = "SvgStringList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_system_language_SVGGraphicsElement(
                self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_system_language_SVGGraphicsElement(
            self_: <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgGraphicsElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_system_language_SVGGraphicsElement(self_)
            };
            <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_ecc04b34e2b53787: [u8; 1472usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}~\x05\0\0\0\0\r\0\0\x02\x12SVGGraphicsElement$__widl_instanceof_SVGGraphicsElement\0\0\0\0%__widl_f_get_b_box_SVGGraphicsElement\x01\0\0\x01\x12SVGGraphicsElement\x01\0\0\x01\x01\x05self_\x07getBBox\0\0\04__widl_f_get_b_box_with_a_options_SVGGraphicsElement\x01\0\0\x01\x12SVGGraphicsElement\x01\0\0\x01\x02\x05self_\ta_options\x07getBBox\0\0\0#__widl_f_get_ctm_SVGGraphicsElement\0\0\0\x01\x12SVGGraphicsElement\x01\0\0\x01\x01\x05self_\x06getCTM\0\0\0*__widl_f_get_screen_ctm_SVGGraphicsElement\0\0\0\x01\x12SVGGraphicsElement\x01\0\0\x01\x01\x05self_\x0CgetScreenCTM\0\0\04__widl_f_get_transform_to_element_SVGGraphicsElement\x01\0\0\x01\x12SVGGraphicsElement\x01\0\0\x01\x02\x05self_\x07element\x15getTransformToElement\0\0\0%__widl_f_transform_SVGGraphicsElement\0\0\0\x01\x12SVGGraphicsElement\x01\0\x01\ttransform\x01\x01\x05self_\ttransform\0\0\04__widl_f_nearest_viewport_element_SVGGraphicsElement\0\0\0\x01\x12SVGGraphicsElement\x01\0\x01\x16nearestViewportElement\x01\x01\x05self_\x16nearestViewportElement\0\0\05__widl_f_farthest_viewport_element_SVGGraphicsElement\0\0\0\x01\x12SVGGraphicsElement\x01\0\x01\x17farthestViewportElement\x01\x01\x05self_\x17farthestViewportElement\0\0\0)__widl_f_has_extension_SVGGraphicsElement\0\0\0\x01\x12SVGGraphicsElement\x01\0\0\x01\x02\x05self_\textension\x0ChasExtension\0\0\0-__widl_f_required_features_SVGGraphicsElement\0\0\0\x01\x12SVGGraphicsElement\x01\0\x01\x10requiredFeatures\x01\x01\x05self_\x10requiredFeatures\0\0\0/__widl_f_required_extensions_SVGGraphicsElement\0\0\0\x01\x12SVGGraphicsElement\x01\0\x01\x12requiredExtensions\x01\x01\x05self_\x12requiredExtensions\0\0\0+__widl_f_system_language_SVGGraphicsElement\0\0\0\x01\x12SVGGraphicsElement\x01\0\x01\x0EsystemLanguage\x01\x01\x05self_\x0EsystemLanguage\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
