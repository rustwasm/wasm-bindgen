use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLFontElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement)\n\n*This API requires the following crate features to be activated: `HtmlFontElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlFontElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlFontElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlFontElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(70u32);
            inform(111u32);
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
    impl core::ops::Deref for HtmlFontElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlFontElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlFontElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlFontElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlFontElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlFontElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlFontElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlFontElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlFontElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlFontElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlFontElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlFontElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlFontElement {
            HtmlFontElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlFontElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlFontElement> for HtmlFontElement {
        #[inline]
        fn as_ref(&self) -> &HtmlFontElement {
            self
        }
    }
    impl From<HtmlFontElement> for JsValue {
        #[inline]
        fn from(obj: HtmlFontElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlFontElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLFontElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLFontElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLFontElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlFontElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlFontElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlFontElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlFontElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlFontElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlFontElement> for Element {
    #[inline]
    fn from(obj: HtmlFontElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlFontElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlFontElement> for Node {
    #[inline]
    fn from(obj: HtmlFontElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlFontElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlFontElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlFontElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlFontElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlFontElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlFontElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlFontElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlFontElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_color_HTMLFontElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFontElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFontElement {
    #[cfg(all(feature = "HtmlFontElement",))]
    #[allow(bad_style)]
    #[doc = "The `color` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/color)\n\n*This API requires the following crate features to be activated: `HtmlFontElement`*"]
    #[allow(clippy::all)]
    pub fn color(&self) -> String {
        #[cfg(all(feature = "HtmlFontElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_color_HTMLFontElement(
                self_: <&HtmlFontElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_color_HTMLFontElement(
            self_: <&HtmlFontElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFontElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_color_HTMLFontElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFontElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_color_HTMLFontElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFontElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFontElement {
    #[cfg(all(feature = "HtmlFontElement",))]
    #[allow(bad_style)]
    #[doc = "The `color` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/color)\n\n*This API requires the following crate features to be activated: `HtmlFontElement`*"]
    #[allow(clippy::all)]
    pub fn set_color(&self, color: &str) {
        #[cfg(all(feature = "HtmlFontElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_color_HTMLFontElement(
                self_: <&HtmlFontElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_color_HTMLFontElement(
            self_: <&HtmlFontElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(color);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFontElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let color = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(color);
                __widl_f_set_color_HTMLFontElement(self_, color)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFontElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_face_HTMLFontElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFontElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFontElement {
    #[cfg(all(feature = "HtmlFontElement",))]
    #[allow(bad_style)]
    #[doc = "The `face` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/face)\n\n*This API requires the following crate features to be activated: `HtmlFontElement`*"]
    #[allow(clippy::all)]
    pub fn face(&self) -> String {
        #[cfg(all(feature = "HtmlFontElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_face_HTMLFontElement(
                self_: <&HtmlFontElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_face_HTMLFontElement(
            self_: <&HtmlFontElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFontElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_face_HTMLFontElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFontElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_face_HTMLFontElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFontElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFontElement {
    #[cfg(all(feature = "HtmlFontElement",))]
    #[allow(bad_style)]
    #[doc = "The `face` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/face)\n\n*This API requires the following crate features to be activated: `HtmlFontElement`*"]
    #[allow(clippy::all)]
    pub fn set_face(&self, face: &str) {
        #[cfg(all(feature = "HtmlFontElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_face_HTMLFontElement(
                self_: <&HtmlFontElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                face: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_face_HTMLFontElement(
            self_: <&HtmlFontElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            face: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(face);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFontElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let face = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(face);
                __widl_f_set_face_HTMLFontElement(self_, face)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFontElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_size_HTMLFontElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFontElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFontElement {
    #[cfg(all(feature = "HtmlFontElement",))]
    #[allow(bad_style)]
    #[doc = "The `size` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/size)\n\n*This API requires the following crate features to be activated: `HtmlFontElement`*"]
    #[allow(clippy::all)]
    pub fn size(&self) -> String {
        #[cfg(all(feature = "HtmlFontElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_size_HTMLFontElement(
                self_: <&HtmlFontElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_size_HTMLFontElement(
            self_: <&HtmlFontElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFontElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_size_HTMLFontElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFontElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_size_HTMLFontElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFontElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFontElement {
    #[cfg(all(feature = "HtmlFontElement",))]
    #[allow(bad_style)]
    #[doc = "The `size` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/size)\n\n*This API requires the following crate features to be activated: `HtmlFontElement`*"]
    #[allow(clippy::all)]
    pub fn set_size(&self, size: &str) {
        #[cfg(all(feature = "HtmlFontElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_size_HTMLFontElement(
                self_: <&HtmlFontElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                size: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_size_HTMLFontElement(
            self_: <&HtmlFontElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            size: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(size);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFontElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let size = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(size);
                __widl_f_set_size_HTMLFontElement(self_, size)
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
pub static __WASM_BINDGEN_GENERATED_3ff79c5d8c8487ec: [u8; 637usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"};\x02\0\0\0\0\x07\0\0\x02\x0FHTMLFontElement!__widl_instanceof_HTMLFontElement\0\0\0\0\x1E__widl_f_color_HTMLFontElement\0\0\0\x01\x0FHTMLFontElement\x01\0\x01\x05color\x01\x01\x05self_\x05color\0\0\0\"__widl_f_set_color_HTMLFontElement\0\0\0\x01\x0FHTMLFontElement\x01\0\x02\x05color\x01\x02\x05self_\x05color\x05color\0\0\0\x1D__widl_f_face_HTMLFontElement\0\0\0\x01\x0FHTMLFontElement\x01\0\x01\x04face\x01\x01\x05self_\x04face\0\0\0!__widl_f_set_face_HTMLFontElement\0\0\0\x01\x0FHTMLFontElement\x01\0\x02\x04face\x01\x02\x05self_\x04face\x04face\0\0\0\x1D__widl_f_size_HTMLFontElement\0\0\0\x01\x0FHTMLFontElement\x01\0\x01\x04size\x01\x01\x05self_\x04size\0\0\0!__widl_f_set_size_HTMLFontElement\0\0\0\x01\x0FHTMLFontElement\x01\0\x02\x04size\x01\x02\x05self_\x04size\x04size\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
