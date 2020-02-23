use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGSymbolElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement)\n\n*This API requires the following crate features to be activated: `SvgSymbolElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgSymbolElement {
    obj: SvgElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgSymbolElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgSymbolElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(83u32);
            inform(121u32);
            inform(109u32);
            inform(98u32);
            inform(111u32);
            inform(108u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgSymbolElement {
        type Target = SvgElement;
        #[inline]
        fn deref(&self) -> &SvgElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgSymbolElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgSymbolElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgSymbolElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgSymbolElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgSymbolElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgSymbolElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgSymbolElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgSymbolElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgSymbolElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgSymbolElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgSymbolElement {
        #[inline]
        fn from(obj: JsValue) -> SvgSymbolElement {
            SvgSymbolElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgSymbolElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgSymbolElement> for SvgSymbolElement {
        #[inline]
        fn as_ref(&self) -> &SvgSymbolElement {
            self
        }
    }
    impl From<SvgSymbolElement> for JsValue {
        #[inline]
        fn from(obj: SvgSymbolElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgSymbolElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGSymbolElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGSymbolElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGSymbolElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgSymbolElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgSymbolElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgSymbolElement> for SvgElement {
    #[inline]
    fn from(obj: SvgSymbolElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgSymbolElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgSymbolElement> for Element {
    #[inline]
    fn from(obj: SvgSymbolElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgSymbolElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgSymbolElement> for Node {
    #[inline]
    fn from(obj: SvgSymbolElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgSymbolElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgSymbolElement> for EventTarget {
    #[inline]
    fn from(obj: SvgSymbolElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgSymbolElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgSymbolElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgSymbolElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgSymbolElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAnimatedRect", feature = "SvgSymbolElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_view_box_SVGSymbolElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgSymbolElement as WasmDescribe>::describe();
    <SvgAnimatedRect as WasmDescribe>::describe();
}
impl SvgSymbolElement {
    #[cfg(all(feature = "SvgAnimatedRect", feature = "SvgSymbolElement",))]
    #[allow(bad_style)]
    #[doc = "The `viewBox` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/viewBox)\n\n*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgSymbolElement`*"]
    #[allow(clippy::all)]
    pub fn view_box(&self) -> SvgAnimatedRect {
        #[cfg(all(feature = "SvgAnimatedRect", feature = "SvgSymbolElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_view_box_SVGSymbolElement(
                self_: <&SvgSymbolElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedRect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_view_box_SVGSymbolElement(
            self_: <&SvgSymbolElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedRect as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgSymbolElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_view_box_SVGSymbolElement(self_)
            };
            <SvgAnimatedRect as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedPreserveAspectRatio",
    feature = "SvgSymbolElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_preserve_aspect_ratio_SVGSymbolElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgSymbolElement as WasmDescribe>::describe();
    <SvgAnimatedPreserveAspectRatio as WasmDescribe>::describe();
}
impl SvgSymbolElement {
    #[cfg(all(
        feature = "SvgAnimatedPreserveAspectRatio",
        feature = "SvgSymbolElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `preserveAspectRatio` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/preserveAspectRatio)\n\n*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgSymbolElement`*"]
    #[allow(clippy::all)]
    pub fn preserve_aspect_ratio(&self) -> SvgAnimatedPreserveAspectRatio {
        #[cfg(all(
            feature = "SvgAnimatedPreserveAspectRatio",
            feature = "SvgSymbolElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_preserve_aspect_ratio_SVGSymbolElement(
                self_: <&SvgSymbolElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedPreserveAspectRatio as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_preserve_aspect_ratio_SVGSymbolElement(
            self_: <&SvgSymbolElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedPreserveAspectRatio as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgSymbolElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_preserve_aspect_ratio_SVGSymbolElement(self_)
            };
            <SvgAnimatedPreserveAspectRatio as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgSymbolElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_extension_SVGSymbolElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgSymbolElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl SvgSymbolElement {
    #[cfg(all(feature = "SvgSymbolElement",))]
    #[allow(bad_style)]
    #[doc = "The `hasExtension()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/hasExtension)\n\n*This API requires the following crate features to be activated: `SvgSymbolElement`*"]
    #[allow(clippy::all)]
    pub fn has_extension(&self, extension: &str) -> bool {
        #[cfg(all(feature = "SvgSymbolElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_extension_SVGSymbolElement(
                self_: <&SvgSymbolElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                extension: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_extension_SVGSymbolElement(
            self_: <&SvgSymbolElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgSymbolElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let extension = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(extension);
                __widl_f_has_extension_SVGSymbolElement(self_, extension)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgStringList", feature = "SvgSymbolElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_required_features_SVGSymbolElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgSymbolElement as WasmDescribe>::describe();
    <SvgStringList as WasmDescribe>::describe();
}
impl SvgSymbolElement {
    #[cfg(all(feature = "SvgStringList", feature = "SvgSymbolElement",))]
    #[allow(bad_style)]
    #[doc = "The `requiredFeatures` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/requiredFeatures)\n\n*This API requires the following crate features to be activated: `SvgStringList`, `SvgSymbolElement`*"]
    #[allow(clippy::all)]
    pub fn required_features(&self) -> SvgStringList {
        #[cfg(all(feature = "SvgStringList", feature = "SvgSymbolElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_required_features_SVGSymbolElement(
                self_: <&SvgSymbolElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_required_features_SVGSymbolElement(
            self_: <&SvgSymbolElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgSymbolElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_required_features_SVGSymbolElement(self_)
            };
            <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgStringList", feature = "SvgSymbolElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_required_extensions_SVGSymbolElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgSymbolElement as WasmDescribe>::describe();
    <SvgStringList as WasmDescribe>::describe();
}
impl SvgSymbolElement {
    #[cfg(all(feature = "SvgStringList", feature = "SvgSymbolElement",))]
    #[allow(bad_style)]
    #[doc = "The `requiredExtensions` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/requiredExtensions)\n\n*This API requires the following crate features to be activated: `SvgStringList`, `SvgSymbolElement`*"]
    #[allow(clippy::all)]
    pub fn required_extensions(&self) -> SvgStringList {
        #[cfg(all(feature = "SvgStringList", feature = "SvgSymbolElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_required_extensions_SVGSymbolElement(
                self_: <&SvgSymbolElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_required_extensions_SVGSymbolElement(
            self_: <&SvgSymbolElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgSymbolElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_required_extensions_SVGSymbolElement(self_)
            };
            <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgStringList", feature = "SvgSymbolElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_system_language_SVGSymbolElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgSymbolElement as WasmDescribe>::describe();
    <SvgStringList as WasmDescribe>::describe();
}
impl SvgSymbolElement {
    #[cfg(all(feature = "SvgStringList", feature = "SvgSymbolElement",))]
    #[allow(bad_style)]
    #[doc = "The `systemLanguage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/systemLanguage)\n\n*This API requires the following crate features to be activated: `SvgStringList`, `SvgSymbolElement`*"]
    #[allow(clippy::all)]
    pub fn system_language(&self) -> SvgStringList {
        #[cfg(all(feature = "SvgStringList", feature = "SvgSymbolElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_system_language_SVGSymbolElement(
                self_: <&SvgSymbolElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_system_language_SVGSymbolElement(
            self_: <&SvgSymbolElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgSymbolElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_system_language_SVGSymbolElement(self_)
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
pub static __WASM_BINDGEN_GENERATED_8b4404813222009c: [u8; 807usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xE5\x02\0\0\0\0\x07\0\0\x02\x10SVGSymbolElement\"__widl_instanceof_SVGSymbolElement\0\0\0\0\"__widl_f_view_box_SVGSymbolElement\0\0\0\x01\x10SVGSymbolElement\x01\0\x01\x07viewBox\x01\x01\x05self_\x07viewBox\0\0\0/__widl_f_preserve_aspect_ratio_SVGSymbolElement\0\0\0\x01\x10SVGSymbolElement\x01\0\x01\x13preserveAspectRatio\x01\x01\x05self_\x13preserveAspectRatio\0\0\0'__widl_f_has_extension_SVGSymbolElement\0\0\0\x01\x10SVGSymbolElement\x01\0\0\x01\x02\x05self_\textension\x0ChasExtension\0\0\0+__widl_f_required_features_SVGSymbolElement\0\0\0\x01\x10SVGSymbolElement\x01\0\x01\x10requiredFeatures\x01\x01\x05self_\x10requiredFeatures\0\0\0-__widl_f_required_extensions_SVGSymbolElement\0\0\0\x01\x10SVGSymbolElement\x01\0\x01\x12requiredExtensions\x01\x01\x05self_\x12requiredExtensions\0\0\0)__widl_f_system_language_SVGSymbolElement\0\0\0\x01\x10SVGSymbolElement\x01\0\x01\x0EsystemLanguage\x01\x01\x05self_\x0EsystemLanguage\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
