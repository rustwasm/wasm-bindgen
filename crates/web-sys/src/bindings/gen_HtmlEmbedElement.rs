use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLEmbedElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement)\n\n*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlEmbedElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlEmbedElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlEmbedElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(69u32);
            inform(109u32);
            inform(98u32);
            inform(101u32);
            inform(100u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlEmbedElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlEmbedElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlEmbedElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlEmbedElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlEmbedElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlEmbedElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlEmbedElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlEmbedElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlEmbedElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlEmbedElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlEmbedElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlEmbedElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlEmbedElement {
            HtmlEmbedElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlEmbedElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlEmbedElement> for HtmlEmbedElement {
        #[inline]
        fn as_ref(&self) -> &HtmlEmbedElement {
            self
        }
    }
    impl From<HtmlEmbedElement> for JsValue {
        #[inline]
        fn from(obj: HtmlEmbedElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlEmbedElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLEmbedElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLEmbedElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLEmbedElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlEmbedElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlEmbedElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlEmbedElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlEmbedElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlEmbedElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlEmbedElement> for Element {
    #[inline]
    fn from(obj: HtmlEmbedElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlEmbedElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlEmbedElement> for Node {
    #[inline]
    fn from(obj: HtmlEmbedElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlEmbedElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlEmbedElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlEmbedElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlEmbedElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlEmbedElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlEmbedElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlEmbedElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Document", feature = "HtmlEmbedElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_svg_document_HTMLEmbedElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlEmbedElement as WasmDescribe>::describe();
    <Option<Document> as WasmDescribe>::describe();
}
impl HtmlEmbedElement {
    #[cfg(all(feature = "Document", feature = "HtmlEmbedElement",))]
    #[allow(bad_style)]
    #[doc = "The `getSVGDocument()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/getSVGDocument)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlEmbedElement`*"]
    #[allow(clippy::all)]
    pub fn get_svg_document(&self) -> Option<Document> {
        #[cfg(all(feature = "Document", feature = "HtmlEmbedElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_svg_document_HTMLEmbedElement(
                self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Document> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_svg_document_HTMLEmbedElement(
            self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Document> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_svg_document_HTMLEmbedElement(self_)
            };
            <Option<Document> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlEmbedElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_src_HTMLEmbedElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlEmbedElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlEmbedElement {
    #[cfg(all(feature = "HtmlEmbedElement",))]
    #[allow(bad_style)]
    #[doc = "The `src` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/src)\n\n*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    #[allow(clippy::all)]
    pub fn src(&self) -> String {
        #[cfg(all(feature = "HtmlEmbedElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_src_HTMLEmbedElement(
                self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_src_HTMLEmbedElement(
            self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_src_HTMLEmbedElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlEmbedElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_src_HTMLEmbedElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlEmbedElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlEmbedElement {
    #[cfg(all(feature = "HtmlEmbedElement",))]
    #[allow(bad_style)]
    #[doc = "The `src` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/src)\n\n*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    #[allow(clippy::all)]
    pub fn set_src(&self, src: &str) {
        #[cfg(all(feature = "HtmlEmbedElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_src_HTMLEmbedElement(
                self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                src: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_src_HTMLEmbedElement(
            self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            src: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(src);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let src = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(src);
                __widl_f_set_src_HTMLEmbedElement(self_, src)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlEmbedElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_HTMLEmbedElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlEmbedElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlEmbedElement {
    #[cfg(all(feature = "HtmlEmbedElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/type)\n\n*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "HtmlEmbedElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_HTMLEmbedElement(
                self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_HTMLEmbedElement(
            self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_HTMLEmbedElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlEmbedElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_type_HTMLEmbedElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlEmbedElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlEmbedElement {
    #[cfg(all(feature = "HtmlEmbedElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/type)\n\n*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    #[allow(clippy::all)]
    pub fn set_type(&self, type_: &str) {
        #[cfg(all(feature = "HtmlEmbedElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_type_HTMLEmbedElement(
                self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_type_HTMLEmbedElement(
            self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_set_type_HTMLEmbedElement(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlEmbedElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_HTMLEmbedElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlEmbedElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlEmbedElement {
    #[cfg(all(feature = "HtmlEmbedElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/width)\n\n*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> String {
        #[cfg(all(feature = "HtmlEmbedElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_HTMLEmbedElement(
                self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_HTMLEmbedElement(
            self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_HTMLEmbedElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlEmbedElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_width_HTMLEmbedElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlEmbedElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlEmbedElement {
    #[cfg(all(feature = "HtmlEmbedElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/width)\n\n*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    #[allow(clippy::all)]
    pub fn set_width(&self, width: &str) {
        #[cfg(all(feature = "HtmlEmbedElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_width_HTMLEmbedElement(
                self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_width_HTMLEmbedElement(
            self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(width);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let width = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                __widl_f_set_width_HTMLEmbedElement(self_, width)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlEmbedElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_HTMLEmbedElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlEmbedElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlEmbedElement {
    #[cfg(all(feature = "HtmlEmbedElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/height)\n\n*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> String {
        #[cfg(all(feature = "HtmlEmbedElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_HTMLEmbedElement(
                self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_HTMLEmbedElement(
            self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_HTMLEmbedElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlEmbedElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_height_HTMLEmbedElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlEmbedElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlEmbedElement {
    #[cfg(all(feature = "HtmlEmbedElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/height)\n\n*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    #[allow(clippy::all)]
    pub fn set_height(&self, height: &str) {
        #[cfg(all(feature = "HtmlEmbedElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_height_HTMLEmbedElement(
                self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_height_HTMLEmbedElement(
            self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(height);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let height = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                __widl_f_set_height_HTMLEmbedElement(self_, height)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlEmbedElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_align_HTMLEmbedElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlEmbedElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlEmbedElement {
    #[cfg(all(feature = "HtmlEmbedElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/align)\n\n*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    #[allow(clippy::all)]
    pub fn align(&self) -> String {
        #[cfg(all(feature = "HtmlEmbedElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_align_HTMLEmbedElement(
                self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_align_HTMLEmbedElement(
            self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_align_HTMLEmbedElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlEmbedElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_align_HTMLEmbedElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlEmbedElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlEmbedElement {
    #[cfg(all(feature = "HtmlEmbedElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/align)\n\n*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    #[allow(clippy::all)]
    pub fn set_align(&self, align: &str) {
        #[cfg(all(feature = "HtmlEmbedElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_align_HTMLEmbedElement(
                self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_align_HTMLEmbedElement(
            self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(align);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let align = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(align);
                __widl_f_set_align_HTMLEmbedElement(self_, align)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlEmbedElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_HTMLEmbedElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlEmbedElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlEmbedElement {
    #[cfg(all(feature = "HtmlEmbedElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/name)\n\n*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "HtmlEmbedElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_HTMLEmbedElement(
                self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_HTMLEmbedElement(
            self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_HTMLEmbedElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlEmbedElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_name_HTMLEmbedElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlEmbedElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlEmbedElement {
    #[cfg(all(feature = "HtmlEmbedElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/name)\n\n*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    #[allow(clippy::all)]
    pub fn set_name(&self, name: &str) {
        #[cfg(all(feature = "HtmlEmbedElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_name_HTMLEmbedElement(
                self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_name_HTMLEmbedElement(
            self_: <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlEmbedElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_set_name_HTMLEmbedElement(self_, name)
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
pub static __WASM_BINDGEN_GENERATED_e4cafb0dc5168fdf: [u8; 1242usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x98\x04\0\0\0\0\x0E\0\0\x02\x10HTMLEmbedElement\"__widl_instanceof_HTMLEmbedElement\0\0\0\0*__widl_f_get_svg_document_HTMLEmbedElement\0\0\0\x01\x10HTMLEmbedElement\x01\0\0\x01\x01\x05self_\x0EgetSVGDocument\0\0\0\x1D__widl_f_src_HTMLEmbedElement\0\0\0\x01\x10HTMLEmbedElement\x01\0\x01\x03src\x01\x01\x05self_\x03src\0\0\0!__widl_f_set_src_HTMLEmbedElement\0\0\0\x01\x10HTMLEmbedElement\x01\0\x02\x03src\x01\x02\x05self_\x03src\x03src\0\0\0\x1E__widl_f_type_HTMLEmbedElement\0\0\0\x01\x10HTMLEmbedElement\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0\"__widl_f_set_type_HTMLEmbedElement\0\0\0\x01\x10HTMLEmbedElement\x01\0\x02\x04type\x01\x02\x05self_\x05type_\x04type\0\0\0\x1F__widl_f_width_HTMLEmbedElement\0\0\0\x01\x10HTMLEmbedElement\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0#__widl_f_set_width_HTMLEmbedElement\0\0\0\x01\x10HTMLEmbedElement\x01\0\x02\x05width\x01\x02\x05self_\x05width\x05width\0\0\0 __widl_f_height_HTMLEmbedElement\0\0\0\x01\x10HTMLEmbedElement\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0$__widl_f_set_height_HTMLEmbedElement\0\0\0\x01\x10HTMLEmbedElement\x01\0\x02\x06height\x01\x02\x05self_\x06height\x06height\0\0\0\x1F__widl_f_align_HTMLEmbedElement\0\0\0\x01\x10HTMLEmbedElement\x01\0\x01\x05align\x01\x01\x05self_\x05align\0\0\0#__widl_f_set_align_HTMLEmbedElement\0\0\0\x01\x10HTMLEmbedElement\x01\0\x02\x05align\x01\x02\x05self_\x05align\x05align\0\0\0\x1E__widl_f_name_HTMLEmbedElement\0\0\0\x01\x10HTMLEmbedElement\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0\"__widl_f_set_name_HTMLEmbedElement\0\0\0\x01\x10HTMLEmbedElement\x01\0\x02\x04name\x01\x02\x05self_\x04name\x04name\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
