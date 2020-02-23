use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGStyleElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement)\n\n*This API requires the following crate features to be activated: `SvgStyleElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgStyleElement {
    obj: SvgElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgStyleElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgStyleElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(83u32);
            inform(116u32);
            inform(121u32);
            inform(108u32);
            inform(101u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgStyleElement {
        type Target = SvgElement;
        #[inline]
        fn deref(&self) -> &SvgElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgStyleElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgStyleElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgStyleElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgStyleElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgStyleElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgStyleElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgStyleElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgStyleElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgStyleElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgStyleElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgStyleElement {
        #[inline]
        fn from(obj: JsValue) -> SvgStyleElement {
            SvgStyleElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgStyleElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgStyleElement> for SvgStyleElement {
        #[inline]
        fn as_ref(&self) -> &SvgStyleElement {
            self
        }
    }
    impl From<SvgStyleElement> for JsValue {
        #[inline]
        fn from(obj: SvgStyleElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgStyleElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGStyleElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGStyleElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGStyleElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgStyleElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgStyleElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgStyleElement> for SvgElement {
    #[inline]
    fn from(obj: SvgStyleElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgStyleElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgStyleElement> for Element {
    #[inline]
    fn from(obj: SvgStyleElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgStyleElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgStyleElement> for Node {
    #[inline]
    fn from(obj: SvgStyleElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgStyleElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgStyleElement> for EventTarget {
    #[inline]
    fn from(obj: SvgStyleElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgStyleElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgStyleElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgStyleElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgStyleElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgStyleElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_xmlspace_SVGStyleElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgStyleElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SvgStyleElement {
    #[cfg(all(feature = "SvgStyleElement",))]
    #[allow(bad_style)]
    #[doc = "The `xmlspace` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/xmlspace)\n\n*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    #[allow(clippy::all)]
    pub fn xmlspace(&self) -> String {
        #[cfg(all(feature = "SvgStyleElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_xmlspace_SVGStyleElement(
                self_: <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_xmlspace_SVGStyleElement(
            self_: <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_xmlspace_SVGStyleElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgStyleElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_xmlspace_SVGStyleElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgStyleElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgStyleElement {
    #[cfg(all(feature = "SvgStyleElement",))]
    #[allow(bad_style)]
    #[doc = "The `xmlspace` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/xmlspace)\n\n*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    #[allow(clippy::all)]
    pub fn set_xmlspace(&self, xmlspace: &str) {
        #[cfg(all(feature = "SvgStyleElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_xmlspace_SVGStyleElement(
                self_: <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                xmlspace: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_xmlspace_SVGStyleElement(
            self_: <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            xmlspace: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(xmlspace);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let xmlspace = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(xmlspace);
                __widl_f_set_xmlspace_SVGStyleElement(self_, xmlspace)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgStyleElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_SVGStyleElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgStyleElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SvgStyleElement {
    #[cfg(all(feature = "SvgStyleElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/type)\n\n*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "SvgStyleElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_SVGStyleElement(
                self_: <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_SVGStyleElement(
            self_: <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_SVGStyleElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgStyleElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_type_SVGStyleElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgStyleElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgStyleElement {
    #[cfg(all(feature = "SvgStyleElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/type)\n\n*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    #[allow(clippy::all)]
    pub fn set_type(&self, type_: &str) {
        #[cfg(all(feature = "SvgStyleElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_type_SVGStyleElement(
                self_: <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_type_SVGStyleElement(
            self_: <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_set_type_SVGStyleElement(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgStyleElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_media_SVGStyleElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgStyleElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SvgStyleElement {
    #[cfg(all(feature = "SvgStyleElement",))]
    #[allow(bad_style)]
    #[doc = "The `media` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/media)\n\n*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    #[allow(clippy::all)]
    pub fn media(&self) -> String {
        #[cfg(all(feature = "SvgStyleElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_media_SVGStyleElement(
                self_: <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_media_SVGStyleElement(
            self_: <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_media_SVGStyleElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgStyleElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_media_SVGStyleElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgStyleElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgStyleElement {
    #[cfg(all(feature = "SvgStyleElement",))]
    #[allow(bad_style)]
    #[doc = "The `media` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/media)\n\n*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    #[allow(clippy::all)]
    pub fn set_media(&self, media: &str) {
        #[cfg(all(feature = "SvgStyleElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_media_SVGStyleElement(
                self_: <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                media: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_media_SVGStyleElement(
            self_: <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            media: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(media);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let media = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(media);
                __widl_f_set_media_SVGStyleElement(self_, media)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgStyleElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_title_SVGStyleElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgStyleElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SvgStyleElement {
    #[cfg(all(feature = "SvgStyleElement",))]
    #[allow(bad_style)]
    #[doc = "The `title` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/title)\n\n*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    #[allow(clippy::all)]
    pub fn title(&self) -> String {
        #[cfg(all(feature = "SvgStyleElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_title_SVGStyleElement(
                self_: <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_title_SVGStyleElement(
            self_: <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_title_SVGStyleElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgStyleElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_title_SVGStyleElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgStyleElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgStyleElement {
    #[cfg(all(feature = "SvgStyleElement",))]
    #[allow(bad_style)]
    #[doc = "The `title` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/title)\n\n*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    #[allow(clippy::all)]
    pub fn set_title(&self, title: &str) {
        #[cfg(all(feature = "SvgStyleElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_title_SVGStyleElement(
                self_: <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_title_SVGStyleElement(
            self_: <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(title);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let title = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(title);
                __widl_f_set_title_SVGStyleElement(self_, title)
            };
            ()
        }
    }
}
#[cfg(all(feature = "StyleSheet", feature = "SvgStyleElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sheet_SVGStyleElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgStyleElement as WasmDescribe>::describe();
    <Option<StyleSheet> as WasmDescribe>::describe();
}
impl SvgStyleElement {
    #[cfg(all(feature = "StyleSheet", feature = "SvgStyleElement",))]
    #[allow(bad_style)]
    #[doc = "The `sheet` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/sheet)\n\n*This API requires the following crate features to be activated: `StyleSheet`, `SvgStyleElement`*"]
    #[allow(clippy::all)]
    pub fn sheet(&self) -> Option<StyleSheet> {
        #[cfg(all(feature = "StyleSheet", feature = "SvgStyleElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sheet_SVGStyleElement(
                self_: <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<StyleSheet> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sheet_SVGStyleElement(
            self_: <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<StyleSheet> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgStyleElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_sheet_SVGStyleElement(self_)
            };
            <Option<StyleSheet> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8379d20bf706fdae: [u8; 907usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}I\x03\0\0\0\0\n\0\0\x02\x0FSVGStyleElement!__widl_instanceof_SVGStyleElement\0\0\0\0!__widl_f_xmlspace_SVGStyleElement\0\0\0\x01\x0FSVGStyleElement\x01\0\x01\x08xmlspace\x01\x01\x05self_\x08xmlspace\0\0\0%__widl_f_set_xmlspace_SVGStyleElement\0\0\0\x01\x0FSVGStyleElement\x01\0\x02\x08xmlspace\x01\x02\x05self_\x08xmlspace\x08xmlspace\0\0\0\x1D__widl_f_type_SVGStyleElement\0\0\0\x01\x0FSVGStyleElement\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0!__widl_f_set_type_SVGStyleElement\0\0\0\x01\x0FSVGStyleElement\x01\0\x02\x04type\x01\x02\x05self_\x05type_\x04type\0\0\0\x1E__widl_f_media_SVGStyleElement\0\0\0\x01\x0FSVGStyleElement\x01\0\x01\x05media\x01\x01\x05self_\x05media\0\0\0\"__widl_f_set_media_SVGStyleElement\0\0\0\x01\x0FSVGStyleElement\x01\0\x02\x05media\x01\x02\x05self_\x05media\x05media\0\0\0\x1E__widl_f_title_SVGStyleElement\0\0\0\x01\x0FSVGStyleElement\x01\0\x01\x05title\x01\x01\x05self_\x05title\0\0\0\"__widl_f_set_title_SVGStyleElement\0\0\0\x01\x0FSVGStyleElement\x01\0\x02\x05title\x01\x02\x05self_\x05title\x05title\0\0\0\x1E__widl_f_sheet_SVGStyleElement\0\0\0\x01\x0FSVGStyleElement\x01\0\x01\x05sheet\x01\x01\x05self_\x05sheet\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
