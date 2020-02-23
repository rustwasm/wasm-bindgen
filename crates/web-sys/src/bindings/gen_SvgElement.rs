use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgElement {
    obj: Element,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgElement {
        type Target = Element;
        #[inline]
        fn deref(&self) -> &Element {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgElement {
        #[inline]
        fn from(obj: JsValue) -> SvgElement {
            SvgElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgElement> for SvgElement {
        #[inline]
        fn as_ref(&self) -> &SvgElement {
            self
        }
    }
    impl From<SvgElement> for JsValue {
        #[inline]
        fn from(obj: SvgElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgElement> for Element {
    #[inline]
    fn from(obj: SvgElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgElement> for Node {
    #[inline]
    fn from(obj: SvgElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgElement> for EventTarget {
    #[inline]
    fn from(obj: SvgElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_blur_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `blur()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/blur)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn blur(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_blur_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_blur_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_blur_SVGElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_focus_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `focus()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/focus)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn focus(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_focus_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_focus_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_focus_SVGElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_id_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `id` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/id)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn id(&self) -> String {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_id_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_id_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_id_SVGElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_id_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `id` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/id)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_id(&self, id: &str) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_id_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_id_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(id);
                __widl_f_set_id_SVGElement(self_, id)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgAnimatedString", feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_class_name_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <SvgAnimatedString as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgAnimatedString", feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `className` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/className)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn class_name(&self) -> SvgAnimatedString {
        #[cfg(all(feature = "SvgAnimatedString", feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_class_name_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_class_name_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_class_name_SVGElement(self_)
            };
            <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomStringMap", feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dataset_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <DomStringMap as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "DomStringMap", feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `dataset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/dataset)\n\n*This API requires the following crate features to be activated: `DomStringMap`, `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn dataset(&self) -> DomStringMap {
        #[cfg(all(feature = "DomStringMap", feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dataset_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomStringMap as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dataset_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomStringMap as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_dataset_SVGElement(self_)
            };
            <DomStringMap as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssStyleDeclaration", feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_style_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <CssStyleDeclaration as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "CssStyleDeclaration", feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `style` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/style)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`, `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn style(&self) -> CssStyleDeclaration {
        #[cfg(all(feature = "CssStyleDeclaration", feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_style_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <CssStyleDeclaration as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_style_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <CssStyleDeclaration as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_style_SVGElement(self_)
            };
            <CssStyleDeclaration as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement", feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_owner_svg_element_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<SvgsvgElement> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement", feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ownerSVGElement` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ownerSVGElement)\n\n*This API requires the following crate features to be activated: `SvgElement`, `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn owner_svg_element(&self) -> Option<SvgsvgElement> {
        #[cfg(all(feature = "SvgElement", feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_owner_svg_element_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<SvgsvgElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_owner_svg_element_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<SvgsvgElement> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_owner_svg_element_SVGElement(self_)
            };
            <Option<SvgsvgElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_viewport_element_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<SvgElement> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `viewportElement` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/viewportElement)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn viewport_element(&self) -> Option<SvgElement> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_viewport_element_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<SvgElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_viewport_element_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<SvgElement> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_viewport_element_SVGElement(self_)
            };
            <Option<SvgElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tab_index_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `tabIndex` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/tabIndex)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn tab_index(&self) -> i32 {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tab_index_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tab_index_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_tab_index_SVGElement(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_tab_index_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `tabIndex` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/tabIndex)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_tab_index(&self, tab_index: i32) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_tab_index_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tab_index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_tab_index_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tab_index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tab_index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tab_index = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tab_index);
                __widl_f_set_tab_index_SVGElement(self_, tab_index)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncopy_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `oncopy` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncopy)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn oncopy(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncopy_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncopy_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncopy_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncopy_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `oncopy` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncopy)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_oncopy(&self, oncopy: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncopy_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncopy: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncopy_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oncopy: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oncopy);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncopy =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncopy,
                    );
                __widl_f_set_oncopy_SVGElement(self_, oncopy)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncut_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `oncut` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncut)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn oncut(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncut_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncut_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncut_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncut_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `oncut` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncut)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_oncut(&self, oncut: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncut_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncut: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncut_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oncut: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oncut);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncut =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncut,
                    );
                __widl_f_set_oncut_SVGElement(self_, oncut)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpaste_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpaste` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpaste)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onpaste(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpaste_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpaste_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpaste_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpaste_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpaste` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpaste)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpaste(&self, onpaste: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpaste_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpaste: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpaste_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpaste: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpaste);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpaste =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpaste,
                    );
                __widl_f_set_onpaste_SVGElement(self_, onpaste)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onabort_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onabort` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onabort)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onabort(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onabort_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onabort_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onabort_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onabort_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onabort` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onabort)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onabort(&self, onabort: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onabort_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onabort: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onabort_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onabort: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onabort);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onabort =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onabort,
                    );
                __widl_f_set_onabort_SVGElement(self_, onabort)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onblur_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onblur` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onblur)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onblur(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onblur_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onblur_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onblur_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onblur_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onblur` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onblur)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onblur(&self, onblur: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onblur_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onblur: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onblur_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onblur: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onblur);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onblur =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onblur,
                    );
                __widl_f_set_onblur_SVGElement(self_, onblur)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onfocus_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onfocus` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onfocus)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onfocus(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onfocus_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onfocus_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onfocus_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onfocus_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onfocus` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onfocus)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onfocus(&self, onfocus: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onfocus_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onfocus: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onfocus_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onfocus: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onfocus);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onfocus =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onfocus,
                    );
                __widl_f_set_onfocus_SVGElement(self_, onfocus)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onauxclick_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onauxclick` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onauxclick)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onauxclick(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onauxclick_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onauxclick_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onauxclick_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onauxclick_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onauxclick` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onauxclick)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onauxclick(&self, onauxclick: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onauxclick_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onauxclick : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onauxclick_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onauxclick: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onauxclick);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onauxclick =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onauxclick,
                    );
                __widl_f_set_onauxclick_SVGElement(self_, onauxclick)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncanplay_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `oncanplay` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncanplay)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn oncanplay(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncanplay_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncanplay_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncanplay_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncanplay_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `oncanplay` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncanplay)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_oncanplay(&self, oncanplay: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncanplay_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncanplay: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncanplay_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oncanplay: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oncanplay);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncanplay =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncanplay,
                    );
                __widl_f_set_oncanplay_SVGElement(self_, oncanplay)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncanplaythrough_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `oncanplaythrough` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncanplaythrough)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn oncanplaythrough(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncanplaythrough_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncanplaythrough_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncanplaythrough_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncanplaythrough_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `oncanplaythrough` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncanplaythrough)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_oncanplaythrough(&self, oncanplaythrough: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncanplaythrough_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncanplaythrough : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncanplaythrough_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oncanplaythrough : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(oncanplaythrough);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncanplaythrough =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncanplaythrough,
                    );
                __widl_f_set_oncanplaythrough_SVGElement(self_, oncanplaythrough)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onchange_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onchange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onchange)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onchange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onchange_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onchange_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onchange_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onchange_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onchange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onchange)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onchange(&self, onchange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onchange_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onchange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onchange_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onchange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onchange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onchange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onchange,
                    );
                __widl_f_set_onchange_SVGElement(self_, onchange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onclick_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onclick` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onclick)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onclick(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onclick_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onclick_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onclick_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onclick_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onclick` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onclick)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onclick(&self, onclick: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onclick_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onclick: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onclick_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onclick: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onclick);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onclick =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onclick,
                    );
                __widl_f_set_onclick_SVGElement(self_, onclick)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onclose_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onclose` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onclose)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onclose(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onclose_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onclose_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onclose_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onclose_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onclose` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onclose)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onclose(&self, onclose: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onclose_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onclose: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onclose_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onclose: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onclose);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onclose =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onclose,
                    );
                __widl_f_set_onclose_SVGElement(self_, onclose)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncontextmenu_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `oncontextmenu` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncontextmenu)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn oncontextmenu(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncontextmenu_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncontextmenu_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncontextmenu_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncontextmenu_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `oncontextmenu` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncontextmenu)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_oncontextmenu(&self, oncontextmenu: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncontextmenu_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncontextmenu : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncontextmenu_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oncontextmenu: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oncontextmenu);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncontextmenu =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncontextmenu,
                    );
                __widl_f_set_oncontextmenu_SVGElement(self_, oncontextmenu)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondblclick_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondblclick` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondblclick)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn ondblclick(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondblclick_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondblclick_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondblclick_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondblclick_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondblclick` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondblclick)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_ondblclick(&self, ondblclick: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondblclick_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondblclick : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondblclick_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondblclick: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondblclick);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondblclick =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondblclick,
                    );
                __widl_f_set_ondblclick_SVGElement(self_, ondblclick)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondrag_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondrag` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondrag)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn ondrag(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondrag_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondrag_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondrag_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondrag_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondrag` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondrag)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_ondrag(&self, ondrag: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondrag_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondrag: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondrag_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondrag: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondrag);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondrag =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondrag,
                    );
                __widl_f_set_ondrag_SVGElement(self_, ondrag)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondragend_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn ondragend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondragend_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondragend_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondragend_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondragend_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_ondragend(&self, ondragend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondragend_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondragend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondragend_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondragend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondragend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondragend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondragend,
                    );
                __widl_f_set_ondragend_SVGElement(self_, ondragend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondragenter_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragenter` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragenter)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn ondragenter(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondragenter_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondragenter_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondragenter_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondragenter_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragenter` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragenter)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_ondragenter(&self, ondragenter: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondragenter_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondragenter : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondragenter_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondragenter: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondragenter);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondragenter =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondragenter,
                    );
                __widl_f_set_ondragenter_SVGElement(self_, ondragenter)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondragexit_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragexit` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragexit)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn ondragexit(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondragexit_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondragexit_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondragexit_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondragexit_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragexit` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragexit)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_ondragexit(&self, ondragexit: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondragexit_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondragexit : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondragexit_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondragexit: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondragexit);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondragexit =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondragexit,
                    );
                __widl_f_set_ondragexit_SVGElement(self_, ondragexit)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondragleave_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragleave` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragleave)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn ondragleave(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondragleave_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondragleave_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondragleave_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondragleave_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragleave` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragleave)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_ondragleave(&self, ondragleave: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondragleave_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondragleave : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondragleave_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondragleave: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondragleave);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondragleave =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondragleave,
                    );
                __widl_f_set_ondragleave_SVGElement(self_, ondragleave)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondragover_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragover` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragover)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn ondragover(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondragover_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondragover_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondragover_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondragover_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragover` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragover)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_ondragover(&self, ondragover: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondragover_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondragover : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondragover_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondragover: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondragover);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondragover =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondragover,
                    );
                __widl_f_set_ondragover_SVGElement(self_, ondragover)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondragstart_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn ondragstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondragstart_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondragstart_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondragstart_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondragstart_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_ondragstart(&self, ondragstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondragstart_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondragstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondragstart_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondragstart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondragstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondragstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondragstart,
                    );
                __widl_f_set_ondragstart_SVGElement(self_, ondragstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondrop_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondrop` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondrop)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn ondrop(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondrop_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondrop_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondrop_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondrop_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondrop` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondrop)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_ondrop(&self, ondrop: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondrop_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondrop: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondrop_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondrop: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondrop);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondrop =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondrop,
                    );
                __widl_f_set_ondrop_SVGElement(self_, ondrop)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondurationchange_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondurationchange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondurationchange)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn ondurationchange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondurationchange_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondurationchange_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondurationchange_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondurationchange_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondurationchange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondurationchange)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_ondurationchange(&self, ondurationchange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondurationchange_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondurationchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondurationchange_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondurationchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(ondurationchange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondurationchange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondurationchange,
                    );
                __widl_f_set_ondurationchange_SVGElement(self_, ondurationchange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onemptied_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onemptied` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onemptied)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onemptied(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onemptied_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onemptied_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onemptied_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onemptied_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onemptied` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onemptied)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onemptied(&self, onemptied: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onemptied_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onemptied: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onemptied_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onemptied: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onemptied);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onemptied =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onemptied,
                    );
                __widl_f_set_onemptied_SVGElement(self_, onemptied)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onended_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onended` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onended)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onended(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onended_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onended_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onended_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onended_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onended` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onended)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onended(&self, onended: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onended_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onended: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onended_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onended: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onended);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onended =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onended,
                    );
                __widl_f_set_onended_SVGElement(self_, onended)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oninput_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `oninput` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oninput)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn oninput(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oninput_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oninput_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oninput_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oninput_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `oninput` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oninput)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_oninput(&self, oninput: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oninput_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oninput: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oninput_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oninput: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oninput);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oninput =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oninput,
                    );
                __widl_f_set_oninput_SVGElement(self_, oninput)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oninvalid_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `oninvalid` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oninvalid)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn oninvalid(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oninvalid_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oninvalid_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oninvalid_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oninvalid_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `oninvalid` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oninvalid)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_oninvalid(&self, oninvalid: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oninvalid_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oninvalid: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oninvalid_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oninvalid: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oninvalid);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oninvalid =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oninvalid,
                    );
                __widl_f_set_oninvalid_SVGElement(self_, oninvalid)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onkeydown_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onkeydown` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeydown)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onkeydown(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onkeydown_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onkeydown_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onkeydown_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onkeydown_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onkeydown` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeydown)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onkeydown(&self, onkeydown: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onkeydown_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onkeydown: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onkeydown_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onkeydown: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onkeydown);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onkeydown =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onkeydown,
                    );
                __widl_f_set_onkeydown_SVGElement(self_, onkeydown)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onkeypress_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onkeypress` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeypress)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onkeypress(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onkeypress_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onkeypress_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onkeypress_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onkeypress_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onkeypress` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeypress)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onkeypress(&self, onkeypress: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onkeypress_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onkeypress : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onkeypress_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onkeypress: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onkeypress);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onkeypress =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onkeypress,
                    );
                __widl_f_set_onkeypress_SVGElement(self_, onkeypress)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onkeyup_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onkeyup` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeyup)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onkeyup(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onkeyup_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onkeyup_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onkeyup_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onkeyup_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onkeyup` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeyup)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onkeyup(&self, onkeyup: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onkeyup_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onkeyup: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onkeyup_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onkeyup: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onkeyup);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onkeyup =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onkeyup,
                    );
                __widl_f_set_onkeyup_SVGElement(self_, onkeyup)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onload_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onload` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onload)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onload(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onload_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onload_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onload_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onload_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onload` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onload)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onload(&self, onload: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onload_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onload: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onload_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onload: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onload);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onload =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onload,
                    );
                __widl_f_set_onload_SVGElement(self_, onload)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onloadeddata_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onloadeddata` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadeddata)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onloadeddata(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onloadeddata_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onloadeddata_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onloadeddata_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onloadeddata_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onloadeddata` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadeddata)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onloadeddata(&self, onloadeddata: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onloadeddata_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onloadeddata : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onloadeddata_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onloadeddata: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onloadeddata);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onloadeddata =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onloadeddata,
                    );
                __widl_f_set_onloadeddata_SVGElement(self_, onloadeddata)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onloadedmetadata_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onloadedmetadata` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadedmetadata)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onloadedmetadata(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onloadedmetadata_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onloadedmetadata_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onloadedmetadata_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onloadedmetadata_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onloadedmetadata` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadedmetadata)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onloadedmetadata(&self, onloadedmetadata: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onloadedmetadata_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onloadedmetadata : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onloadedmetadata_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onloadedmetadata : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onloadedmetadata);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onloadedmetadata =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onloadedmetadata,
                    );
                __widl_f_set_onloadedmetadata_SVGElement(self_, onloadedmetadata)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onloadend_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onloadend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onloadend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onloadend_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onloadend_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onloadend_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onloadend_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onloadend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onloadend(&self, onloadend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onloadend_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onloadend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onloadend_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onloadend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onloadend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onloadend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onloadend,
                    );
                __widl_f_set_onloadend_SVGElement(self_, onloadend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onloadstart_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onloadstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onloadstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onloadstart_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onloadstart_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onloadstart_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onloadstart_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onloadstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onloadstart(&self, onloadstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onloadstart_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onloadstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onloadstart_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onloadstart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onloadstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onloadstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onloadstart,
                    );
                __widl_f_set_onloadstart_SVGElement(self_, onloadstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmousedown_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmousedown` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmousedown)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onmousedown(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmousedown_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmousedown_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmousedown_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmousedown_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmousedown` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmousedown)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onmousedown(&self, onmousedown: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmousedown_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmousedown : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmousedown_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmousedown: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmousedown);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmousedown =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmousedown,
                    );
                __widl_f_set_onmousedown_SVGElement(self_, onmousedown)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmouseenter_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseenter` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseenter)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onmouseenter(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmouseenter_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmouseenter_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmouseenter_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmouseenter_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseenter` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseenter)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onmouseenter(&self, onmouseenter: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmouseenter_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmouseenter : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmouseenter_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmouseenter: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmouseenter);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmouseenter =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmouseenter,
                    );
                __widl_f_set_onmouseenter_SVGElement(self_, onmouseenter)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmouseleave_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseleave` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseleave)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onmouseleave(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmouseleave_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmouseleave_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmouseleave_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmouseleave_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseleave` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseleave)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onmouseleave(&self, onmouseleave: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmouseleave_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmouseleave : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmouseleave_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmouseleave: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmouseleave);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmouseleave =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmouseleave,
                    );
                __widl_f_set_onmouseleave_SVGElement(self_, onmouseleave)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmousemove_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmousemove` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmousemove)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onmousemove(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmousemove_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmousemove_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmousemove_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmousemove_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmousemove` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmousemove)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onmousemove(&self, onmousemove: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmousemove_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmousemove : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmousemove_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmousemove: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmousemove);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmousemove =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmousemove,
                    );
                __widl_f_set_onmousemove_SVGElement(self_, onmousemove)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmouseout_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseout` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseout)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onmouseout(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmouseout_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmouseout_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmouseout_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmouseout_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseout` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseout)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onmouseout(&self, onmouseout: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmouseout_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmouseout : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmouseout_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmouseout: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmouseout);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmouseout =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmouseout,
                    );
                __widl_f_set_onmouseout_SVGElement(self_, onmouseout)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmouseover_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseover` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseover)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onmouseover(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmouseover_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmouseover_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmouseover_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmouseover_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseover` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseover)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onmouseover(&self, onmouseover: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmouseover_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmouseover : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmouseover_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmouseover: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmouseover);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmouseover =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmouseover,
                    );
                __widl_f_set_onmouseover_SVGElement(self_, onmouseover)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmouseup_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseup` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseup)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onmouseup(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmouseup_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmouseup_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmouseup_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmouseup_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseup` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseup)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onmouseup(&self, onmouseup: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmouseup_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmouseup: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmouseup_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmouseup: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmouseup);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmouseup =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmouseup,
                    );
                __widl_f_set_onmouseup_SVGElement(self_, onmouseup)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onwheel_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwheel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwheel)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onwheel(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onwheel_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onwheel_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onwheel_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onwheel_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwheel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwheel)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onwheel(&self, onwheel: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onwheel_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onwheel: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onwheel_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onwheel: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onwheel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onwheel =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onwheel,
                    );
                __widl_f_set_onwheel_SVGElement(self_, onwheel)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpause_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpause` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpause)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onpause(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpause_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpause_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpause_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpause_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpause` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpause)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpause(&self, onpause: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpause_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpause: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpause_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpause: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpause);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpause =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpause,
                    );
                __widl_f_set_onpause_SVGElement(self_, onpause)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onplay_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onplay` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onplay)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onplay(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onplay_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onplay_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onplay_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onplay_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onplay` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onplay)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onplay(&self, onplay: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onplay_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onplay: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onplay_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onplay: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onplay);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onplay =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onplay,
                    );
                __widl_f_set_onplay_SVGElement(self_, onplay)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onplaying_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onplaying` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onplaying)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onplaying(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onplaying_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onplaying_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onplaying_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onplaying_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onplaying` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onplaying)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onplaying(&self, onplaying: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onplaying_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onplaying: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onplaying_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onplaying: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onplaying);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onplaying =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onplaying,
                    );
                __widl_f_set_onplaying_SVGElement(self_, onplaying)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onprogress_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onprogress` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onprogress)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onprogress(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onprogress_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onprogress_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onprogress_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onprogress_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onprogress` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onprogress)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onprogress(&self, onprogress: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onprogress_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onprogress : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onprogress_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onprogress: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onprogress);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onprogress =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onprogress,
                    );
                __widl_f_set_onprogress_SVGElement(self_, onprogress)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onratechange_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onratechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onratechange)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onratechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onratechange_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onratechange_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onratechange_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onratechange_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onratechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onratechange)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onratechange(&self, onratechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onratechange_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onratechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onratechange_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onratechange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onratechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onratechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onratechange,
                    );
                __widl_f_set_onratechange_SVGElement(self_, onratechange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onreset_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onreset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onreset)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onreset(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onreset_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onreset_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onreset_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onreset_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onreset` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onreset)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onreset(&self, onreset: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onreset_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onreset: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onreset_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onreset: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onreset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onreset =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onreset,
                    );
                __widl_f_set_onreset_SVGElement(self_, onreset)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onresize_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onresize` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onresize)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onresize(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onresize_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onresize_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onresize_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onresize_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onresize` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onresize)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onresize(&self, onresize: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onresize_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onresize: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onresize_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onresize: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onresize);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onresize =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onresize,
                    );
                __widl_f_set_onresize_SVGElement(self_, onresize)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onscroll_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onscroll` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onscroll)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onscroll(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onscroll_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onscroll_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onscroll_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onscroll_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onscroll` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onscroll)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onscroll(&self, onscroll: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onscroll_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onscroll: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onscroll_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onscroll: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onscroll);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onscroll =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onscroll,
                    );
                __widl_f_set_onscroll_SVGElement(self_, onscroll)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onseeked_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onseeked` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onseeked)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onseeked(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onseeked_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onseeked_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onseeked_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onseeked_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onseeked` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onseeked)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onseeked(&self, onseeked: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onseeked_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onseeked: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onseeked_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onseeked: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onseeked);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onseeked =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onseeked,
                    );
                __widl_f_set_onseeked_SVGElement(self_, onseeked)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onseeking_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onseeking` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onseeking)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onseeking(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onseeking_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onseeking_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onseeking_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onseeking_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onseeking` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onseeking)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onseeking(&self, onseeking: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onseeking_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onseeking: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onseeking_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onseeking: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onseeking);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onseeking =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onseeking,
                    );
                __widl_f_set_onseeking_SVGElement(self_, onseeking)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onselect_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onselect` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onselect)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onselect(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onselect_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onselect_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onselect_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onselect_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onselect` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onselect)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onselect(&self, onselect: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onselect_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onselect: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onselect_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onselect: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onselect);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onselect =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onselect,
                    );
                __widl_f_set_onselect_SVGElement(self_, onselect)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onshow_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onshow` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onshow)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onshow(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onshow_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onshow_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onshow_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onshow_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onshow` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onshow)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onshow(&self, onshow: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onshow_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onshow: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onshow_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onshow: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onshow);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onshow =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onshow,
                    );
                __widl_f_set_onshow_SVGElement(self_, onshow)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onstalled_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onstalled` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onstalled)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onstalled(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onstalled_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onstalled_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onstalled_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onstalled_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onstalled` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onstalled)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onstalled(&self, onstalled: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onstalled_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onstalled: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onstalled_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onstalled: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onstalled);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onstalled =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onstalled,
                    );
                __widl_f_set_onstalled_SVGElement(self_, onstalled)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onsubmit_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onsubmit` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onsubmit)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onsubmit(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onsubmit_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onsubmit_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onsubmit_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onsubmit_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onsubmit` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onsubmit)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onsubmit(&self, onsubmit: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onsubmit_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onsubmit: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onsubmit_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onsubmit: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onsubmit);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onsubmit =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onsubmit,
                    );
                __widl_f_set_onsubmit_SVGElement(self_, onsubmit)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onsuspend_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onsuspend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onsuspend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onsuspend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onsuspend_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onsuspend_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onsuspend_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onsuspend_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onsuspend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onsuspend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onsuspend(&self, onsuspend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onsuspend_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onsuspend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onsuspend_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onsuspend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onsuspend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onsuspend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onsuspend,
                    );
                __widl_f_set_onsuspend_SVGElement(self_, onsuspend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontimeupdate_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontimeupdate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontimeupdate)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn ontimeupdate(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontimeupdate_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontimeupdate_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontimeupdate_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontimeupdate_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontimeupdate` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontimeupdate)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_ontimeupdate(&self, ontimeupdate: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontimeupdate_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontimeupdate : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontimeupdate_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontimeupdate: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontimeupdate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontimeupdate =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontimeupdate,
                    );
                __widl_f_set_ontimeupdate_SVGElement(self_, ontimeupdate)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onvolumechange_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onvolumechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onvolumechange)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onvolumechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onvolumechange_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onvolumechange_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onvolumechange_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onvolumechange_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onvolumechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onvolumechange)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onvolumechange(&self, onvolumechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onvolumechange_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onvolumechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onvolumechange_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onvolumechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onvolumechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onvolumechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onvolumechange,
                    );
                __widl_f_set_onvolumechange_SVGElement(self_, onvolumechange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onwaiting_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwaiting` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwaiting)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onwaiting(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onwaiting_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onwaiting_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onwaiting_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onwaiting_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwaiting` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwaiting)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onwaiting(&self, onwaiting: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onwaiting_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onwaiting: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onwaiting_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onwaiting: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onwaiting);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onwaiting =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onwaiting,
                    );
                __widl_f_set_onwaiting_SVGElement(self_, onwaiting)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onselectstart_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onselectstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onselectstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onselectstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onselectstart_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onselectstart_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onselectstart_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onselectstart_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onselectstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onselectstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onselectstart(&self, onselectstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onselectstart_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onselectstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onselectstart_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onselectstart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onselectstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onselectstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onselectstart,
                    );
                __widl_f_set_onselectstart_SVGElement(self_, onselectstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontoggle_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontoggle` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontoggle)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn ontoggle(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontoggle_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontoggle_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontoggle_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontoggle_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontoggle` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontoggle)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_ontoggle(&self, ontoggle: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontoggle_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontoggle: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontoggle_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontoggle: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontoggle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontoggle =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontoggle,
                    );
                __widl_f_set_ontoggle_SVGElement(self_, ontoggle)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointercancel_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointercancel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointercancel)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onpointercancel(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointercancel_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointercancel_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointercancel_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointercancel_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointercancel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointercancel)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpointercancel(&self, onpointercancel: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointercancel_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointercancel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointercancel_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointercancel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onpointercancel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointercancel =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointercancel,
                    );
                __widl_f_set_onpointercancel_SVGElement(self_, onpointercancel)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointerdown_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerdown` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerdown)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onpointerdown(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointerdown_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointerdown_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointerdown_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointerdown_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerdown` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerdown)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpointerdown(&self, onpointerdown: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointerdown_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointerdown : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointerdown_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointerdown: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpointerdown);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointerdown =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointerdown,
                    );
                __widl_f_set_onpointerdown_SVGElement(self_, onpointerdown)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointerup_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerup` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerup)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onpointerup(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointerup_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointerup_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointerup_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointerup_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerup` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerup)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpointerup(&self, onpointerup: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointerup_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointerup : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointerup_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointerup: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpointerup);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointerup =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointerup,
                    );
                __widl_f_set_onpointerup_SVGElement(self_, onpointerup)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointermove_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointermove` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointermove)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onpointermove(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointermove_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointermove_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointermove_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointermove_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointermove` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointermove)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpointermove(&self, onpointermove: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointermove_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointermove : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointermove_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointermove: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpointermove);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointermove =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointermove,
                    );
                __widl_f_set_onpointermove_SVGElement(self_, onpointermove)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointerout_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerout` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerout)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onpointerout(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointerout_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointerout_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointerout_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointerout_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerout` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerout)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpointerout(&self, onpointerout: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointerout_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointerout : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointerout_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointerout: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpointerout);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointerout =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointerout,
                    );
                __widl_f_set_onpointerout_SVGElement(self_, onpointerout)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointerover_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerover` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerover)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onpointerover(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointerover_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointerover_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointerover_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointerover_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerover` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerover)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpointerover(&self, onpointerover: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointerover_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointerover : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointerover_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointerover: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpointerover);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointerover =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointerover,
                    );
                __widl_f_set_onpointerover_SVGElement(self_, onpointerover)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointerenter_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerenter` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerenter)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onpointerenter(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointerenter_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointerenter_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointerenter_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointerenter_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerenter` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerenter)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpointerenter(&self, onpointerenter: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointerenter_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointerenter : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointerenter_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointerenter : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onpointerenter);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointerenter =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointerenter,
                    );
                __widl_f_set_onpointerenter_SVGElement(self_, onpointerenter)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointerleave_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerleave` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerleave)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onpointerleave(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointerleave_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointerleave_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointerleave_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointerleave_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerleave` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerleave)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpointerleave(&self, onpointerleave: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointerleave_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointerleave : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointerleave_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointerleave : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onpointerleave);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointerleave =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointerleave,
                    );
                __widl_f_set_onpointerleave_SVGElement(self_, onpointerleave)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ongotpointercapture_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ongotpointercapture` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ongotpointercapture)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn ongotpointercapture(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ongotpointercapture_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ongotpointercapture_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ongotpointercapture_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ongotpointercapture_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ongotpointercapture` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ongotpointercapture)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_ongotpointercapture(&self, ongotpointercapture: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ongotpointercapture_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ongotpointercapture : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ongotpointercapture_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ongotpointercapture : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(ongotpointercapture);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ongotpointercapture =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ongotpointercapture,
                    );
                __widl_f_set_ongotpointercapture_SVGElement(self_, ongotpointercapture)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onlostpointercapture_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onlostpointercapture` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onlostpointercapture)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onlostpointercapture(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onlostpointercapture_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onlostpointercapture_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onlostpointercapture_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onlostpointercapture_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onlostpointercapture` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onlostpointercapture)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onlostpointercapture(&self, onlostpointercapture: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onlostpointercapture_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onlostpointercapture : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onlostpointercapture_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onlostpointercapture : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onlostpointercapture);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onlostpointercapture =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onlostpointercapture,
                    );
                __widl_f_set_onlostpointercapture_SVGElement(self_, onlostpointercapture)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onanimationcancel_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationcancel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationcancel)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onanimationcancel(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onanimationcancel_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onanimationcancel_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onanimationcancel_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onanimationcancel_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationcancel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationcancel)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onanimationcancel(&self, onanimationcancel: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onanimationcancel_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onanimationcancel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onanimationcancel_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onanimationcancel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onanimationcancel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onanimationcancel =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onanimationcancel,
                    );
                __widl_f_set_onanimationcancel_SVGElement(self_, onanimationcancel)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onanimationend_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onanimationend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onanimationend_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onanimationend_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onanimationend_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onanimationend_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onanimationend(&self, onanimationend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onanimationend_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onanimationend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onanimationend_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onanimationend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onanimationend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onanimationend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onanimationend,
                    );
                __widl_f_set_onanimationend_SVGElement(self_, onanimationend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onanimationiteration_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationiteration` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationiteration)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onanimationiteration(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onanimationiteration_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onanimationiteration_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onanimationiteration_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onanimationiteration_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationiteration` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationiteration)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onanimationiteration(&self, onanimationiteration: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onanimationiteration_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onanimationiteration : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onanimationiteration_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onanimationiteration : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onanimationiteration);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onanimationiteration =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onanimationiteration,
                    );
                __widl_f_set_onanimationiteration_SVGElement(self_, onanimationiteration)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onanimationstart_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onanimationstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onanimationstart_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onanimationstart_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onanimationstart_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onanimationstart_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onanimationstart(&self, onanimationstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onanimationstart_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onanimationstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onanimationstart_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onanimationstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onanimationstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onanimationstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onanimationstart,
                    );
                __widl_f_set_onanimationstart_SVGElement(self_, onanimationstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontransitioncancel_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitioncancel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitioncancel)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn ontransitioncancel(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontransitioncancel_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontransitioncancel_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontransitioncancel_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontransitioncancel_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitioncancel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitioncancel)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_ontransitioncancel(&self, ontransitioncancel: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontransitioncancel_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontransitioncancel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontransitioncancel_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontransitioncancel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(ontransitioncancel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontransitioncancel =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontransitioncancel,
                    );
                __widl_f_set_ontransitioncancel_SVGElement(self_, ontransitioncancel)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontransitionend_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitionend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn ontransitionend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontransitionend_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontransitionend_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontransitionend_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontransitionend_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitionend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_ontransitionend(&self, ontransitionend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontransitionend_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontransitionend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontransitionend_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontransitionend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(ontransitionend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontransitionend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontransitionend,
                    );
                __widl_f_set_ontransitionend_SVGElement(self_, ontransitionend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontransitionrun_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitionrun` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionrun)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn ontransitionrun(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontransitionrun_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontransitionrun_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontransitionrun_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontransitionrun_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitionrun` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionrun)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_ontransitionrun(&self, ontransitionrun: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontransitionrun_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontransitionrun : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontransitionrun_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontransitionrun : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(ontransitionrun);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontransitionrun =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontransitionrun,
                    );
                __widl_f_set_ontransitionrun_SVGElement(self_, ontransitionrun)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontransitionstart_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitionstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn ontransitionstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontransitionstart_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontransitionstart_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontransitionstart_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontransitionstart_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitionstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_ontransitionstart(&self, ontransitionstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontransitionstart_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontransitionstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontransitionstart_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontransitionstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(ontransitionstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontransitionstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontransitionstart,
                    );
                __widl_f_set_ontransitionstart_SVGElement(self_, ontransitionstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onwebkitanimationend_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkitanimationend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onwebkitanimationend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onwebkitanimationend_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onwebkitanimationend_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onwebkitanimationend_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onwebkitanimationend_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkitanimationend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onwebkitanimationend(&self, onwebkitanimationend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onwebkitanimationend_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onwebkitanimationend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onwebkitanimationend_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onwebkitanimationend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onwebkitanimationend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onwebkitanimationend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onwebkitanimationend,
                    );
                __widl_f_set_onwebkitanimationend_SVGElement(self_, onwebkitanimationend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onwebkitanimationiteration_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkitanimationiteration` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationiteration)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onwebkitanimationiteration(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onwebkitanimationiteration_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onwebkitanimationiteration_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onwebkitanimationiteration_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onwebkitanimationiteration_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkitanimationiteration` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationiteration)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onwebkitanimationiteration(
        &self,
        onwebkitanimationiteration: Option<&::js_sys::Function>,
    ) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onwebkitanimationiteration_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onwebkitanimationiteration : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onwebkitanimationiteration_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onwebkitanimationiteration : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onwebkitanimationiteration);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onwebkitanimationiteration =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onwebkitanimationiteration,
                    );
                __widl_f_set_onwebkitanimationiteration_SVGElement(
                    self_,
                    onwebkitanimationiteration,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onwebkitanimationstart_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkitanimationstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onwebkitanimationstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onwebkitanimationstart_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onwebkitanimationstart_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onwebkitanimationstart_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onwebkitanimationstart_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkitanimationstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onwebkitanimationstart(&self, onwebkitanimationstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onwebkitanimationstart_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onwebkitanimationstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onwebkitanimationstart_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onwebkitanimationstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onwebkitanimationstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onwebkitanimationstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onwebkitanimationstart,
                    );
                __widl_f_set_onwebkitanimationstart_SVGElement(self_, onwebkitanimationstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onwebkittransitionend_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkittransitionend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkittransitionend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onwebkittransitionend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onwebkittransitionend_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onwebkittransitionend_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onwebkittransitionend_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onwebkittransitionend_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkittransitionend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkittransitionend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onwebkittransitionend(&self, onwebkittransitionend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onwebkittransitionend_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onwebkittransitionend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onwebkittransitionend_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onwebkittransitionend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onwebkittransitionend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onwebkittransitionend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onwebkittransitionend,
                    );
                __widl_f_set_onwebkittransitionend_SVGElement(self_, onwebkittransitionend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onerror)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onerror)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onerror);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_SVGElement(self_, onerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontouchstart_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn ontouchstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontouchstart_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontouchstart_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontouchstart_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontouchstart_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchstart)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_ontouchstart(&self, ontouchstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontouchstart_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontouchstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontouchstart_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontouchstart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontouchstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontouchstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontouchstart,
                    );
                __widl_f_set_ontouchstart_SVGElement(self_, ontouchstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontouchend_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn ontouchend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontouchend_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontouchend_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontouchend_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontouchend_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchend)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_ontouchend(&self, ontouchend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontouchend_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontouchend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontouchend_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontouchend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontouchend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontouchend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontouchend,
                    );
                __widl_f_set_ontouchend_SVGElement(self_, ontouchend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontouchmove_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchmove` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchmove)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn ontouchmove(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontouchmove_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontouchmove_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontouchmove_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontouchmove_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchmove` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchmove)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_ontouchmove(&self, ontouchmove: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontouchmove_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontouchmove : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontouchmove_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontouchmove: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontouchmove);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontouchmove =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontouchmove,
                    );
                __widl_f_set_ontouchmove_SVGElement(self_, ontouchmove)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontouchcancel_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchcancel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchcancel)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn ontouchcancel(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontouchcancel_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontouchcancel_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontouchcancel_SVGElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontouchcancel_SVGElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgElement {
    #[cfg(all(feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchcancel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchcancel)\n\n*This API requires the following crate features to be activated: `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn set_ontouchcancel(&self, ontouchcancel: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontouchcancel_SVGElement(
                self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontouchcancel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontouchcancel_SVGElement(
            self_: <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontouchcancel: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontouchcancel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontouchcancel =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontouchcancel,
                    );
                __widl_f_set_ontouchcancel_SVGElement(self_, ontouchcancel)
            };
            ()
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_35356820d571c487: [u8; 17795usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}AE\0\0\0\0\xC0\x01\0\0\x02\nSVGElement\x1C__widl_instanceof_SVGElement\0\0\0\0\x18__widl_f_blur_SVGElement\x01\0\0\x01\nSVGElement\x01\0\0\x01\x01\x05self_\x04blur\0\0\0\x19__widl_f_focus_SVGElement\x01\0\0\x01\nSVGElement\x01\0\0\x01\x01\x05self_\x05focus\0\0\0\x16__widl_f_id_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x02id\x01\x01\x05self_\x02id\0\0\0\x1A__widl_f_set_id_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x02id\x01\x02\x05self_\x02id\x02id\0\0\0\x1E__widl_f_class_name_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\tclassName\x01\x01\x05self_\tclassName\0\0\0\x1B__widl_f_dataset_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x07dataset\x01\x01\x05self_\x07dataset\0\0\0\x19__widl_f_style_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x05style\x01\x01\x05self_\x05style\0\0\0%__widl_f_owner_svg_element_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0FownerSVGElement\x01\x01\x05self_\x0FownerSVGElement\0\0\0$__widl_f_viewport_element_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0FviewportElement\x01\x01\x05self_\x0FviewportElement\0\0\0\x1D__widl_f_tab_index_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x08tabIndex\x01\x01\x05self_\x08tabIndex\0\0\0!__widl_f_set_tab_index_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x08tabIndex\x01\x02\x05self_\ttab_index\x08tabIndex\0\0\0\x1A__widl_f_oncopy_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x06oncopy\x01\x01\x05self_\x06oncopy\0\0\0\x1E__widl_f_set_oncopy_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x06oncopy\x01\x02\x05self_\x06oncopy\x06oncopy\0\0\0\x19__widl_f_oncut_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x05oncut\x01\x01\x05self_\x05oncut\0\0\0\x1D__widl_f_set_oncut_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x05oncut\x01\x02\x05self_\x05oncut\x05oncut\0\0\0\x1B__widl_f_onpaste_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x07onpaste\x01\x01\x05self_\x07onpaste\0\0\0\x1F__widl_f_set_onpaste_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x07onpaste\x01\x02\x05self_\x07onpaste\x07onpaste\0\0\0\x1B__widl_f_onabort_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x07onabort\x01\x01\x05self_\x07onabort\0\0\0\x1F__widl_f_set_onabort_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x07onabort\x01\x02\x05self_\x07onabort\x07onabort\0\0\0\x1A__widl_f_onblur_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x06onblur\x01\x01\x05self_\x06onblur\0\0\0\x1E__widl_f_set_onblur_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x06onblur\x01\x02\x05self_\x06onblur\x06onblur\0\0\0\x1B__widl_f_onfocus_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x07onfocus\x01\x01\x05self_\x07onfocus\0\0\0\x1F__widl_f_set_onfocus_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x07onfocus\x01\x02\x05self_\x07onfocus\x07onfocus\0\0\0\x1E__widl_f_onauxclick_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\nonauxclick\x01\x01\x05self_\nonauxclick\0\0\0\"__widl_f_set_onauxclick_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\nonauxclick\x01\x02\x05self_\nonauxclick\nonauxclick\0\0\0\x1D__widl_f_oncanplay_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\toncanplay\x01\x01\x05self_\toncanplay\0\0\0!__widl_f_set_oncanplay_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\toncanplay\x01\x02\x05self_\toncanplay\toncanplay\0\0\0$__widl_f_oncanplaythrough_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x10oncanplaythrough\x01\x01\x05self_\x10oncanplaythrough\0\0\0(__widl_f_set_oncanplaythrough_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x10oncanplaythrough\x01\x02\x05self_\x10oncanplaythrough\x10oncanplaythrough\0\0\0\x1C__widl_f_onchange_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x08onchange\x01\x01\x05self_\x08onchange\0\0\0 __widl_f_set_onchange_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x08onchange\x01\x02\x05self_\x08onchange\x08onchange\0\0\0\x1B__widl_f_onclick_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x07onclick\x01\x01\x05self_\x07onclick\0\0\0\x1F__widl_f_set_onclick_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x07onclick\x01\x02\x05self_\x07onclick\x07onclick\0\0\0\x1B__widl_f_onclose_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x07onclose\x01\x01\x05self_\x07onclose\0\0\0\x1F__widl_f_set_onclose_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x07onclose\x01\x02\x05self_\x07onclose\x07onclose\0\0\0!__widl_f_oncontextmenu_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\roncontextmenu\x01\x01\x05self_\roncontextmenu\0\0\0%__widl_f_set_oncontextmenu_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\roncontextmenu\x01\x02\x05self_\roncontextmenu\roncontextmenu\0\0\0\x1E__widl_f_ondblclick_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\nondblclick\x01\x01\x05self_\nondblclick\0\0\0\"__widl_f_set_ondblclick_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\nondblclick\x01\x02\x05self_\nondblclick\nondblclick\0\0\0\x1A__widl_f_ondrag_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x06ondrag\x01\x01\x05self_\x06ondrag\0\0\0\x1E__widl_f_set_ondrag_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x06ondrag\x01\x02\x05self_\x06ondrag\x06ondrag\0\0\0\x1D__widl_f_ondragend_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\tondragend\x01\x01\x05self_\tondragend\0\0\0!__widl_f_set_ondragend_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\tondragend\x01\x02\x05self_\tondragend\tondragend\0\0\0\x1F__widl_f_ondragenter_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Bondragenter\x01\x01\x05self_\x0Bondragenter\0\0\0#__widl_f_set_ondragenter_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Bondragenter\x01\x02\x05self_\x0Bondragenter\x0Bondragenter\0\0\0\x1E__widl_f_ondragexit_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\nondragexit\x01\x01\x05self_\nondragexit\0\0\0\"__widl_f_set_ondragexit_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\nondragexit\x01\x02\x05self_\nondragexit\nondragexit\0\0\0\x1F__widl_f_ondragleave_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Bondragleave\x01\x01\x05self_\x0Bondragleave\0\0\0#__widl_f_set_ondragleave_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Bondragleave\x01\x02\x05self_\x0Bondragleave\x0Bondragleave\0\0\0\x1E__widl_f_ondragover_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\nondragover\x01\x01\x05self_\nondragover\0\0\0\"__widl_f_set_ondragover_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\nondragover\x01\x02\x05self_\nondragover\nondragover\0\0\0\x1F__widl_f_ondragstart_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Bondragstart\x01\x01\x05self_\x0Bondragstart\0\0\0#__widl_f_set_ondragstart_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Bondragstart\x01\x02\x05self_\x0Bondragstart\x0Bondragstart\0\0\0\x1A__widl_f_ondrop_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x06ondrop\x01\x01\x05self_\x06ondrop\0\0\0\x1E__widl_f_set_ondrop_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x06ondrop\x01\x02\x05self_\x06ondrop\x06ondrop\0\0\0$__widl_f_ondurationchange_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x10ondurationchange\x01\x01\x05self_\x10ondurationchange\0\0\0(__widl_f_set_ondurationchange_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x10ondurationchange\x01\x02\x05self_\x10ondurationchange\x10ondurationchange\0\0\0\x1D__widl_f_onemptied_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\tonemptied\x01\x01\x05self_\tonemptied\0\0\0!__widl_f_set_onemptied_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\tonemptied\x01\x02\x05self_\tonemptied\tonemptied\0\0\0\x1B__widl_f_onended_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x07onended\x01\x01\x05self_\x07onended\0\0\0\x1F__widl_f_set_onended_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x07onended\x01\x02\x05self_\x07onended\x07onended\0\0\0\x1B__widl_f_oninput_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x07oninput\x01\x01\x05self_\x07oninput\0\0\0\x1F__widl_f_set_oninput_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x07oninput\x01\x02\x05self_\x07oninput\x07oninput\0\0\0\x1D__widl_f_oninvalid_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\toninvalid\x01\x01\x05self_\toninvalid\0\0\0!__widl_f_set_oninvalid_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\toninvalid\x01\x02\x05self_\toninvalid\toninvalid\0\0\0\x1D__widl_f_onkeydown_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\tonkeydown\x01\x01\x05self_\tonkeydown\0\0\0!__widl_f_set_onkeydown_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\tonkeydown\x01\x02\x05self_\tonkeydown\tonkeydown\0\0\0\x1E__widl_f_onkeypress_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\nonkeypress\x01\x01\x05self_\nonkeypress\0\0\0\"__widl_f_set_onkeypress_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\nonkeypress\x01\x02\x05self_\nonkeypress\nonkeypress\0\0\0\x1B__widl_f_onkeyup_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x07onkeyup\x01\x01\x05self_\x07onkeyup\0\0\0\x1F__widl_f_set_onkeyup_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x07onkeyup\x01\x02\x05self_\x07onkeyup\x07onkeyup\0\0\0\x1A__widl_f_onload_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x06onload\x01\x01\x05self_\x06onload\0\0\0\x1E__widl_f_set_onload_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x06onload\x01\x02\x05self_\x06onload\x06onload\0\0\0 __widl_f_onloadeddata_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Conloadeddata\x01\x01\x05self_\x0Conloadeddata\0\0\0$__widl_f_set_onloadeddata_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Conloadeddata\x01\x02\x05self_\x0Conloadeddata\x0Conloadeddata\0\0\0$__widl_f_onloadedmetadata_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x10onloadedmetadata\x01\x01\x05self_\x10onloadedmetadata\0\0\0(__widl_f_set_onloadedmetadata_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x10onloadedmetadata\x01\x02\x05self_\x10onloadedmetadata\x10onloadedmetadata\0\0\0\x1D__widl_f_onloadend_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\tonloadend\x01\x01\x05self_\tonloadend\0\0\0!__widl_f_set_onloadend_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\tonloadend\x01\x02\x05self_\tonloadend\tonloadend\0\0\0\x1F__widl_f_onloadstart_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Bonloadstart\x01\x01\x05self_\x0Bonloadstart\0\0\0#__widl_f_set_onloadstart_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Bonloadstart\x01\x02\x05self_\x0Bonloadstart\x0Bonloadstart\0\0\0\x1F__widl_f_onmousedown_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Bonmousedown\x01\x01\x05self_\x0Bonmousedown\0\0\0#__widl_f_set_onmousedown_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Bonmousedown\x01\x02\x05self_\x0Bonmousedown\x0Bonmousedown\0\0\0 __widl_f_onmouseenter_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Conmouseenter\x01\x01\x05self_\x0Conmouseenter\0\0\0$__widl_f_set_onmouseenter_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Conmouseenter\x01\x02\x05self_\x0Conmouseenter\x0Conmouseenter\0\0\0 __widl_f_onmouseleave_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Conmouseleave\x01\x01\x05self_\x0Conmouseleave\0\0\0$__widl_f_set_onmouseleave_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Conmouseleave\x01\x02\x05self_\x0Conmouseleave\x0Conmouseleave\0\0\0\x1F__widl_f_onmousemove_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Bonmousemove\x01\x01\x05self_\x0Bonmousemove\0\0\0#__widl_f_set_onmousemove_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Bonmousemove\x01\x02\x05self_\x0Bonmousemove\x0Bonmousemove\0\0\0\x1E__widl_f_onmouseout_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\nonmouseout\x01\x01\x05self_\nonmouseout\0\0\0\"__widl_f_set_onmouseout_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\nonmouseout\x01\x02\x05self_\nonmouseout\nonmouseout\0\0\0\x1F__widl_f_onmouseover_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Bonmouseover\x01\x01\x05self_\x0Bonmouseover\0\0\0#__widl_f_set_onmouseover_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Bonmouseover\x01\x02\x05self_\x0Bonmouseover\x0Bonmouseover\0\0\0\x1D__widl_f_onmouseup_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\tonmouseup\x01\x01\x05self_\tonmouseup\0\0\0!__widl_f_set_onmouseup_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\tonmouseup\x01\x02\x05self_\tonmouseup\tonmouseup\0\0\0\x1B__widl_f_onwheel_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x07onwheel\x01\x01\x05self_\x07onwheel\0\0\0\x1F__widl_f_set_onwheel_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x07onwheel\x01\x02\x05self_\x07onwheel\x07onwheel\0\0\0\x1B__widl_f_onpause_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x07onpause\x01\x01\x05self_\x07onpause\0\0\0\x1F__widl_f_set_onpause_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x07onpause\x01\x02\x05self_\x07onpause\x07onpause\0\0\0\x1A__widl_f_onplay_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x06onplay\x01\x01\x05self_\x06onplay\0\0\0\x1E__widl_f_set_onplay_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x06onplay\x01\x02\x05self_\x06onplay\x06onplay\0\0\0\x1D__widl_f_onplaying_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\tonplaying\x01\x01\x05self_\tonplaying\0\0\0!__widl_f_set_onplaying_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\tonplaying\x01\x02\x05self_\tonplaying\tonplaying\0\0\0\x1E__widl_f_onprogress_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\nonprogress\x01\x01\x05self_\nonprogress\0\0\0\"__widl_f_set_onprogress_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\nonprogress\x01\x02\x05self_\nonprogress\nonprogress\0\0\0 __widl_f_onratechange_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Conratechange\x01\x01\x05self_\x0Conratechange\0\0\0$__widl_f_set_onratechange_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Conratechange\x01\x02\x05self_\x0Conratechange\x0Conratechange\0\0\0\x1B__widl_f_onreset_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x07onreset\x01\x01\x05self_\x07onreset\0\0\0\x1F__widl_f_set_onreset_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x07onreset\x01\x02\x05self_\x07onreset\x07onreset\0\0\0\x1C__widl_f_onresize_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x08onresize\x01\x01\x05self_\x08onresize\0\0\0 __widl_f_set_onresize_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x08onresize\x01\x02\x05self_\x08onresize\x08onresize\0\0\0\x1C__widl_f_onscroll_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x08onscroll\x01\x01\x05self_\x08onscroll\0\0\0 __widl_f_set_onscroll_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x08onscroll\x01\x02\x05self_\x08onscroll\x08onscroll\0\0\0\x1C__widl_f_onseeked_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x08onseeked\x01\x01\x05self_\x08onseeked\0\0\0 __widl_f_set_onseeked_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x08onseeked\x01\x02\x05self_\x08onseeked\x08onseeked\0\0\0\x1D__widl_f_onseeking_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\tonseeking\x01\x01\x05self_\tonseeking\0\0\0!__widl_f_set_onseeking_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\tonseeking\x01\x02\x05self_\tonseeking\tonseeking\0\0\0\x1C__widl_f_onselect_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x08onselect\x01\x01\x05self_\x08onselect\0\0\0 __widl_f_set_onselect_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x08onselect\x01\x02\x05self_\x08onselect\x08onselect\0\0\0\x1A__widl_f_onshow_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x06onshow\x01\x01\x05self_\x06onshow\0\0\0\x1E__widl_f_set_onshow_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x06onshow\x01\x02\x05self_\x06onshow\x06onshow\0\0\0\x1D__widl_f_onstalled_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\tonstalled\x01\x01\x05self_\tonstalled\0\0\0!__widl_f_set_onstalled_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\tonstalled\x01\x02\x05self_\tonstalled\tonstalled\0\0\0\x1C__widl_f_onsubmit_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x08onsubmit\x01\x01\x05self_\x08onsubmit\0\0\0 __widl_f_set_onsubmit_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x08onsubmit\x01\x02\x05self_\x08onsubmit\x08onsubmit\0\0\0\x1D__widl_f_onsuspend_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\tonsuspend\x01\x01\x05self_\tonsuspend\0\0\0!__widl_f_set_onsuspend_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\tonsuspend\x01\x02\x05self_\tonsuspend\tonsuspend\0\0\0 __widl_f_ontimeupdate_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Contimeupdate\x01\x01\x05self_\x0Contimeupdate\0\0\0$__widl_f_set_ontimeupdate_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Contimeupdate\x01\x02\x05self_\x0Contimeupdate\x0Contimeupdate\0\0\0\"__widl_f_onvolumechange_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Eonvolumechange\x01\x01\x05self_\x0Eonvolumechange\0\0\0&__widl_f_set_onvolumechange_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Eonvolumechange\x01\x02\x05self_\x0Eonvolumechange\x0Eonvolumechange\0\0\0\x1D__widl_f_onwaiting_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\tonwaiting\x01\x01\x05self_\tonwaiting\0\0\0!__widl_f_set_onwaiting_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\tonwaiting\x01\x02\x05self_\tonwaiting\tonwaiting\0\0\0!__widl_f_onselectstart_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\ronselectstart\x01\x01\x05self_\ronselectstart\0\0\0%__widl_f_set_onselectstart_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\ronselectstart\x01\x02\x05self_\ronselectstart\ronselectstart\0\0\0\x1C__widl_f_ontoggle_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x08ontoggle\x01\x01\x05self_\x08ontoggle\0\0\0 __widl_f_set_ontoggle_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x08ontoggle\x01\x02\x05self_\x08ontoggle\x08ontoggle\0\0\0#__widl_f_onpointercancel_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Fonpointercancel\x01\x01\x05self_\x0Fonpointercancel\0\0\0'__widl_f_set_onpointercancel_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Fonpointercancel\x01\x02\x05self_\x0Fonpointercancel\x0Fonpointercancel\0\0\0!__widl_f_onpointerdown_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\ronpointerdown\x01\x01\x05self_\ronpointerdown\0\0\0%__widl_f_set_onpointerdown_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\ronpointerdown\x01\x02\x05self_\ronpointerdown\ronpointerdown\0\0\0\x1F__widl_f_onpointerup_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Bonpointerup\x01\x01\x05self_\x0Bonpointerup\0\0\0#__widl_f_set_onpointerup_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Bonpointerup\x01\x02\x05self_\x0Bonpointerup\x0Bonpointerup\0\0\0!__widl_f_onpointermove_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\ronpointermove\x01\x01\x05self_\ronpointermove\0\0\0%__widl_f_set_onpointermove_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\ronpointermove\x01\x02\x05self_\ronpointermove\ronpointermove\0\0\0 __widl_f_onpointerout_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Conpointerout\x01\x01\x05self_\x0Conpointerout\0\0\0$__widl_f_set_onpointerout_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Conpointerout\x01\x02\x05self_\x0Conpointerout\x0Conpointerout\0\0\0!__widl_f_onpointerover_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\ronpointerover\x01\x01\x05self_\ronpointerover\0\0\0%__widl_f_set_onpointerover_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\ronpointerover\x01\x02\x05self_\ronpointerover\ronpointerover\0\0\0\"__widl_f_onpointerenter_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Eonpointerenter\x01\x01\x05self_\x0Eonpointerenter\0\0\0&__widl_f_set_onpointerenter_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Eonpointerenter\x01\x02\x05self_\x0Eonpointerenter\x0Eonpointerenter\0\0\0\"__widl_f_onpointerleave_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Eonpointerleave\x01\x01\x05self_\x0Eonpointerleave\0\0\0&__widl_f_set_onpointerleave_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Eonpointerleave\x01\x02\x05self_\x0Eonpointerleave\x0Eonpointerleave\0\0\0'__widl_f_ongotpointercapture_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x13ongotpointercapture\x01\x01\x05self_\x13ongotpointercapture\0\0\0+__widl_f_set_ongotpointercapture_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x13ongotpointercapture\x01\x02\x05self_\x13ongotpointercapture\x13ongotpointercapture\0\0\0(__widl_f_onlostpointercapture_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x14onlostpointercapture\x01\x01\x05self_\x14onlostpointercapture\0\0\0,__widl_f_set_onlostpointercapture_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x14onlostpointercapture\x01\x02\x05self_\x14onlostpointercapture\x14onlostpointercapture\0\0\0%__widl_f_onanimationcancel_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x11onanimationcancel\x01\x01\x05self_\x11onanimationcancel\0\0\0)__widl_f_set_onanimationcancel_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x11onanimationcancel\x01\x02\x05self_\x11onanimationcancel\x11onanimationcancel\0\0\0\"__widl_f_onanimationend_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Eonanimationend\x01\x01\x05self_\x0Eonanimationend\0\0\0&__widl_f_set_onanimationend_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Eonanimationend\x01\x02\x05self_\x0Eonanimationend\x0Eonanimationend\0\0\0(__widl_f_onanimationiteration_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x14onanimationiteration\x01\x01\x05self_\x14onanimationiteration\0\0\0,__widl_f_set_onanimationiteration_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x14onanimationiteration\x01\x02\x05self_\x14onanimationiteration\x14onanimationiteration\0\0\0$__widl_f_onanimationstart_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x10onanimationstart\x01\x01\x05self_\x10onanimationstart\0\0\0(__widl_f_set_onanimationstart_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x10onanimationstart\x01\x02\x05self_\x10onanimationstart\x10onanimationstart\0\0\0&__widl_f_ontransitioncancel_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x12ontransitioncancel\x01\x01\x05self_\x12ontransitioncancel\0\0\0*__widl_f_set_ontransitioncancel_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x12ontransitioncancel\x01\x02\x05self_\x12ontransitioncancel\x12ontransitioncancel\0\0\0#__widl_f_ontransitionend_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Fontransitionend\x01\x01\x05self_\x0Fontransitionend\0\0\0'__widl_f_set_ontransitionend_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Fontransitionend\x01\x02\x05self_\x0Fontransitionend\x0Fontransitionend\0\0\0#__widl_f_ontransitionrun_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Fontransitionrun\x01\x01\x05self_\x0Fontransitionrun\0\0\0'__widl_f_set_ontransitionrun_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Fontransitionrun\x01\x02\x05self_\x0Fontransitionrun\x0Fontransitionrun\0\0\0%__widl_f_ontransitionstart_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x11ontransitionstart\x01\x01\x05self_\x11ontransitionstart\0\0\0)__widl_f_set_ontransitionstart_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x11ontransitionstart\x01\x02\x05self_\x11ontransitionstart\x11ontransitionstart\0\0\0(__widl_f_onwebkitanimationend_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x14onwebkitanimationend\x01\x01\x05self_\x14onwebkitanimationend\0\0\0,__widl_f_set_onwebkitanimationend_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x14onwebkitanimationend\x01\x02\x05self_\x14onwebkitanimationend\x14onwebkitanimationend\0\0\0.__widl_f_onwebkitanimationiteration_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x1Aonwebkitanimationiteration\x01\x01\x05self_\x1Aonwebkitanimationiteration\0\0\02__widl_f_set_onwebkitanimationiteration_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x1Aonwebkitanimationiteration\x01\x02\x05self_\x1Aonwebkitanimationiteration\x1Aonwebkitanimationiteration\0\0\0*__widl_f_onwebkitanimationstart_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x16onwebkitanimationstart\x01\x01\x05self_\x16onwebkitanimationstart\0\0\0.__widl_f_set_onwebkitanimationstart_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x16onwebkitanimationstart\x01\x02\x05self_\x16onwebkitanimationstart\x16onwebkitanimationstart\0\0\0)__widl_f_onwebkittransitionend_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x15onwebkittransitionend\x01\x01\x05self_\x15onwebkittransitionend\0\0\0-__widl_f_set_onwebkittransitionend_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x15onwebkittransitionend\x01\x02\x05self_\x15onwebkittransitionend\x15onwebkittransitionend\0\0\0\x1B__widl_f_onerror_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0\x1F__widl_f_set_onerror_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0 __widl_f_ontouchstart_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Contouchstart\x01\x01\x05self_\x0Contouchstart\0\0\0$__widl_f_set_ontouchstart_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Contouchstart\x01\x02\x05self_\x0Contouchstart\x0Contouchstart\0\0\0\x1E__widl_f_ontouchend_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\nontouchend\x01\x01\x05self_\nontouchend\0\0\0\"__widl_f_set_ontouchend_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\nontouchend\x01\x02\x05self_\nontouchend\nontouchend\0\0\0\x1F__widl_f_ontouchmove_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\x0Bontouchmove\x01\x01\x05self_\x0Bontouchmove\0\0\0#__widl_f_set_ontouchmove_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\x0Bontouchmove\x01\x02\x05self_\x0Bontouchmove\x0Bontouchmove\0\0\0!__widl_f_ontouchcancel_SVGElement\0\0\0\x01\nSVGElement\x01\0\x01\rontouchcancel\x01\x01\x05self_\rontouchcancel\0\0\0%__widl_f_set_ontouchcancel_SVGElement\0\0\0\x01\nSVGElement\x01\0\x02\rontouchcancel\x01\x02\x05self_\rontouchcancel\rontouchcancel\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
